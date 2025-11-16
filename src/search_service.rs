use crate::query_parser::CustomQueryParser;
use anyhow::Result;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;
use tantivy::{
    Index, IndexReader, TantivyDocument, collector::TopDocs,
    ReloadPolicy, DocAddress, Term
};
use tantivy::query::{Query, BooleanQuery, Occur, TermQuery};
use tantivy::schema::{Value, IndexRecordOption};
use serde_json::json;

const MAX_RESULTS: usize = 10_000;

/// Search service that keeps the index reader open for fast repeated searches
pub struct SearchService {
    index: Arc<Index>,
    reader: IndexReader,
    query_parser: CustomQueryParser,
}

impl SearchService {
    /// Create a new search service with an open index reader
    pub fn new(index_dir: &str) -> Result<Self> {
        let open_start = Instant::now();
        let index = Arc::new(Index::open_in_dir(index_dir)?);
        let schema = index.schema();

        // Use Manual reload policy - we'll reload manually if needed
        // For HTTP server, the reader stays open and segments are cached
        let reader = index.reader_builder()
            .reload_policy(ReloadPolicy::Manual)
            .try_into()?;

        let query_parser = CustomQueryParser::new(schema.clone(), (*index).clone());

        let open_time = open_start.elapsed();
        eprintln!("Index opened in {:.3}s", open_time.as_secs_f64());

        Ok(Self {
            index,
            reader,
            query_parser,
        })
    }

    /// Execute a search query and return results
    pub fn search(&self, query_str: &str) -> Result<SearchResults> {
        let search_start = Instant::now();
        let searcher = self.reader.searcher();

        // Parse query
        let parse_start = Instant::now();
        let parsed_query = self.query_parser.parse(query_str)?;
        let parse_time = parse_start.elapsed();

        // Execute search
        let execute_start = Instant::now();
        let is_mobile_search = parsed_query.clauses.len() == 1
            && parsed_query.clauses[0].field == "mobile";

        let all_doc_addresses = if is_mobile_search {
            // Mobile fan-out logic
            let mobile_value = self.query_parser.normalize_value("mobile", &parsed_query.clauses[0].value);
            self.execute_mobile_fanout(&searcher, &mobile_value)?
        } else {
            // Regular query execution
            let query = self.query_parser.build_query(&parsed_query)?;
            searcher.search(&*query, &TopDocs::with_limit(MAX_RESULTS))?
                .into_iter()
                .map(|(_score, addr)| addr)
                .collect()
        };

        let execute_time = execute_start.elapsed();
        let total_results = all_doc_addresses.len();

        // Retrieve documents
        let retrieve_start = Instant::now();
        let schema = searcher.schema();
        let mut results: Vec<TantivyDocument> = Vec::new();

        for addr in all_doc_addresses.iter().take(MAX_RESULTS) {
            let retrieved: TantivyDocument = searcher.doc(*addr)?;
            results.push(retrieved);
        }

        let retrieve_time = retrieve_start.elapsed();
        let total_time = search_start.elapsed();

        // Convert to JSON
        let json_results: Vec<serde_json::Value> = results.iter()
            .filter_map(|doc| document_to_json(doc, &schema).ok())
            .filter_map(|json_str| serde_json::from_str(&json_str).ok())
            .collect();

        Ok(SearchResults {
            results: json_results,
            total_matches: total_results,
            results_returned: results.len(),
            query_parse_time_ms: parse_time.as_secs_f64() * 1000.0,
            search_execution_time_ms: execute_time.as_secs_f64() * 1000.0,
            document_retrieval_time_ms: retrieve_time.as_secs_f64() * 1000.0,
            total_time_ms: total_time.as_secs_f64() * 1000.0,
        })
    }

    /// Execute mobile fan-out search
    fn execute_mobile_fanout(
        &self,
        searcher: &tantivy::Searcher,
        mobile_value: &str,
    ) -> Result<HashSet<DocAddress>> {
        let mut all_addresses: HashSet<DocAddress> = HashSet::new();
        let schema = self.index.schema();

        let mobile_field = schema.get_field("mobile").unwrap();
        let master_id_field = schema.get_field("master_id").unwrap();
        let alt_field = schema.get_field("alt").unwrap();

        // Step 1: Find all rows where mobile = X
        let mobile_term = Term::from_field_text(mobile_field, mobile_value);
        let mobile_query = TermQuery::new(mobile_term, IndexRecordOption::Basic);
        let mobile_docs = searcher.search(&mobile_query, &TopDocs::with_limit(MAX_RESULTS))?;

        let mut master_ids: HashSet<String> = HashSet::new();

        for (_score, addr) in &mobile_docs {
            all_addresses.insert(*addr);

            // Extract master_id
            let doc: TantivyDocument = searcher.doc(*addr)?;
            if let Some(master_id_val) = doc.get_first(master_id_field)
                .and_then(|v| Value::as_str(&v))
            {
                let master_id = master_id_val.trim();
                if !master_id.is_empty() {
                    master_ids.insert(master_id.to_string());
                }
            }
        }

        // Step 2 & 3: Find all rows with those master_id values
        if !master_ids.is_empty() {
            let mut master_id_queries: Vec<(Occur, Box<dyn Query>)> = Vec::new();
            for master_id in &master_ids {
                let master_id = master_id.trim();
                if master_id.is_empty() {
                    continue;
                }
                let master_id_term = Term::from_field_text(master_id_field, master_id);
                let master_id_query = TermQuery::new(master_id_term, IndexRecordOption::Basic);
                master_id_queries.push((Occur::Should, Box::new(master_id_query)));
            }

            if master_id_queries.len() == 1 {
                let master_id_docs = searcher.search(master_id_queries[0].1.as_ref(), &TopDocs::with_limit(MAX_RESULTS))?;
                for (_score, addr) in &master_id_docs {
                    all_addresses.insert(*addr);
                }
            } else if !master_id_queries.is_empty() {
                let master_id_bool_query = BooleanQuery::new(master_id_queries);
                let master_id_docs = searcher.search(&master_id_bool_query, &TopDocs::with_limit(MAX_RESULTS))?;
                for (_score, addr) in &master_id_docs {
                    all_addresses.insert(*addr);
                }
            }
        }

        // Step 4: Find all rows where alt = X
        if !mobile_value.trim().is_empty() {
            let alt_term = Term::from_field_text(alt_field, mobile_value);
            let alt_query = TermQuery::new(alt_term, IndexRecordOption::Basic);
            let alt_docs = searcher.search(&alt_query, &TopDocs::with_limit(MAX_RESULTS))?;
            for (_score, addr) in &alt_docs {
                all_addresses.insert(*addr);
            }
        }

        Ok(all_addresses)
    }
}

/// Search results with timing information
#[derive(Debug)]
pub struct SearchResults {
    pub results: Vec<serde_json::Value>,
    pub total_matches: usize,
    pub results_returned: usize,
    pub query_parse_time_ms: f64,
    pub search_execution_time_ms: f64,
    pub document_retrieval_time_ms: f64,
    pub total_time_ms: f64,
}

/// Convert TantivyDocument to JSON format
fn document_to_json(doc: &TantivyDocument, schema: &tantivy::schema::Schema) -> Result<String> {
    let master_id_field = schema.get_field("master_id").unwrap();
    let mobile_field = schema.get_field("mobile").unwrap();
    let alt_field = schema.get_field("alt").unwrap();
    let name_field = schema.get_field("name").unwrap();
    let fname_field = schema.get_field("fname").unwrap();
    let address_field = schema.get_field("address").unwrap();
    let email_field = schema.get_field("email").unwrap();

    let extract_values = |field: tantivy::schema::Field| -> Vec<String> {
        doc.get_all(field)
            .filter_map(|v| Value::as_str(&v).map(|s| s.to_string()))
            .collect()
    };

    let json_obj = json!({
        "master_id": extract_values(master_id_field).first().cloned().unwrap_or_default(),
        "mobile": extract_values(mobile_field).first().cloned().unwrap_or_default(),
        "alt": extract_values(alt_field).first().cloned().unwrap_or_default(),
        "name": extract_values(name_field).first().cloned().unwrap_or_default(),
        "fname": extract_values(fname_field).first().cloned().unwrap_or_default(),
        "address": extract_values(address_field).first().cloned().unwrap_or_default(),
        "email": extract_values(email_field).first().cloned().unwrap_or_default(),
    });

    Ok(serde_json::to_string(&json_obj)?)
}

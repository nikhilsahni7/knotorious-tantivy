use crate::query_parser::CustomQueryParser;
use anyhow::Result;
use std::collections::HashSet;
use std::time::Instant;
use tantivy::{
    Index, TantivyDocument, collector::TopDocs,
    ReloadPolicy, DocAddress, Term
};
use tantivy::query::{Query, BooleanQuery, Occur, TermQuery};
use tantivy::schema::{Value, IndexRecordOption};
use serde_json::json;

const MAX_RESULTS: usize = 10_000;

pub fn search(index_dir: &str, query_str: &str) -> Result<()> {
    let search_start = Instant::now();

    println!("Opening index from: {}", index_dir);
    let open_start = Instant::now();
    let index = Index::open_in_dir(index_dir)?;

    // Use the actual schema from the index (not build_schema)
    let schema = index.schema();

    // Use Manual reload policy
    let reader = index.reader_builder()
        .reload_policy(ReloadPolicy::Manual)
        .try_into()?;

    let searcher = reader.searcher();
    let open_time = open_start.elapsed();
    println!("Index opened in {:.3}s", open_time.as_secs_f64());

    let query_parser = CustomQueryParser::new(schema.clone(), index.clone());

    println!("Parsing query: {}", query_str);
    let parse_start = Instant::now();
    let parsed_query = query_parser.parse(query_str)?;
    let parse_time = parse_start.elapsed();
    println!("Query parsed in {:.3}ms", parse_time.as_secs_f64() * 1000.0);

    println!("Executing search...");
    let execute_start = Instant::now();

    // Check if this is a mobile search (needs fan-out)
    let is_mobile_search = parsed_query.clauses.len() == 1
        && parsed_query.clauses[0].field == "mobile";

    let all_doc_addresses = if is_mobile_search {
        // Mobile fan-out logic
        let mobile_value = query_parser.normalize_value("mobile", &parsed_query.clauses[0].value);
        execute_mobile_fanout(&searcher, &schema, &query_parser, &mobile_value)?
    } else {
        // Regular query execution
        let query = query_parser.build_query(&parsed_query)?;
        searcher.search(&*query, &TopDocs::with_limit(MAX_RESULTS))?
            .into_iter()
            .map(|(_score, addr)| addr)
            .collect()
    };

    let execute_time = execute_start.elapsed();
    let total_results = all_doc_addresses.len();

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Search Results:");
    println!("  Total matches: {}", total_results);
    println!("  Query parse time: {:.3}ms", parse_time.as_secs_f64() * 1000.0);
    println!("  Search execution time: {:.3}ms", execute_time.as_secs_f64() * 1000.0);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // Retrieve documents - NO DEDUPLICATION
    // Return ALL results including duplicates for maximum performance
    let retrieve_start = Instant::now();
    let mut results: Vec<TantivyDocument> = Vec::new();

    // Return all results without deduplication for maximum speed
    for addr in all_doc_addresses.iter().take(MAX_RESULTS) {
        let retrieved: TantivyDocument = searcher.doc(*addr)?;
        results.push(retrieved);
    }

    let retrieve_time = retrieve_start.elapsed();

    // Output results in JSON format
    for doc in &results {
        let json_doc = document_to_json(doc, &schema)?;
        println!("{}", json_doc);
    }

    let total_time = search_start.elapsed();
    let actual_results_count = results.len();
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Summary:");
    println!("  Total matches found: {}", total_results);
    println!("  Results returned: {}", actual_results_count);
    println!("  Document retrieval time: {:.3}ms", retrieve_time.as_secs_f64() * 1000.0);
    println!("  Total time: {:.3}ms", total_time.as_secs_f64() * 1000.0);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    Ok(())
}

/// Execute mobile fan-out search:
/// 1. Find all rows where mobile = X
/// 2. Extract master_id from those rows
/// 3. Find all rows with those master_id values
/// 4. Find all rows where alt = X
/// 5. Return union of all results
fn execute_mobile_fanout(
    searcher: &tantivy::Searcher,
    schema: &tantivy::schema::Schema,
    _query_parser: &CustomQueryParser,
    mobile_value: &str,
) -> Result<HashSet<DocAddress>> {
    let mut all_addresses: HashSet<DocAddress> = HashSet::new();

    let mobile_field = schema.get_field("mobile").unwrap();
    let master_id_field = schema.get_field("master_id").unwrap();
    let alt_field = schema.get_field("alt").unwrap();

    // Step 1: Find all rows where mobile = X
    // Use TermQuery for STRING field - fastest for exact matches
    let mobile_term = Term::from_field_text(mobile_field, mobile_value);
    let mobile_query = TermQuery::new(mobile_term, IndexRecordOption::Basic);
    let mobile_docs = searcher.search(&mobile_query, &TopDocs::with_limit(MAX_RESULTS))?;

    let mut master_ids: HashSet<String> = HashSet::new();

    for (_score, addr) in &mobile_docs {
        all_addresses.insert(*addr);

        // Extract master_id (skip empty values)
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
    // Use TermQuery for STRING field - fastest for exact matches
    if !master_ids.is_empty() {
        // Build OR query for all master_ids using TermQuery
        let mut master_id_queries: Vec<(Occur, Box<dyn Query>)> = Vec::new();
        for master_id in &master_ids {
            let master_id = master_id.trim();
            if master_id.is_empty() {
                continue;
            }
            // Use TermQuery for STRING field - much faster
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

    // Step 4: Find all rows where alt = X (only if mobile_value is not empty)
    // Use TermQuery for STRING field - fastest for exact matches
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

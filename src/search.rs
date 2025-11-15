use crate::schema::build_schema;
use anyhow::Result;
use std::time::Instant;
use tantivy::{Index, TantivyDocument, Document, collector::TopDocs, ReloadPolicy, Term};
use tantivy::query::{QueryParser, TermQuery};
use tantivy::schema::IndexRecordOption; 

pub fn search(index_dir: &str, query_str: &str) -> Result<()> {
    let search_start = Instant::now();
    
    println!("Opening index from: {}", index_dir);
    let open_start = Instant::now();
    let schema = build_schema();
    let index = Index::open_in_dir(index_dir)?;
    
    // Use ReloadPolicy::Manual for better performance (no auto-reload overhead)
    let reader = index.reader_builder()
        .reload_policy(ReloadPolicy::Manual)
        .try_into()?;
    
    let searcher = reader.searcher();
    let open_time = open_start.elapsed();
    println!("Index opened in {:.3}s", open_time.as_secs_f64());
    
    // Build field mapping for fast lookup
    let master_id_field = schema.get_field("master_id").unwrap();
    let mobile_field = schema.get_field("mobile").unwrap();
    let alt_field = schema.get_field("alt").unwrap();
    let name_field = schema.get_field("name").unwrap();
    let fname_field = schema.get_field("fname").unwrap();
    let address_field = schema.get_field("address").unwrap();
    let email_field = schema.get_field("email").unwrap();
    
    let fields = vec![
        master_id_field,
        mobile_field,
        alt_field,
        name_field,
        fname_field,
        address_field,
        email_field,
    ];
    
    // Create field name to field ID mapping for fast exact match queries
    let mut field_map: std::collections::HashMap<&str, tantivy::schema::Field> = std::collections::HashMap::new();
    field_map.insert("master_id", master_id_field);
    field_map.insert("mobile", mobile_field);
    field_map.insert("alt", alt_field);
    field_map.insert("name", name_field);
    field_map.insert("fname", fname_field);
    field_map.insert("address", address_field);
    field_map.insert("email", email_field);

    println!("Parsing query: {}", query_str);
    let parse_start = Instant::now();
    
    // Optimize for exact field:value queries (e.g., "mobile:8800244926")
    // TermQuery is faster than QueryParser for exact matches
    let query: Box<dyn tantivy::query::Query> = if let Some((field_name, value)) = query_str.split_once(':') {
        // Try to find the field in our mapping
        if let Some(&field) = field_map.get(field_name.trim()) {
            // Use TermQuery for exact match - much faster!
            let term = Term::from_field_text(field, value.trim());
            let term_query = TermQuery::new(term, IndexRecordOption::Basic);
            Box::new(term_query)
        } else {
            // Fallback to QueryParser if field not found
            let parser = QueryParser::for_index(&index, fields);
            parser.parse_query(query_str)?
        }
    } else {
        // Use QueryParser for complex queries
        let parser = QueryParser::for_index(&index, fields);
        parser.parse_query(query_str)?
    };
    
    let parse_time = parse_start.elapsed();
    println!("Query parsed in {:.3}ms", parse_time.as_secs_f64() * 1000.0);
    
    println!("Executing search...");
    let execute_start = Instant::now();
    let docs = searcher.search(&query, &TopDocs::with_limit(20))?;
    let execute_time = execute_start.elapsed();
    
    let total_results = docs.len();
    let total_search_time = search_start.elapsed();
    
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Search Results:");
    println!("  Total matches: {}", total_results);
    println!("  Query parse time: {:.3}ms", parse_time.as_secs_f64() * 1000.0);
    println!("  Search execution time: {:.3}ms", execute_time.as_secs_f64() * 1000.0);
    println!("  Total search latency: {:.3}ms", total_search_time.as_secs_f64() * 1000.0);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    let retrieve_start = Instant::now();
    for (_score, addr) in docs {
        let retrieved: TantivyDocument = searcher.doc(addr)?;
        println!("{}", retrieved.to_json(&schema));
    }
    let retrieve_time = retrieve_start.elapsed();
    
    let total_time = search_start.elapsed();
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Summary:");
    println!("  Results retrieved: {}", total_results);
    println!("  Document retrieval time: {:.3}ms", retrieve_time.as_secs_f64() * 1000.0);
    println!("  Total time (including I/O): {:.3}ms", total_time.as_secs_f64() * 1000.0);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    Ok(())
}

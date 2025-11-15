use crate::schema::build_schema;
use anyhow::Result;
use tantivy::{Index, TantivyDocument, Document, collector::TopDocs};
use tantivy::query::QueryParser; 

pub fn search(index_dir: &str, query_str: &str) -> Result<()> {
    let schema = build_schema();
    let index = Index::open_in_dir(index_dir)?;
    let reader = index.reader()?;
    let searcher = reader.searcher();

    let fields = vec![
        schema.get_field("master_id").unwrap(),
        schema.get_field("mobile").unwrap(),
        schema.get_field("alt").unwrap(),
        schema.get_field("name").unwrap(),
        schema.get_field("fname").unwrap(),
        schema.get_field("address").unwrap(),
        schema.get_field("email").unwrap(),
    ];

    let parser = QueryParser::for_index(&index, fields);
    let query = parser.parse_query(query_str)?;
    let docs = searcher.search(&query, &TopDocs::with_limit(20))?;

    for (_score, addr) in docs {
        let retrieved: TantivyDocument = searcher.doc(addr)?;
        println!("{}", retrieved.to_json(&schema));
    }

    Ok(())
}

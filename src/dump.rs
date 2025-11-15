use anyhow::Result;
use tantivy::{Index, ReloadPolicy, collector::TopDocs, TantivyDocument};
use tantivy::query::AllQuery;
use tantivy::schema::Value;
use serde_json::json;

pub fn dump_index(index_dir: &str, limit: usize) -> Result<()> {
    println!("Opening index from: {}", index_dir);
    let index = Index::open_in_dir(index_dir)?;
    let schema = index.schema();

    let reader = index.reader_builder()
        .reload_policy(ReloadPolicy::Manual)
        .try_into()?;

    let searcher = reader.searcher();

    println!("Dumping first {} records...\n", limit);

    // Get all documents
    let docs = searcher.search(&AllQuery, &TopDocs::with_limit(limit))?;

    let master_id_field = schema.get_field("master_id").unwrap();
    let mobile_field = schema.get_field("mobile").unwrap();
    let alt_field = schema.get_field("alt").unwrap();
    let name_field = schema.get_field("name").unwrap();
    let fname_field = schema.get_field("fname").unwrap();
    let address_field = schema.get_field("address").unwrap();
    let email_field = schema.get_field("email").unwrap();

    for (idx, (_score, addr)) in docs.iter().enumerate() {
        let doc: TantivyDocument = searcher.doc(*addr)?;

        let extract_first = |field| -> String {
            doc.get_first(field)
                .and_then(|v| Value::as_str(&v))
                .unwrap_or("")
                .to_string()
        };

        let json_obj = json!({
            "row": idx + 1,
            "master_id": extract_first(master_id_field),
            "mobile": extract_first(mobile_field),
            "alt": extract_first(alt_field),
            "name": extract_first(name_field),
            "fname": extract_first(fname_field),
            "address": extract_first(address_field),
            "email": extract_first(email_field),
        });

        println!("{}", serde_json::to_string(&json_obj)?);
    }

    println!("\nTotal records dumped: {}", docs.len());
    Ok(())
}

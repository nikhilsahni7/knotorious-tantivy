use crate::schema::build_schema;
use anyhow::Result;
use std::path::Path;
use tantivy::{Index, Document};
use csv::ReaderBuilder;

pub fn build_index(csv_path: &str, index_dir: &str) -> Result<()> {
    let schema = build_schema();
    let index = Index::create_in_dir(Path::new(index_dir), schema.clone())?;
    let mut writer = index.writer(400_000_000)?; // 400MB writer buffer

    let master = schema.get_field("master_id").unwrap();
    let mobile = schema.get_field("mobile").unwrap();
    let alt    = schema.get_field("alt").unwrap();
    let name   = schema.get_field("name").unwrap();
    let fname  = schema.get_field("fname").unwrap();
    let addr   = schema.get_field("address").unwrap();
    let email  = schema.get_field("email").unwrap();

    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path(csv_path)?;

    for row in rdr.records() {
        let row = row?;

        let mut doc = Document::new();
        doc.add_text(master, &row[0]);
        doc.add_text(mobile, &row[1]);
        doc.add_text(alt,    &row[2]);
        doc.add_text(name,   &row[3]);
        doc.add_text(fname,  &row[4]);
        doc.add_text(addr,   &row[5]);
        doc.add_text(email,  &row[6]);

        writer.add_document(doc)?;
    }

    writer.commit()?;
    Ok(())
}

use crate::schema::build_schema;
use anyhow::Result;
use std::path::Path;
use std::time::Instant;
use tantivy::{Index, TantivyDocument};
use csv::ReaderBuilder;

pub fn build_index(csv_path: &str, index_dir: &str) -> Result<()> {
    println!("Starting index build...");
    println!("CSV file: {}", csv_path);
    println!("Index directory: {}", index_dir);

    let start_time = Instant::now();
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

    let mut record_count = 0u64;
    let mut last_log_time = Instant::now();
    let log_interval_secs = 5.0; // Log every 5 seconds
    let log_interval_records = 100_000; // Also log every 100k records

    println!("Reading CSV and indexing documents...");

    for row in rdr.records() {
        let row = row?;

        let mut doc = TantivyDocument::default();
        doc.add_text(master, &row[0]);
        doc.add_text(mobile, &row[1]);
        doc.add_text(alt,    &row[2]);
        doc.add_text(name,   &row[3]);
        doc.add_text(fname,  &row[4]);
        doc.add_text(addr,   &row[5]);
        doc.add_text(email,  &row[6]);

        writer.add_document(doc)?;

        record_count += 1;

        // Log progress every N seconds or every N records
        let elapsed = last_log_time.elapsed().as_secs_f64();
        if elapsed >= log_interval_secs || record_count % log_interval_records == 0 {
            let total_elapsed = start_time.elapsed().as_secs_f64();
            let records_per_sec = record_count as f64 / total_elapsed;
            println!(
                "[Progress] Processed {} records | Elapsed: {:.1}s | Speed: {:.0} records/sec",
                record_count, total_elapsed, records_per_sec
            );
            last_log_time = Instant::now();
        }
    }

    println!("Committing index...");
    writer.commit()?;

    let total_elapsed = start_time.elapsed();
    let records_per_sec = record_count as f64 / total_elapsed.as_secs_f64();

    println!("✓ Indexing complete!");
    println!("  Total records indexed: {}", record_count);
    println!("  Total time: {:.2} seconds ({:.2} minutes)",
             total_elapsed.as_secs_f64(),
             total_elapsed.as_secs_f64() / 60.0);
    println!("  Average speed: {:.0} records/second", records_per_sec);

    Ok(())
}

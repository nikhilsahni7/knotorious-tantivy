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
    // Increased buffer to 1GB for faster ingestion (was 400MB)
    // Larger buffer = fewer flushes = faster indexing
    let mut writer = index.writer(1_000_000_000)?; // 1GB writer buffer

    let master = schema.get_field("master_id").unwrap();
    let mobile = schema.get_field("mobile").unwrap();
    let alt    = schema.get_field("alt").unwrap();
    let name   = schema.get_field("name").unwrap();
    let fname  = schema.get_field("fname").unwrap();
    let addr   = schema.get_field("address").unwrap();
    let email  = schema.get_field("email").unwrap();

    // Optimize CSV reading: larger buffer, no trimming overhead
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .buffer_capacity(1_048_576) // 1MB buffer for CSV reading
        .flexible(false) // Strict parsing for speed
        .from_path(csv_path)?;

    let mut record_count = 0u64;
    let mut last_log_time = Instant::now();
    let log_interval_secs = 5.0; // Log every 5 seconds
    let log_interval_records = 100_000; // Also log every 100k records

    println!("Reading CSV and indexing documents...");

    for row in rdr.records() {
        let row = row?;

        let mut doc = TantivyDocument::default();
        // CSV column order: id,mobile,fname,name,alt,email,address
        doc.add_text(master, &row[0]);  // id -> master_id
        doc.add_text(mobile, &row[1]);  // mobile -> mobile
        doc.add_text(fname,  &row[2]);  // fname -> fname
        doc.add_text(name,   &row[3]);  // name -> name
        doc.add_text(alt,    &row[4]);  // alt -> alt
        doc.add_text(email,  &row[5]);  // email -> email
        doc.add_text(addr,   &row[6]);  // address -> address

        writer.add_document(doc)?;

        record_count += 1;

        // Periodic commits for large datasets (every 10M records) to prevent memory issues
        // This also makes progress visible if process is interrupted
        // Note: After commit(), the writer can continue to be used - no need to recreate
        if record_count % 10_000_000 == 0 {
            println!("[Checkpoint] Committing at {} records...", record_count);
            writer.commit()?;
            // Writer can continue to be used after commit - no recreation needed
        }

        // Log progress every N seconds or every N records
        let elapsed = last_log_time.elapsed().as_secs_f64();
        if elapsed >= log_interval_secs || record_count % log_interval_records == 0 {
            let total_elapsed = start_time.elapsed().as_secs_f64();
            let records_per_sec = record_count as f64 / total_elapsed;
            let estimated_total_time = if records_per_sec > 0.0 {
                let estimated_records = 1_800_000_000u64; // Estimate for your dataset
                let remaining = (estimated_records.saturating_sub(record_count)) as f64 / records_per_sec;
                format!(" | ETA: {:.1} hours", remaining / 3600.0)
            } else {
                String::new()
            };
            println!(
                "[Progress] Processed {} records | Elapsed: {:.1}s | Speed: {:.0} records/sec{}",
                record_count, total_elapsed, records_per_sec, estimated_total_time
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

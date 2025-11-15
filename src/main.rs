mod schema;
mod indexer;
mod search;
mod query_parser;

use anyhow::Result;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(String::as_str) {
        Some("index") => {
            let csv = &args[2];
            let index_dir = &args[3];
            indexer::build_index(csv, index_dir)?;
        }
        Some("search") => {
            let index_dir = &args[2];
            let query = &args[3];
            search::search(index_dir, query)?;
        }
        _ => {
            println!("Usage:");
            println!("  cargo run --release index <csv> <index_dir>");
            println!("  cargo run --release search <index_dir> \"query\"");
        }
    }

    Ok(())
}

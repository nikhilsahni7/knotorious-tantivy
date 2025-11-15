mod schema;
mod indexer;
mod search;
mod query_parser;
mod dump;

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
        Some("dump") => {
            let index_dir = &args[2];
            let limit = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(1000);
            dump::dump_index(index_dir, limit)?;
        }
        _ => {
            println!("Usage:");
            println!("  cargo run --release index <csv> <index_dir>");
            println!("  cargo run --release search <index_dir> \"query\"");
            println!("  cargo run --release dump <index_dir> [limit]");
        }
    }

    Ok(())
}

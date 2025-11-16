use crate::search_service::SearchService;
use actix_web::{web, App, HttpServer, HttpResponse, Result as ActixResult};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use anyhow::anyhow;
use std::result::Result;

#[derive(Debug, Deserialize)]
pub struct SearchRequest {
    pub name: Option<String>,
    pub fname: Option<String>,
    pub address: Option<String>,
    pub mobile: Option<String>,
    pub alt: Option<String>,
    pub master_id: Option<String>,
    pub email: Option<String>,
    pub filter: Option<String>, // "AND" or "OR", default is "AND"
}

#[derive(Debug, Serialize)]
pub struct SearchResponse {
    pub results: Vec<serde_json::Value>,
    pub total_matches: usize,
    pub results_returned: usize,
    pub query_parse_time_ms: f64,
    pub search_execution_time_ms: f64,
    pub document_retrieval_time_ms: f64,
    pub total_time_ms: f64,
}

/// Convert SearchRequest to query string
fn build_query_string(req: &SearchRequest) -> Result<String, anyhow::Error> {
    let mut clauses = Vec::new();
    let filter_op = req.filter.as_deref().unwrap_or("AND").to_uppercase();

    if let Some(ref name) = req.name {
        if !name.trim().is_empty() {
            clauses.push(format!("name:{}", name.trim()));
        }
    }
    if let Some(ref fname) = req.fname {
        if !fname.trim().is_empty() {
            clauses.push(format!("fname:{}", fname.trim()));
        }
    }
    if let Some(ref address) = req.address {
        if !address.trim().is_empty() {
            clauses.push(format!("address:{}", address.trim()));
        }
    }
    if let Some(ref mobile) = req.mobile {
        if !mobile.trim().is_empty() {
            clauses.push(format!("mobile:{}", mobile.trim()));
        }
    }
    if let Some(ref alt) = req.alt {
        if !alt.trim().is_empty() {
            clauses.push(format!("alt:{}", alt.trim()));
        }
    }
    if let Some(ref master_id) = req.master_id {
        if !master_id.trim().is_empty() {
            clauses.push(format!("master_id:{}", master_id.trim()));
        }
    }
    if let Some(ref email) = req.email {
        if !email.trim().is_empty() {
            clauses.push(format!("email:{}", email.trim()));
        }
    }

    if clauses.is_empty() {
        return Err(anyhow!("No search fields provided"));
    }

    let op = if filter_op == "OR" { " OR " } else { " AND " };
    Ok(clauses.join(op))
}

/// Search endpoint handler
async fn search_handler(
    req: web::Json<SearchRequest>,
    service: web::Data<Arc<SearchService>>,
) -> ActixResult<HttpResponse> {
    // Build query string from request
    let query_str = match build_query_string(&req) {
        Ok(q) => q,
        Err(e) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": format!("Invalid request: {}", e)
            })));
        }
    };

    // Execute search
    match service.search(&query_str) {
        Ok(results) => {
            let response = SearchResponse {
                results: results.results,
                total_matches: results.total_matches,
                results_returned: results.results_returned,
                query_parse_time_ms: results.query_parse_time_ms,
                search_execution_time_ms: results.search_execution_time_ms,
                document_retrieval_time_ms: results.document_retrieval_time_ms,
                total_time_ms: results.total_time_ms,
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Search failed: {}", e)
            })))
        }
    }
}

/// Health check endpoint
async fn health_handler() -> ActixResult<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "ok"
    })))
}

/// Start the HTTP API server
pub async fn start_server(index_dir: String, host: String, port: u16) -> Result<(), std::io::Error> {
    // Initialize search service
    let service = match SearchService::new(&index_dir) {
        Ok(s) => Arc::new(s),
        Err(e) => {
            eprintln!("Failed to initialize search service: {}", e);
            std::process::exit(1);
        }
    };

    println!("Starting HTTP server on {}:{}", host, port);
    println!("Index directory: {}", index_dir);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(service.clone()))
            .route("/search", web::post().to(search_handler))
            .route("/health", web::get().to(health_handler))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}

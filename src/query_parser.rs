use anyhow::{Result, anyhow};
use std::collections::HashMap;
use tantivy::schema::{Field, Schema};
use tantivy::{Index, Term};
use tantivy::query::{Query, TermQuery, BooleanQuery, Occur};
use tantivy::schema::IndexRecordOption;

#[derive(Debug, Clone)]
pub enum QueryOp {
    And,
    Or,
}

#[derive(Debug, Clone)]
pub struct QueryClause {
    pub field: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct ParsedQuery {
    pub clauses: Vec<QueryClause>,
    pub ops: Vec<QueryOp>, // ops[i] connects clauses[i] and clauses[i+1]
}

pub struct CustomQueryParser {
    schema: Schema,
    index: Index,
    field_map: HashMap<String, Field>,
}

impl CustomQueryParser {
    pub fn new(schema: Schema, index: Index) -> Self {
        let mut field_map = HashMap::new();
        field_map.insert("master_id".to_string(), schema.get_field("master_id").unwrap());
        field_map.insert("mobile".to_string(), schema.get_field("mobile").unwrap());
        field_map.insert("alt".to_string(), schema.get_field("alt").unwrap());
        field_map.insert("name".to_string(), schema.get_field("name").unwrap());
        field_map.insert("fname".to_string(), schema.get_field("fname").unwrap());
        field_map.insert("address".to_string(), schema.get_field("address").unwrap());
        field_map.insert("email".to_string(), schema.get_field("email").unwrap());

        Self {
            schema,
            index,
            field_map,
        }
    }

    /// Parse query string into clauses and operators
    /// Supports: "field:value", "field:value AND field:value", "field:value OR field:value"
    pub fn parse(&self, query_str: &str) -> Result<ParsedQuery> {
        let query_str = query_str.trim();
        let mut clauses = Vec::new();
        let mut ops = Vec::new();

        // Handle comma-separated queries (treated as AND)
        // Also handle AND/OR operators
        let query_str = query_str.replace(',', " AND ");

        // Split by whitespace and operators
        let parts: Vec<&str> = query_str
            .split_whitespace()
            .collect();

        let mut current_clause = String::new();
        let mut current_op: Option<QueryOp> = None;

        for part in parts {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }

            // Check for AND/OR operators
            if part.eq_ignore_ascii_case("AND") {
                if !current_clause.is_empty() {
                    if let Some(clause) = self.parse_clause(&current_clause)? {
                        clauses.push(clause);
                        if let Some(op) = current_op.take() {
                            ops.push(op);
                        }
                    }
                    current_clause.clear();
                }
                current_op = Some(QueryOp::And);
                continue;
            } else if part.eq_ignore_ascii_case("OR") {
                if !current_clause.is_empty() {
                    if let Some(clause) = self.parse_clause(&current_clause)? {
                        clauses.push(clause);
                        if let Some(op) = current_op.take() {
                            ops.push(op);
                        }
                    }
                    current_clause.clear();
                }
                current_op = Some(QueryOp::Or);
                continue;
            }

            // Accumulate clause parts
            if current_clause.is_empty() {
                current_clause = part.to_string();
            } else {
                current_clause.push(' ');
                current_clause.push_str(part);
            }
        }

        // Handle last clause
        if !current_clause.is_empty() {
            if let Some(clause) = self.parse_clause(&current_clause)? {
                clauses.push(clause);
            }
        }

        // Default to AND if no operators specified
        if clauses.len() > 1 && ops.is_empty() {
            for _ in 0..clauses.len() - 1 {
                ops.push(QueryOp::And);
            }
        }

        Ok(ParsedQuery { clauses, ops })
    }

    fn parse_clause(&self, clause_str: &str) -> Result<Option<QueryClause>> {
        let clause_str = clause_str.trim();
        if clause_str.is_empty() {
            return Ok(None);
        }

        // Handle field:value format
        if let Some((field_name, value)) = clause_str.split_once(':') {
            let field_name = field_name.trim().to_lowercase();
            let value = value.trim();

            if self.field_map.contains_key(&field_name) {
                return Ok(Some(QueryClause {
                    field: field_name,
                    value: value.to_string(),
                }));
            }
        }

        // If no field specified, try to infer or use default fields
        // For now, return error for malformed queries
        Err(anyhow!("Invalid clause format: {}", clause_str))
    }

    /// Normalize value: remove spaces, convert to lowercase for mobile/alt/master_id
    pub fn normalize_value(&self, field: &str, value: &str) -> String {
        match field {
            "mobile" | "alt" | "master_id" => {
                // Remove all spaces and convert to lowercase
                value.replace(' ', "").to_lowercase()
            }
            _ => {
                // For text fields, just lowercase
                value.to_lowercase()
            }
        }
    }

    /// Build Tantivy query from parsed query
    pub fn build_query(&self, parsed: &ParsedQuery) -> Result<Box<dyn Query>> {
        if parsed.clauses.is_empty() {
            return Err(anyhow!("No query clauses"));
        }

        // Build queries for each clause
        let mut query_clauses: Vec<(Occur, Box<dyn Query>)> = Vec::new();

        for (idx, clause) in parsed.clauses.iter().enumerate() {
            let normalized_value = self.normalize_value(&clause.field, &clause.value);
            let field = self.field_map.get(&clause.field)
                .ok_or_else(|| anyhow!("Unknown field: {}", clause.field))?;

            // All fields are TEXT fields in the existing index
            // For mobile/alt/master_id: treat as exact match (single term)
            // For name/fname/address/email: handle multi-word queries
            let query: Box<dyn Query> = match clause.field.as_str() {
                "mobile" | "alt" | "master_id" => {
                    // TEXT fields but we want exact match
                    // Normalized value should match exactly (no spaces, lowercase)
                    // TEXT tokenizer will handle this correctly
                    let term = Term::from_field_text(*field, &normalized_value);
                    Box::new(TermQuery::new(term, IndexRecordOption::WithFreqsAndPositions))
                }
                "name" | "fname" | "address" | "email" => {
                    // TEXT fields - handle multi-word queries
                    // Split by whitespace and create a BooleanQuery with Must for each word
                    let words: Vec<&str> = normalized_value.split_whitespace().collect();

                    if words.len() == 1 {
                        // Single word - use TermQuery
                        let term = Term::from_field_text(*field, words[0]);
                        Box::new(TermQuery::new(term, IndexRecordOption::WithFreqsAndPositions))
                    } else {
                        // Multiple words - create BooleanQuery with Must for each word
                        // This ensures all words must be present (AND logic)
                        let mut term_queries: Vec<(Occur, Box<dyn Query>)> = Vec::new();
                        for word in words {
                            let term = Term::from_field_text(*field, word);
                            term_queries.push((
                                Occur::Must,
                                Box::new(TermQuery::new(term, IndexRecordOption::WithFreqsAndPositions)) as Box<dyn Query>
                            ));
                        }
                        Box::new(BooleanQuery::new(term_queries))
                    }
                }
                _ => {
                    return Err(anyhow!("Unsupported field: {}", clause.field));
                }
            };

            // Determine Occur based on operator
            let occur = if idx == 0 {
                Occur::Must // First clause is always Must
            } else {
                match parsed.ops.get(idx - 1) {
                    Some(QueryOp::And) => Occur::Must,
                    Some(QueryOp::Or) => Occur::Should,
                    None => Occur::Must, // Default to AND
                }
            };

            query_clauses.push((occur, query));
        }

        // Build BooleanQuery
        if query_clauses.len() == 1 {
            Ok(query_clauses.into_iter().next().unwrap().1)
        } else {
            Ok(Box::new(BooleanQuery::new(query_clauses)))
        }
    }

    /// Get field reference by name
    pub fn get_field(&self, field_name: &str) -> Option<Field> {
        self.field_map.get(field_name).copied()
    }
}

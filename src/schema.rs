use tantivy::schema::*;

pub fn build_schema() -> Schema {
    let mut schema_builder = Schema::builder();

    // STRING + FAST fields for exact matches (mobile, alt, master_id)
    // - STRING: No tokenization, exact match only (fastest for exact lookups)
    // - FAST: Enables fast field access for filtering/sorting
    // - STORED: Store original value for retrieval
    schema_builder.add_text_field("master_id", STRING | STORED | FAST);
    schema_builder.add_text_field("mobile", STRING | STORED | FAST);
    schema_builder.add_text_field("alt", STRING | STORED | FAST);

    // TEXT fields for partial/prefix matches (name, fname, address, email)
    // - TEXT: Tokenized for partial matching
    // - Default tokenizer: case-insensitive, handles partial matches
    // - STORED: Store original value for retrieval
    let text_options = TextOptions::default()
        .set_stored()
        .set_indexing_options(
            TextFieldIndexing::default()
                .set_tokenizer("default") // Case-insensitive tokenizer
                .set_index_option(IndexRecordOption::WithFreqsAndPositions)
        );

    schema_builder.add_text_field("name", text_options.clone());
    schema_builder.add_text_field("fname", text_options.clone());
    schema_builder.add_text_field("address", text_options.clone());
    schema_builder.add_text_field("email", text_options);

    schema_builder.build()
}

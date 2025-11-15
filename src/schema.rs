use tantivy::schema::*;

pub fn build_schema() -> Schema {
    let mut schema_builder = Schema::builder();

    schema_builder.add_text_field("master_id", TEXT | STORED);
    schema_builder.add_text_field("mobile", TEXT | STORED);
    schema_builder.add_text_field("alt", TEXT | STORED);
    schema_builder.add_text_field("name", TEXT | STORED);
    schema_builder.add_text_field("fname", TEXT | STORED);
    schema_builder.add_text_field("address", TEXT | STORED);
    schema_builder.add_text_field("email", TEXT | STORED);

    schema_builder.build()
}

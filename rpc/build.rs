fn main() -> std::io::Result<()> {
    let attribute = r#"
#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize, async_graphql::InputObject)]
#[serde(rename_all = "camelCase")]"#;
    tonic_build::configure()
        .type_attribute("todo.Todo", attribute)
        .type_attribute("todo.TodoUpdate", attribute)
        .type_attribute("todo.TodoAdd", attribute)
        .type_attribute("todo.QueryResult", attribute)
        .compile(&["proto/todo.proto"], &["proto"])?;
    Ok(())
}

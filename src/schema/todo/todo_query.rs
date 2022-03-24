use super::Todo;
use async_graphql::*;
pub struct QueryTodo;

#[Object]
impl QueryTodo {
    async fn todos(&self) -> Result<Vec<Todo>> {
        Ok(vec![])
    }
}

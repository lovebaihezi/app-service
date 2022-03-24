use super::Todo;
use async_graphql::*;
use rpc::{todo_rpc::todo_service_client, RPC_ADDRESS};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UpdateResult {
    update_amount: u32,
    create_amount: u32,
    delete_amount: u32,
}

scalar!(UpdateResult);

pub struct MutationTodo;

#[Object]
impl MutationTodo {
    async fn update_todos(&self, todos: Vec<Todo>) -> Result<UpdateResult> {
        let mut client =
            todo_service_client::TodoServiceClient::connect(RPC_ADDRESS).await?;
        let res = client
            .update_todo(tonic::Request::new(rpc::todo_rpc::TodoUpdate {
                todos: todos
                    .into_iter()
                    .map(|todo| rpc::todo_rpc::Todo {
                        content: todo.content,
                        start_time: todo.start_time,
                        overdue_time: todo.overdue_time,
                        is_completed: todo.is_completed,
                    })
                    .collect(),
            }))
            .await?;
        let msg = res.into_inner();
        Ok(UpdateResult {
            update_amount: msg.update_amount,
            create_amount: msg.create_amount,
            delete_amount: msg.delete_amount,
        })
    }
}

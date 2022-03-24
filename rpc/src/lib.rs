pub mod todo;
pub mod todo_rpc {
    tonic::include_proto!("todo");
}

pub const RPC_ADDRESS: &str = env!("RPC_ADDRESS");

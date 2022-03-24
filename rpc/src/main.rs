use tonic::transport::Server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let addr =  RPC_ADDRESS.parse().unwrap();
    let mut server = Server::build()
        .add_service()
        .serve(addr)
        .await?
}

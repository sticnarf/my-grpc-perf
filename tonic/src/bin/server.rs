use mimalloc::MiMalloc;
use tonic::{transport::Server, Request, Response, Status};
use tonic_demo::{
    tikv_server::{Tikv, TikvServer},
    GetRequest, GetResponse,
};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

struct TikvImpl;

#[tonic::async_trait]
impl Tikv for TikvImpl {
    async fn kv_get(&self, request: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        let key = request.into_inner().key;
        let resp = GetResponse {
            value: key,
            ..Default::default()
        };
        Ok(Response::new(resp))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let tikv = TikvImpl;

    println!("TikvServer listening on {}", addr);

    Server::builder()
        .add_service(TikvServer::new(tikv))
        .serve(addr)
        .await?;

    Ok(())
}

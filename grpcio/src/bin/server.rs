use std::{sync::Arc, thread};

use futures::FutureExt;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};
use grpcio_demo::tikvpb::{create_tikv, GetRequest, GetResponse, Tikv};
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[derive(Clone)]
struct TikvImpl;

impl Tikv for TikvImpl {
    fn kv_get(&mut self, ctx: RpcContext<'_>, req: GetRequest, sink: UnarySink<GetResponse>) {
        let key = req.key;
        let resp = GetResponse {
            value: key,
            ..Default::default()
        };
        ctx.spawn(sink.success(resp).map(|_| ()));
    }
}

fn main() {
    let env = Arc::new(Environment::new(16));
    let service = create_tikv(TikvImpl);

    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("[::1]", 50051)
        .build()
        .unwrap();
    server.start();
    loop {
        thread::park();
    }
}

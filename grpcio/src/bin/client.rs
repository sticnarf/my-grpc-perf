use std::{
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::Duration,
};

use grpcio::{ChannelBuilder, EnvBuilder};
use grpcio_demo::tikvpb::{GetRequest, TikvClient};

static COUNTER: AtomicU64 = AtomicU64::new(0);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = Arc::new(EnvBuilder::new().cq_count(16).build());
    let ch = ChannelBuilder::new(env).connect("[::1]:50051");
    let tikv = TikvClient::new(ch);

    let mut handles = Vec::new();
    for _ in 0..16 {
        let tikv = tikv.clone();
        let handle = tokio::spawn(async move {
            loop {
                let req = GetRequest {
                    key: b"key".to_vec(),
                    ..Default::default()
                };
                let resp = tikv.kv_get_async(&req).unwrap().await.unwrap();
                assert_eq!(resp.value, b"key");
                COUNTER.fetch_add(1, Ordering::Release);
            }
        });
        handles.push(handle);
    }
    handles.push(tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(1));
        interval.tick().await;
        let mut last = COUNTER.load(Ordering::Acquire);
        loop {
            interval.tick().await;
            let curr = COUNTER.load(Ordering::Acquire);
            println!("{}", curr - last);
            last = curr;
        }
    }));
    for handle in handles {
        handle.await?;
    }
    Ok(())
}

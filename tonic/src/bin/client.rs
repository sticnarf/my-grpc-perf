use std::{
    sync::atomic::{AtomicU64, Ordering},
    time::{Duration, Instant},
};

use hdrhistogram::{Histogram, SyncHistogram};
use mimalloc::MiMalloc;
use tonic_demo::{tikv_client::TikvClient, GetRequest};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

static COUNTER: AtomicU64 = AtomicU64::new(0);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tikv = TikvClient::connect("http://[::1]:50051").await?;
    let mut hist: Histogram<_> = Histogram::<u64>::new_with_max(1_000_000, 3).unwrap().into();

    let mut handles = Vec::new();
    for _ in 0..16 {
        let mut tikv = tikv.clone();
        let mut hist = hist.clone();
        let handle = tokio::spawn(async move {
            loop {
                let req = GetRequest {
                    key: b"key".to_vec(),
                    ..Default::default()
                };
                let begin = Instant::now();
                let resp = tikv.kv_get(req).await.unwrap();
                hist.record(begin.elapsed().as_micros() as u64).unwrap();
                assert_eq!(resp.into_inner().value, b"key");
                if COUNTER.fetch_add(1, Ordering::Release) > 1000000 {
                    break;
                }
            }
            hist
        });
        handles.push(handle);
    }
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(1));
        interval.tick().await;
        let mut last = COUNTER.load(Ordering::Acquire);
        while last <= 1000000 {
            interval.tick().await;
            let curr = COUNTER.load(Ordering::Acquire);
            println!("{}", curr - last);
            last = curr;
        }
    });
    for handle in handles {
        hist += handle.await?;
    }
    println!(
        "mean: {}, 99th: {}",
        hist.mean(),
        hist.value_at_quantile(0.99)
    );
    Ok(())
}

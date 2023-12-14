use tokio::{
    self, io,
    net::TcpListener,
    sync::{mpsc, oneshot},
};

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let (tx, mut rx) = mpsc::channel(4096);
    let (shutdown_tx, shutdown_rx) = oneshot::channel();
    tokio::spawn(async move {
        let mut counter: i64 = 0;
        while let Some(message) = rx.recv().await {
            counter += 1;
        }
        println!("done: {}", counter);
        shutdown_tx.send(()).unwrap();
    });
    for x in 0u64..999999 {
        let tx = tx.clone();
        tokio::spawn(async move {
            // tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            tx.send(x).await.unwrap();
        });
    }
    drop(tx);
    shutdown_rx.await.unwrap();
    // let listener = TcpListener::bind("127.0.0.1:6666").await?;
    // tokio::task::spawn_blocking(|| loop {
    //     thread::sleep(tokio::time::Duration::from_secs(1));
    //     println!("1s in blocking");
    // });
    // loop {
    //     let (conn, ip) = listener.accept().await?;
    //     tokio::spawn(async move {
    //         println!("{} connected", ip);
    //         tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    //         println!("{} 1s reached", ip);
    //     });
    // }
    return Ok(());
}

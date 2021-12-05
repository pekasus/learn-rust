use futures::{stream, StreamExt}; // 0.3.5
use reqwest::Client; // 0.10.6
use tokio; // 0.2.21, features = ["macros"]
use std::collections::VecDeque;
use std::sync::Mutex;

const CONCURRENT_REQUESTS: usize = 2;

#[tokio::main]
async fn main() -> Result<(), ()> {//Box<dyn Error>> {


    let client = Client::new();
    // let tank: Mutex<VecDeque<u32>> = Mutex::new(VecDeque::with_capacity(10));
    let mut tank: VecDeque<u32> = VecDeque::with_capacity(10);
    tank.push_back(2);
    tank.push_back(3);

    println!("{:?}", tank);

    Ok(())
}

/*
    let client = Client::new();

    let urls = vec!["https://api.ipify.org", "https://cnn.com"];

    let bodies = stream::iter(urls)
        .map(|url| {
            let client = &client;
            async move {
                let resp = client.get(url).send().await?;
                resp.bytes().await
            }
        })
        .buffer_unordered(CONCURRENT_REQUESTS);

    bodies
        .for_each(|b| async {
            match b {
                Ok(b) => println!("Got {} bytes", b.len()),
                Err(e) => eprintln!("Got an error: {}", e),
            }
        })
        .await;

*/
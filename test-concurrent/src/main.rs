use futures::{stream, StreamExt}; // 0.3.5
use reqwest::Client; // 0.10.6
use tokio; // 0.2.21, features = ["macros"]
use std::collections::VecDeque;
use std::sync::Mutex;

use std::any::type_name;

const CONCURRENT_REQUESTS: usize = 2;

// #[derive(Debug)]
// struct Tank {
//     numbers: VecDeque<u32>, // add Mutex
//     len: u32,
// }

// impl Tank {
//     fn new() -> Self {
//         Tank {
//             numbers: VecDeque::new(), // add Mutex
//             len: 0,
//         }
//     }

//     fn push_back(self, x:u32) {
//         self.numbers.push_back(x);
//     }
// }

#[tokio::main]
async fn main() -> Result<(), ()> {  // -> Result<(), Box<dyn Error>> {

    // let client = Client::new();

    // let mut tank = Tank::new(); //Mutex::new(VecDeque::new());
    let mut tank = Mutex::new(VecDeque::new());
    // let mut tank: VecDeque<u32> = VecDeque::with_capacity(10);
    
    {
        let mut tank = tank.lock().unwrap();
        tank.push_back(3);
        // tank.push_back(2);
    // tank.push_back(3);
    }



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
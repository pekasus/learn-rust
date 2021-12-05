use futures::{stream, StreamExt}; 
use reqwest::Client;
use tokio;
// use crate::tokio::io::Error;
use std::io::Error;
use std::collections::VecDeque;
use std::sync::Mutex;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

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

#[derive(Deserialize, Debug)]
struct Json {
    #[serde(rename = "type")]
    type_: String,
    length: u32,
    data: Vec<u32>,
    success: bool,
}


#[tokio::main]
async fn main()  -> Result<(), reqwest::Error> {  // -> Result<(), Box<dyn Error>> {

    let client = Client::new();
    let size_response = 1;
    let request_url = format!("https://qrng.anu.edu.au/API/jsonI.php?length={}&type=uint16&size=12", size_response);
    println!("{}", &request_url);

    // let mut tank = Tank::new();
    // let mut tank = Mutex::new(VecDeque::new());
    let mut tank = Mutex::new(VecDeque::with_capacity(10));
    
    {
        let response = client.clone().get(&request_url).send().await?;
        let result = response.json::<Json>().await?;
        println!("{:?}", &result);
        
        let mut tank = tank.lock().unwrap();
        tank.push_back(result.data[0]);
    }

    {
        let response = client.clone().get(&request_url).send().await?;
        let result = response.json::<Json>().await?;
        println!("{:?}", &result);
        
        let mut tank = tank.lock().unwrap();
        tank.push_back(result.data[0]);
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
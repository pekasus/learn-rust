#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use tokio;
use reqwest::Error;
use std::collections::HashMap;

// let api_call = "https://qrng.anu.edu.au/API/jsonI.php?length=15&type=uint16&size=12";

#[derive(Deserialize, Debug)]
struct Json {
    #[serde(rename = "type")]
    type_: String,
    length: u32,
    data: Vec<u32>,
    success: bool,
}

#[tokio::main]
async fn main() -> Result<(), Error> {

    // We want to use reqwest::Client to create a reusable Client which is faster.
    let client = reqwest::Client::new();

    // can change the length={value up to 1024 in request_url}
    let request_url = "https://qrng.anu.edu.au/API/jsonI.php?length=15&type=uint16&size=12";
    println!("{}", &request_url);

    // making a call this way creates a new Client each get call
    // let response = reqwest::get(request_url).await?; 

    // This uses a reusable Client
    let response = client.get(request_url).send().await?;

// Explore using ClientBuilder, potentially with client.request(...)

    println!("Status: {}", &response.status());
    // println!("Status: {:#?}", &response.text().await?);

    let result = &response.json::<Json>().await?;
    println!("{:?}", &result);
    println!("{:?}", &result.data);
    println!("{:?}", &result.data[0]);

    for i in &result.data {
        println!("{}", i);
    }

    if result.success {
        println!("Success!")
    }
    
    Ok(())
}

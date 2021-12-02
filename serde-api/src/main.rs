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
    length: u32,
    data: Vec<u32>,
    success: bool,
}

#[tokio::main]
async fn main() -> Result<(), Error> {

    // can change the length={value up to 1024 in request_url}
    let request_url = "https://qrng.anu.edu.au/API/jsonI.php?length=15&type=uint16&size=12";
    println!("{}", &request_url);
    let response = reqwest::get(request_url).await?;

    println!("Status: {}", &response.status());
    // println!("Status: {:#?}", &response.text().await?);

    let result = &response.json::<Json>().await?;
    println!("{:?}", &result);
    println!("{:?}", &result.data);
    println!("{:?}", &result.data[0]);

    
    Ok(())
}

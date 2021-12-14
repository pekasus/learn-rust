use reqwest::Client;
use tokio;
use std::io::Error;
use std::collections::VecDeque;
use std::sync::Mutex; // use parking_lot as per StackOverflow
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

const CONCURRENT_REQUESTS: usize = 2;

#[derive(Deserialize, Debug)]
struct Json {
    #[serde(rename = "type")]
    type_: String,
    length: u32,
    data: Vec<u32>,
    success: bool,
}

async fn get_random_number() -> Result<(), reqwest::Error> {  //vd: <Mutex<VecDeque>>)
    let size_response = 1;
    let request_url = format!("https://qrng.anu.edu.au/API/jsonI.php?length={}&type=uint16&size=12", size_response);
    let client = Client::new();
    let response = client.clone().get(&request_url).send().await?;
    let result = response.json::<Json>().await?;
    println!("{:?}", &result);
    // let mut tank = tank.lock().unwrap().push_back(result.data[0]);
    Ok(())
}  

#[tokio::main]
async fn main()  -> Result<(), reqwest::Error> {  // -> Result<(), Box<dyn Error>> {

    // let client = Client::new();
    // let size_response = 1;
    // let request_url = format!("https://qrng.anu.edu.au/API/jsonI.php?length={}&type=uint16&size=12", size_response);

    // let mut tank = Mutex::new(VecDeque::new());
    // let mut tank = Mutex::new(VecDeque::with_capacity(10));
    
    // while tank.lock().unwrap().len() < 10 {
    //     let response = client.clone().get(&request_url).send().await?;
    //     let result = response.json::<Json>().await?;
    //     println!("{:?}", &result);
    //     let mut tank = tank.lock().unwrap().push_back(result.data[0]);
    // }

    // println!("{:?}", tank);
    

    tokio::join!(get_random_number(),get_random_number(),get_random_number(),get_random_number(),get_random_number()
    ,get_random_number(),get_random_number(),get_random_number(),get_random_number(),get_random_number());
    Ok(())
}


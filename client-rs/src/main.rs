
mod swapi;

use swapi::client::SwapiClient;
use swapi::model::SwapiPeople;

#[tokio::main]
async fn main() {
    println!("Lets network!");

    let client = SwapiClient::new();
    let resp: Result<SwapiPeople, reqwest::Error> = client.get_people(1).await;
    match resp {
        Ok(value) => println!("{:?}", value),
        Err(e) => println!("{:?}", e),
    }
}

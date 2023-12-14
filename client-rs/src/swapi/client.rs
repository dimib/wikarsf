
use reqwest::Client;
use reqwest::ClientBuilder;
use crate::swapi::model::SwapiPeople;

pub struct SwapiClient {
    pub client: Client,
    pub base_url: String,
}

impl SwapiClient {
    pub fn new() -> Self {
        let client = ClientBuilder::new().build().unwrap();
        let base_url = "https://swapi.dev/api".to_string();
        Self { client, base_url }
    }

    pub async fn get_people(&self, id: i32) -> Result<SwapiPeople, reqwest::Error> {
        let url = format!("{}/people/{}", self.base_url, id);
        let response = self.client.get(&url).send().await?;
        // let value = response.jso n::<Value>().await?;
        let value = response.json::<SwapiPeople>().await?;
        Ok(value)
    }
}
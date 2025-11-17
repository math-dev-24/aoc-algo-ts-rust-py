use dotenvy::dotenv;
use std::env;
use reqwest::{Client, Response};


pub struct Input {
    client: Client,
    cookie: String,
    base_url: String,
}

impl Input {
    pub fn new() -> Self {

        dotenv().ok();

        let session = env::var("SESSION").unwrap();
        if session.is_empty() {
            panic!("SESSION is empty");
        }

        let base_url = env::var("URL").unwrap();
        if base_url.is_empty() {
            panic!("URL is empty");
        }

        Self {
            client: Client::new(),
            cookie: format!("session={}", session),
            base_url,
        }
    }

    pub async fn get_input(&self, year: u32, day: u32) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!("{}/{}/day/{}/input", &self.base_url, year, day);

        println!("{}", url);

        let response: Response = self
            .client
            .get(url)
            .header("Cookie", &self.cookie)
            .send()
            .await
            .unwrap();

        if !response.status().is_success() {
            return Err(format!("HTTP Error: {}", response.status()).into());
        }

        let input = response.text().await?;
        Ok(input)
    }
}
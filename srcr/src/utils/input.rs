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
        let base_url = env::var("URL").unwrap();
        Self {
            client: Client::new(),
            cookie: format!("session={}", session),
            base_url,
        }
    }
    pub async fn get_input(&self, year: u32, day: u32) -> String {
        let url = format!("{}/{}/day/{}/input", &self.base_url, year, day);

        let response: Response = self
            .client
            .get(url)
            .header("Cookie", &self.cookie)
            .send()
            .await
            .unwrap();

        if !response.status().is_success() {
            panic!("Failed to get input");
        }
        response.text().await.unwrap()
    }
}
use reqwest;
use std::error::Error;
use url::Url;

pub struct Rest {
    base: Url,
    client: reqwest::Client,
}

impl Rest {
    pub fn new(base: Url) -> Self {
        let client = reqwest::Client::new();
        Self { base, client }
    }

    // TODO add error type
    pub async fn get(&self, path: &str) -> Result<reqwest::Response, Box<dyn Error>> {
        match path {
            "" => {
                let url = self.base.clone();
                let resp = self.client.get(url).send().await?;
                return Ok(resp);
            }
            _ => {
                let url = self.base.join(path)?;
                let resp = self.client.get(url).send().await?;
                return Ok(resp);
            }
        }
    }
}

pub mod accounts;
pub mod blocks;

use std::fmt::Display;

use serde::de::DeserializeOwned;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AptosClient;

pub async fn core_get<Request: Display, Response: DeserializeOwned>(
    request: Request,
    api_url: &str,
) -> anyhow::Result<Response> {
    println!("request = {}", request);
    let request_url = format!("{}{}", api_url, request);
    println!("responst_url = {}", request_url);
    let response = reqwest::Client::new()
        .get(request_url)
        .header("content-type", "application/json")
        .send()
        .await?;

    println!("status: {:#?}", response.status());

    if response.status() != 200 {
        let res = response.text().await?;
        Err(anyhow::anyhow!(res))
    } else {
        let response = response.json::<Response>().await?;
        Ok(response)
    }
}

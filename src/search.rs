use crate::{
    doujin::Doujin,
    utility::{api::methods, error::Result},
};
use hyper::{
    body::{self, Buf},
    Client,
};
use hyper_tls::HttpsConnector;
use serde::Deserialize;
use std::str::FromStr;

#[derive(Deserialize)]
pub struct Search {
    pub result: Vec<Doujin>,
    pub num_pages: u32,
    pub per_page: u32,
}

impl Search {
    pub async fn new(id: u32, page: Option<u32>) -> Result<Self> {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        let url = methods::search_tag(id, page.unwrap_or(1));

        let response = client
            .get(hyper::Uri::from_str(&url)?)
            .await
            .expect("Failed to request url");
        let content = body::aggregate(response)
            .await
            .expect("Failed to aggregate body");
        let result: Self =
            serde_json::from_reader(content.reader()).expect("Failed to deserialize json");

        Ok(result)
    }
}

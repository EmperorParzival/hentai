use crate::{doujin::Doujin, utility::api::methods};
use serde::Deserialize;
use hyper::{
    body::{self, Buf},
    Client,
};
use hyper_tls::HttpsConnector;

#[derive(Deserialize)]
pub struct Search {
    pub result: Vec<Doujin>,
    pub num_pages: u32,
    pub per_page: u32,
}

impl Search {
    pub fn new(id: u32, page: Option<u32>) -> Result<Self> {
        let page_num = match page {
            Some(num) => num,
            None => 1,
        };

        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);

        let response = client
            .get(hyper::Uri::from_str(methods::search_tag(id, page_num))
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

use reqwest::header::{HeaderMap, USER_AGENT, CONTENT_TYPE, ACCEPT};
use std::io;

pub struct HttpClient {
    client: reqwest::Client
}

impl HttpClient {
    pub fn new() -> Result<HttpClient, io::Error> {
        let client = reqwest::Client::new();
        Ok(HttpClient { client })
    }

    pub fn construct_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, "reqwest".parse().unwrap());
        headers.insert(ACCEPT, "application/json".parse().unwrap());
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers
    }

    pub fn get(&self, url: &str) -> Result<reqwest::Response, reqwest::Error> {
        self.client
            .get(url)
            .headers(self.construct_headers())
            .send()
    }

    pub fn post(&self, url: &str, body: String) -> Result<reqwest::Response, reqwest::Error> {
        self.client
            .post(url)
            .headers(self.construct_headers())
            .body(body)
            .send()
    }
}

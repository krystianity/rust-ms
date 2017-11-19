pub mod mshttp_c {

    extern crate reqwest;
    use self::reqwest::header::{Headers, UserAgent, ContentType};
    use std::io;

    pub struct HttpClient {
        client: reqwest::Client
    }

    impl HttpClient {

        pub fn new() -> Result<HttpClient, io::Error> {
            let client = reqwest::Client::new();
            Ok(HttpClient{client})
        }

        pub fn construct_headers(&self) -> Headers {
            let mut headers = Headers::new();
            headers.set(UserAgent::new("reqwest"));
            headers.set_raw("accept", "application/json");
            headers.set(ContentType::json());
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
}
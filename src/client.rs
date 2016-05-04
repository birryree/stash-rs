use std::result;

use hyper::status::StatusCode;
use hyper::client;
use hyper::header::{Headers, Authorization, Basic};

#[derive(Debug)]
pub struct StashClient {
    stash_url: String,
    http: hyper::Client,
    headers: Headers
}

impl StashClient {
    /// Creates a new `StashClient`.
    pub fn new(host: &str, port: u32, user_name: &str, password: &str) -> Result<Self, hyper::Error> {
        StashClient {
            http: hyper::Client::new(),
            stash_url: format!("http://{}:{}", host, port),
            headers: Self::generate_headers(user_name, password)
        }
    }

    fn generate_header(user_name: &str, password &str) {
        let mut headers = Headers::new();

        headers.set(Authorization(
                Basic {
                    username: user_name.to_owned(),
                    password: password.to_owned()
                }
            )
        );
        headers
    }
}

//! Stash is an (experimental) library for using the Atlassian Stash REST API.

extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate url;

#[macro_use]
extern crate serializable_enum;

#[macro_use]
extern crate log;

use serde::de::Deserialize;

pub mod core;
pub mod repos;
pub mod errors;
pub mod types;

pub use types::*;
pub use errors::StashError;
use core::{Projects};
use repos::{ProjectRepositories};

use hyper::header::{Authorization, Basic, ContentType, Headers};
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use hyper::method::Method;
use hyper::client::RequestBuilder;
use hyper::status::StatusCode;


use std::io::Read;


#[derive(Debug)]
pub enum Credentials {
    OAuth(String),
    Basic(String, String),
}

#[derive(Debug)]
pub struct Stash<'a> {
    host: String,
    http: &'a hyper::Client,
    credentials: Credentials
}

impl<'a> Stash<'a> {
    /// Creates a new `Stash` instance.
    pub fn new<T>(host: T, client: &'a hyper::Client, credentials: Credentials) -> Stash<'a>
        where T: Into<String>
    {
        Stash {
            http: client,
            host: host.into(),
            credentials: credentials
        }
    }
    
    /// Get a reference to the list of projects in Stash 
    pub fn projects(&self) -> Projects {
        Projects::new(self)
    }
    
    /// Get a reference to the list of source repositories for a given project key
    pub fn project_repos<T>(&self, project: T) -> ProjectRepositories
        where T: Into<String>
    {
        ProjectRepositories::new(self, project)
    }
    
    fn content_type(&self) -> ContentType {
        ContentType(Mime(TopLevel::Application, SubLevel::Json,
                         vec![(Attr::Charset, Value::Utf8)]))
    }

    fn generate_request(&self, method: Method, uri: &str) -> RequestBuilder {
        let url = format!("{}{}", self.host, uri);
        trace!("Sending {:#?} request to {}", method, url);
        
        let mut headers = Headers::new();
        headers.set(self.content_type());
        
        match self.credentials {
            Credentials::OAuth(ref token) =>
                headers.set(Authorization(format!("token {}", token))),
            Credentials::Basic(ref user, ref password) =>
                headers.set(Authorization( Basic { username: user.to_owned(),
                                                   password: Some(password.to_owned()) }))
        }
        
        self.http.request(method, &url).headers(headers)
    }
    
    fn get<T>(&self, uri: &str) -> Result<T, StashError>
        where T: Deserialize
    {
        self.request(Method::Get, uri, None)
    }
    
    fn post<T>(&self, uri: &str, body: &[u8]) -> Result<T, StashError>
        where T: Deserialize
    {
        self.request(Method::Post, uri, Some(body))
    }
    
    fn delete<T>(&self, uri: &str) -> Result<T, StashError>
        where T: Deserialize
    {
        self.request(Method::Delete, uri, None)
    }
    
    fn put<T>(&self, uri: &str, body: &[u8]) -> Result<T, StashError>
        where T: Deserialize
    {
        self.request(Method::Put, uri, Some(body))
    }

    fn request<T>(&self, method: Method, uri: &str, body: Option<&'a [u8]>) -> Result<T, StashError>
        where T: Deserialize
    {
        let builder = self.generate_request(method, uri);
        let mut rsp = try!(match body {
            Some(ref b) => builder.body(*b).send(),
            _ => builder.send(),
        });
        
        // optimize to get content length
        let mut body = String::new();
        
        try!(rsp.read_to_string(&mut body));
        debug!("response {:#?} {:#?} {:#?}", rsp.status, rsp.headers, body);
        
        match rsp.status {
            // 400, 401, 403, 404, 409 will generate errors from API
            StatusCode::BadRequest |
            StatusCode::Unauthorized |
            StatusCode::Forbidden |
            StatusCode::NotFound |
            StatusCode::Conflict => {
                Err(StashError::Client {
                    code: rsp.status,
                    error: try!(serde_json::from_str::<ClientError>(&body)),
                })
            }
            _ => Ok(try!(serde_json::from_str::<T>(&body))),
        }
    }
}

#[cfg(test)]
mod tests {
    use hyper::Client;
    use hyper::header::{ContentType};
    use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
    use super::*;
    
    #[test]
    fn test_request_content_type() {
        // test that the generated request from Stash client
        // has correct information on it
        let h = "http://hostname.com";
        let cred = Credentials::OAuth("mytoken".to_owned());
        let hyper = Client::new();
        let stash = Stash::new(h, &hyper, cred);
        let ct = stash.content_type();
        
        assert_eq!(ct, ContentType(Mime(TopLevel::Application, SubLevel::Json,
                                     vec![(Attr::Charset, Value::Utf8)])));
    }
}
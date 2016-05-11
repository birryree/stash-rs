//! Stash is an (experimental) library for using the Atlassian Stash REST API.

#![crate_type="lib"]

extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate url;

use serde::de::Deserialize;

pub mod core;
pub mod errors;

pub use errors::StashError;
use core::{Project, Projects};

use hyper::Error;
use hyper::status::StatusCode;
use hyper::header::{Authorization, Basic, ContentLength};
use hyper::method::Method;
use hyper::client::RequestBuilder;


use std::fmt;
use std::result;
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
    pub fn new<S>(host: S, client: &'a hyper::Client, credentials: Credentials) -> Stash<'a>
        where S: Into<String>
    {
        Stash {
            http: client,
            host: host.into(),
            credentials: credentials
        }
    }
    
    pub fn projects(&self) -> Projects {
        Projects::new(self)
    }

    fn generate_request(&self, method: Method, uri: &str) -> RequestBuilder {
        let url = format!("{}{}", self.host, uri);
        
        match self.credentials {
            Credentials::OAuth(ref token) => {
                self.http.request(method, &url).header(Authorization(format!("token {}", token)))
            }
            Credentials::Basic(ref user, ref password) => {
                self.http.request(method, &url).header(Authorization(
                                                        Basic { username: user.to_owned(),
                                                        password: Some(password.to_owned()) }))
            }
        }
    }
    
    fn get<T>(&self, uri: &str) -> Result<T, StashError>
        where T: Deserialize
    {
        self.request(Method::Get, uri, None)
    }

    fn request<T>(&self, method: Method, uri: &str, body: Option<&'a [u8]>) -> Result<T, StashError>
        where T: Deserialize {
        let builder = self.generate_request(method, uri);
        let mut rsp = try!(match body {
            Some(ref b) => builder.body(*b).send(),
            _ => builder.send(),
        });
        
        // optimize to get content length
        let mut body = String::new();
        
       try!(rsp.read_to_string(&mut body));
       Ok(try!(serde_json::from_str::<T>(&body)))
        //match rsp.status {
        //    _ => Ok(try!(serde_json::from_str::<T>(&body))),
        //}
    }
}

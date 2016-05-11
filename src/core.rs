extern crate hyper;
extern crate serde;
extern crate serde_json;

use self::super::{Stash, StashError};

use std::collections::HashMap;
use std::option::Option;
use url::form_urlencoded;

include!(concat!(env!("OUT_DIR"), "/core.rs"));

#[derive(Default)]
pub struct ProjectListParams {
    params: HashMap<&'static str, String>
}

impl ProjectListParams {
    pub fn builder() -> ProjectListParamsBuilder {
        ProjectListParamsBuilder::new()
    }
    
    /// Generate a query string of the parameters. Returns None if no options are defined.
    pub fn to_query_string(&self) -> Option<String> {
        if self.params.is_empty() {
            None
        } else {
            let query_string = form_urlencoded::Serializer::new(String::new())
                .extend_pairs(&self.params)
                .finish();
            Some(query_string)
        }
    }
}

#[derive(Default)]
pub struct ProjectListParamsBuilder {
    params: HashMap<&'static str, String>
}

impl ProjectListParamsBuilder {
    pub fn new() -> ProjectListParamsBuilder {
        ProjectListParamsBuilder { ..Default::default() }
    }
    
    // set the 'name' field of the Projects request
    pub fn repo_name<'a>(&mut self, arg: String) -> &mut ProjectListParamsBuilder {
        self.params.insert("name", arg);
        self
    }
    
    // Set the 'permissions' field of the Projects request
    pub fn permissions(&mut self, permissions: String) -> &mut ProjectListParamsBuilder {
        self.params.insert("permissions", permissions);
        self
    }
    
    pub fn build(&self) -> ProjectListParams {
        ProjectListParams { params: self.params.clone() }
    }
}

pub struct Projects<'a> {
    stash: &'a Stash<'a>,
}

impl<'a> Projects<'a> {
    pub fn new(stash: &'a Stash<'a>) -> Projects<'a> {
        Projects { stash: stash }
    }
    
    fn resource(&self, params: &str) -> String {
        format!("/projects/{}", params)
    }

    /// List projects
    pub fn list(&self, params: &ProjectListParams) -> Result<Vec<Project>, StashError> {
        let mut uri = vec![self.resource("")];
        if let Some(query) = params.to_query_string() {
            uri.push(query);
        }
        self.stash.get::<Vec<Project>>(&uri.join("?"))
    }
}

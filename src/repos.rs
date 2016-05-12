extern crate hyper;
extern crate serde;
extern crate serde_json;

use self::super::{Stash, StashError};
use types::{Repository, RepositoryParams, PagedResponse};

pub struct ProjectRepositories<'a> {
    stash: &'a Stash<'a>,
    project_key: String,
}

impl<'a> ProjectRepositories<'a> {
    pub fn new<T>(stash: &'a Stash<'a>, project_key: T) -> ProjectRepositories<'a>
        where T: Into<String>
    {
        ProjectRepositories {
            stash: stash,
            project_key: project_key.into(),
        }
    }
    
    fn resource(&self, extra: &str) -> String {
        format!("/projects/{}/repos{}", &self.project_key, extra)
    }
   
    /// List repos under the project key
    pub fn list(&self) -> Result<PagedResponse<Repository>, StashError> {
        self.stash.get::<PagedResponse<Repository>>(&self.resource(""))
    }
    
    /// Create a new repository under an existing project.
    pub fn create(&self, params: &RepositoryParams) -> Result<Repository, StashError> {
        let data = try!(serde_json::to_string(&params));
        self.stash.post::<Repository>(&self.resource(""), data.as_bytes())
    }
    
    /// Get information about a specific repository
    pub fn get(&self, slug: &str) -> Result<Repository, StashError> {
        self.stash.get::<Repository>(&self.resource(&format!("/{}", slug)))
    }
    
    /// Delete a repository by its slug name.
    pub fn delete(&self, slug: &str) -> Result<(), StashError> {
        self.stash.delete(&self.resource(&format!("/{}", slug)))
    }
}
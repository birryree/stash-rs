use self::super::{Stash, StashError};
use types::{Repository, RepositoryParams, PagedResponse, ApiMessage};

use serde_json;

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
        self.stash.get(&self.resource(""))
    }
    
    /// Create a new repository under an existing project.
    pub fn create(&self, params: &RepositoryParams) -> Result<Repository, StashError> {
        let data = try!(serde_json::to_string(&params));
        self.stash.post(&self.resource(""), data.as_bytes())
    }
    
    /// Get information about a specific repository
    pub fn get(&self, slug: &str) -> Result<Repository, StashError> {
        self.stash.get(&self.resource(&format!("/{}", slug)))
    }
    
    /// Delete a repository by its slug name.
    pub fn delete(&self, slug: &str) -> Result<ApiMessage, StashError> {
        self.stash.delete(&self.resource(&format!("/{}", slug)))
    }
}
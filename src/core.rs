use self::super::{Stash, StashError};
use types::{PagedResponse, Project, Repository, ProjectListParams, ProjectParams};

use serde_json;

pub struct Projects<'a> {
    stash: &'a Stash<'a>,
}

impl<'a> Projects<'a> {
    /// Returns a new Projects object
    pub fn new(stash: &'a Stash<'a>) -> Projects<'a> {
        Projects { stash: stash }
    }
    
    fn resource(&self, extra: &str) -> String {
        format!("/projects{}", extra)
    }
   
    /// Get project by a project key 
    pub fn get_project(&self, key: &str) -> Result<Project, StashError> {
        self.stash.get(&self.resource(&format!("/{}", key)))
    }
    
    /// Create a new project.
    pub fn create_project(&self, params: &ProjectParams) -> Result<Project, StashError> {
        let data = try!(serde_json::to_string(&params));
        self.stash.post(&self.resource(""), data.as_bytes())
    }

    /// Updates a project 
    pub fn update_project(&self, key: &str, params: &ProjectParams) -> Result<Project, StashError> {
        let data = try!(serde_json::to_string(&params));
        self.stash.put(&self.resource(&format!("/{}", key)), data.as_bytes())
    }
    
    /// Deletes a project by its key
    pub fn delete_project(&self, key: &str) -> Result<(), StashError> {
        self.stash.delete(&self.resource(&format!("/{}", key)))
    }

    /// List projects
    pub fn list(&self, params: &ProjectListParams) -> Result<PagedResponse<Project>, StashError> {
        let mut uri = vec![self.resource("")];
        if let Some(query) = params.to_query_string() {
            uri.push(query);
        }
        self.stash.get(&uri.join("?"))
    }

    /// Fetches all repositories under a project.
    pub fn repos(&self, key: &str) -> Result<PagedResponse<Repository>, StashError> {
        self.stash.get(&self.resource(&format!("/{}/repos", key)))
    }
}

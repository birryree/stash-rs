//! Stash API models as Rust structs
use std::collections::HashMap;
use std::option::Option;
use url::form_urlencoded;
use serde_json;

use ::errors::StashError;

include!(concat!(env!("OUT_DIR"), "/types.rs"));

serializable_enum! {
    /// Enumeration for valid SCM types in Atlassian Stash API.
    #[derive(Clone, Debug, PartialEq)]
    pub enum ScmType {
        /// Git (the only available one)
        Git
    }
    ScmTypeVisitor
}

impl_as_ref_from_str! {
    ScmType {
        Git => "git",
    }
    StashError::Other
}


serializable_enum! {
    /// Enumeration for repository states in Atlassian Stash API.
    #[derive(Clone, Debug, PartialEq)]
    pub enum RepositoryState {
        /// AVAILABLE state
        Available,
        /// INITIALISING state
        Initialising,
        /// INITIALISATION_FAILED state
        InitFailed
    }
    RepositoryStateVisitor
}

impl_as_ref_from_str! {
    RepositoryState {
        Available => "AVAILABLE",
        Initialising => "INITIALISING",
        InitFailed => "INITIALISATION_FAILED",
    }
    StashError::Other
}

serializable_enum! {
    /// Enumeration for Stash permissions
    #[derive(Clone, Debug, PartialEq)]
    pub enum Permission {
        /// Admin permission
        Admin,
        /// Licensed user permission
        LicensedUser,
        /// Project administrator permission
        ProjectAdmin,
        /// Project create permission
        ProjectCreate,
        /// Project read permission
        ProjectRead,
        /// Project write permission
        ProjectWrite,
        /// Project admin permission
        RepoAdmin,
        /// Repository read permission
        RepoRead,
        /// Repository write permission
        RepoWrite,
        /// System administrator permission
        SysAdmin
    }
    PermissionVisitor
}

impl_as_ref_from_str! {
    Permission {
        Admin => "ADMIN",
        LicensedUser => "LICENSED_USER",
        ProjectAdmin => "PROJECT_ADMIN",
        ProjectCreate => "PROJECT_CREATE",
        ProjectRead => "PROJECT_READ",
        ProjectWrite => "PROJECT_WRITE",
        RepoAdmin => "REPO_ADMIN",
        RepoRead => "REPO_READ",
        RepoWrite => "REPO_WRITE",
        SysAdmin => "SYS_ADMIN",
    }
    StashError::Other
}

#[cfg(test)]
mod tests {
    use serde::ser::Serialize;
    use std::collections::{BTreeMap, HashMap};
    use serde_json;
    use super::*;
    
    fn test_serialization<T>(tests: Vec<(T, &str)>) where T: Serialize {
        for test in tests {
            match test {
                (t, v) => assert_eq!(serde_json::to_string(&t).unwrap(), v),
            }
        }
    }
    
    #[test]
    fn serialize_repository_params() {
        let tests = vec![
            (
                RepositoryParams::new(None as Option<String>, None as Option<ScmType>, None as Option<bool>),
                "{}"
            ),
            (
                RepositoryParams::new(Some("repo"), Some(ScmType::Git), Some(true)),
                r#"{"name":"repo","scmId":"git","forkable":true}"#
            )
        ];
        test_serialization(tests);
    }
    
    #[test]
    fn serialize_project_params() {
        let tests = vec![
            (
                ProjectParams::new(None as Option<String>, None as Option<String>, None as Option<String>, None as Option<String>),
                "{}"
            ),
            (
                ProjectParams::new(Some("proj"), Some("project"), Some("My project"), None),
                r#"{"key":"proj","name":"project","description":"My project"}"#
            )
        ];
        test_serialization(tests);
    }

    #[test]
    fn deserialize_paged_responses() {
        let json = r#"{"size": 1, 
                       "limit": 25,
                       "isLastPage": true,
                       "values": [
                           {
                               "key": "PRJ",
                               "id": 1,
                               "name": "Demo Project",
                               "description": "A demo project",
                               "public": true,
                               "type": "NORMAL",
                               "link": {
                                   "url": "http://link/to/project",
                                   "rel": "self" 
                                },
                                "links": {
                                    "self": [
                                        {
                                            "href": "http://link/to/project"
                                        }
                                    ] 
                                }
                            }
                        ],
                        "start": 0
                    }"#;
                    
        let rsp = PagedResponse {
            size: 1,
            limit: 25,
            is_last_page: true,
            values: vec![
                Project {
                    key: "PRJ".to_owned(),
                    id: 1,
                    name: "Demo Project".to_owned(),
                    description: Some("A demo project".to_owned()),
                    public: true,
                    project_type: "NORMAL".to_owned(),
                    link: serde_json::from_str(r#"{"url": "http://link/to/project", "rel": "self"}"#).unwrap(),
                    links: serde_json::from_str(r#"{"self": [{"href": "http://link/to/project"}] }"#).unwrap()
                }
            ],
            start: 0,
            filter: None,
            next_page_start: None
        };
        
        assert_eq!(serde_json::from_str::<PagedResponse<Project>>(json).unwrap(), rsp);
    }
    
    #[test]
    fn project_list_params() {
        fn test_serialized_query(tests: Vec<(ProjectListParams, Option<String>)>) {
            for test in tests {
                match test {
                   (p, v) => assert_eq!(p.to_query_string(), v),
                }
            }
        }
        
        let tests = vec![
            (
                ProjectListParams::builder().build(),
                None
            ),
            (
                ProjectListParams::builder().permissions(Permission::Admin).build(),
                Some("permission=ADMIN".to_owned())
            ),
            (
                ProjectListParams::builder().repo_name("demo".to_owned()).build(),
                Some("name=demo".to_owned())
            )
        ];
        test_serialized_query(tests);
    }
   
    #[test]
    fn deserialize_permissions() {
        for (value, e) in vec![(r#""ADMIN""#, Permission::Admin),
                               (r#""LICENSED_USER""#, Permission::LicensedUser),
                               (r#""PROJECT_ADMIN""#, Permission::ProjectAdmin),
                               (r#""PROJECT_CREATE""#, Permission::ProjectCreate),
                               (r#""PROJECT_READ""#, Permission::ProjectRead),
                               (r#""PROJECT_WRITE""#, Permission::ProjectWrite),
                               (r#""REPO_ADMIN""#, Permission::RepoAdmin),
                               (r#""REPO_READ""#, Permission::RepoRead),
                               (r#""REPO_WRITE""#, Permission::RepoWrite),
                               (r#""SYS_ADMIN""#, Permission::SysAdmin)] {
            assert_eq!(serde_json::from_str::<Permission>(value).unwrap(), e);
        }
    }
    
    #[test]
    fn deserialize_scm_type() {
        for (value, e) in vec![("\"git\"", ScmType::Git)] {
            assert_eq!(serde_json::from_str::<ScmType>(value).unwrap(), e);
        }
    }
    
    #[test]
    fn deserialize_repository_state() {
        for (value, e) in vec![(r#""AVAILABLE""#, RepositoryState::Available),
                               (r#""INITIALISING""#, RepositoryState::Initialising),
                               (r#""INITIALISATION_FAILED""#, RepositoryState::InitFailed)] {
            assert_eq!(serde_json::from_str::<RepositoryState>(value).unwrap(), e);
        }
    }
}
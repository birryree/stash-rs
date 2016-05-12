//! Stash API models as Rust structs

extern crate serde;
extern crate serde_json;
extern crate serializable_enum;

use super::{StashError};

use std::collections::HashMap;
use std::option::Option;
use url::form_urlencoded;

include!(concat!(env!("OUT_DIR"), "/types.rs"));

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
        Initialising => "INITIALSING",
        InitFailed => "INITIALISATION_FAILED",
    }
    StashError::Other
}

serializable_enum! {
    /// Enumeration for user permissions
    #[derive(Clone, Debug, PartialEq)]
    pub enum UserPermission {
        /// Licensed User permission
        LicensedUser,
        /// Project Create permission
        ProjectCreate,
        /// Admin permission
        Admin,
        /// Sys Admin permission
        SysAdmin
    }
    UserPermissionVisitor
}

impl_as_ref_from_str! {
    UserPermission {
        LicensedUser => "LICENSED_USER",
        ProjectCreate => "PROJECT_CREATE",
        Admin => "ADMIN",
        SysAdmin => "SYS_ADMIN",
    }
    StashError::Other
}

serializable_enum! {
    /// Enumeration for Repository Permissions
    #[derive(Clone, Debug, PartialEq)]
    pub enum RepositoryPermission {
        /// Read permission
        Read,
        /// Write permission
        Write,
        /// Admin permission
        Admin
    }
    RepositoryPermissionVisitor
}

impl_as_ref_from_str! {
    RepositoryPermission {
        Read => "REPO_READ",
        Write => "REPO_WRITE",
        Admin => "REPO_ADMIN",
    }
    StashError::Other
}
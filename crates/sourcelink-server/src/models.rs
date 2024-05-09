#![warn(missing_docs)]

//! Service data models.
//!
//! These represent database rows, API parameters, and
//! API response data.

use serde::{Deserialize, Serialize};

/// A row in the `projects` table.
/// Does not contain links.
#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectRow {
    pub id: String,
    pub name: String,
}

/// A row in the `links` table.
/// Does not contain the project.
#[derive(Serialize, Deserialize, Debug)]
pub struct LinkRow {
    pub id: String,
    pub url: String,
    pub project_id: String,
}

/// A project with links.
#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub links: Vec<ProjectLink>,
}

/// A project's link.
#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectLink {
    pub id: String,
    pub url: String,
}

/// A link with its project.
#[derive(Serialize, Deserialize, Debug)]
pub struct Link {
    pub id: String,
    pub url: String,
    pub project: ProjectRow,
}

/// Response for `GET /projects`.
#[derive(Serialize, Debug)]
pub struct GetProjects {
    pub projects: Vec<ProjectRow>,
}

/// Params for `POST /projects`.
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateProject {
    pub name: String,
}

/// Params for `POST /links`.
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateLink {
    pub project_id: String,
    pub url: String,
}

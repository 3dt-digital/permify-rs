use serde::{Deserialize, Serialize};

/// Generic API response model
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub data: T,
    pub message: Option<String>,
    pub errors: Option<Vec<String>>,
}

/// Model for an attribute request
#[derive(Debug, Serialize, Deserialize)]
pub struct Attribute {
    pub key: String,
    pub value: serde_json::Value,
}

/// Model for authorisation data
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorizationData {
    pub subject: String,
    pub relation: String,
    pub object: String,
}

/// Filter for data queries (e.g. Read Relationships, Delete Data)
#[derive(Debug, Serialize, Deserialize)]
pub struct DataFilter {
    pub subject: Option<String>,
    pub relation: Option<String>,
    pub object: Option<String>,
}

/// Model for a bundle
#[derive(Debug, Serialize, Deserialize)]
pub struct Bundle {
    pub id: Option<String>,
    pub name: String,
    pub content: serde_json::Value,
}

/// Filter for the search for subjects
#[derive(Debug, Serialize, Deserialize)]
pub struct SubjectFilter {
    pub object: String,
    pub relation: String,
}

/// Filter for entities (e.g. lookup entity)
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityFilter {
    pub subject: String,
    pub relation: String,
}

/// Model for an entity
#[derive(Debug, Serialize, Deserialize)]
pub struct Entity {
    pub id: String,
    pub name: String,
}

/// Filter for authorisations of a subject
#[derive(Debug, Serialize, Deserialize)]
pub struct SubjectPermissionFilter {
    pub subject: String,
}

/// Model for access checks
#[derive(Debug, Serialize, Deserialize)]
pub struct AccessCheckRequest {
    pub subject: String,
    pub relation: String,
    pub object: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessCheckResponse {
    pub allowed: bool,
}

/// Model for the permission extension
#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionExpandRequest {
    pub subject: String,
    pub object: String,
    pub relation: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionExpandResponse {
    pub tree: serde_json::Value,
}

/// Model for a tenant
#[derive(Debug, Serialize, Deserialize)]
pub struct Tenant {
    pub id: String,
    pub name: String,
}

/// Filter for the watch function
#[derive(Debug, Serialize, Deserialize)]
pub struct WatchFilter {
    pub object: String,
}

/// Model for a watch event
#[derive(Debug, Serialize, Deserialize)]
pub struct WatchEvent {
    pub id: String,
    pub changes: serde_json::Value,
}

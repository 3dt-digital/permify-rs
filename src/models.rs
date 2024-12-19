use serde::{Deserialize, Serialize};

/// Modell für die Schema-Antwort
#[derive(Debug, Serialize, Deserialize)]
pub struct SchemaRes {
    pub schema_version: String,
}

/// Modell für Attribute
#[derive(Debug, Serialize, Deserialize)]
pub struct Attribute {
    pub key: String,
    pub value: serde_json::Value,
}

/// Modell für Autorisierungsdaten
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorizationData {
    pub subject: String,
    pub relation: String,
    pub object: String,
}

/// Filter für Datenabfragen
#[derive(Debug, Serialize, Deserialize)]
pub struct DataFilter {
    pub subject: Option<String>,
    pub relation: Option<String>,
    pub object: Option<String>,
}

/// Modell für ein Bundle
#[derive(Debug, Serialize, Deserialize)]
pub struct Bundle {
    pub id: Option<String>,
    pub name: String,
    pub content: serde_json::Value,
}

/// Filter für die Subjekt-Suche
#[derive(Debug, Serialize, Deserialize)]
pub struct SubjectFilter {
    pub object: String,
    pub relation: String,
}

/// Filter für Entitäten
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityFilter {
    pub subject: String,
    pub relation: String,
}

/// Modell für eine Entität
#[derive(Debug, Serialize, Deserialize)]
pub struct Entity {
    pub id: String,
    pub name: String,
}

/// Modell für die Berechtigungen eines Subjekts
#[derive(Debug, Serialize, Deserialize)]
pub struct SubjectPermissionFilter {
    pub subject: String,
}

/// Modell für Zugriffskontrollen
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

/// Modell für Permission-Erweiterungen
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

/// Modell für einen Tenant
#[derive(Debug, Serialize, Deserialize)]
pub struct Tenant {
    pub id: String,
    pub name: String,
}

/// Filter für die Watch-Funktion
#[derive(Debug, Serialize, Deserialize)]
pub struct WatchFilter {
    pub object: String,
}

/// Modell für ein Watch-Event
#[derive(Debug, Serialize, Deserialize)]
pub struct WatchEvent {
    pub id: String,
    pub changes: serde_json::Value,
}

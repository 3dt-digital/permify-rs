use reqwest::{Client, Error};
use crate::{Attribute, Bundle, DataFilter, Entity, EntityFilter, SchemaRes, SubjectFilter, WatchEvent, WatchFilter};

/// Der Hauptclient für die Permify-API
pub struct PermifyClient {
    client: Client,
    base_url: String,
    api_key: String,
}

impl PermifyClient {
    /// Erstelle einen neuen PermifyClient
    pub fn new(base_url: &str, api_key: &str) -> Self {
        PermifyClient {
            client: Client::new(),
            base_url: base_url.to_string(),
            api_key: api_key.to_string(),
        }
    }

    // ------------------
    // Schema Service
    // ------------------

    pub async fn write_schema(&self, schema: &str) -> Result<SchemaRes, Error> {
        let url = format!("{}/schemas/write", self.base_url);
        self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&serde_json::json!({ "schema": schema }))
            .send()
            .await?
            .json::<SchemaRes>()
            .await
    }

    // ------------------
    // Data Service
    // ------------------

    pub async fn read_attributes(
        &self,
        filters: DataFilter,
    ) -> Result<Vec<Attribute>, Error> {
        let url = format!("{}/data/read-attributes", self.base_url);
        self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&filters)
            .send()
            .await?
            .json::<Vec<Attribute>>()
            .await
    }

    pub async fn run_bundle(&self, bundle: Bundle) -> Result<String, Error> {
        let url = format!("{}/data/run-bundle", self.base_url);
        self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&bundle)
            .send()
            .await?
            .json::<String>()
            .await
    }

    // ------------------
    // Permission Service
    // ------------------

    pub async fn subject_filtering(
        &self,
        filters: SubjectFilter,
    ) -> Result<Vec<String>, Error> {
        let url = format!("{}/permission/lookup-subject", self.base_url);
        self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&filters)
            .send()
            .await?
            .json::<Vec<String>>()
            .await
    }

    pub async fn lookup_entity(&self, filters: EntityFilter) -> Result<Vec<Entity>, Error> {
        let url = format!("{}/permission/lookup-entity", self.base_url);
        self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&filters)
            .send()
            .await?
            .json::<Vec<Entity>>()
            .await
    }

    // ------------------
    // Watch Service
    // ------------------

    pub async fn watch_changes(&self, filters: WatchFilter) -> Result<WatchEvent, Error> {
        let url = format!("{}/watch/watch-changes", self.base_url);
        self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&filters)
            .send()
            .await?
            .json::<WatchEvent>()
            .await
    }
}

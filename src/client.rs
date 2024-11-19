use reqwest::{Client, Error};
use crate::{ApiResponse, Attribute, Bundle, DataFilter, Entity, EntityFilter, SubjectFilter, SubjectPermissionFilter, WatchEvent, WatchFilter};

/// The main client for interacting with the Permify API
pub struct PermifyClient {
    client: Client,
    base_url: String,
    api_key: String,
}

impl PermifyClient {
    /// Create a new PermifyClient
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

    pub async fn write_schema(&self, schema: &str) -> Result<ApiResponse<String>, Error> {
        let url = format!("{}/schema/write", self.base_url);
        self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&serde_json::json!({ "schema": schema }))
            .send()
            .await?
            .json::<ApiResponse<String>>()
            .await
    }

    // ------------------
    // Data Service
    // ------------------

    pub async fn read_attributes(
        &self,
        filters: DataFilter,
    ) -> Result<ApiResponse<Vec<Attribute>>, Error> {
        let url = format!("{}/data/read-attributes", self.base_url);
        self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&filters)
            .send()
            .await?
            .json::<ApiResponse<Vec<Attribute>>>()
            .await
    }

    pub async fn run_bundle(
        &self,
        bundle: Bundle,
    ) -> Result<ApiResponse<String>, Error> {
        let url = format!("{}/data/run-bundle", self.base_url);
        self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&bundle)
            .send()
            .await?
            .json::<ApiResponse<String>>()
            .await
    }

    // ------------------
    // Permission Service
    // ------------------

    pub async fn subject_filtering(
        &self,
        filters: SubjectFilter,
    ) -> Result<ApiResponse<Vec<String>>, Error> {
        let url = format!("{}/permission/lookup-subject", self.base_url);
        self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&filters)
            .send()
            .await?
            .json::<ApiResponse<Vec<String>>>()
            .await
    }

    pub async fn lookup_entity(
        &self,
        filters: EntityFilter,
    ) -> Result<ApiResponse<Vec<Entity>>, Error> {
        let url = format!("{}/permission/lookup-entity", self.base_url);
        self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&filters)
            .send()
            .await?
            .json::<ApiResponse<Vec<Entity>>>()
            .await
    }

    pub async fn lookup_entity_streaming(
        &self,
        filters: EntityFilter,
    ) -> Result<ApiResponse<Vec<Entity>>, Error> {
        let url = format!("{}/permission/lookup-entity-stream", self.base_url);
        self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&filters)
            .send()
            .await?
            .json::<ApiResponse<Vec<Entity>>>()
            .await
    }

    pub async fn subject_permission_list(
        &self,
        filters: SubjectPermissionFilter,
    ) -> Result<ApiResponse<Vec<String>>, Error> {
        let url = format!("{}/permission/subject-permission", self.base_url);
        self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&filters)
            .send()
            .await?
            .json::<ApiResponse<Vec<String>>>()
            .await
    }

    // ------------------
    // Bundle Service
    // ------------------

    pub async fn write_bundle(&self, bundle: Bundle) -> Result<ApiResponse<String>, Error> {
        let url = format!("{}/bundle/write", self.base_url);
        self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&bundle)
            .send()
            .await?
            .json::<ApiResponse<String>>()
            .await
    }

    pub async fn read_bundle(&self, bundle_id: &str) -> Result<ApiResponse<Bundle>, Error> {
        let url = format!("{}/bundle/read", self.base_url);
        self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&serde_json::json!({ "id": bundle_id }))
            .send()
            .await?
            .json::<ApiResponse<Bundle>>()
            .await
    }

    pub async fn delete_bundle(&self, bundle_id: &str) -> Result<ApiResponse<String>, Error> {
        let url = format!("{}/bundle/delete", self.base_url);
        self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&serde_json::json!({ "id": bundle_id }))
            .send()
            .await?
            .json::<ApiResponse<String>>()
            .await
    }

    // ------------------
    // Watch Service
    // ------------------

    pub async fn watch_changes(&self, filters: WatchFilter) -> Result<ApiResponse<WatchEvent>, Error> {
        let url = format!("{}/watch/watch-changes", self.base_url);
        self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&filters)
            .send()
            .await?
            .json::<ApiResponse<WatchEvent>>()
            .await
    }
}

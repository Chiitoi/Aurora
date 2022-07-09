use crate::{constants::APPLICATION_ID, database::Database};
use hyper::{Body, client::{Client, HttpConnector}};
use hyper_tls::HttpsConnector;
use twilight_cache_inmemory::{InMemoryCache, ResourceType};
use twilight_gateway::Cluster;
use twilight_http::client::{Client as HttpClient, InteractionClient};
use twilight_model::id::{Id, marker::ApplicationMarker};

pub struct Context {
    application_id: Id<ApplicationMarker>,
    cache: InMemoryCache,
    cluster: Cluster,
    database: Database,
    http: HttpClient,
    hyper: Client<HttpsConnector<HttpConnector>>
}

impl Context {
    pub fn new(http: HttpClient, cluster: Cluster) -> Self {
        let resource_types = ResourceType::CHANNEL 
            | ResourceType::GUILD
            | ResourceType::MEMBER
            | ResourceType::MESSAGE
            | ResourceType::ROLE 
            | ResourceType::USER_CURRENT;
        let https = HttpsConnector::new();
        
        Self {
            application_id: *APPLICATION_ID,
            cache: InMemoryCache::builder()
                .message_cache_size(15)
                .resource_types(resource_types)
                .build(),
            cluster,
            database: Database::new(),
            http,
            hyper: Client::builder()
                .build::<_, Body>(https),
        }
    }

    pub fn cache(&self) -> &InMemoryCache {
        &self.cache
    }

    pub fn cluster(&self) -> &Cluster {
        &self.cluster
    }

    pub fn database(&self) -> &Database {
        &self.database
    }

    pub fn http(&self) -> &HttpClient {
        &self.http
    }

    pub fn hyper(&self) -> &Client<HttpsConnector<HttpConnector>> {
        &self.hyper
    }

    pub fn interaction_client(&self) -> InteractionClient {
        self.http.interaction(self.application_id)
    }
}
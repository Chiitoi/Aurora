use twilight_cache_inmemory::{InMemoryCache, ResourceType};
use twilight_gateway::Cluster;

pub struct Context {
    pub cache: InMemoryCache,
    pub cluster: Cluster,
}

impl Context {
    pub fn new(cluster: Cluster) -> Self {
        let resource_types = ResourceType::CHANNEL 
            | ResourceType::GUILD
            | ResourceType::MEMBER
            | ResourceType::MESSAGE
            | ResourceType::ROLE 
            | ResourceType::USER_CURRENT;
        
        Self {
            cache: InMemoryCache::builder()
                .message_cache_size(15)
                .resource_types(resource_types)
                .build(),
            cluster
        }
    }
}
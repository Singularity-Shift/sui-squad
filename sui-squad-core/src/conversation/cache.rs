use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use std::sync::Arc;

#[derive(Clone)]
pub struct ConversationEntry {
    pub response_id: String,
    pub last_activity: Instant,
}

#[derive(Clone)]
pub struct ConversationCache {
    cache: Arc<RwLock<HashMap<(String, String), ConversationEntry>>>,
    ttl: Duration,
}

impl ConversationCache {
    pub fn new(ttl: Duration) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            ttl,
        }
    }

    pub async fn get(&self, key: &(String, String)) -> Option<String> {
        let cache = self.cache.read().await;
        cache.get(key)
            .filter(|entry| entry.last_activity.elapsed() < self.ttl)
            .map(|entry| entry.response_id.clone())
    }

    pub async fn update(&self, key: (String, String), response_id: String) {
        let mut cache = self.cache.write().await;
        cache.insert(key, ConversationEntry {
            response_id,
            last_activity: Instant::now(),
        });
    }

    pub async fn cleanup_expired(&self) {
        let mut cache = self.cache.write().await;
        let now = Instant::now();
        cache.retain(|_, entry| now.duration_since(entry.last_activity) < self.ttl);
    }
} 
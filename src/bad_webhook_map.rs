use std::{sync::Arc, time::Duration};

use crate::expiring_lru::{Builder, ExpiringLru};

#[derive(Clone)]
pub struct BadWebhookMap {
    cache: Arc<ExpiringLru<String, ()>>,
}

impl BadWebhookMap {
    pub fn new(expiration: Duration) -> Self {
        Self {
            cache: Arc::new(Builder::new().expiration(expiration).build()),
        }
    }

    pub fn is_known_bad(&self, id: u64, token: &str) -> bool {
        let key = Self::build_key(id, token);
        self.cache.get(&key).is_some()
    }

    pub fn mark_bad(&self, id: u64, token: &str) {
        let key = Self::build_key(id, token);
        self.cache.insert(key, ());
    }

    fn build_key(id: u64, token: &str) -> String {
        format!("{}/{}", id, token)
    }
}

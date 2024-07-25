use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{Notify, RwLock};

#[derive(Clone)]
pub struct SyncService {
    waiters: Arc<RwLock<HashMap<String, Arc<Notify>>>>,
}

impl SyncService {
    pub fn new() -> Self {
        SyncService {
            waiters: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn wait_for_second_party(
        &self,
        unique_id: String,
    ) -> Result<&'static str, &'static str> {
        let notify = {
            let mut waiters = self.waiters.write().await;
            if let Some(notify) = waiters.remove(&unique_id) {
                notify.notify_waiters();
                return Ok("Both parties synced");
            } else {
                let notify = Arc::new(Notify::new());
                waiters.insert(unique_id.clone(), notify.clone());
                notify
            }
        };

        tokio::select! {
            _ = notify.notified() => Ok("Both parties synced"),
            _ = tokio::time::sleep(Duration::from_secs(10)) => {
                let mut waiters = self.waiters.write().await;
                waiters.remove(&unique_id);
                Err("Timeout")
            },
        }
    }
}

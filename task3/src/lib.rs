use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{Notify, RwLock};

#[derive(Clone)]
pub struct SyncService {
    pub waiters: Arc<RwLock<HashMap<String, Arc<Notify>>>>,
}

/// Basic synchronization service that awaits
/// one connection with unique identifier and blocks
/// until another connection with the same identifier is received
impl SyncService {
    pub fn new() -> Self {
        SyncService {
            waiters: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    // Post endpoint awaiting incoming synchronization connection
    pub async fn wait_for_second_party(
        &self,
        unique_id: String,
    ) -> Result<&'static str, &'static str> {
        let notify = {
            let mut waiters = self.waiters.write().await;
            if let Some(notify) = waiters.remove(&unique_id) {
                // Matching waiter was found
                notify.notify_waiters();
                return Ok("Both parties synced");
            } else {
                // First connection with such an identifier
                let notify = Arc::new(Notify::new());
                waiters.insert(unique_id.clone(), notify.clone());
                notify
            }
        };

        tokio::select! {
            _ = notify.notified() => Ok("Both parties synced"),
            _ = tokio::time::sleep(Duration::from_secs(10)) =>
            {
                // Cleanup waiter when timeout is reached.
                let mut waiters = self.waiters.write().await;
                waiters.remove(&unique_id);
                Err("Timeout")
            },
        }
    }
}

#[cfg(test)]
mod test {
    use std::{collections::HashMap, sync::Arc};

    use tokio::sync::RwLock;

    use crate::SyncService;

    #[tokio::test]
    async fn add_test_timeout() {
        let service = SyncService {
            waiters: Arc::new(RwLock::new(HashMap::new())),
        };

        let identifier = "ident324";

        let res = service.wait_for_second_party(identifier.to_string()).await;
        assert_eq!(res, Err("Timeout"))
    }

    #[tokio::test]
    async fn add_test_synced() {
        let service = SyncService {
            waiters: Arc::new(RwLock::new(HashMap::new())),
        };

        let identifier = "ident324";

        {
            // Spawn another connection that will cause the connection to sync
            let service = service.clone();
            let _ = tokio::spawn(async move {
                let _ = service.wait_for_second_party(identifier.to_string()).await;
            });
        }

        let res = service.wait_for_second_party(identifier.to_string()).await;
        assert_eq!(res, Ok("Both parties synced"))
    }
}

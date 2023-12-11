use std::{
    sync::{Arc, RwLock},
    time::Duration,
};

use notify_debouncer_full::new_debouncer;
use tokio::{sync::mpsc::channel, task};

use crate::{
    error::FsWatchError,
    watcher::{FsWatcher, RWWatcherHealth, WatchHealth},
};

#[derive(Debug)]
pub struct FsWatchOptions {
    pub debounce_s: u64,
    pub is_recursive: bool,
    pub path: String,
}

impl Default for FsWatchOptions {
    fn default() -> Self {
        Self {
            debounce_s: Default::default(),
            is_recursive: false,
            path: ".".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct FsWatcherBuilder {
    options: FsWatchOptions,
}

impl FsWatcherBuilder {
    pub fn new() -> Self {
        Self {
            options: FsWatchOptions::default(),
        }
    }

    pub fn debounce(mut self, seconds: u64) -> Self {
        self.options.debounce_s = seconds;
        self
    }

    pub fn recursive(mut self, is_recursive: bool) -> Self {
        self.options.is_recursive = is_recursive;
        self
    }

    pub fn path(mut self, path: String) -> Self {
        self.options.path = path;
        self
    }

    pub fn build(self) -> Result<FsWatcher, FsWatchError> {
        let (mut tx, receiver) = channel(1);
        let watch_health: RWWatcherHealth = Arc::new(RwLock::new(WatchHealth::Ok));
        let local_watch_health = watch_health.clone();
        let debouncer = new_debouncer(
            Duration::from_secs(self.options.debounce_s),
            None,
            move |res| {
                task::spawn(async {
                    match tx.send(res).await {
                        Ok(_) => (),
                        Err(err) => {
                            let msg = err.to_string();
                            if let Ok(mut current_state) = local_watch_health.clone().write() {
                                *current_state = WatchHealth::Halted(err);
                            }
                        }
                    }
                });
                ()
            },
        )
        .map_err(|e| FsWatchError::InitError(e.to_string()))?;
        Ok(FsWatcher {
            options: self.options,
            debouncer,
            receiver,
            watch_health,
        })
    }
}

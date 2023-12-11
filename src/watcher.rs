use std::sync::{Arc, RwLock};

use std::path::Path;

use notify::RecursiveMode;
use tokio::sync::mpsc::error::SendError;

use notify_debouncer_full::{new_debouncer, notify::*, DebounceEventResult};

use crate::{
    builder::FsWatchOptions,
    error::FsWatchError,
    interfaces::{FsWatchDebouncer, NotifyReceiver, WatcherOut},
};

#[derive(Debug, Default)]
pub enum WatchHealth {
    #[default]
    Ok,
    Halted(SendError<WatcherOut>),
}

pub type RWWatcherHealth = Arc<RwLock<WatchHealth>>;

pub struct FsWatcher {
    pub debouncer: FsWatchDebouncer,
    pub receiver: NotifyReceiver,
    pub watch_health: RWWatcherHealth,
    pub options: FsWatchOptions,
}

impl FsWatcher {
    pub fn watch<'a>(&mut self) -> std::result::Result<(), FsWatchError> {
        let recursive_mode = match self.options.is_recursive {
            false => RecursiveMode::NonRecursive,
            true => RecursiveMode::Recursive,
        };
        self.debouncer
            .watcher()
            .watch(&Path::new(&self.options.path), recursive_mode)
            .map_err(|e| FsWatchError::InitError(e.to_string()))?;
        Ok(())
    }
}

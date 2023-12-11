// use std::sync::{Arc, RwLock};

use notify::FsEventWatcher;
use notify_debouncer_full::{
    // new_debouncer,
    DebouncedEvent,
    Debouncer,
    FileIdMap,
};
use tokio::sync::mpsc::Receiver;

pub type WatcherOut = Result<Vec<DebouncedEvent>, Vec<notify::Error>>;
pub type NotifyReceiver = Receiver<WatcherOut>;
pub type FsWatchDebouncer = Debouncer<FsEventWatcher, FileIdMap>;

// pub type RWWatchHealth = Arc<RwLock<Result<(), String>>>;

use fs_watch_tokio::builder::FsWatcherBuilder;

#[tokio::main]
async fn main() -> Result<(), String> {
    let watcher = FsWatcherBuilder::new()
        .debounce(1)
        .recursive(false)
        .build()
        .unwrap();
    let _ = watcher.watch().map_err(|e| e.to_string());

    for event in watcher.receiver.next() {
        event
    }

    Ok(())
}

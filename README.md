# wineventhook

[![CI](https://github.com/OpenByteDev/wineventhook-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/OpenByteDev/wineventhook-rs/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/wineventhook.svg)](https://crates.io/crates/wineventhook)
[![Documentation](https://docs.rs/wineventhook/badge.svg)](https://docs.rs/wineventhook)
[![dependency status](https://deps.rs/repo/github/openbytedev/wineventhook-rs/status.svg)](https://deps.rs/repo/github/openbytedev/wineventhook-rs)
[![MIT](https://img.shields.io/crates/l/wineventhook.svg)](https://github.com/OpenByteDev/wineventhook-rs/blob/master/LICENSE)

A rusty wrapper over SetWinEventHook and UnhookWinEvent.

# Example
This example shows how to listen for all window events and print them to the console.
```rust
use wineventhook::{EventFilter, WindowEventHook};

#[tokio::main]
async fn main() {
    // Create a new hook
    let (event_tx, mut event_rx) = tokio::sync::mpsc::unbounded_channel();
    let hook = WindowEventHook::hook(
        EventFilter::default(),
        event_tx,
    ).await.unwrap();
    
    // Wait and print events
    while let Some(event) = event_rx.recv().await {
        println!("{:#?}", event);
    }
    
    // Unhook the hook
    hook.unhook().await.unwrap();
}
```

## License
Licensed under MIT license ([LICENSE](https://github.com/OpenByteDev/wineventhook-rs/blob/master/LICENSE) or http://opensource.org/licenses/MIT)

use wineventhook::{EventFilter, WindowEventHook};

#[tokio::main]
async fn main() {
    // Create a new hook
    let (event_tx, mut event_rx) = tokio::sync::mpsc::unbounded_channel();
    let hook = WindowEventHook::hook(EventFilter::default(), event_tx)
        .await
        .unwrap();

    // Wait and print events
    while let Some(event) = event_rx.recv().await {
        println!("{:#?}", event);
    }

    // Unhook the hook
    hook.unhook().await.unwrap();
}

use std::{io, mem::MaybeUninit};

use winapi::um::winuser::GetCursorPos;
use wineventhook::{raw_event, AccessibleObjectId, EventFilter, WindowEventHook};

#[tokio::main]
async fn main() {
    let (event_tx, mut event_rx) = tokio::sync::mpsc::unbounded_channel();
    let hook = WindowEventHook::hook(
        EventFilter::default().event(raw_event::OBJECT_LOCATIONCHANGE),
        event_tx,
    )
    .await
    .unwrap();

    while let Some(event) = event_rx.recv().await {
        if event.object_type() == AccessibleObjectId::Cursor {
            let mut pos = MaybeUninit::uninit();
            let result = unsafe { GetCursorPos(pos.as_mut_ptr()) };
            if result != 0 {
                let pos = unsafe { pos.assume_init() };
                println!("[{}, {}]", pos.x, pos.y);
            } else {
                panic!("GetCursorPos failed: {}", io::Error::last_os_error());
            }
        }
    }

    hook.unhook().await.unwrap();
}

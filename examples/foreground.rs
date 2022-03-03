use std::{
    io,
    ptr::{self, NonNull},
};

use winapi::{
    shared::windef::HWND,
    um::{winuser::{GetWindowTextLengthW, GetWindowTextW}, errhandlingapi::{SetLastError, GetLastError}},
};
use wineventhook::{raw_event, AccessibleObjectId, EventFilter, MaybeKnown, WindowEventHook};

#[tokio::main]
async fn main() {
    let (event_tx, mut event_rx) = tokio::sync::mpsc::unbounded_channel();
    let hook = WindowEventHook::hook(
        *EventFilter::default().event(raw_event::SYSTEM_FOREGROUND),
        event_tx,
    )
    .await
    .unwrap();

    while let Some(event) = event_rx.recv().await {
        if matches!(
            event.object_type(),
            MaybeKnown::Known(AccessibleObjectId::Window)
        ) {
            let title = get_window_text(
                event
                    .window_handle()
                    .map_or_else(ptr::null_mut, NonNull::as_ptr),
            )
            .unwrap();
            println!("{}", title);
        }
    }

    hook.unhook().await.unwrap();
}

fn get_window_text_length(window: HWND) -> io::Result<usize> {
    unsafe { SetLastError(0) };
    let result = unsafe { GetWindowTextLengthW(window) };
    if result == 0 && unsafe { GetLastError() } != 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(result as _)
    }
}

fn get_window_text(window: HWND) -> io::Result<String> {
    let length = get_window_text_length(window)?;
    if length == 0 {
        return Ok(String::new());
    }
    
    let mut text = vec![0u16; length + 1];
    let result = unsafe { GetWindowTextW(window, text.as_mut_ptr(), text.len() as i32) };
    if result != 0 {
        let text = String::from_utf16_lossy(&text[..text.len()-1]);
        Ok(text)
    } else {
        Err(io::Error::last_os_error())
    }
}

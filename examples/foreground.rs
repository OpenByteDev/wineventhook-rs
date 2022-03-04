use std::{
    io,
    num::NonZeroUsize,
    ptr::{self, NonNull},
};

use winapi::{
    shared::windef::HWND,
    um::{
        errhandlingapi::{GetLastError, SetLastError},
        winuser::{GetWindowTextLengthW, GetWindowTextW},
    },
};
use wineventhook::{raw_event, AccessibleObjectId, EventFilter, MaybeKnown, WindowEventHook};

#[tokio::main]
async fn main() {
    let (event_tx, mut event_rx) = tokio::sync::mpsc::unbounded_channel();
    let hook = WindowEventHook::hook(
        EventFilter::default().event(raw_event::SYSTEM_FOREGROUND),
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
            println!("{:?}", title);
        }
    }

    hook.unhook().await.unwrap();
}

fn get_window_text_length(window: HWND) -> io::Result<Option<NonZeroUsize>> {
    unsafe { SetLastError(0) };
    let result = unsafe { GetWindowTextLengthW(window) };
    if result == 0 && unsafe { GetLastError() } != 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(NonZeroUsize::new(result as usize))
    }
}

fn get_window_text(window: HWND) -> io::Result<Option<String>> {
    let text_len = if let Some(length) = get_window_text_length(window)? {
        length.get()
    } else {
        return Ok(None);
    };

    let mut text = Vec::with_capacity(text_len + 1); // +1 for null terminator
    let result = unsafe { GetWindowTextW(window, text.as_mut_ptr(), text.capacity() as i32) };
    if result != 0 {
        unsafe { text.set_len(text_len) };
        let text = String::from_utf16_lossy(&text);
        Ok(Some(text))
    } else {
        Err(io::Error::last_os_error())
    }
}

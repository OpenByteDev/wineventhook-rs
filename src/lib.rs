#![cfg(windows)]
#![warn(
    unsafe_op_in_unsafe_fn,
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    rust_2018_idioms,
    clippy::todo,
    clippy::manual_assert,
    clippy::must_use_candidate,
    clippy::inconsistent_struct_constructor,
    clippy::wrong_self_convention,
    clippy::new_without_default,
    rustdoc::broken_intra_doc_links,
    rustdoc::private_intra_doc_links
)]
#![allow(
    clippy::module_inception,
    clippy::module_name_repetitions,
    clippy::missing_errors_doc,
    clippy::borrow_as_ptr
)]

//! A rusty wrapper over the window event API specifically [`SetWinEventHook`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwineventhook) and [`UnhookWinEvent`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-unhookwinevent).
//!
//! For the type of events that can be hooked, see [`WindowEvent`](crate::WindowEvent).
//!
//! # Example
//! This example shows how to listen for all window events and print them to the console.
//! ```rust no_run
//! use wineventhook::{EventFilter, WindowEventHook};
//!
//! #[tokio::main]
//! async fn main() {
//!     // Create a new hook
//!     let (event_tx, mut event_rx) = tokio::sync::mpsc::unbounded_channel();
//!     let hook = WindowEventHook::hook(
//!         EventFilter::default(),
//!         event_tx,
//!     ).await.unwrap();
//!     
//!     // Wait and print events
//!     while let Some(event) = event_rx.recv().await {
//!         println!("{:#?}", event);
//!     }
//!     
//!     // Unhook the hook
//!     hook.unhook().await.unwrap();
//! }
//! ```

pub(crate) mod message_loop;
/// Module containing the raw event codes and ranges.
pub mod raw_event;

mod event;
pub use event::*;

mod hook;
pub use hook::*;

#[cfg(test)]
mod tests {
    use std::{ptr::NonNull, time::Instant};

    use winapi::um::processthreadsapi::GetCurrentThreadId;

    use crate::{
        message_loop::MessageOnlyWindow, raw_event, AccessibleObjectId, EventFilter, MaybeKnown,
        ObjectWindowEvent, WindowEventHook, WindowEventType,
    };

    #[tokio::test]
    async fn recv_object_create_on_window_create() {
        let (event_tx, mut event_rx) = tokio::sync::mpsc::unbounded_channel();
        let hook = WindowEventHook::hook(
            EventFilter::default()
                .event(raw_event::OBJECT_CREATE)
                .skip_own_process(false)
                .skip_own_thread(false),
            event_tx,
        )
        .await
        .unwrap();

        let window = MessageOnlyWindow::new().unwrap();
        let window_thread_id = unsafe { GetCurrentThreadId() };

        while let Some(event) = event_rx.recv().await {
            if event.event_type()
                == WindowEventType::Object(MaybeKnown::Known(ObjectWindowEvent::Create))
                && event.thread_id() == window_thread_id
            {
                assert_eq!(event.window_handle(), NonNull::new(window.handle()));
                assert_eq!(event.child_id(), None);
                assert_eq!(
                    event.object_type(),
                    MaybeKnown::Known(AccessibleObjectId::Window)
                );
                assert!(event.timestamp() <= Instant::now());
                break;
            }
        }

        hook.unhook().await.unwrap();
    }

    #[tokio::test]
    async fn can_unhook_with_unread_events() {
        let (event_tx, _event_rx) = tokio::sync::mpsc::unbounded_channel();
        let hook = WindowEventHook::hook(EventFilter::default(), event_tx)
            .await
            .unwrap();

        // generate some events
        for _ in 0..100 {
            MessageOnlyWindow::new().unwrap().destroy().unwrap();
        }

        hook.unhook().await.unwrap();
    }

    #[tokio::test]
    async fn can_have_multiple_hooks() {
        let (event_tx, _event_rx) = tokio::sync::mpsc::unbounded_channel();
        let hook1 = WindowEventHook::hook(EventFilter::default(), event_tx.clone())
            .await
            .unwrap();
        let hook2 = WindowEventHook::hook(EventFilter::default(), event_tx)
            .await
            .unwrap();

        // generate some events
        for _ in 0..100 {
            MessageOnlyWindow::new().unwrap().destroy().unwrap();
        }

        hook1.unhook().await.unwrap();
        hook2.unhook().await.unwrap();
    }
}

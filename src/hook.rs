use std::{fmt::Debug, hash::Hash, io, cell::OnceCell, num::NonZeroU32, ops::Range, ptr};

use winapi::{
    shared::{
        minwindef::{DWORD, FALSE},
        windef::{HWINEVENTHOOK, HWND},
    },
    um::{
        winnt::LONG,
        winuser::{SetWinEventHook, UnhookWinEvent},
    },
};

use crate::{message_loop::run_dummy_message_loop, raw_event, RawWindowEvent, WindowEvent};

thread_local! {
    static HOOK_EVENT_TX: OnceCell<(tokio::sync::mpsc::UnboundedSender<WindowEvent>, EventPredicate)> = OnceCell::new();
}

extern "system" fn win_event_hook_callback(
    hook: HWINEVENTHOOK,
    event: DWORD,
    h_wnd: HWND,
    id_object: LONG,
    id_child: LONG,
    event_thread: DWORD,
    event_time: DWORD,
) {
    HOOK_EVENT_TX.with(|event_tx| {
        if let Some((event_tx, predicate)) = event_tx.get() {
            let event = WindowEvent::from_raw(RawWindowEvent {
                hook_handle: hook,
                event_id: event,
                window_handle: h_wnd,
                object_id: id_object,
                child_id: id_child,
                thread_id: event_thread,
                timestamp: event_time,
            });
            if predicate(&event) {
                let _ = event_tx.send(event);
            }
        }
    });
}

#[derive(Debug)]
/// A hook with a message loop that listens for window events.
pub struct WindowEventHook {
    abort_tx: tokio::sync::oneshot::Sender<()>,
    event_thread: async_thread::JoinHandle<Result<(), std::io::Error>>,
}

impl WindowEventHook {
    /// Creates a new [`WindowEventHook`] that listens to the events matching the given filter.
    /// An [`WindowEvent`] is sent to the given `event_tx` whenever an event matching the filter is received.
    /// A dedicated event loop thread is created to be able to receive events.
    pub async fn hook(
        filter: EventFilter,
        event_tx: tokio::sync::mpsc::UnboundedSender<WindowEvent>,
    ) -> Result<Self, io::Error> {
        let (handle_tx, handle_rx) = tokio::sync::oneshot::channel();
        let (abort_tx, abort_rx) = tokio::sync::oneshot::channel();

        let event_thread = async_thread::Builder::new()
            .name("WindowEventHook EventLoop".to_string())
            .spawn(move || {
                let mut flags = HookFlags::OUT_OF_CONTEXT;
                if filter.skip_own_process {
                    flags |= HookFlags::SKIP_OWN_PROCESS;
                }
                if filter.skip_own_thread {
                    flags |= HookFlags::SKIP_OWN_THREAD;
                }

                let hook = unsafe {
                    SetWinEventHook(
                        filter.events.0 as _,
                        filter.events.1 as _,
                        ptr::null_mut(),
                        Some(win_event_hook_callback),
                        filter.process_id,
                        filter.thread_id,
                        flags.bits,
                    )
                };

                let hook_result = if !hook.is_null() {
                    HOOK_EVENT_TX.with(|tx| {
                        tx.set((event_tx, filter.predicate.get()))
                            .map_err(|_| ())
                            .unwrap();
                    });
                    Ok(())
                } else {
                    Err(io::Error::last_os_error())
                };

                handle_tx.send(hook_result).unwrap();

                run_dummy_message_loop(abort_rx).unwrap();

                let unhook_result = unsafe { UnhookWinEvent(hook) };
                if unhook_result == FALSE {
                    Err(io::Error::last_os_error())
                } else {
                    Ok(())
                }
            })
            .unwrap();

        handle_rx.await.unwrap().map(|_| Self {
            abort_tx,
            event_thread,
        })
    }

    /// Unhook this hook and stop the event loop.
    pub async fn unhook(self) -> Result<(), io::Error> {
        self.abort_tx.send(()).unwrap();
        self.event_thread.join().await.unwrap()
    }
}

/// A filter for window events.
pub type EventPredicate = fn(&WindowEvent) -> bool;

#[derive(Clone, Copy)]
struct EventPredicateHolder(Option<EventPredicate>);

impl EventPredicateHolder {
    fn new() -> Self {
        Self(None)
    }
    fn set(&mut self, predicate: EventPredicate) {
        self.0 = Some(predicate);
    }
    fn get(&self) -> EventPredicate {
        self.0.unwrap_or(|_| true)
    }
}

impl Debug for EventPredicateHolder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0.as_ref().map(|_| "some predicate"))
    }
}

#[derive(Debug, Clone, Copy)]
/// A filter for window events to be received.
pub struct EventFilter {
    events: (i32, i32), // Range<i32> is not Copy
    skip_own_thread: bool,
    skip_own_process: bool,
    thread_id: u32,
    process_id: u32,
    predicate: EventPredicateHolder,
}

impl Default for EventFilter {
    fn default() -> Self {
        Self {
            events: (raw_event::MIN, raw_event::MAX),
            skip_own_thread: true,
            skip_own_process: false,
            thread_id: 0,
            process_id: 0,
            predicate: EventPredicateHolder::new(),
        }
    }
}

impl EventFilter {
    /// Set the event that should be received by the hook.
    #[must_use]
    pub fn event(self, event: i32) -> Self {
        self.events(Range {
            start: event,
            end: event,
        })
    }

    /// Set the range of events that should be received by the hook.
    #[must_use]
    pub fn events(mut self, events: Range<i32>) -> Self {
        self.events = (events.start, events.end);
        self
    }

    /// If true, the event will be skipped if it is generated by the current thread.
    #[must_use]
    pub fn skip_own_process(mut self, value: bool) -> Self {
        self.skip_own_process = value;
        self
    }

    /// If true, the event will be skipped if it is generated by the event loop thread.
    #[must_use]
    pub fn skip_own_thread(mut self, value: bool) -> Self {
        self.skip_own_thread = value;
        self
    }

    /// Only include events from the process with the given id.
    #[must_use]
    pub fn process(mut self, id: NonZeroU32) -> Self {
        self.process_id = id.get();
        self
    }

    /// Only include events from the thread with the given id.
    #[must_use]
    pub fn thread(mut self, id: NonZeroU32) -> Self {
        self.thread_id = id.get();
        self
    }

    /// Include events from all processes.
    /// To receive events from the current process `skip_own_process` must be set to false.
    #[must_use]
    pub fn all_processes(mut self) -> Self {
        self.process_id = 0;
        self
    }

    /// Include events from all threads.
    /// To receive events from the event loop thread `skip_own_thread` must be set to false.
    #[must_use]
    pub fn all_threads(mut self) -> Self {
        self.thread_id = 0;
        self
    }

    /// Set the predicate that determines whether an event should be accepted.
    #[must_use]
    pub fn predicate(mut self, predicate: EventPredicate) -> EventFilter {
        self.predicate.set(predicate);
        self
    }
}

bitflags::bitflags! {
    struct HookFlags: u32 {
        /// The callback function is not mapped into the address space of the process that generates the event.
        /// Because the hook function is called across process boundaries, the system must queue events.
        /// Although this method is asynchronous, events are guaranteed to be in sequential order.
        const OUT_OF_CONTEXT = 0x0000;

        /// Prevents this instance of the hook from receiving the events that are generated by the thread that is registering this hook.
        const SKIP_OWN_THREAD = 0x0001;

        /// Prevents this instance of the hook from receiving the events that are generated by threads in this process.
        /// This flag does not prevent threads from generating events.
        const SKIP_OWN_PROCESS = 0x0002;

        /// The DLL that contains the callback function is mapped into the address space of the process that generates the event.
        /// With this flag, the system sends event notifications to the callback function as they occur.
        /// The hook function must be in a DLL when this flag is specified.
        /// This flag has no effect when both the calling process and the generating process are not 32-bit or 64-bit processes, or when the generating process is a console application.
        const IN_CONTEXT = 0x0004;
    }
}

impl Default for HookFlags {
    fn default() -> Self {
        Self::OUT_OF_CONTEXT
    }
}

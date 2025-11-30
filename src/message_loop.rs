use std::{io, mem, ptr, time::Duration};

use tokio::sync::oneshot;
use winapi::{
    shared::{
        minwindef::FALSE,
        windef::{HWND, POINT},
    },
    um::{
        winbase::WAIT_FAILED,
        winuser::{
            CreateWindowExA, DestroyWindow, HWND_MESSAGE, MSG, MsgWaitForMultipleObjects,
            PM_REMOVE, PeekMessageW, QS_ALLEVENTS,
        },
    },
};

pub fn run_dummy_message_loop(mut abort_receiver: oneshot::Receiver<()>) -> io::Result<()> {
    let window = MessageOnlyWindow::new()?;

    let mut msg = MSG {
        hwnd: ptr::null_mut(),
        message: 0,
        wParam: 0,
        lParam: 0,
        time: 0,
        pt: POINT { x: 0, y: 0 },
    };

    while matches!(
        abort_receiver.try_recv(),
        Err(oneshot::error::TryRecvError::Empty)
    ) {
        while unsafe { PeekMessageW(&mut msg, ptr::null_mut(), 0, 0, PM_REMOVE) } != 0 {}

        if unsafe {
            MsgWaitForMultipleObjects(
                0,
                ptr::null(),
                FALSE,
                Duration::from_secs(1).as_millis() as _,
                QS_ALLEVENTS,
            )
        } == WAIT_FAILED
        {
            break;
        }
    }

    window.destroy()
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct MessageOnlyWindow(HWND);

impl MessageOnlyWindow {
    pub fn new() -> io::Result<Self> {
        let handle = unsafe {
            CreateWindowExA(
                0,
                c"STATIC".as_ptr().cast(), // pre-defined window class for buttons and other ui elements
                ptr::null_mut(),
                0,
                0,
                0,
                0,
                0,
                HWND_MESSAGE, // creates a message-only window
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
            )
        };

        if handle.is_null() {
            return Err(io::Error::last_os_error());
        }

        Ok(Self(handle.cast()))
    }

    pub fn handle(&self) -> HWND {
        self.0
    }

    pub fn destroy(mut self) -> io::Result<()> {
        let result = unsafe { self.destroy_core() };
        mem::forget(self);
        result
    }

    unsafe fn destroy_core(&mut self) -> io::Result<()> {
        let result = unsafe { DestroyWindow(self.handle()) };

        if result == 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

impl Drop for MessageOnlyWindow {
    fn drop(&mut self) {
        let result = unsafe { self.destroy_core() };
        debug_assert!(
            result.is_ok(),
            "MessageOnlyWindow::destroy() failed with {:?}",
            result.unwrap_err()
        );
    }
}

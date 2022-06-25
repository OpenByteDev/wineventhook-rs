use std::ops::Range;

/// The lowest possible event value.
pub const MIN: i32 = 0x00000001;

/// The lowest system event value.
pub const SYSTEM_START: i32 = 0x0001;

/// A sound has been played.
/// The system sends this event when a system sound, such as one for a menu, is played even if no sound is audible (for example, due to the lack of a sound file or a sound card).
/// Servers send this event whenever a custom UI element generates a sound.
pub const SYSTEM_SOUND: i32 = 0x0001;
/// An alert has been generated.
pub const SYSTEM_ALERT: i32 = 0x0002;
/// The foreground window has changed.
/// The system sends this event even if the foreground window has changed to another window in the same thread.
pub const SYSTEM_FOREGROUND: i32 = 0x0003;
/// A menu item on the menu bar has been selected.
/// The system sends this event for standard menus, which are identified by `HMENU`, created using menu-template resources or Win32 menu API elements.
/// Servers send this event for custom menus, which are user interface elements that function as menus but are not created in the standard way.
/// For this event, the `WinEventProc` callback function's `hwnd`, `idObject`, and `idChild` parameters refer to the control that contains the menu bar or the control that activates the context menu.
/// The hwnd parameter is the handle to the window related to the event. The `idObject` parameter is `OBJID_MENU` or `OBJID_SYSMENU` for a menu, or `OBJID_WINDOW` for a pop-up menu.
/// The `idChild` parameter is `CHILDID_SELF`.
/// The system triggers more than one [`SYSTEM_MENUSTART`] event that does not always correspond with the [`SYSTEM_MENUEND`] event.
pub const SYSTEM_MENUSTART: i32 = 0x0004;
/// A menu from the menu bar has been closed.
/// The system sends this event for standard menus; servers send it for custom menus.
/// For this event, the `WinEventProc` callback function's `hwnd`, `idObject, and `idChild` parameters refer to the control that contains the menu bar or the control that activates the context menu.
/// The hwnd parameter is the handle to the window that is related to the event.
/// The idObject parameter is `OBJID_MENU` or `OBJID_SYSMENU` for a menu, or `OBJID_WINDOW` for a pop-up menu.
/// The idChild parameter is `CHILDID_SELF`.
pub const SYSTEM_MENUEND: i32 = 0x0005;
/// A pop-up menu has been displayed.
/// The system sends this event for standard menus, which are identified by `HMENU`, and are created using menu-template resources or Win32 menu functions.
pub const SYSTEM_MENUPOPUPSTART: i32 = 0x0006;
/// A pop-up menu has been closed. The system sends this event for standard menus; servers send it for custom menus.
/// When a pop-up menu is closed, the client receives this message, and then the [`SYSTEM_MENUEND`] event.
/// This event is not sent consistently by the system.
pub const SYSTEM_MENUPOPUPEND: i32 = 0x0007;
/// A window has received mouse capture.
/// This event is sent by the system, never by servers.
pub const SYSTEM_CAPTURESTART: i32 = 0x0008;
/// A window has lost mouse capture.
/// This event is sent by the system, never by servers.
pub const SYSTEM_CAPTUREEND: i32 = 0x0009;
/// A window is being moved or resized.
pub const SYSTEM_MOVESIZESTART: i32 = 0x000A;
/// The movement or resizing of a window has finished.
pub const SYSTEM_MOVESIZEEND: i32 = 0x000B;
/// A window has entered context-sensitive Help mode.
/// This event is not sent consistently by the system.
pub const SYSTEM_CONTEXTHELPSTART: i32 = 0x000C;
/// A window has exited context-sensitive Help mode.
/// This event is not sent consistently by the system.
pub const SYSTEM_CONTEXTHELPEND: i32 = 0x000D;
/// An application is about to enter drag-and-drop mode.
/// Applications that support drag-and-drop operations must send this event because the system does not send it.
pub const SYSTEM_DRAGDROPSTART: i32 = 0x000E;
/// An application is about to exit drag-and-drop mode.
/// Applications that support drag-and-drop operations must send this event; the system does not send this event.
pub const SYSTEM_DRAGDROPEND: i32 = 0x000F;
/// A dialog box has been displayed.
/// The system sends this event for standard dialog boxes, which are created using resource templates or Win32 dialog box functions.
/// Servers send this event for custom dialog boxes, which are windows that function as dialog boxes but are not created in the standard way.
/// This event is not sent consistently by the system.
pub const SYSTEM_DIALOGSTART: i32 = 0x0010;
/// An application is about to exit drag-and-drop mode.
/// Applications that support drag-and-drop operations must send this event; the system does not send this event.
pub const SYSTEM_DIALOGEND: i32 = 0x0011;
/// Scrolling has started on a scroll bar.
/// The system sends this event for standard scroll bar controls and for scroll bars attached to a window.
/// Servers send this event for custom scroll bars, which are user interface elements that function as scroll bars but are not created in the standard way.
/// The idObject parameter that is sent to the `WinEventProc` callback function is `OBJID_HSCROLL` for horizontal scrolls bars, and `OBJID_VSCROLL` for vertical scroll bars.
pub const SYSTEM_SCROLLINGSTART: i32 = 0x0012;
/// Scrolling has ended on a scroll bar.
/// This event is sent by the system for standard scroll bar controls and for scroll bars that are attached to a window.
/// Servers send this event for custom scroll bars, which are user interface elements that function as scroll bars but are not created in the standard way.
/// The idObject parameter that is sent to the WinEventProc callback function is `OBJID_HSCROLL` for horizontal scroll bar, and `OBJID_VSCROLL` for vertical scroll bars.
pub const SYSTEM_SCROLLINGEND: i32 = 0x0013;
/// The user has pressed ALT+TAB, which activates the switch window.
/// This event is sent by the system, never by servers.
/// The hwnd parameter of the `WinEventProc` callback function identifies the window to which the user is switching.
/// If only one application is running when the user presses ALT+TAB, the system sends an [`SYSTEM_SWITCHEND`] event without a corresponding [`SYSTEM_SWITCHSTART`] event.
pub const SYSTEM_SWITCHSTART: i32 = 0x0014;
/// The user has released ALT+TAB.
/// This event is sent by the system, never by servers.
/// The hwnd parameter of the WinEventProc callback function identifies the window to which the user has switched.
/// If only one application is running when the user presses ALT+TAB, the system sends this event without a corresponding [`SYSTEM_SWITCHSTART`] event.
pub const SYSTEM_SWITCHEND: i32 = 0x0015;
/// A window object is about to be minimized.
pub const SYSTEM_MINIMIZESTART: i32 = 0x0016;
/// A window object is about to be restored.
pub const SYSTEM_MINIMIZEEND: i32 = 0x0017;
/// The active desktop has been switched.
pub const SYSTEM_DESKTOPSWITCH: i32 = 0x0020;
/// The highest system event value.
pub const SYSTEM_END: i32 = 0x00FF;

/// The lowest event value reserved for OEMs.
pub const OEM_DEFINED_START: i32 = 0x0101;
/// The highest event value reserved for OEMs.
pub const OEM_DEFINED_END: i32 = 0x01FF;

#[allow(missing_docs)]
pub const CONSOLE_START: i32 = 0x4001;
#[allow(missing_docs)]
pub const CONSOLE_CARET: i32 = 0x4001;
#[allow(missing_docs)]
pub const CONSOLE_UPDATE_REGION: i32 = 0x4002;
#[allow(missing_docs)]
pub const CONSOLE_UPDATE_SIMPLE: i32 = 0x4003;
#[allow(missing_docs)]
pub const CONSOLE_UPDATE_SCROLL: i32 = 0x4004;
#[allow(missing_docs)]
pub const CONSOLE_LAYOUT: i32 = 0x4005;
#[allow(missing_docs)]
pub const CONSOLE_START_APPLICATION: i32 = 0x4006;
#[allow(missing_docs)]
pub const CONSOLE_END_APPLICATION: i32 = 0x4007;
#[allow(missing_docs)]
pub const CONSOLE_END: i32 = 0x40FF;

/// The lowest event value reserved for UI Automation event identifiers.
pub const UIA_EVENTID_START: i32 = 0x4E00;
/// The highest event value reserved for UI Automation event identifiers.
pub const UIA_EVENTID_END: i32 = 0x4EFF;

/// The lowest event value reserved for UI Automation property-changed event identifiers.
pub const UIA_PROPID_START: i32 = 0x7500;
/// The highest event value reserved for UI Automation property-changed event identifiers.
pub const UIA_PROPID_END: i32 = 0x75FF;

/// The lowest object event value.
pub const OBJECT_START: i32 = 0x8000;
/// An object has been created.
/// The system sends this event for the following user interface elements:
/// caret, header control, list-view control, tab control, toolbar control, tree view control, and window object.
pub const OBJECT_CREATE: i32 = 0x8000;
/// An object has been destroyed.
/// The system sends this event for the following user interface elements:
/// caret, header control, list-view control, tab control, toolbar control, tree view control, and window object.
pub const OBJECT_DESTROY: i32 = 0x8001;
/// A hidden object is shown.
/// The system sends this event for the following user interface elements: caret, cursor, and window object.
pub const OBJECT_SHOW: i32 = 0x8002;
/// An object is hidden.
/// The system sends this event for the following user interface elements: caret and cursor.
pub const OBJECT_HIDE: i32 = 0x8003;
/// A container object has added, removed, or reordered its children.
/// The system sends this event for the following user interface elements: header control, list-view control, toolbar control, and window object.
pub const OBJECT_REORDER: i32 = 0x8004;
/// An object has received the keyboard focus.
/// The system sends this event for the following user interface elements:
/// list-view control, menu bar, pop-up menu, switch window, tab control, tree view control, and window object.
pub const OBJECT_FOCUS: i32 = 0x8005;
/// The selection within a container object has changed.
/// The system sends this event for the following user interface elements:
/// list-view control, tab control, tree view control, and window object.
pub const OBJECT_SELECTION: i32 = 0x8006;
/// A child within a container object has been added to an existing selection.
/// The system sends this event for the following user interface elements:
/// list box, list-view control, and tree view control.
pub const OBJECT_SELECTIONADD: i32 = 0x8007;
/// An item within a container object has been removed from the selection.
/// The system sends this event for the following user interface elements:
/// list box, list-view control, and tree view control.
pub const OBJECT_SELECTIONREMOVE: i32 = 0x8008;
/// Numerous selection changes have occurred within a container object.
/// The system sends this event for list boxes.
pub const OBJECT_SELECTIONWITHIN: i32 = 0x8009;
/// An object's state has changed.
/// The system sends this event for the following user interface elements:
/// check box, combo box, header control, push button, radio button, scroll bar, toolbar control, tree view control, up-down control, and window object.
pub const OBJECT_STATECHANGE: i32 = 0x800A;
/// An object has changed location, shape, or size.
/// The system sends this event for the following user interface elements:
/// caret and window objects.
pub const OBJECT_LOCATIONCHANGE: i32 = 0x800B;
/// An object's Name property has changed.
/// The system sends this event for the following user interface elements: check box, cursor, list-view control, push button, radio button, status bar control, tree view control, and window object.
pub const OBJECT_NAMECHANGE: i32 = 0x800C;
/// An object's Description property has changed.
pub const OBJECT_DESCRIPTIONCHANGE: i32 = 0x800D;
/// An object's Value property has changed.
/// The system sends this event for the user interface elements that include the scroll bar and the following controls:
/// edit, header, hot key, progress bar, slider, and up-down.
pub const OBJECT_VALUECHANGE: i32 = 0x800E;
/// An object has a new parent object.
pub const OBJECT_PARENTCHANGE: i32 = 0x800F;
/// An object's Help property has changed.
pub const OBJECT_HELPCHANGE: i32 = 0x8010;
/// An object's DefaultAction property has changed. The system sends this event for dialog boxes.
pub const OBJECT_DEFACTIONCHANGE: i32 = 0x8011;
/// An object's KeyboardShortcut property has changed.
pub const OBJECT_ACCELERATORCHANGE: i32 = 0x8012;
/// An object has been invoked; for example, the user has clicked a button.
/// This event is supported by common controls and is used by UI Automation.
pub const OBJECT_INVOKED: i32 = 0x8013;
/// An object's text selection has changed.
/// This event is supported by common controls and is used by UI Automation.
pub const OBJECT_TEXTSELECTIONCHANGED: i32 = 0x8014;
/// A window object's scrolling has ended.
/// Unlike [`SYSTEM_SCROLLINGEND`], this event is associated with the scrolling window.
/// Whether the scrolling is horizontal or vertical scrolling, this event should be sent whenever the scroll action is completed.
pub const OBJECT_CONTENTSCROLLED: i32 = 0x8015;
/// A preview rectangle is being displayed.
pub const SYSTEM_ARRANGMENTPREVIEW: i32 = 0x8016;
/// Sent when a window is cloaked.
/// A cloaked window still exists, but is invisible to the user.
pub const OBJECT_CLOAKED: i32 = 0x8017;
/// Sent when a window is uncloaked.
/// A cloaked window still exists, but is invisible to the user.
pub const OBJECT_UNCLOAKED: i32 = 0x8018;
/// An object that is part of a live region has changed.
/// A live region is an area of an application that changes frequently and/or asynchronously.
pub const OBJECT_LIVEREGIONCHANGED: i32 = 0x8019;
/// A window that hosts other accessible objects has changed the hosted objects.
/// A client might need to query the host window to discover the new hosted objects, especially if the client has been monitoring events from the window.
/// A hosted object is an object from an accessibility framework (MSAA or UI Automation) that is different from that of the host.
/// Changes in hosted objects that are from the same framework as the host should be handed with the structural change events, such as [`OBJECT_CREATE`] for MSAA.
pub const OBJECT_HOSTEDOBJECTSINVALIDATED: i32 = 0x8020;
/// The user started to drag an element.
pub const OBJECT_DRAGSTART: i32 = 0x8021;
/// The user has ended a drag operation before dropping the dragged element on a drop target.
pub const OBJECT_DRAGCANCEL: i32 = 0x8022;
/// The user dropped an element on a drop target.
pub const OBJECT_DRAG_COMPLETE: i32 = 0x8023;
/// The user dragged an element into a drop target's boundary.
pub const OBJECT_DRAGENTER: i32 = 0x8024;
/// The user dragged an element out of a drop target's boundary.
pub const OBJECT_DRAGLEAVE: i32 = 0x8025;
/// The user dropped an element on a drop target.
pub const OBJECT_DRAGDROPPED: i32 = 0x8026;
/// An IME window has become visible.
pub const OBJECT_IME_SHOW: i32 = 0x8027;
/// An IME window has become hidden.
pub const OBJECT_IME_HIDE: i32 = 0x8028;
/// The size or position of an IME window has changed.
pub const OBJECT_IME_CHANGE: i32 = 0x8029;
/// The conversion target within an IME composition has changed.
/// The conversion target is the subset of the IME composition which is actively selected as the target for user-initiated conversions.
pub const OBJECT_TEXTEDIT_CONVERSIONTARGETCHANGED: i32 = 0x8030;

/// The highest object event value.
pub const OBJECT_END: i32 = 0x80FF;

/// The lowest event value reserved for custom events allocated at runtime.
pub const ATOM_START: i32 = 0xC000;

/// The lowest event value specified by the Accessibility Interoperability Alliance (AIA) for use across the industry.
pub const AIA_START: i32 = 0xA000;
/// The highest event value specified by the Accessibility Interoperability Alliance (AIA) for use across the industry.
pub const AIA_END: i32 = 0xAFFF;

/// The highest event value reserved for custom events allocated at runtime.
pub const ATOM_END: i32 = 0xFFFF;

/// The highest possible event value.
pub const MAX: i32 = 0x7FFFFFFF;

/// Returns the range of all event values.
#[must_use]
pub fn all() -> Range<i32> {
    Range {
        start: MIN,
        end: MAX,
    }
}

/// Returns the range of all system-level events.
/// These events describe situations affecting all applications in the system.
#[must_use]
pub fn all_system() -> Range<i32> {
    Range {
        start: SYSTEM_START,
        end: SYSTEM_END,
    }
}

/// Returns the range of OEM reserved events.
/// The OEM reserved range is open to anyone who needs to use WinEvents as a communication mechanism.
/// Developers should define and publish event definitions along with their parameters (or also with associated object types) for event processing so that accidental collisions of event IDs can be avoided.
/// The IAccessible2 specification uses part of the OEM reserved range.
#[must_use]
pub fn all_oem_defined() -> Range<i32> {
    Range {
        start: OEM_DEFINED_START,
        end: OEM_DEFINED_END,
    }
}

/// Returns the range of all console-level events.
/// These events indicate changes in console windows.
#[must_use]
pub fn all_console() -> Range<i32> {
    Range {
        start: CONSOLE_START,
        end: CONSOLE_END,
    }
}

/// Returns the range of all UI Automation event IDs.
#[must_use]
pub fn all_uia_event() -> Range<i32> {
    Range {
        start: UIA_EVENTID_START,
        end: UIA_EVENTID_END,
    }
}

/// Returns the range of all UI Automation property-changed event IDs.
#[must_use]
pub fn all_uia_property_change() -> Range<i32> {
    Range {
        start: UIA_PROPID_START,
        end: UIA_PROPID_END,
    }
}

/// Returns the range of all object-level events.
/// These events pertain to situations specific to objects within one application.
#[must_use]
pub fn all_object() -> Range<i32> {
    Range {
        start: OBJECT_START,
        end: OBJECT_END,
    }
}

/// Returns the range of reserved for ATOM events.
/// The ATOM range is reserved for event IDs that are allocated at runtime through the UI Automation extensibility API.
/// Do not use the values from the ATOM range for any other purpose.
/// Using the [`GlobalAddAtom`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-globaladdatoma) function with a string GUID is the recommended method of allocating WinEvents from the ATOM range.
#[must_use]
pub fn all_atom() -> Range<i32> {
    Range {
        start: ATOM_START,
        end: ATOM_END,
    }
}

/// Returns the range of all events reserved for use by the Accessibility Interoperability Alliance (AIA).
#[must_use]
pub fn all_aia() -> Range<i32> {
    Range {
        start: AIA_START,
        end: AIA_END,
    }
}

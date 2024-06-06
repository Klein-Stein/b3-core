//! This module contains all event that can be captured.

use dpi::{PhysicalPosition, PhysicalSize};

use crate::{ActiveApplication, WindowId};

/// Life cycle events.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LifeCycle {
    /// The application has just been successfully launched.
    Started,
    /// The application is preparing to shut down.
    Finished,
}

/// Mouse buttons.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MouseButton {
    /// Left mouse button.
    Left,
    /// Middle mouse button (or mouse wheel).
    Middle,
    /// Right mouse button.
    Right,
    /// Back mouse button.
    Back,
    /// Forward mouse button.
    Forward,
    /// Other mouse button.
    Other {
        /// Other mouse button ID/number (starts with 5).
        id: u16,
    },
}

/// Mouse button state.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MouseButtonState {
    /// Mouse button has been pressed.
    Pressed,
    /// Mouse button has been released.
    Released,
}

/// Scrolling delta.
#[derive(Debug, PartialEq, PartialOrd)]
pub enum ScrollingDelta {
    /// A delta value in lines.
    Line(f32, f32),
    /// A delta value in pixels.
    Pixel(f64, f64),
}

/// The scrolling phase for a scroll or flick gesture.
#[derive(Debug, PartialEq, PartialOrd)]
pub enum ScrollingPhase {
    /// An event phase has begun.
    Started,
    /// An event phase is in progress but hasn't moved since the previous event.
    Stationary,
    /// An event phase has changed.
    Changed,
    /// The event phase ended or cancelled.
    Ended,
}

/// Mouse events.
#[derive(Debug, PartialEq, PartialOrd)]
pub enum MouseEvent {
    /// Mouse button input event.
    Input {
        /// Mouse button (see [MouseButton]).
        button: MouseButton,
        /// Mouse button state (see [MouseButtonState]).
        state:  MouseButtonState,
    },
    /// Mouse wheel scroll event.
    Scroll {
        /// Scrolling delta value.
        delta: ScrollingDelta,
        /// Scrolling phase.
        phase: ScrollingPhase,
    },
    /// Mouse cursor has been moved.
    Moved {
        /// New cursor position.
        position: PhysicalPosition<f64>,
    },
    /// Mouse position has entered into a window view frame.
    Entered,
    /// Mouse position has left a window view frame.
    Exited,
}

/// Window events.
#[derive(Debug, PartialEq, PartialOrd)]
pub enum WindowEvent {
    /// The window has been displayed.
    Showed,
    /// The window has been resized.
    Resized(PhysicalSize<u32>),
    /// The window has been moved.
    Moved(PhysicalPosition<i32>),
    /// The window has gained or lost focus.
    Focused(bool),
    /// The window scale factor has been changed.
    ScaleFactorChanged(f64),
    /// The window will be redrawn.
    RedrawRequested,
    /// The window has been closed.
    CloseRequested,
    /// The window has been destroyed.
    Destroyed,
    /// Mouse event (see [MouseEvent]).
    Mouse(MouseEvent),
}

/// Main event enumeration.
///
/// This enumeration is an entrypoint to all captured events.
#[derive(Debug, PartialEq, PartialOrd)]
pub enum Event {
    /// The event indicates that a menu item has been clicked.
    ///
    /// It stores an action name of the clicked menu item.
    Menu(String),
    /// Life cycle events (see [LifeCycle]).
    LifeCycle(LifeCycle),
    /// Window events (see [WindowEvent]).
    Window(WindowEvent, WindowId),
}

/// Event handler.
///
/// Implement this trait to capture events.
pub trait EventHandler {
    /// Override this method to capture events.
    ///
    /// # Parameters:
    /// * `app` - Active application.
    /// * `event` - [Event].
    fn on_event(&mut self, app: &mut ActiveApplication, event: Event);
}

impl<F> EventHandler for F
where
    F: FnMut(&mut ActiveApplication, Event),
{
    fn on_event(&mut self, app: &mut ActiveApplication, event: Event) { self(app, event); }
}

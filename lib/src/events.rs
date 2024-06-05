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

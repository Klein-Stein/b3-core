//! This module contains all event that can be captured.

use crate::ActiveApplication;

/// Life cycle events.
#[derive(Debug)]
pub enum LifeCycle {
    /// The application has just been successfully launched.
    Start,
    /// The application is preparing to shut down.
    Finish,
}

/// Main event enumeration.
///
/// This enumeration is an entrypoint to all captured events.
#[derive(Debug)]
pub enum Event {
    /// The event indicates that a menu item has been clicked.
    ///
    /// It stores an action name of the clicked menu item.
    Menu(String),
    /// Life cycle events (see [LifeCycle]).
    LifeCycle(LifeCycle),
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

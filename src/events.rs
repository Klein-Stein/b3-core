use crate::Application;

#[derive(Debug)]
pub enum LifeCycleEvent {
    Started,
    UpdateRequsted,
    Finished,
}

#[derive(Debug)]
pub enum MenuEvent {
    MenuItemClicked,
}

#[derive(Debug)]
pub enum Event {
    Menu(MenuEvent),
    LifeCycle(LifeCycleEvent),
}

pub trait EventHandler {
    fn on_event(&mut self, app: &Application, event: Event);
}

impl<F> EventHandler for F
where
    F: FnMut(&Application, Event),
{
    fn on_event(&mut self, app: &Application, event: Event) { self(app, event); }
}

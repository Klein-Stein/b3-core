#[derive(Debug)]
pub enum LifeCycleEvent {
    Start,
    UpdateRequst,
    Finish,
}

#[derive(Debug)]
pub enum MenuEvent {
    MenuItemClick,
}

#[derive(Debug)]
pub enum Event {
    Menu(MenuEvent),
    LifeCycle(LifeCycleEvent),
}

pub trait EventHandler {
    fn on_event(&self, event: Event);
}

impl<F> EventHandler for F
where
    F: Fn(Event),
{
    fn on_event(&self, event: Event) { self(event); }
}

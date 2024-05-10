#[derive(Debug)]
pub enum LifeCycleEvent {
    Start,
    UpdateRequst,
    Finish,
}

#[derive(Debug)]
pub enum Event {
    Menu(String),
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

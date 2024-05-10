use b3_platform::{Application, Event, LifeCycleEvent};

fn main() {
    let app = Application::new();
    app.run(|event: Event| match event {
        Event::Menu(menu_event) => println!("{:?}", menu_event),
        Event::LifeCycle(lc_event) => match lc_event {
            LifeCycleEvent::Start => println!("Applicaiton started!"),
            LifeCycleEvent::Finish => println!("Application finished!"),
            _ => {}
        },
    });
}

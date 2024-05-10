use b3_platform::{Action, Application, Event, LifeCycleEvent, Menu, MenuItem};

fn create_menu(app: &Application) -> Menu {
    let quit_menu_item = MenuItem::builder()
        .with_title("Quit")
        .with_action(Action::new_event("quit"))
        .with_macos_short_code("q")
        .build(app);
    let app_menu = Menu::builder().with_item(quit_menu_item).build(app);

    let app_menu_item = MenuItem::builder()
        .with_title("Bioma")
        .with_submenu(app_menu)
        .build(app);
    Menu::builder().with_item(app_menu_item).build(app)
}

fn main() {
    let mut app = Application::new();
    let menu = create_menu(&app);
    app.set_menu(Some(menu));
    app.run(|event: Event| match event {
        Event::Menu(action_name) => println!("The {:?} menu item clicked!", action_name),
        Event::LifeCycle(lc_event) => match lc_event {
            LifeCycleEvent::Start => println!("Applicaiton started!"),
            LifeCycleEvent::Finish => println!("Application finished!"),
            _ => {}
        },
    });
}

use b3_platform::{Action, Application, Event, LifeCycleEvent, Menu, MenuItem, Window};

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

    let mut window = Window::builder().with_title("B3").build(&app);
    window.show();
    app.add_window(window);

    app.run(|event: Event| match event {
        Event::LifeCycle(LifeCycleEvent::Start) => {}
        _ => {}
    })
}

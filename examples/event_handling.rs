use b3_platform::{Action, ActiveApplication, Application, Event, LifeCycle, Menu, MenuItem};

fn create_menu() -> Menu {
    let quit_menu_item = MenuItem::builder()
        .with_title("Quit")
        .with_action(Action::new_event("quit"))
        .with_macos_short_code("q")
        .build();
    let app_menu = Menu::builder().with_item(quit_menu_item).build();

    let app_menu_item = MenuItem::builder()
        .with_title("Bioma")
        .with_submenu(app_menu)
        .build();
    Menu::builder().with_item(app_menu_item).build()
}

fn main() {
    let app = Application::new();
    app.run(|app: &mut ActiveApplication, event: Event| match event {
        Event::Menu(action_name) => println!("The {:?} menu item clicked!", action_name),
        Event::LifeCycle(lc_event) => match lc_event {
            LifeCycle::Start => {
                println!("Applicaiton started!");
                let menu = create_menu();
                app.set_menu(Some(&menu));
            }
            LifeCycle::Finish => println!("Application finished!"),
        },
    });
}

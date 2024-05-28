//! This example shows a trivial implementation of the runnable application with
//! the main menu.

use b3_core::{
    Action,
    ActiveApplication,
    Application,
    ContextOwner,
    Event,
    LifeCycle,
    Menu,
    MenuItem,
};

fn create_menu(ctx: &impl ContextOwner) -> Menu {
    // App menu
    let quit_item = MenuItem::builder()
        .with_title("Quit")
        .with_macos_short_code("q")
        .with_action(Action::new_event("quit"))
        .build(ctx);

    let app_menu = Menu::builder().with_item(quit_item).build(ctx);

    // Main menu
    let app_item = MenuItem::builder().with_submenu(app_menu).build(ctx);
    Menu::builder().with_item(app_item).build(ctx)
}

fn main() {
    let app = Application::new().unwrap();
    let menu = create_menu(&app);
    app.run(
        move |app: &mut ActiveApplication, event: Event| match event {
            Event::Menu(action) => {
                if action == "quit" {
                    app.stop();
                }
            }
            Event::LifeCycle(LifeCycle::Start) => {
                println!("Hello, World!");
                app.set_menu(Some(&menu));
            }
            _ => (),
        },
    );
}

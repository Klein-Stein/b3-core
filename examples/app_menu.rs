use b3_platform::{
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
    let global_settings_menu_item = MenuItem::builder()
        .with_title("Global Settings...")
        .with_action(Action::Callback(|| {
            println!("`Global Settings` clicked!");
        }))
        .with_macos_short_code("P")
        .with_enabled(false)
        .build(ctx);
    let local_settigs_menu_item = MenuItem::builder()
        .with_title("Local Setting...")
        .with_action(Action::Callback(|| {
            println!("`Local Settings` clicked!");
        }))
        .with_macos_short_code("q")
        .build(ctx);
    let settings_menu = Menu::builder()
        .with_item(global_settings_menu_item)
        .with_item(local_settigs_menu_item)
        .build(ctx);

    let settings_menu_item = MenuItem::builder()
        .with_title("Settings")
        .with_submenu(settings_menu)
        .build(ctx);
    let quit_menu_item = MenuItem::builder()
        .with_title("Quit")
        .with_action(Action::Callback(|| {
            println!("`Quit` clicked!");
        }))
        .with_macos_short_code("q")
        .build(ctx);
    let app_menu = Menu::builder()
        .with_item(settings_menu_item)
        .with_item(MenuItem::separator(ctx))
        .with_item(quit_menu_item)
        .build(ctx);

    let close_all_menu_item = MenuItem::builder()
        .with_title("Close All")
        .with_action(Action::Callback(|| {
            println!("`Close All` clicked!");
        }))
        .build(ctx);
    let window_menu = Menu::builder().with_item(close_all_menu_item).build(ctx);

    let app_menu_item = MenuItem::builder()
        .with_title("Bioma")
        .with_submenu(app_menu)
        .build(ctx);
    let window_item = MenuItem::builder()
        .with_title("Window")
        .with_submenu(window_menu)
        .build(ctx);
    Menu::builder()
        .with_item(app_menu_item)
        .with_item(window_item)
        .build(ctx)
}

fn main() {
    let app = Application::new().unwrap();
    app.run(|app: &mut ActiveApplication, event: Event| match event {
        Event::LifeCycle(LifeCycle::Start) => {
            let menu = create_menu(app);
            app.set_menu(Some(&menu));
        }
        _ => {}
    });
}

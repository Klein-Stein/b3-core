use b3_platform::{Application, Menu, MenuItem};

fn create_menu(app: &Application) -> Menu {
    let quit_menu_item = MenuItem::builder()
        .with_title("Quit")
        .with_action(|| {
            println!("`Quit` clicked!");
        })
        .build(app);
    let app_menu = Menu::builder().with_item(quit_menu_item).build(app);

    let close_all_menu_item = MenuItem::builder()
        .with_title("Close All")
        .with_action(|| {
            println!("`Close All` clicked!");
        })
        .build(app);
    let window_menu = Menu::builder().with_item(close_all_menu_item).build(app);

    let app_menu_item = MenuItem::builder()
        .with_title("Bioma")
        .with_submenu(app_menu)
        .build(app);
    let window_item = MenuItem::builder()
        .with_title("Window")
        .with_submenu(window_menu)
        .build(app);
    Menu::builder()
        .with_item(app_menu_item)
        .with_item(window_item)
        .build(app)
}

fn main() {
    let mut app = Application::new();
    let menu = create_menu(&app);
    app.set_menu(Some(menu));
    app.run();
}

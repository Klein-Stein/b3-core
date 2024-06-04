use std::collections::HashMap;

use b3_core::{
    Action,
    ActiveApplication,
    Application,
    ContextOwner,
    Event,
    EventHandler,
    Icon,
    IconType,
    LifeCycle,
    Menu,
    MenuItem,
    NotificationBuilder,
    Window,
    WindowEvent,
    WindowId,
};

fn create_menu(ctx: &impl ContextOwner) -> Menu {
    // App menu
    let settings_item = MenuItem::builder()
        .with_title("Preferences...")
        .with_icon(Icon::from_str(ctx, "gear").unwrap())
        .with_macos_short_code("P")
        .with_enabled(false) // Stub item
        .build(ctx);

    let quit_item = MenuItem::builder()
        .with_title("Quit")
        .with_macos_short_code("q")
        .with_action(Action::new_event("quit"))
        .build(ctx);

    let app_menu = Menu::builder()
        .with_item(settings_item)
        .with_item(MenuItem::separator(ctx))
        .with_item(quit_item)
        .build(ctx);

    // Window menu

    let new_window_item = MenuItem::builder()
        .with_title("New Window")
        .with_macos_short_code("n")
        .with_action(Action::new_event("new-window"))
        .build(ctx);

    let new_modal_item = MenuItem::builder()
        .with_title("New Modal Window")
        .with_macos_short_code("m")
        .with_action(Action::new_event("new-modal-window"))
        .build(ctx);

    let close_all_item = MenuItem::builder()
        .with_title("Close All")
        .with_macos_short_code("k")
        .with_action(Action::new_event("close-all"))
        .build(ctx);

    let window_menu = Menu::builder()
        .with_item(new_window_item)
        .with_item(new_modal_item)
        .with_item(MenuItem::separator(ctx))
        .with_item(close_all_item)
        .build(ctx);

    // Notifications
    let new_notification_item = MenuItem::builder()
        .with_title("New Notification")
        .with_macos_short_code("a")
        .with_action(Action::new_event("new-notification"))
        .build(ctx);

    let notifications_menu = Menu::builder().with_item(new_notification_item).build(ctx);

    // Main menu
    let app_item = MenuItem::builder().with_submenu(app_menu).build(ctx);

    let window_item = MenuItem::builder()
        .with_title("Window")
        .with_submenu(window_menu)
        .build(ctx);

    let notifications_item = MenuItem::builder()
        .with_title("Notifications")
        .with_submenu(notifications_menu)
        .build(ctx);

    Menu::builder()
        .with_item(app_item)
        .with_item(window_item)
        .with_item(notifications_item)
        .build(ctx)
}

struct State {
    menu:           Menu,
    windows:        HashMap<WindowId, Window>,
    window_counter: u32,
    modal_counter:  u32,
}

impl State {
    fn new(ctx: &impl ContextOwner) -> Self {
        let menu = create_menu(ctx);

        let window = Window::builder()
            .with_title("Window 1")
            .with_physical_size((1920, 1280))
            .build(ctx);
        let mut windows = HashMap::new();
        windows.insert(window.id(), window);

        Self {
            menu,
            windows,
            window_counter: 1,
            modal_counter: 0,
        }
    }

    fn new_window(&mut self, app: &ActiveApplication) {
        self.window_counter += 1;
        let mut window = Window::builder()
            .with_title(format!("Window {}", self.window_counter))
            .with_physical_size((1920, 1280))
            .build(app);
        window.show(app);
        self.windows.insert(window.id(), window);
    }

    fn new_notification(&mut self, app: &ActiveApplication) {
        NotificationBuilder::new()
            .with_title("B3-Core Notification")
            .with_message("Notification body message")
            .build(app);
    }

    fn new_modal_window(&mut self, app: &ActiveApplication) {
        self.modal_counter += 1;
        let mut window = Window::builder()
            .with_title(format!("Modal Window {}", self.modal_counter))
            .build(app);
        window.show_modal(app);
        self.windows.insert(window.id(), window);
    }

    fn close_all(&mut self) {
        for (_, window) in self.windows.iter_mut() {
            window.close();
        }
    }

    fn delete_window(&mut self, window_id: WindowId) { self.windows.remove(&window_id); }
}

impl EventHandler for State {
    fn on_event(&mut self, app: &mut ActiveApplication, event: Event) {
        match event {
            Event::Menu(action) => match action.as_ref() {
                "new-window" => self.new_window(app),
                "new-modal-window" => self.new_modal_window(app),
                "new-notification" => self.new_notification(app),
                "close-all" => self.close_all(),
                "quit" => app.stop(),
                _ => (),
            },
            Event::LifeCycle(LifeCycle::Start) => {
                let icon_data = include_bytes!("assets/gears.png").to_vec();
                let app_icon = Icon::from_data(app, &icon_data, IconType::Png).unwrap();
                app.set_icon(Some(&app_icon));

                app.set_menu(Some(&self.menu));

                for (_, window) in self.windows.iter_mut() {
                    window.show(app);
                }
            }
            Event::Window(WindowEvent::Close, window_id) => self.delete_window(window_id),
            _ => (),
        }
    }
}

fn main() {
    let app = Application::new().unwrap();
    let state = State::new(&app);
    app.run(state);
}

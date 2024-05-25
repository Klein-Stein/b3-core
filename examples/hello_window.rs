use std::collections::HashMap;

use b3_core::{
    Action,
    ActiveApplication,
    Application,
    ContextOwner,
    Event,
    EventHandler,
    Image,
    LifeCycle,
    Menu,
    MenuItem,
    Window,
    WindowEvent,
    WindowId,
};

fn create_menu(ctx: &impl ContextOwner) -> Menu {
    // App menu
    let settings_item = MenuItem::builder()
        .with_title("Preferences...")
        .with_icon(Image::from_str(ctx, "gear").unwrap())
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

    let close_all_item = MenuItem::builder()
        .with_title("Close All")
        .with_macos_short_code("k")
        .with_action(Action::new_event("close-all"))
        .build(ctx);

    let window_menu = Menu::builder()
        .with_item(new_window_item)
        .with_item(close_all_item)
        .build(ctx);

    // Main menu
    let app_item = MenuItem::builder().with_submenu(app_menu).build(ctx);
    let window_item = MenuItem::builder()
        .with_title("Window")
        .with_submenu(window_menu)
        .build(ctx);
    Menu::builder()
        .with_item(app_item)
        .with_item(window_item)
        .build(ctx)
}

struct State {
    menu:    Menu,
    windows: HashMap<WindowId, Window>,
    number:  u32,
}

impl State {
    fn new(ctx: &impl ContextOwner) -> Self {
        let menu = create_menu(ctx);

        let window = Window::builder().with_title("Window 1").build(ctx);
        let mut windows = HashMap::new();
        windows.insert(window.id(), window);

        Self {
            menu,
            windows,
            number: 1,
        }
    }

    fn new_window(&mut self, app: &ActiveApplication) {
        self.number += 1;
        let mut window = Window::builder()
            .with_title(format!("Window {}", self.number))
            .build(app);
        window.show(app);
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
                "close-all" => self.close_all(),
                "quit" => app.stop(),
                _ => (),
            },
            Event::LifeCycle(LifeCycle::Start) => {
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
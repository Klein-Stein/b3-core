use b3_core::{
    Action,
    ActiveApplication,
    Application,
    ContextOwner,
    Event,
    EventHandler,
    LifeCycle,
    Menu,
    MenuItem,
    Window,
};

fn create_menu(ctx: &impl ContextOwner) -> Menu {
    let quit_menu_item = MenuItem::builder()
        .with_title("Quit")
        .with_action(Action::new_event("quit"))
        .with_macos_short_code("q")
        .build(ctx);
    let app_menu = Menu::builder().with_item(quit_menu_item).build(ctx);

    let app_menu_item = MenuItem::builder()
        .with_title("Bioma")
        .with_submenu(app_menu)
        .build(ctx);
    Menu::builder().with_item(app_menu_item).build(ctx)
}

struct State {
    menu:   Menu,
    window: Window,
}

impl State {
    fn new(ctx: &impl ContextOwner) -> Self {
        let menu = create_menu(ctx);
        let window = Window::builder().with_title("B3 Platform").build(ctx);
        Self {
            menu,
            window,
        }
    }
}

impl EventHandler for State {
    fn on_event(&mut self, app: &mut ActiveApplication, event: Event) {
        match event {
            Event::Menu(action) => {
                if action == "quit" {
                    app.stop();
                }
            }
            Event::LifeCycle(LifeCycle::Start) => {
                app.set_menu(Some(&self.menu));

                self.window.show(app);
            }
            _ => (),
        }
    }
}

fn main() {
    let app = Application::new().unwrap();
    let state = State::new(&app);
    app.run(state);
}

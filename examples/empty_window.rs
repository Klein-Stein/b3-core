use b3_platform::{
    Action,
    ActiveApplication,
    Application,
    Event,
    EventHandler,
    LifeCycle,
    Menu,
    MenuItem,
    Window,
    WindowEvent,
};

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

struct State {
    menu:   Menu,
    window: Window,
}

impl State {
    fn new() -> Self {
        let menu = create_menu();
        let window = Window::builder().with_title("B3 Platform").build();
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
            Event::Window(WindowEvent::Show, window_id) => {
                println!("The window has been displayed: {:?}", window_id);
            }
            Event::Window(WindowEvent::Close, window_id) => {
                println!("The window has been closed: {:?}", window_id);
            }
            _ => (),
        }
    }
}

fn main() {
    let app = Application::new();
    let state = State::new();
    app.run(state);
}

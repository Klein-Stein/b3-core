use b3_platform::{Application, Event};

fn main() {
    let app = Application::new();
    app.run(|_app: &Application, event: Event| println!("{:?}", event));
}

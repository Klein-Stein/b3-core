use b3_platform::{Application, Event};

fn main() {
    let app = Application::new();
    app.run(|event: Event| println!("{:?}", event));
}

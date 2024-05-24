use b3_core::{ActiveApplication, Application, Event};

fn main() {
    let app = Application::new().unwrap();
    app.run(|_app: &mut ActiveApplication, event: Event| println!("{:?}", event));
}

use engage::app;

fn main() {
    let descriptor = app::AppDescriptor {
        size: (1600, 900),
        title: "Hello world".to_string()
    };
    let app = app::App::new(&descriptor);
    app.run();
}

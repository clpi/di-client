use app::DiApp;

fn main() {
    std::env::set_var("RUST_LOG", "trace cargo run");
    let app = DiApp::new();
    app.run();
}

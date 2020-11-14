use divcli::app::DivApp;

fn main() {
    std::env::set_var("RUST_LOG", "trace cargo run");
    let app = DivApp::new();
    app.run();
}

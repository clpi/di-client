pub mod util;
pub mod models;
pub mod app;
pub mod config;
pub mod cmd;
pub mod terminal;
pub mod lex;
pub mod error;

pub use app::DivApp;

pub fn run() -> std::io::Result<()> {
    // DivApp::run()?
    Ok(())
}

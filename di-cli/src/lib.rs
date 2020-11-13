pub mod util;
pub mod models;
pub mod app;
pub mod config;
pub mod error;
pub mod cmd;
pub mod terminal;
pub mod lex;

pub use app::{
    DivApp,
};


pub fn run() -> std::io::Result<()> {
    // DivApp::run()?
}

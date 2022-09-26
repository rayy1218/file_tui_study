pub mod app;
pub mod inputs;

use std::cell::RefCell;
use std::rc::Rc;

use eyre::Result;
use log::LevelFilter;
use file_tui::app::App;
use file_tui::start_ui;

fn main() -> Result<()> {
    tui_logger::init_logger(LevelFilter::Debug).unwrap();
    tui_logger::set_default_level(log::LevelFilter::Debug);

    let app = Rc::new(RefCell::new(App::new()));
    start_ui(app)?;

    Ok(())
}



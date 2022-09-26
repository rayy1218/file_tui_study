use std::cell::RefCell;
use std::io::stdout;
use std::rc::Rc;
use std::time::Duration;

use app::App;
use eyre::Result;
use tui::backend::CrosstermBackend;
use tui::Terminal;

use crate::app::{AppReturn, ui};
use crate::inputs::event::Events;
use crate::inputs::InputEvent;

pub mod app;
pub mod inputs;

pub fn start_ui(app: Rc<RefCell<App>>) -> Result<()> {
    // Configure Crossterm backend for tui
    let stdout = stdout();
    crossterm::terminal::enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    let tick_rate = Duration::from_millis(200);
    let events = Events::new(tick_rate);

    loop {
        let mut app = app.borrow_mut();

        // Render
        terminal.draw(|rect| ui::draw(rect, &mut app))?;

        let result = match events.next()? {
            InputEvent::Input(key) => app.do_action(key),
            InputEvent::Tick => app.update_on_tick(),
        };

        if result == AppReturn::Exit {
            break;
        }
    }

    // Restore the terminal and close application
    terminal.clear()?;
    terminal.show_cursor()?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}

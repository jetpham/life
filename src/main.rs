use std::io;

use crossterm::event::MouseEventKind;
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::{
    app::{App, AppResult},
    event::{Event, EventHandler},
    handler::handle_key_events,
    tui::Tui,
};
use log::info;
pub mod app;
pub mod event;
pub mod handler;
pub mod tui;
pub mod ui;

#[tokio::main]
async fn main() -> AppResult<()> {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    info!("Application starting...");
    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(100);
    let init_size = terminal.size()?;
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Create an application.
    let mut app = App::new(init_size.width, init_size.height);

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next().await? {
            Event::Tick => (),
            Event::Key(key_event) => {
                // info!("Key Pressed: {:?}", key_event);
                handle_key_events(key_event, &mut app)?
            }
            Event::Mouse(mouse_event) => {
                info!("Mouse Pressed: {:?}", mouse_event);
                if mouse_event.kind != MouseEventKind::Up(crossterm::event::MouseButton::Left) {
                    let max_row = app.automaton.size().0.saturating_sub(1);
                    let row = max_row.saturating_sub(mouse_event.row.into());
                    let column = mouse_event.column.saturating_add(1);

                    if row < app.automaton.size().0 && usize::from(column) < app.automaton.size().1
                    {
                        app.draw(row, column.into());
                    }
                }
            }
            Event::Resize(_resize_x, _resize_yy) => {
                // info!("Terimnal Resized to: ({}, {})", resize_x, resize_y)
            }
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}

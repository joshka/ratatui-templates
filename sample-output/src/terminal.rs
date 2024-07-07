//! Terminal utilities.
use std::io::{stdout, Stdout};

use color_eyre::{config::HookBuilder, Result};
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
};

/// A type alias for a terminal with a crossterm backend.
pub type Terminal = ratatui::Terminal<CrosstermBackend<Stdout>>;

/// Initialize the terminal.
///
/// This function will enable raw mode, switch to the alternate screen, and create a new terminal.
pub fn init() -> Result<Terminal> {
    install_error_hooks()?;
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

/// Restore the terminal.
///
/// This function will disable raw mode and switch back to the main screen.
pub fn restore() -> Result<()> {
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;
    Ok(())
}

/// Install error hooks.
///
/// This function will install panic and error hooks that will restore the terminal before printing
/// the error.
fn install_error_hooks() -> Result<()> {
    let (panic_hook, eyre_hook) = HookBuilder::default().into_hooks();
    let panic_hook = panic_hook.into_panic_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = restore();
        panic_hook(panic_info);
    }));
    let eyre_hook = eyre_hook.into_eyre_hook();
    color_eyre::eyre::set_hook(Box::new(move |error| {
        let _ = restore();
        eyre_hook(error)
    }))?;
    Ok(())
}

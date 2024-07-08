use app::App;
use color_eyre::Result;

mod app;
mod terminal;

fn main() -> Result<()> {
    let mut app = App::default();
    app.run()?;
    Ok(())
}

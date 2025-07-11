use app::APP_NAME;
use clap::Parser;
use cli::CliOpt;

use crate::app::App;

pub mod app;
pub mod cli;
pub mod event;
pub mod ui;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let cli_opt = CliOpt::parse();
    // Print version and exit
    if cli_opt.version() {
        let pkg_version = env!("CARGO_PKG_VERSION");
        println!("{} {})", APP_NAME, pkg_version);
        return Ok(());
    }
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new(cli_opt).run(terminal).await;
    ratatui::restore();
    result
}

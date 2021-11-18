// Import clap
use clap::{App, AppSettings, SubCommand};
// Modules
mod cli;
mod utils;

fn main() {
    // Create the clap cli app
    let app = App::new("pak")
        .version("0.0.1")
        .author("Interfiber <webmaster@interfiber.dev>")
        .about("MacOS package installer builder")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(SubCommand::with_name("init")
            .about("Creates a new project")
            .author("Interfiber <webmaster@interfiber.dev>"))
        .subcommand(SubCommand::with_name("build")
            .about("Build the current project")
            .author("Interfiber <webmaster@interfiber.dev>"));
    // Get matches
    let matches = app.get_matches();

    // Parse cli args
    if let Some(_) = matches.subcommand_matches("init") {
        cli::init::init();
    }
    if let Some(_) = matches.subcommand_matches("build") {
        cli::build::build();
    }
}

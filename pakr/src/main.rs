mod homepage;
mod pak_cli;
mod project;
use std::env;
mod project_screen;
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Parsing cli args");
    if args.len() != 1 {
        if args[1] == "load_project" {
            if args.len() != 3 {
                println!("Failed to load project: folder not given");
                std::process::exit(1);
            } else {
                println!("Loading project...");
                println!("Project Folder: {}", args[2]);
                std::fs::write("/tmp/pakr_project_folder", &args[2].to_string()).expect("Failed to write temp file");
                let options = eframe::NativeOptions {
                    drag_and_drop_support: true,
                    ..Default::default()
                };
                eframe::run_native(Box::new(project_screen::ProjectScreen::default()), options);
            }
        }
    } else {
        let options = eframe::NativeOptions {
            drag_and_drop_support: true,
            ..Default::default()
        };
        eframe::run_native(Box::new(homepage::WelcomeScreen::default()), options);
    }
}
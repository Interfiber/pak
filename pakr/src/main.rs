use std::env;

mod pak_cli;
mod main_screen;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 1 {
        if args[1] == "load_project" {
            println!("Attempting to load project...");
            if args.len() < 2 {
                println!("Failed to load project: Path not given");
                std::process::exit(1);
            }
            let project_folder = &args[2].to_string();
            println!("Load project from: {}", project_folder);
        } else {
            main_screen::create_welcome();
        }
    } else {
        main_screen::create_welcome();
    }
}



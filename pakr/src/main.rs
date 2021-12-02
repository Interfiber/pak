mod welcome;
mod project;
mod pak_cli;
mod editor;
use std::env;
use std::path::Path;

fn main(){
    let args: Vec<String> = env::args().collect();
    println!("Argument length: {}", args.len());
    println!("Argument data: {:?}", args);
    if args.len() == 1 {
        println!("No args: open welcome view.");
        welcome::open_welcome();
    } else {
        println!("Looking for project");
        if args.len() != 2 {
            println!("No project path specified on the command-line!");
            println!("Example: pakr /Users/ash/example");
            std::process::exit(1);
        } else {
            let project_path = args[1].to_string();
            println!("Project path: {}", project_path);
            if !Path::new(&project_path).exists(){
                println!("Loading project failed: No such file or directory {}", project_path);
                std::process::exit(1);
            } else {
                println!("Writing temp file...");
                match std::fs::write("/tmp/pakr.project_path", project_path){
                    Ok(_) => print!(""),
                    Err(err) => {
                        println!("Failed to write project temp file due to a unexpected error");
                        println!("Error log: {}", err);
                        std::process::exit(1);
                    }
                }
                println!("Opening editor...");
                editor::open_editor();
            }
        }
    }
}
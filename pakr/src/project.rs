use std::fs;
use std::env;
use std::process::Command;
pub fn create_project(folder: String) -> bool{
    println!("Creating project in: {}", folder);
    match fs::create_dir_all(folder.to_string()){
        Ok(_) => print!(""),
        Err(err) => {
            println!("Failed to create project folder.");
            println!("Error: {}", err);
            std::process::exit(1);
        }
    }
    match std::env::set_current_dir(folder){
        Ok(_) => print!(""),
        Err(err) => {
            println!("Failed to switch folders due to a fatal error!");
            println!("Error: {}", err);
            std::process::exit(1);
        }
    }
    // run the init command
    crate::pak_cli::execute_pak_cmd("init");
    return true;
}
// Launch the pakr editor
pub fn open_project(folder: String){
    let current_exe = env::current_exe().expect("Failed to unwrap current_exe() pathbuf").into_os_string().to_str().expect("Failed to unwrap!").to_string();
    println!("Current executable location is: {}", current_exe);
    println!("Spawning editor...");
    println!("{} {}", current_exe, folder);
    let _cmd = Command::new(current_exe).arg(folder).spawn();
    println!("Exiting main window...");
    std::process::exit(0);
}

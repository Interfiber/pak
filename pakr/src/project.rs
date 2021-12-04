use std::fs;
use std::env;
use std::process::Command;
use std::path::Path;
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
pub fn get_current_project() -> String {
    let project_tmp_file = "/tmp/pakr.project_path";
    let project_path = std::fs::read_to_string(project_tmp_file).expect("Failed to read contents of pakr project temp file");
    //     Check if the file exists
    let project_exists = Path::new(&project_path).exists();
    let project_file_exists = Path::new(&format!("{}/pak.project.json", project_path)).exists();
    if !project_exists {
        println!("Failed to load project: No such file or directory {}", project_path);
        std::process::exit(1);
    }
    if !project_file_exists {
        println!("Failed to load project: Failed to find project file in the project directory!");
        std::process::exit(1);
    }
    return project_path;
}
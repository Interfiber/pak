pub fn create_project(folder: String) -> bool{
    println!("Creating project in: {}", folder);
    let path = std::path::Path::new(&folder);
    if !path.exists(){
        println!("Failed to create project: dest does not exist!");
        return false;
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

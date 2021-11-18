use std::fs;
pub fn create_dir(path: &str){
    match fs::create_dir(path){
        Ok(_) => print!(""),
        Err(error) => {
            println!("Failed to create dir '{}'", path);
            println!("Error log: {}", error);
            std::process::exit(1);
        }
    }
}
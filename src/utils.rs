use std::fs;
use std::path::Path;

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

pub fn require_path(path: String){
    if !Path::new(&path).exists(){
        println!("ERROR! Failed to complete build:");
        println!("File must be present: {}", path);
        std::process::exit(1);
    }
}
pub fn log_error(message: &str){
    println!("ERROR! Failed to complete build!");
    println!("Error: {}", message);
    std::process::exit(1);
}
pub fn copy_file(orig: &str, new: &str){
    match fs::copy(orig, new){
        Ok(_) => print!(""),
        Err(err) => {
            log_error(&err.to_string());
        }
    }
}
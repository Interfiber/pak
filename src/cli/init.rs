// Spinners
use spinners::{Spinner, Spinners};
use crate::utils;
use serde_json::json;

// init: Create a new project
pub fn init(){
    let project_file_name = "pak.project.json";
    let sp = Spinner::new(&Spinners::Dots11, "Creating new project".into());
    // Create folders
    utils::create_dir("payloads");
    utils::create_dir("builds");
    // Create main project file
    let config = json!({
        "projectName": "my cool project",
        "components": ["default"],
        "version": "0.0.1",
        "orgName": "org.cool",
        "component_default": {
            "$name": "default",
            "$pkgName": "defaultPkg",
            "$selected": true,
            "$visible": false,
            "$payloadName": "default",
            "$installDir": "/opt/installer"
        }
    });
    std::fs::write(project_file_name, serde_json::to_string_pretty(&config).unwrap()).expect("Failed to write to config");
    sp.stop();
}
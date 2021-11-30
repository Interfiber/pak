use eframe::{egui, epi};
use std::path::Path;
use crate::pak_cli::execute_pak_cmd;
use serde_json::{json, Value};
use serde_json::Value::Null;

pub struct ProjectScreen {
    project_name: String,
    org_id: String,
    version: String,
    current_text_component_name: String,
    components_array: Vec<String>
}
impl Default for ProjectScreen {
    fn default() -> Self {
        let project_folder = std::fs::read_to_string("/tmp/pakr_project_folder").expect("Failed to read data!");
        if !Path::new(&project_folder).exists(){
            println!("Failed to open project: No such file or directory /tmp/pakr_project_folder");
            println!("NOTE: If you're loading the project manually from the commandline try loading it from the GUI");
            std::process::exit(1);
        }
        std::env::set_current_dir(project_folder).expect("Failed to change working directory!");
        println!("Parsing pak config...");
        let json_data = std::fs::read_to_string("pak.project.json").expect("Failed to open file: pak.project.json");
        let config: Value = serde_json::from_str(&json_data).expect("Failed to parse json file!");
        let components = &config["components"].to_string().replace("\"", "").to_owned();
        let split = components.split(",");
        let mut components = Vec::new();
        for s in split {
            components.push(s.replace("[", "").replace("]", ""));
        }
        println!("{:?}", components);
        Self {
            project_name: config["projectName"].to_string().replace("\"", "").to_owned(),
            org_id: config["orgName"].to_string().replace("\"", "").to_owned(),
            version: config["version"].to_string().replace("\"", "").to_owned(),
            current_text_component_name: "Component Name".to_owned(),
            components_array: components
        }
    }
}

impl epi::App for ProjectScreen {
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let Self { project_name, org_id, version,current_text_component_name, components_array } = self;
        egui::CentralPanel::default().show(ctx, |ui| {
            // Title
            ui.heading("Pakr Project");
            // Spacer
            ui.heading("  ");
            if ui.button("Build Project").clicked(){
                println!("Building Project");
                execute_pak_cmd("build");
            }
            if ui.button("Save Project Config").clicked(){
                println!("Saving project config");
                // Load written config
                let json_data = std::fs::read_to_string("pak.project.json").expect("Failed to open file: pak.project.json");
                let config: Value = serde_json::from_str(&json_data).expect("Failed to parse json file!");
                let mut written_config = json!({
                    "projectName": project_name,
                    "orgName": org_id,
                    "version": version,
                    "components": components_array
                });
                let config_array = written_config.as_object_mut().unwrap();
                for i in 0..components_array.len() {
                    let comp = &components_array[i];
                    if config[format!("component_{}", comp.to_string())] == Null {
                        config_array.insert(format!("component_{}", comp.to_string()), json!({
                            "$name": "Placeholder name",
                            "$installerDir": "/opt/placeholder",
                            "$desc": "Description of component",
                            "$payloadName": "nameOfPayload",
                            "$pkgName": "smallPackageName",
                            "$selectable": false,
                            "$selected": true,
                            "$visible": true
                        }));
                    } else {
                        config_array.insert(format!("component_{}", comp.to_string()), config[format!("component_{}", comp.to_string())].to_owned());
                    }
                }
                println!("Writing config: {:?}", config_array);
                std::fs::write("pak.project.json", serde_json::to_string_pretty(&config_array).unwrap());
            }
            ui.heading("");
            // Config
            ui.heading("Project Name:");
            ui.text_edit_singleline(project_name);
            ui.heading("Org ID:");
            ui.text_edit_singleline(org_id);
            ui.heading("Version:");
            ui.text_edit_singleline(version);
            // Component editor
            ui.heading("");
            ui.heading("Components");
            ui.text_edit_singleline(current_text_component_name);
            if ui.button("+").clicked(){
                println!("Adding new component with name {}", current_text_component_name);
                components_array.push(current_text_component_name.to_string());
                println!("Added, make sure to save the project");
            }
            // Editor
            ui.collapsing("Edit current component", |ui| {
                ui.horizontal_wrapped(|ui| {
                    if ui.button("Load component info").clicked(){
                        println!("Loading...");

                    }
                });
            });
        });

        // Resize the native window to be just the size we need it to be:
        frame.set_window_size(ctx.used_size());
    }

    fn name(&self) -> &str {
        "Pakr Project"
    }
}
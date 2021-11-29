use eframe::{egui, epi};
use std::path::Path;
use crate::pak_cli::execute_pak_cmd;
use serde_json::{json, Value};

pub struct ProjectScreen {
    project_name: String,
    org_id: String,
    version: String,
    current_text_component_name: String,

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
        Self {
            project_name: config["projectName"].to_string().replace("\"", "").to_owned(),
            org_id: config["orgName"].to_string().replace("\"", "").to_owned(),
            version: config["version"].to_string().replace("\"", "").to_owned(),
            current_text_component_name: "Component Name".to_owned()
        }
    }
}

impl epi::App for ProjectScreen {
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let Self { project_name, org_id, version,current_text_component_name } = self;
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
                let written_config = json!({
                    "projectName": project_name,
                    "orgName": org_id,
                    "version": version
                });
                std::fs::write("pak.project.json", serde_json::to_string_pretty(&written_config).unwrap());
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
            if ui.button("Add component").clicked(){
                println!("Adding new component with name {}", current_text_component_name);

            }
            if ui.button("Edit component").clicked(){
                println!("Opening editor for {}", current_text_component_name);
            }

        });

        // Resize the native window to be just the size we need it to be:
        frame.set_window_size(ctx.used_size());
    }

    fn name(&self) -> &str {
        "Pakr Project"
    }
}
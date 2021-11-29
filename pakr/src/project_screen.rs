use eframe::{egui, epi};
use std::path::Path;

pub struct ProjectScreen {
    project_name: String
}
impl Default for ProjectScreen {
    fn default() -> Self {
        Self {
            project_name: "".to_owned()
        }
    }
}

impl epi::App for ProjectScreen {
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let Self { project_name } = self;
        let project_folder = std::fs::read_to_string("/tmp/pakr_project_folder").expect("Failed to read data!");
        if !Path::new(&project_folder).exists(){
            println!("Failed to open project: No such file or directory");
            std::process::exit(1);
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            // Title
            ui.heading("Pakr Project");
            // Spacer
            ui.heading("  ");
            if ui.button("Build Project").clicked(){
                println!("Building Project");
            }
            if ui.button("Save Project Config").clicked(){
                println!("Saving project config");
            }
            ui.heading("");
            // Config
            ui.heading("Project Name:");
            ui.text_edit_singleline(project_name);
        });

        // Resize the native window to be just the size we need it to be:
        frame.set_window_size(ctx.used_size());
    }

    fn name(&self) -> &str {
        "Pakr Project"
    }
}
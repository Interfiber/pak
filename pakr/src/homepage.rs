use eframe::{egui, epi};

pub struct WelcomeScreen {
    folder_label: String
}
impl Default for WelcomeScreen {
    fn default() -> Self {
        Self {
            folder_label: "".to_owned()
        }
    }
}

impl epi::App for WelcomeScreen {
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        // let Self { name, age } = self;
        egui::CentralPanel::default().show(ctx, |ui| {
            let Self { folder_label } = self;
            // Title
            ui.heading("Pakr Welcome Screen");
            // Spacer
            ui.heading("  ");
            // Create Project Button
            if ui.button("Create Project").clicked() {
                println!("Adding new project");
                crate::project::create_project(folder_label.to_string());
            }
            ui.heading("");
            ui.heading("Project Folder:");
            ui.text_edit_singleline(folder_label);
        });

        // Resize the native window to be just the size we need it to be:
        frame.set_window_size(ctx.used_size());
    }

    fn name(&self) -> &str {
        "Pakr Welcome Screen"
    }
}
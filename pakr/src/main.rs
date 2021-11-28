use gtk::prelude::*;
use gtk::{self, Application, ApplicationWindow, Button, Orientation, Label};
use nfd::Response;

mod pak_cli;


fn main() {
    // Create a new application
    let app = Application::builder()
        .application_id("dev.interfiber.pak.pakr")
        .build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Pakr")
        .width_request(800)
        .height_request(800)
        .resizable(false)
        .build();
    // Create buttons/labels
    let pakr_title = Label::builder()
        .label("Pakr")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    let new_project_button = Button::builder()
        .label("New Project")
        .width_request(25)
        .height_request(25)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Create boxes for buttons
    let gtk_box = gtk::Box::new(Orientation::Vertical, 0);
    window.set_child(Some(&gtk_box));
    gtk_box.append(&pakr_title);
    gtk_box.append(&new_project_button);
    // Connect to "clicked" signal of `button`
    new_project_button.connect_clicked(move |_| {
        println!("Creating new project");
        let result = nfd::open_pick_folder(None).unwrap_or_else(|e| {
            panic!("{}", e);
        });
        match result {
            Response::Okay(file_path) => {
                let path = file_path.to_string().replace("\"", "");
                println!("Creating project at path: {}", path);
                std::env::set_current_dir(path.to_string()).expect("Failed to change working dir into project");
                pak_cli::execute_pak_cmd("init");
                println!("Loading project...");
                // Load project using the cli
                let current_path = std::env::current_exe().expect("Failed to get current executable location");
                std::process::Command::new(current_path.into_os_string().into_string().unwrap()).arg("load_project").arg(path.to_string()).exec();
            },
            Response::OkayMultiple(_) => {
                println!("Failed to open project: too many files selected!");
                std::process::exit(1);
            },
            Response::Cancel => println!("User canceled"),
        }
    });
    // Present window to the user
    window.present();
}

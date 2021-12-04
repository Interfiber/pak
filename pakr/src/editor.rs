use std::path::Path;
use iced::button;
use iced::{Button, Column, Text, Sandbox, Settings, Element, Align};
use hashbrown::HashMap;
use iced::text_input::TextInput;
use serde_json::Value;

#[derive(Default)]
pub struct Editor {
    project_name_input: iced::text_input::State,
    project_org_id_input: iced::text_input::State,
    build_project_button: button::State,
    load_project_button: button::State,
    project_info: HashMap<String, String>,
    project_info_name: String,
    project_info_orgid: String
}


impl Sandbox for Editor {

    type Message = Message;
    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Pakr Project")
    }

    fn update(&mut self, message: Message) {
        match message {
           Message::BuildProject => {
               println!("Building project...");
           },
            Message::ProjectNameUpdated(name) => {
                println!("Updating project name to {}...", name);
                self.project_info.insert("projectName".to_string(), name.to_string());
                self.project_info_name = name.to_string();
            },
            Message::ProjectOrgIdUpdated(org) => {
                println!("Updating project org id to {}...", org);
                self.project_info.insert("orgId".to_string(), org.to_string());
                self.project_info_orgid = org.to_string();
            },
            Message::LoadProjectInfo => {
                println!("Loading project from on-disk files...");
                let project_tmp_file = "/tmp/pakr.project_path";
                // read from file
                println!("Reading from: {}", project_tmp_file);
                let project_path = std::fs::read_to_string(project_tmp_file).expect("Failed to read contents of pakr project temp file");
                println!("File path: {}", project_path);
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
                let project_raw_data = std::fs::read_to_string(&format!("{}/pak.project.json", project_path)).expect("Failed to open project file!");
                //          Parse project config
                let project_config: Value = serde_json::from_str(&project_raw_data).expect("Failed to parse project data");
                println!("{:?}", project_config);
                self.project_info_name = project_config["projectName"].to_string().replace("\"", "");
                self.project_info_orgid = project_config["orgName"].to_string().replace("\"", "");
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        // Text boxes
        let project_name_input = TextInput::new(&mut self.project_name_input,  "Project Name", &self.project_info_name, Message::ProjectNameUpdated).padding(10);
        let project_org_id_input = TextInput::new(&mut self.project_org_id_input,  "Project Orgid", &self.project_info_orgid, Message::ProjectOrgIdUpdated).padding(10);
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            // Title
            .push(
                Text::new("Pakr Project").size(40)
            )
            .push(
                Text::new("\n").height(iced::Length::Units(10))
            )

            .push(
                Button::new(&mut self.load_project_button, Text::new("Load Project Info")).on_press(Message::LoadProjectInfo)
            )
            .push(
                Text::new("\n").height(iced::Length::Units(10))
            )
            .push(
                Button::new(&mut self.build_project_button, Text::new("Build Project")).on_press(Message::BuildProject)
            )
            .push(
                Text::new("\n").height(iced::Length::Units(15))
            )
            .push(
                project_name_input
            )
            .push(
                project_org_id_input
            )
            .into()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    BuildProject,
    ProjectNameUpdated(String),
    ProjectOrgIdUpdated(String),
    LoadProjectInfo

}
pub fn open_editor(){
    Editor::run(Settings::default()).unwrap();
}

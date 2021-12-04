use crate::project;
use crate::pak_cli;
use iced::button;
use serde_json::json;
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
    components: HashMap<String, Value>,
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
                let project_dir = project::get_current_project();
                println!("Switching to working directory: {}", project_dir);
                match std::env::set_current_dir(project_dir.to_string()){
                    Ok(_) => print!(""),
                    Err(err) => {
                        println!("Failed to change current working directory!");
                        println!("Error Log: {}", err);
                        std::process::exit(1);
                    }
                }
                println!("Saving config to disk...");
                // read the config off disk
                let project_raw = std::fs::read_to_string(&format!("{}/pak.project.json", project_dir.to_string())).expect("Failed to read project config off disk!");
                let mut project_config: Value = serde_json::from_str(&project_raw).unwrap();
                let config_obj = project_config.as_object_mut().unwrap();
                let project_name = self.project_info.get("projectName").expect("Failed to find data: projectName not in file").to_string();
                let org_id =  self.project_info.get("orgId").expect("Failed to find data: orgId not in file").to_string();
                config_obj.insert("projectName".to_string(), Value::String(project_name));
                config_obj.insert("orgName".to_string(), Value::String(org_id));
                match std::fs::write(&format!("{}/pak.project.json", project_dir.to_string()), serde_json::to_string_pretty(&config_obj).unwrap()){
                    Ok(_) => print!(""),
                    Err(err) => {
                        println!("Failed to write project file!");
                        println!("Error: {}", err);
                        std::process::exit(1);
                    }
                }
                // update it for the new config
                // write it to disk
                println!("Executing build command");
                pak_cli::execute_pak_cmd("build");
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
                let project_path = project::get_current_project();
                let project_raw_data = std::fs::read_to_string(&format!("{}/pak.project.json", project_path)).expect("Failed to open project file!");
                //  Parse project config
                let project_config: Value = serde_json::from_str(&project_raw_data).expect("Failed to parse project data");
                println!("{:?}", project_config);
                self.project_info_name = project_config["projectName"].to_string().replace("\"", "");
                self.project_info_orgid = project_config["orgName"].to_string().replace("\"", "");
                self.project_info.insert("orgId".to_string(), project_config["orgName"].to_string().replace("\"", ""));
                self.project_info.insert("projectName".to_string(), project_config["projectName"].to_string().replace("\"", ""));
                println!("Loaded project info");
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
                Button::new(&mut self.build_project_button, Text::new("Build/Save Project")).on_press(Message::BuildProject)
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

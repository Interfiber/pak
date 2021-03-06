use crate::project;
use crate::pak_cli;
use iced::button;
use iced::{Button, Column, Text, Sandbox, Settings, Element, Align};
use hashbrown::HashMap;
use iced::text_input::TextInput;
use std::path::Path;
use serde_json::json;
use serde_json::Value;


#[derive(Default)]
pub struct Editor {
    project_name_input: iced::text_input::State,
    project_org_id_input: iced::text_input::State,
    project_component_input: iced::text_input::State,
    component_editor_name_input: iced::text_input::State,
    component_editor_desc_input: iced::text_input::State,
    component_editor_install_dir_input: iced::text_input::State,
    component_editor_payload_name_input: iced::text_input::State,
    component_editor_script_folder_input: iced::text_input::State,
    edit_component_button: button::State,
    new_component_button: button::State,
    build_project_button: button::State,
    load_project_button: button::State,
    project_info: HashMap<String, String>,
    components: HashMap<String, Value>,
    components_array: Vec<String>,
    project_info_name: String,
    project_info_orgid: String,
    current_component_name: String,
    component_name: String,
    component_desc: String,
    component_install_dir: String,
    component_payload_name: String,
    component_script_folder: String,
    component_selected: bool,
    // the internal component name is the same as the component  name but its only updated using code, not the user
    internal_component_name: String,
    has_unsaved_changes: bool,
    loaded_component: bool,
    component_visible: bool,
    component_selectable: bool
}


impl Sandbox for Editor {

    type Message = Message;
    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        if self.has_unsaved_changes {
            return "Pakr Project*".to_string()
        } else {
            return "Pakr Project".to_string();
        }
    }

    fn update(&mut self, message: Message) {
        match message {
           Message::BuildProject => {
                println!("Building project...");
                let project_dir = project::get_current_project();
                println!("Switching to working directory: {}", project_dir);
                std::env::set_current_dir(project_dir.to_string()).expect("Failed to change current working directory!");
                println!("Saving config to disk...");
                // read the config off disk
                let components_array = &self.components_array;
                let mut project_config: Value = json!({
                    "components": components_array
                });
                let config_obj = project_config.as_object_mut().unwrap();
                let project_name = self.project_info.get("projectName").expect("Failed to find data: projectName not in file").to_string();
                let org_id =  self.project_info.get("orgId").expect("Failed to find data: orgId not in file").to_string();
                config_obj.insert("projectName".to_string(), Value::String(project_name));
                config_obj.insert("orgName".to_string(), Value::String(org_id));
                // Add components
                for i in 0..components_array.len() {
                    let comp_name = &components_array[i];
                    println!("Saving component: {}", comp_name);
                    let comp_data = self.components.get(comp_name).expect("Failed to find data: component not found in components");
                    let comp_raw_json = comp_data.to_string();
                    println!("Raw json: {}", comp_raw_json);
                    config_obj.insert(format!("component_{}", comp_name), serde_json::from_str(&comp_raw_json).expect("Failed to parse component info"));
                    println!("Checking for payload...");
                    let payload_exists = Path::new(&format!("payloads/{}", comp_data["$payloadName"].to_string().replace("\"", ""))).exists();
                    if !payload_exists {
                        std::fs::create_dir_all(&format!("payloads/{}", comp_data["$payloadName"].to_string().replace("\"", ""))).expect("Failed to create dir due to a error");
                    }
                }
                match std::fs::write(&format!("{}/pak.project.json", project_dir.to_string()), serde_json::to_string_pretty(&config_obj).unwrap()){
                    Ok(_) => print!(""),
                    Err(err) => {
                        println!("Failed to write project file!");
                        println!("Error: {}", err);
                        std::process::exit(1);
                    }
                }
                self.has_unsaved_changes = false;
                println!("Executing build command");
                subprocess::Exec::shell("rm -rvf .build_cache").join().expect("Failed to remove build cache");
                pak_cli::execute_pak_cmd("build");
           },
            Message::ProjectNameUpdated(name) => {
                println!("Updating project name to {}...", name);
                self.project_info.insert("projectName".to_string(), name.to_string());
                self.project_info_name = name.to_string();
                self.has_unsaved_changes = true;
            },
            Message::ProjectOrgIdUpdated(org) => {
                println!("Updating project org id to {}...", org);
                self.project_info.insert("orgId".to_string(), org.to_string());
                self.project_info_orgid = org.to_string();
                self.has_unsaved_changes = true;
            },
            Message::CurrentComponentNameUpdated(name) => {
                println!("Updating current component id to {}...", name);
                self.current_component_name = name;
            }
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
                println!("Indexing components...");
                let components = &project_config["components"].to_string().replace("\"", "").to_owned();
                let split = components.split(",");
                for s in split {
                    let name = s.replace("[", "").replace("]", "");
                    println!("Loading component with name: {}", name);
                    self.components_array.push(name.to_string());
                    // Insert data into the table
                    self.components.insert(name.to_string(), serde_json::from_str(&project_config[&format!("component_{}", name)].to_string()).expect("Failed to parse component info"));
                }
                println!("{:?}", self.components);
                println!("Loaded project info");
            },
            Message::AddComponent => {
                let name = &self.current_component_name.to_string();
                println!("Adding component with name {}", name);
                self.current_component_name = "".to_string();
                self.components.insert(name.to_string(), json!({
                    "$name": "Placeholder name",
                    "$installDir": "/opt/placeholder",
                    "$desc": "Description of component",
                    "$payloadName": "nameOfPayload",
                    "$pkgName": "smallPackageName",
                    "$selectable": false,
                    "$selected": true,
                    "$visible": true
                }));
                self.components_array.push(name.to_string());
                println!("{:?}", self.components);
                println!("Added component");
                self.has_unsaved_changes = true;
            },
            Message::EditComponent => {
                let name = &self.current_component_name.to_string();
                println!("Loading component info for: {}", name);
                let info = self.components.get(name).expect("Failed to load component info: invalid name");
                self.component_name = info["$name"].to_string().replace("\"", "");
                self.component_desc = info["$desc"].to_string().replace("\"", "");
                self.component_install_dir = info["$installDir"].to_string().replace("\"", "");
                self.internal_component_name = name.to_string();
                self.component_selected = info["$selected"].as_bool().expect("Failed to convert Value to bool");
                self.component_selectable = info["$selectable"].as_bool().expect("Failed to convert Value to bool");
                self.component_visible = info["$visible"].as_bool().expect("Failed to convert Value to bool");
                self.loaded_component = true;
                self.component_payload_name = info["$payloadName"].to_string().replace("\"", "");
                self.component_script_folder = info["$scriptsFolder"].to_string().replace("\"", "");
                println!("{:?}", info);
            },
            Message::ComponentNameUpdate(value) => {
                if self.loaded_component {
                    println!("Updating component name to {}", value);
                    self.component_name = value.to_string();
                    // Get json for current component
                    let mut info_one = self.components.get(&self.internal_component_name).expect("Failed to load component info: invalid name").to_owned();
                    let info = info_one.as_object_mut().unwrap();
                    info["$name"] = serde_json::Value::String(value);
                    println!("Updating in-memory component...");
                    self.components.remove(&self.internal_component_name.to_string());
                    self.components.insert(self.internal_component_name.to_string(), crate::utils::get_component_json_from_info(info));
                    self.has_unsaved_changes = true;
                }   
            },
            Message::ComponentDescUpdated(value) => {
                if self.loaded_component {
                    println!("Updating component desc to {}", value);
                    self.component_desc = value.to_string();
                    // Get json for current component
                    let mut info_one = self.components.get(&self.internal_component_name).expect("Failed to load component info: invalid name").to_owned();
                    let info = info_one.as_object_mut().unwrap();
                    info["$desc"] = serde_json::Value::String(value);
                    println!("Updating in-memory component...");
                    self.components.remove(&self.internal_component_name.to_string());
                    self.components.insert(self.internal_component_name.to_string(), crate::utils::get_component_json_from_info(info));
                    self.has_unsaved_changes = true;
                }   
            },
            Message::ComponentInstallDirUpdated(value) => {
                if self.loaded_component {
                    println!("Updating component install dir to {}", value);
                    self.component_install_dir = value.to_string();
                    // Get json for current component
                    let mut info_one = self.components.get(&self.internal_component_name).expect("Failed to load component info: invalid name").to_owned();
                    let info = info_one.as_object_mut().unwrap();
                    info["$installDir"] = serde_json::Value::String(value);
                    println!("Updating in-memory component...");
                    self.components.remove(&self.internal_component_name.to_string());
                    self.components.insert(self.internal_component_name.to_string(), crate::utils::get_component_json_from_info(info));
                    self.has_unsaved_changes = true;
                } 
            },
            Message::ComponentPayloadNameUpdated(value) => {
                if self.loaded_component {
                    println!("Updating component payload name to {}", value);
                    self.component_payload_name = value.to_string();
                    // Get json for current component
                    let mut info_one = self.components.get(&self.internal_component_name).expect("Failed to load component info: invalid name").to_owned();
                    let info = info_one.as_object_mut().unwrap();
                    info["$payloadName"] = serde_json::Value::String(value);
                    println!("Updating in-memory component...");
                    self.components.remove(&self.internal_component_name.to_string());
                    self.components.insert(self.internal_component_name.to_string(), crate::utils::get_component_json_from_info(info));
                    self.has_unsaved_changes = true;
                } 
            },
            Message::ComponentSelectedUpdated(toggle) => {
                if self.loaded_component {
                    println!("Updating selected status to: {}", toggle);
                    self.component_selected = toggle;
                    let mut info_one = self.components.get(&self.internal_component_name).expect("Failed to load component info: invalid name").to_owned();
                    let info = info_one.as_object_mut().unwrap();
                    info["$selected"] = serde_json::Value::Bool(toggle);
                    println!("Updating in-memory component...");
                    self.components.remove(&self.internal_component_name.to_string());
                    self.components.insert(self.internal_component_name.to_string(), crate::utils::get_component_json_from_info(info));
                    println!("{}", crate::utils::get_component_json_from_info(info));
                    self.has_unsaved_changes = true;
                }
            },
            Message::ComponentScriptFolderUpdated(value) => {
                if self.loaded_component {
                    println!("Updating selected status to: {}", &value.to_string());
                    self.component_script_folder = value.to_string();
                    let mut info_one = self.components.get(&self.internal_component_name).expect("Failed to load component info: invalid name").to_owned();
                    let info = info_one.as_object_mut().unwrap();
                    if info.get("$scriptsFolder").unwrap_or(&serde_json::Value::Null) == &serde_json::Value::Null {
                        info.insert("$scriptsFolder".to_string(), serde_json::Value::String(value.to_string()));
                    } else {
                        info["$scriptsFolder"] = serde_json::Value::String(value.to_string());
                    }
                    println!("Updating in-memory component...");
                    self.components.remove(&self.internal_component_name.to_string());
                    self.components.insert(self.internal_component_name.to_string(), crate::utils::get_component_json_from_info(info));
                    println!("{}", crate::utils::get_component_json_from_info(info));
                    self.has_unsaved_changes = true;
                }
            },
            Message::ComponentSelectableUpdated(toggle) => {
                if self.loaded_component {
                    println!("Updating selectable status to: {}", toggle);
                    self.component_selectable = toggle;
                    let mut info_one = self.components.get(&self.internal_component_name).expect("Failed to load component info: invalid name").to_owned();
                    let info = info_one.as_object_mut().unwrap();
                    info["$selectable"] = serde_json::Value::Bool(toggle);
                    println!("Updating in-memory component...");
                    self.components.remove(&self.internal_component_name.to_string());
                    self.components.insert(self.internal_component_name.to_string(), crate::utils::get_component_json_from_info(info));
                    println!("{}", crate::utils::get_component_json_from_info(info));
                    self.has_unsaved_changes = true;
                }
            },
            Message::ComponentVisibleUpdated(toggle) => {
                if self.loaded_component {
                    println!("Updating visible status to: {}", toggle);
                    self.component_visible = toggle;
                    let mut info_one = self.components.get(&self.internal_component_name).expect("Failed to load component info: invalid name").to_owned();
                    let info = info_one.as_object_mut().unwrap();
                    info["$visible"] = serde_json::Value::Bool(toggle);
                    println!("Updating in-memory component...");
                    self.components.remove(&self.internal_component_name.to_string());
                    self.components.insert(self.internal_component_name.to_string(), crate::utils::get_component_json_from_info(info));
                    println!("{}", crate::utils::get_component_json_from_info(info));
                    self.has_unsaved_changes = true;
                }
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        // Text boxes
        let project_name_input = TextInput::new(&mut self.project_name_input,  "Project Name", &self.project_info_name, Message::ProjectNameUpdated).padding(10);
        let project_org_id_input = TextInput::new(&mut self.project_org_id_input,  "Project Orgid", &self.project_info_orgid, Message::ProjectOrgIdUpdated).padding(10);
        let current_component_input = TextInput::new(&mut self.project_component_input,  "Component Name to edit/create", &self.current_component_name, Message::CurrentComponentNameUpdated).padding(10);
        let component_name_input = TextInput::new(&mut self.component_editor_name_input,  "Component Name", &self.component_name, Message::ComponentNameUpdate).padding(10);
        let component_desc_input = TextInput::new(&mut self.component_editor_desc_input,  "Component Description", &self.component_desc, Message::ComponentDescUpdated).padding(10);
        let component_install_dir_input = TextInput::new(&mut self.component_editor_install_dir_input,  "Component Install Folder", &self.component_install_dir, Message::ComponentInstallDirUpdated).padding(10);
        let component_payload_name_input = TextInput::new(&mut self.component_editor_payload_name_input,  "Component Payload Name", &self.component_payload_name, Message::ComponentPayloadNameUpdated).padding(10);
        let component_script_folder_input = TextInput::new(&mut self.component_editor_script_folder_input,  "Component Script Folder", &self.component_script_folder, Message::ComponentScriptFolderUpdated).padding(10);
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
            // Buttons for loading and stuff
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
            // Basic project info
            .push(
                project_name_input
            )
            .push(
                Text::new("\n").height(iced::Length::Units(10))
            )
            .push(
                project_org_id_input
            )
            .push(
                Text::new("\n").height(iced::Length::Units(34))
            )
            .push(
                current_component_input
            )
            .push(
                Text::new("\n").height(iced::Length::Units(10))
            )
            // Component editor buttons
            .push(
                Button::new(&mut self.new_component_button, Text::new("Add component").size(20)).padding(6).on_press(Message::AddComponent).style(style::Button::FilterSelected)
            )
            .push(
                Text::new("\n").height(iced::Length::Units(3))
            )
            .push(
                Button::new(&mut self.edit_component_button, Text::new("Edit component").size(20)).padding(6).on_press(Message::EditComponent).style(style::Button::FilterSelected)
            )
            .push(
                Text::new("\n").height(iced::Length::Units(15))
            )
            // Component Editor
            .push(
                component_name_input
            )
            .push(
                Text::new("\n").height(iced::Length::Units(10))
            )
            .push(
                component_desc_input
            )
            .push(
                Text::new("\n").height(iced::Length::Units(10))
            )
            .push(
                component_install_dir_input
            )
            .push(
                Text::new("\n").height(iced::Length::Units(10))
            )
            .push(
                component_payload_name_input
            )
            .push(
                Text::new("\n").height(iced::Length::Units(10))
            )
            .push(component_script_folder_input)
            .push(
                Text::new("\n").height(iced::Length::Units(15))
            )
            .push(
                iced::Checkbox::new(self.component_selected, "Selected", Message::ComponentSelectedUpdated)
            )
            .push(
                Text::new("\n").height(iced::Length::Units(15))
            )
            .push(
                iced::Checkbox::new(self.component_selectable, "Selectable", Message::ComponentSelectableUpdated)
            )
            .push(
                Text::new("\n").height(iced::Length::Units(15))
            )
            .push(
                iced::Checkbox::new(self.component_visible, "Visible", Message::ComponentVisibleUpdated)
            )
            .into()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    BuildProject,
    ProjectNameUpdated(String),
    ProjectOrgIdUpdated(String),
    LoadProjectInfo,
    AddComponent,
    CurrentComponentNameUpdated(String),
    EditComponent,
    // Component Editor Events
    ComponentNameUpdate(String),
    ComponentDescUpdated(String),
    ComponentInstallDirUpdated(String),
    ComponentPayloadNameUpdated(String),
    ComponentScriptFolderUpdated(String),
    ComponentSelectedUpdated(bool),
    ComponentSelectableUpdated(bool),
    ComponentVisibleUpdated(bool)
}
pub fn open_editor(){
    Editor::run(Settings::default()).unwrap();
}
mod style {
    use iced::{button, Background, Color, Vector};

    pub enum Button {
        FilterSelected,
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            match self {
                Button::FilterSelected => button::Style {
                    background: Some(Background::Color(Color::from_rgb(
                        0.2, 0.2, 0.7,
                    ))),
                    border_radius: 5.0,
                    text_color: Color::WHITE,
                    ..button::Style::default()
                }
            }
        }

        fn hovered(&self) -> button::Style {
            let active = self.active();

            button::Style {
                text_color: match self {
                    _ => active.text_color,
                },
                shadow_offset: active.shadow_offset + Vector::new(0.0, 1.0),
                ..active
            }
        }
    }
}

use iced::button;
use iced::{Button, Column, Text, Sandbox, Settings, Element, Align};
use hashbrown::HashMap;
use iced::text_input::TextInput;

#[derive(Default)]
pub struct Editor {
    project_folder_input: iced::text_input::State,
    build_project_button: button::State,
    project_info: HashMap<String, String>,
    project_info_name: String
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
               println!("Saving project...");
           },
            Message::ProjectNameUpdated(name) => {
                println!("Updating project name to {}...", name);
                self.project_info.insert("projectName".to_string(), name.to_string());
                self.project_info_name = name.to_string();
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let project_name_input = TextInput::new(&mut self.project_folder_input,  "Project Name", &self.project_info_name, Message::ProjectNameUpdated).padding(10);
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            // Title
            .push(
                Text::new("Pakr Project").size(40)
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
            .into()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    BuildProject,
    ProjectNameUpdated(String)
}
pub fn open_editor(){
    Editor::run(Settings::default()).unwrap();
}

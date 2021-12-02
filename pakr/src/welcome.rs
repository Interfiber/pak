use iced::button;
use iced::{Button, Column, Text, Sandbox, Settings, Element, Align};
use std::path::Path;
use iced::text_input::TextInput;
use crate::project::create_project;
use crate::project::open_project;

#[derive(Default)]
pub struct Welcome {
    project_folder_status: String,
    increment_button: button::State,
    project_folder_input: iced::text_input::State,
    project_folder: String
}


impl Sandbox for Welcome {

    type Message = Message;
    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Pakr - GUI frontend for pak")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::NewProject => {
                println!("Creating new project...");
                create_project(self.project_folder.to_string());
                self.project_folder_status = "Loading project...".to_string();
                //     Open project in the pakr editor
                open_project(self.project_folder.to_string());
            },
            Message::ProjectFolderUpdated(value) => {
                self.project_folder = value.to_string();
                let path = Path::new(&value);
                if path.exists(){
                    self.project_folder_status = "Project NOT creatable".to_string();
                } else {
                    self.project_folder_status = "Project creatable".to_string();
                }
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let project_folder_textbox = TextInput::new(&mut self.project_folder_input,  "Project Path", &self.project_folder, Message::ProjectFolderUpdated).padding(15);
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(
                Button::new(&mut self.increment_button, Text::new("New Project"))
                .on_press(Message::NewProject)
            )
            .push(
                Text::new("\n").height(iced::Length::Units(15))
            )
            .push(
                project_folder_textbox
            )
            .push(
                Text::new("\n").height(iced::Length::Units(15))
            )
            .push(
                Text::new(&self.project_folder_status.to_string()).size(24),
            )
            .into()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    NewProject,
    ProjectFolderUpdated(String)
}
pub fn open_welcome(){
    Welcome::run(Settings::default()).unwrap();
}

use iced::button;
use iced::{Button, Column, Text, Sandbox, Settings, Element, Align};
use std::path::Path;
use iced::text_input::TextInput;
use crate::project::create_project;

#[derive(Default)]
pub struct Editor {
    value: i32
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
           Message::SaveProject => {
               println!("Saving project...");
               self.value += 1;
           }
        }
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(
                Text::new("Welcome to pakr!").size(40)
            )
            .into()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    SaveProject
}
pub fn open_editor(){
    Editor::run(Settings::default()).unwrap();
}

#![windows_subsystem = "windows"]

use iced::{
    button, scrollable, text_input, Align, Button, Column, Container, Element, HorizontalAlignment,
    Length, Row, Sandbox, Scrollable, Settings, Space, Text, TextInput,
};

const SPACING: u16 = 10;

pub fn main() -> iced::Result {
    MainGui::run(Settings::default())
}

#[derive(Clone, Default)]
struct MainGui {}

impl MainGui {}

#[derive(Debug, Clone)]
enum Message {}

impl Sandbox for MainGui {
    type Message = Message;

    fn new() -> Self {
        let gui = Self::default();
        gui
    }

    fn title(&self) -> String {
        String::from("Hall Angle")
    }

    fn update(&mut self, message: Message) {
        match message {}
    }

    fn view(&mut self) -> Element<Message> {
        let title = Text::new("todos")
            .width(Length::Fill)
            .size(100)
            .color([0.5, 0.5, 0.5])
            .horizontal_alignment(HorizontalAlignment::Center);

        Column::new()
            .padding(SPACING)
            .spacing(SPACING)
            .push(title)
            .into()
    }
}

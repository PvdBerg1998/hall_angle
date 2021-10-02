#![windows_subsystem = "windows"]

use iced::{
    text_input, Column, Element, Font, HorizontalAlignment, Length, Row, Sandbox, Settings, Text,
    TextInput,
};
use std::str::FromStr;

const PADDING: u16 = 10;
const SPACING: u16 = 30;
const FONT_SIZE: u16 = 64;
const FONT: Font = Font::External {
    name: "Source Code Pro",
    bytes: include_bytes!("SourceCodePro-Regular.ttf"),
};

pub fn main() -> iced::Result {
    MainGui::run(Settings::default())
}

#[derive(Clone, Default)]
struct MainGui {
    v0_input_state: text_input::State,
    v0_input_value: String,
    v0_input: Option<f64>,
    phi_input_state: text_input::State,
    phi_input_value: String,
    phi_input: Option<f64>,
    v_input_state: text_input::State,
    v_input_value: String,
    v_input: Option<f64>,
}

impl MainGui {}

#[derive(Debug, Clone)]
enum Message {
    V0Input(String),
    VInput(String),
    PhiInput(String),
}

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
        match message {
            Message::V0Input(s) => {
                self.v0_input = f64::from_str(&s).ok();
                self.v0_input_value = s;
            }
            Message::PhiInput(s) => {
                self.phi_input = f64::from_str(&s).ok().and_then(|phi| {
                    if (0.0..=90.0).contains(&phi) {
                        Some(phi)
                    } else {
                        None
                    }
                });
                self.phi_input_value = s;
            }
            Message::VInput(s) => {
                self.v_input = f64::from_str(&s).ok();
                self.v_input_value = s;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let title = Text::new("Hall Angle Tool")
            .width(Length::Fill)
            .size(FONT_SIZE)
            .font(FONT)
            .color([0.5, 0.5, 0.5])
            .horizontal_alignment(HorizontalAlignment::Center);

        let v0_label = Text::new("V max").size(FONT_SIZE).font(FONT);
        let v0_input = TextInput::new(
            &mut self.v0_input_state,
            "",
            &self.v0_input_value,
            Message::V0Input,
        )
        .font(FONT)
        .size(FONT_SIZE);
        let v0_row = Row::new()
            .padding(PADDING)
            .spacing(SPACING)
            .push(v0_label)
            .push(v0_input);

        let phi_label = Text::new("Theta").size(FONT_SIZE).font(FONT);
        let phi_input = TextInput::new(
            &mut self.phi_input_state,
            "",
            &self.phi_input_value,
            Message::PhiInput,
        )
        .size(FONT_SIZE)
        .font(FONT);
        let target_v = self
            .phi_input
            .zip(self.v0_input)
            .map(|(phi, v0)| {
                if phi == 90.0 {
                    0.0
                } else {
                    v0 * phi.to_radians().cos()
                }
            })
            .and_then(|target_v| {
                if target_v.is_nan() {
                    None
                } else {
                    Some(target_v)
                }
            })
            .map_or_else(
                || "=            ".to_owned(),
                |target_v| format!("= {: >10} ", format!("{:.3e}", target_v)),
            );
        let target_v_label = Text::new(&target_v).size(FONT_SIZE).font(FONT);
        let phi_row = Row::new()
            .padding(PADDING)
            .spacing(SPACING)
            .push(phi_label)
            .push(phi_input)
            .push(target_v_label);

        let v_label = Text::new("V now").size(FONT_SIZE).font(FONT);
        let v_input = TextInput::new(
            &mut self.v_input_state,
            "",
            &self.v_input_value,
            Message::VInput,
        )
        .size(FONT_SIZE)
        .font(FONT);
        let phi = self
            .v_input
            .zip(self.v0_input)
            .map(|(v, v0)| (v / v0).acos().to_degrees())
            .and_then(|phi| if phi.is_nan() { None } else { Some(phi) })
            .map_or_else(
                || "=            ".to_owned(),
                |phi| format!("= {: >10}°", format!("{:0>2.3}", phi)),
            );
        let phi_label = Text::new(&phi).size(FONT_SIZE).font(FONT);
        let v_row = Row::new()
            .padding(PADDING)
            .spacing(SPACING)
            .push(v_label)
            .push(v_input)
            .push(phi_label);

        Column::new()
            .padding(PADDING)
            .spacing(SPACING)
            .push(title)
            .push(v0_row)
            .push(phi_row)
            .push(v_row)
            .into()
    }
}

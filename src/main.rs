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
    theta_input_state: text_input::State,
    theta_input_value: String,
    theta_input: Option<f64>,
    v_input_state: text_input::State,
    v_input_value: String,
    v_input: Option<f64>,
}

impl MainGui {}

#[derive(Debug, Clone)]
enum Message {
    V0Input(String),
    ThetaInput(String),
    VInput(String),
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
            Message::ThetaInput(s) => {
                self.theta_input = f64::from_str(&s).ok().and_then(|theta| {
                    if (0.0..=90.0).contains(&theta) {
                        Some(theta)
                    } else {
                        None
                    }
                });
                self.theta_input_value = s;
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

        let v0_label = Text::new("V0").size(FONT_SIZE).font(FONT);
        let v0_input = TextInput::new(
            &mut self.v0_input_state,
            "",
            &self.v0_input_value,
            Message::V0Input,
        )
        .font(FONT)
        .size(FONT_SIZE);
        let v0_row = Row::new().spacing(SPACING).push(v0_label).push(v0_input);

        let theta_label = Text::new("Target angle").size(FONT_SIZE).font(FONT);
        let theta_input = TextInput::new(
            &mut self.theta_input_state,
            "",
            &self.theta_input_value,
            Message::ThetaInput,
        )
        .size(FONT_SIZE)
        .font(FONT);
        let theta_row = Row::new()
            .spacing(SPACING)
            .push(theta_label)
            .push(theta_input);

        let target_v = self
            .theta_input
            .zip(self.v0_input)
            .map(|(theta, v0)| {
                if theta == 90.0 {
                    0.0
                } else {
                    v0 * theta.to_radians().cos()
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
                || "Target V = ?".to_owned(),
                |target_v| format!("Target V = {:0>15.12}", target_v),
            );
        let target_v_label = Text::new(&target_v).size(FONT_SIZE).font(FONT);

        // todo: exponent notation checkbox

        let v_label = Text::new("Actual V").size(FONT_SIZE).font(FONT);
        let v_input = TextInput::new(
            &mut self.v_input_state,
            "",
            &self.v_input_value,
            Message::VInput,
        )
        .size(FONT_SIZE)
        .font(FONT);
        let v_row = Row::new().spacing(SPACING).push(v_label).push(v_input);

        let theta = self
            .v_input
            .zip(self.v0_input)
            .map(|(v, v0)| (v / v0).acos().to_degrees())
            .and_then(|theta| if theta.is_nan() { None } else { Some(theta) })
            .map_or_else(
                || "Actual angle = ?".to_owned(),
                |theta| format!("Actual angle = {:0>6.3}", theta),
            );
        let theta_label = Text::new(&theta).size(FONT_SIZE).font(FONT);

        Column::new()
            .padding(PADDING)
            .spacing(SPACING)
            .push(title)
            .push(v0_row)
            .push(theta_row)
            .push(target_v_label)
            .push(v_row)
            .push(theta_label)
            .into()
    }
}

#![windows_subsystem = "windows"]

use iced::{
    button, button::Button, checkbox::Checkbox, text_input, Column, Element, Font,
    HorizontalAlignment, Length, Row, Sandbox, Settings, Text, TextInput,
};
use std::str::FromStr;

const PADDING: u16 = 10;
const SPACING: u16 = 30;
const FONT_SIZE: u16 = 48;
const FONT: Font = Font::External {
    name: "Source Code Pro",
    bytes: include_bytes!("SourceCodePro-Regular.ttf"),
};

pub fn main() -> iced::Result {
    let mut settings = Settings::default();
    settings.window.size = (900, 600);
    settings.window.min_size = Some((900, 600));
    MainGui::run(settings)
}

#[derive(Clone, Default)]
struct MainGui {
    v0_input_state: text_input::State,
    v0_input_value: String,
    v0_input: Option<f64>,
    theta_input_state: text_input::State,
    theta_input_value: String,
    theta_input: Option<i64>,
    theta_up_1_button: button::State,
    theta_up_5_button: button::State,
    theta_down_1_button: button::State,
    theta_down_5_button: button::State,
    v_input_state: text_input::State,
    v_input_value: String,
    v_input: Option<f64>,
    scientific: bool,
}

impl MainGui {}

#[derive(Debug, Clone)]
enum Message {
    V0Input(String),
    ThetaInput(String),
    VInput(String),
    ThetaChange(i64),
    ScientificChange(bool),
}

impl Sandbox for MainGui {
    type Message = Message;

    fn new() -> Self {
        MainGui {
            theta_input: Some(0),
            theta_input_value: "0".to_owned(),
            ..Default::default()
        }
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
                self.theta_input = i64::from_str(&s).ok().and_then(|theta| {
                    if (0..=90).contains(&theta) {
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
            Message::ThetaChange(amount) => {
                self.theta_input = self
                    .theta_input
                    .map(|theta| (theta + amount).max(0).min(90));
                if let Some(theta) = self.theta_input {
                    self.theta_input_value = theta.to_string();
                }
            }
            Message::ScientificChange(b) => {
                self.scientific = b;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let title = Text::new("Hall Angle Tool")
            .width(Length::Fill)
            .size(FONT_SIZE * 3 / 2)
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
        let theta_up_1_button = Button::new(
            &mut self.theta_up_1_button,
            Text::new("+1").size(FONT_SIZE).font(FONT),
        )
        .on_press(Message::ThetaChange(1));
        let theta_up_5_button = Button::new(
            &mut self.theta_up_5_button,
            Text::new("+5").size(FONT_SIZE).font(FONT),
        )
        .on_press(Message::ThetaChange(5));
        let theta_down_1_button = Button::new(
            &mut self.theta_down_1_button,
            Text::new("-1").size(FONT_SIZE).font(FONT),
        )
        .on_press(Message::ThetaChange(-1));
        let theta_down_5_button = Button::new(
            &mut self.theta_down_5_button,
            Text::new("-5").size(FONT_SIZE).font(FONT),
        )
        .on_press(Message::ThetaChange(-5));
        let theta_row = Row::new()
            .spacing(SPACING)
            .push(theta_label)
            .push(theta_input)
            .push(theta_up_1_button)
            .push(theta_up_5_button)
            .push(theta_down_1_button)
            .push(theta_down_5_button);

        let target_v = self
            .theta_input
            .zip(self.v0_input)
            .map(|(theta, v0)| {
                if theta == 90 {
                    0.0
                } else {
                    v0 * (theta as f64).to_radians().cos()
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
                |target_v| {
                    if self.scientific {
                        format!("Target V = {:.8e}", target_v)
                    } else {
                        format!("Target V = {:.16}", target_v)
                    }
                },
            );
        let target_v_label = Text::new(&target_v).size(FONT_SIZE).font(FONT);

        let scientific_checkbox = Checkbox::new(
            self.scientific,
            "Scientific notation",
            Message::ScientificChange,
        )
        .size(FONT_SIZE / 2)
        .text_size(FONT_SIZE / 2)
        .font(FONT);

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
            .push(scientific_checkbox)
            .push(v_row)
            .push(theta_label)
            .into()
    }
}

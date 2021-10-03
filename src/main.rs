#![windows_subsystem = "windows"]

use iced::{
    button, button::Button, checkbox::Checkbox, text_input, Column, Element, Font,
    HorizontalAlignment, Length, Row, Sandbox, Settings, Text, TextInput,
};
use std::io::Write;
use std::path::Path;
use std::{fs::OpenOptions, str::FromStr};

const PADDING: u16 = 10;
const SPACING: u16 = 30;
const FONT_SIZE: u16 = 48;
const FONT: Font = Font::External {
    name: "Source Code Pro",
    bytes: include_bytes!("SourceCodePro-Regular.ttf"),
};

pub fn main() -> iced::Result {
    let mut settings = Settings::default();
    settings.window.size = (960, 640);
    settings.window.min_size = Some((960, 640));
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
    filename_input_state: text_input::State,
    filename_input_value: String,
    save_button: button::State,
    restart_button: button::State,
}

#[derive(Debug, Clone)]
enum Message {
    V0Input(String),
    ThetaInput(String),
    VInput(String),
    ThetaChange(i64),
    ScientificChange(bool),
    FilenameInput(String),
    Save,
    Restart,
}

impl Sandbox for MainGui {
    type Message = Message;

    fn new() -> Self {
        let filename = {
            let mut i = 1;
            loop {
                let filename = format!("hall_angles_{}", i);
                let path = format!("{}.csv", filename);
                if !Path::new(&path).exists() {
                    break filename;
                }
                i += 1;
            }
        };

        MainGui {
            theta_input: Some(0),
            theta_input_value: "0".to_owned(),
            filename_input_value: filename,
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
            Message::FilenameInput(s) => {
                self.filename_input_value = s;
            }
            Message::Save => {
                if self.filename_input_value.is_empty() {
                    return;
                }

                if let Some((v0, v)) = self.v0_input.zip(self.v_input) {
                    let path = format!("{}.csv", self.filename_input_value);
                    let add_header = !Path::new(&path).exists();

                    let mut file = OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(&path)
                        .expect("Appending to storage file failed");

                    if add_header {
                        writeln!(&mut file, "v0,v,theta").expect("Writing to storage file failed");
                    }

                    let theta = (v / v0).acos().to_degrees();

                    writeln!(&mut file, "{},{},{}", v0, v, theta)
                        .expect("Writing to storage file failed");
                }
            }
            Message::Restart => {
                *self = Self::new();
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

        let filename_input = TextInput::new(
            &mut self.filename_input_state,
            "",
            &self.filename_input_value,
            Message::FilenameInput,
        )
        .size(FONT_SIZE)
        .font(FONT);
        let mut save_button = Button::new(
            &mut self.save_button,
            Text::new("Save angle").size(FONT_SIZE).font(FONT),
        );
        if self.v0_input.is_some() && self.v_input.is_some() {
            save_button = save_button.on_press(Message::Save);
        }
        let restart_button = Button::new(
            &mut self.restart_button,
            Text::new("Restart").size(FONT_SIZE).font(FONT),
        )
        .on_press(Message::Restart);
        let save_row = Row::new()
            .spacing(SPACING)
            .push(filename_input)
            .push(save_button)
            .push(restart_button);

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
            .push(save_row)
            .into()
    }
}

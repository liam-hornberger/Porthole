use iced::widget::{column, text, text_input, button, Text};
use iced::{Element, Fill, Font, Size};
use std::path::PathBuf;

mod holesail;
mod theme;

fn main() -> iced::Result {
    iced::application(State::default, update, view)
        .title("Porthole")
        .theme(|_state: &State| theme::dark_theme()) 
        .window(iced::window::Settings {
            // Set the initial launch size
            size: Size::new(600.0, 300.0),
            resizable: false, 
            ..Default::default()
        })
        .run()
}

struct State {
    content: String,
    status: String,
    port: String,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    Submit(String),
    PortChanged(String),
}

// Defaults for State (Content and string)
impl Default for State {
    fn default() -> Self {
        if holesail::is_active() {
            Self {
                content: String::new(),
                status: String::from("Disconnect"),
                port: String::new()
            }
        } else {
            Self {
                content: String::new(),
                status: String::from("Connect"),
                port: String::new(),
            }
        } 
    }
}
fn update(state: &mut State, message: Message) {
    match message {
        Message::InputChanged(new_content) => {
            state.content = new_content;
        }
        Message::Submit(code) => {
            println!("Code: {} ", code);
            // i know it looks bad...
            if state.status == "Connect".to_string() {
                state.status = "Downloading...".to_string();
                let holesail_path: PathBuf = holesail::download().expect("Failed to download Holesail");
                state.status = "Connecting...".to_string();
                holesail::connect(&holesail_path, &code);
                state.status = "Disconnect".to_string();
            } else {
                state.status = "Stopping".to_string();
                holesail::stop();
                state.status = "Connect".to_string();
            }
        }
        Message::PortChanged(new_stuff) => {
            state.port = new_stuff;
        }
    }
}

fn view(state: &State) -> Element<'_, Message> {
    column![
        text("Porthole")
            .font(Font::MONOSPACE)
            .size(30)
            .line_height(1.5)
            .width(Fill)
            .center(),
        // Holesail Code Input
        text_input("Enter the Holesail code (hs://...)", &state.content)
            .on_input(Message::InputChanged)
            .style(theme::round_text_input)
            .line_height(2.0),
        column![
            text_input("Port... [NOT IMPLEMENTED]", &state.port)
            .on_input(Message::PortChanged)
            .style(theme::round_text_input)
            .width(300)
            .line_height(2.0)
        ].spacing(6),
        button(Text::new(state.status.clone()).font(Font::MONOSPACE).width(Fill).center())
            .on_press(Message::Submit(state.content.clone()))
            .width(Fill)
            .style(theme::round_button)
            .height(60),
    ]
    .spacing(20)
    .padding(20)
    .into()
}

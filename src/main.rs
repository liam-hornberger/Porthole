use iced::widget::{column, text, text_input, button, Text};
use iced::{Element, Fill, Font};

mod holesail;

fn main() -> iced::Result {
    iced::run(update, view)
}

struct State {
    content: String,
    status: String,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    Submit(String),
}

// Defaults for State (Content and string)
impl Default for State {
    fn default() -> Self {
        Self {
            content: String::new(),
            status: String::from("Connect"),
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
            state.status = "Downloading...".to_string();
            let holesail_path = holesail::download();
            state.status = "Connecting...".to_string();
            // Connect TODO
            state.status = "Disconnect".to_string();
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
            .on_input(Message::InputChanged),
        button(Text::new(state.status.clone()).font(Font::MONOSPACE).width(Fill).center())
            .on_press(Message::Submit(state.content.clone()))
            .width(Fill),
    ]
    .spacing(20)
    .padding(20)
    .into()
}

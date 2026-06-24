use iced::widget::{column, text, text_input, button, Text};
use iced::{Element, Fill, Font};

mod holesail;

fn main() -> iced::Result {
    // Pass only update and view here
    iced::run(update, view)
}

// Derive Default so iced knows how to initialize the struct
#[derive(Default)]
struct State {
    content: String,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    Submit(String), // Replaced Submit::Submit here
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::InputChanged(new_content) => {
            state.content = new_content;
        }
        Message::Submit(code) => {
            println!("Code: {} ", code);
            holesail::main(code);
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
        button(Text::new("Connect").font(Font::MONOSPACE).width(Fill).center())
            .on_press(Message::Submit(state.content.clone()))
            .width(Fill),
    ]
    .spacing(20)
    .padding(20)
    .into()
}

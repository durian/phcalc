use iced::{
    widget::{button, column, text},
    Element,
};

#[derive(Debug, Default)]
struct AppState {}

#[derive(Debug, Clone)]
enum Message {
    Exit,
}

impl AppState {
    fn update(state: &mut AppState, message: Message) {
        match message {
            Message::Exit => eprintln!("Exit"),
        }
    }

    fn view(state: &AppState) -> Element<Message> {
        column![text("PHCalc"), button("Exit").on_press(Message::Exit),].into()
    }
}

fn main() -> iced::Result {
    iced::application("PHCalc", AppState::update, AppState::view)
        //.theme(|_| iced::Theme::KanagawaDragon)
        .run()
}

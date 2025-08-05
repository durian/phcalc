use iced::{
    widget::{button, column, row, slider, text},
    Element, Task,
};

#[derive(Debug, Default)]
struct AppState {
    ph_diameter: f32,
    ph_thickness: f32,
    ph_viewangle: f32,
}

#[derive(Debug, Clone)]
enum Message {
    Exit,
    UpdatePhDiameter(f32),
}

impl AppState {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                ph_diameter: 0.05,
                ph_thickness: 0.1,
                ph_viewangle: 0.,
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Exit => eprintln!("Exit"),
            Message::UpdatePhDiameter(v) => self.ph_diameter = v,
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            text("PHCalc").size(32),
            column![
                text("View angle").size(32),
                row![
                    text(self.ph_diameter),
                    slider(0.01..=1.00, self.ph_diameter, |v| {
                        Message::UpdatePhDiameter(v)
                    })
                    .step(0.01),
                ],
                text(self.ph_viewangle),
            ],
            column![text("Focal length").size(32),],
            button(text("Exit").size(24))
                .padding(8)
                .on_press(Message::Exit),
        ]
        .into()
    }
}

fn main() -> iced::Result {
    iced::application("PHCalc", AppState::update, AppState::view)
        .theme(|_| iced::Theme::KanagawaDragon)
        .run_with(AppState::new)
}

use iced::{
    widget::{button, column, horizontal_rule, row, slider, text},
    Element, Length, Task,
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
    UpdatePhThickness(f32),
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
            Message::UpdatePhDiameter(v) => {
                self.ph_diameter = v;
                self.ph_viewangle = calc_viewangle(self.ph_diameter, self.ph_thickness)
            }
            Message::UpdatePhThickness(v) => {
                self.ph_thickness = v;
                self.ph_viewangle = calc_viewangle(self.ph_diameter, self.ph_thickness)
            }
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            text("PHCalc").size(32),
            horizontal_rule(48),
            column![
                // Label for this pane.
                text("View angle").size(32),
                // Value and slider.
                row![
                    text(format!("diameter {:.2} mm  ", self.ph_diameter))
                        .width(Length::FillPortion(1)),
                    slider(0.01..=1.00, self.ph_diameter, |v| {
                        Message::UpdatePhDiameter(v)
                    })
                    .step(0.01)
                    .width(Length::FillPortion(4)),
                ]
                .padding(16),
                row![
                    text(format!("thickness {:.2} mm  ", self.ph_thickness))
                        .width(Length::FillPortion(1)),
                    slider(0.01..=1.00, self.ph_thickness, |v| {
                        Message::UpdatePhThickness(v)
                    })
                    .step(0.01)
                    .width(Length::FillPortion(4)),
                ]
                .padding(16),
                // Calculated value.
                text(format!("View angle {:.0} degrees", self.ph_viewangle)),
            ],
            column![text("Focal length").size(32),],
            button(text("Exit").size(24))
                .padding(8)
                .on_press(Message::Exit),
        ]
        .spacing(20)
        .padding(20)
        .into()
    }
}

fn main() -> iced::Result {
    iced::application("PHCalc", AppState::update, AppState::view)
        .theme(|_| iced::Theme::KanagawaDragon)
        .run_with(AppState::new)
}

fn calc_viewangle(diameter: f32, thickness: f32) -> f32 {
    let div = diameter / thickness;
    let viewangle: f32 = 2. * div.atan();
    viewangle / 3.1415 * 180.0
}

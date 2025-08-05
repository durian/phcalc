use iced::{
    widget::{button, column, horizontal_rule, row, slider, text},
    window, Element, Length, Task,
};

#[derive(Debug, Default)]
struct AppState {
    ph_diameter: f32,
    ph_thickness: f32,
    ph_viewangle: f32,
    ph_diagonal: f32,
    ph_focallength: f32,
}

#[derive(Debug, Clone)]
enum Message {
    Exit,
    UpdatePhDiameter(f32),
    UpdatePhThickness(f32),
    UpdatePhDiagonal(f32),
}

impl AppState {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                ph_diameter: 0.05,
                ph_thickness: 0.1,
                ph_viewangle: 0.,
                ph_diagonal: 42.,
                ph_focallength: 0.,
            },
            Task::none(),
        )
    }

    fn _close(id: window::Id) -> Task<Message> {
        window::close(id)
    }

    fn calc_viewangle(&self) -> f32 {
        let div = self.ph_diameter / self.ph_thickness;
        let viewangle: f32 = div.atan();
        viewangle / 3.1415 * 180.0
    }

    fn calc_focallength(&self) -> f32 {
        let div = self.ph_diameter / self.ph_thickness;
        0.5 * self.ph_diagonal / div
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Exit => eprintln!("Exit"),
            Message::UpdatePhDiameter(v) => {
                self.ph_diameter = v;
                self.ph_viewangle = self.calc_viewangle();
                self.ph_focallength = self.calc_focallength()
            }
            Message::UpdatePhThickness(v) => {
                self.ph_thickness = v;
                self.ph_viewangle = self.calc_viewangle();
                self.ph_focallength = self.calc_focallength()
            }
            Message::UpdatePhDiagonal(v) => {
                self.ph_diagonal = v;
                self.ph_focallength = self.calc_focallength()
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
                .padding(8),
                row![
                    text(format!("thickness {:.2} mm  ", self.ph_thickness))
                        .width(Length::FillPortion(1)),
                    slider(0.01..=1.00, self.ph_thickness, |v| {
                        Message::UpdatePhThickness(v)
                    })
                    .step(0.01)
                    .width(Length::FillPortion(4)),
                ]
                .padding(8),
                // Calculated value.
                text(format!("View angle {:.0} degrees", 2. * self.ph_viewangle)),
            ],
            horizontal_rule(48),
            column![
                text("Focal length").size(32),
                row![
                    text(format!("diagonal {:.0} mm  ", self.ph_diagonal))
                        .width(Length::FillPortion(1)),
                    slider(10.0..=200., self.ph_diagonal, |v| {
                        Message::UpdatePhDiagonal(v)
                    })
                    .step(1.)
                    .width(Length::FillPortion(4)),
                ]
                .padding(8),
                // Calculated value.
                text(format!("Focal length {:.0}  ", self.ph_focallength)),
            ],
            horizontal_rule(48),
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

// These could be Impl in Appstate...
fn _calc_viewangle(diameter: f32, thickness: f32) -> f32 {
    let div = diameter / thickness;
    let viewangle: f32 = div.atan();
    viewangle / 3.1415 * 180.0
}

fn _calc_focallength(diagonal: f32, diameter: f32, thickness: f32) -> f32 {
    let div = diameter / thickness;
    0.5 * diagonal / div
}

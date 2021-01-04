use iced::button::State as ButtonState;
use iced::{Align, Button, Column, Element, Row, Sandbox, Settings, Text};

#[derive(Default)]
pub struct TrackingView {
    connect_button: ButtonState,
    disconnect_button: ButtonState,
    start_tracking_button: ButtonState,
    longitude: f32,
    latitude: f32,
    altitude: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum TrackingMessage {
    ConnectButtonPressed,
    DisconnectButtonPressed,
    StartTrackingPressed,
}

impl Sandbox for TrackingView {
    type Message = TrackingMessage;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Flyg")
    }

    fn update(&mut self, _message: TrackingMessage) {}

    fn view(&mut self) -> Element<TrackingMessage> {
        Column::new()
            .padding(20)
            .align_items(Align::Start)
            .spacing(10)
            .push(Text::new("Flyg").size(50))
            .push(Text::new("Before any tracking can start, you have to successfully connect to the simulator. Do this, by pressing the 'Connect' button:"))
            .push(Row::new().spacing(10).push(Button::new(&mut self.connect_button, Text::new("Connect")).on_press(TrackingMessage::ConnectButtonPressed)).push(Button::new(&mut self.disconnect_button, Text::new("Disconnect")).on_press(TrackingMessage::DisconnectButtonPressed)))
            .push(Text::new("As soon as you are connected to the simulator and a plane is loaded up, you can start the tracking by pressing the 'Start tracking now' button:"))
            .push(Button::new(&mut self.start_tracking_button, Text::new("Start tracking now")).on_press(TrackingMessage::StartTrackingPressed))
            .push(Row::new().spacing(10).push(Text::new("Longitude:")).push(Text::new(&self.longitude.to_string())))
            .push(Row::new().spacing(10).push(Text::new("Latitude:")).push(Text::new(&self.latitude.to_string())))
            .push(Row::new().spacing(10).push(Text::new("Altitude:")).push(Text::new(&self.altitude.to_string())))
            .into()
    }
}

fn main() -> iced::Result {
    let mut settings = Settings::default();
    settings.window.resizable = false;
    settings.window.size = (700, 380);
    TrackingView::run(settings)
}

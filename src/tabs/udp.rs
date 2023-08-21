use crate::tabs::Tab;
use crate::App;

pub struct Udp;

impl Tab for Udp {
    fn ui(&self, _app: &mut App, ui: &mut egui::Ui) {
        ui.label("Udp");
    }

    fn title(&self) -> &str {
        "Udp"
    }
}
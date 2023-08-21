use crate::tabs::Tab;
use crate::App;

pub struct About;

impl Tab for About {
    fn ui(&self, _app: &mut App, ui: &mut egui::Ui) {
        ui.label("About");
    }

    fn title(&self) -> &str {
        "About"
    }
}
mod client;
mod server;
mod udp;
mod about;

use egui_dock::Tree;

use client::Client;
use server::Server;
use udp::Udp;
use about::About;

use crate::App;

pub trait Tab {
    fn ui(&self, app: &mut App, ui: &mut egui::Ui);
    fn title(&self) -> &str;
}

impl egui_dock::TabViewer for App {
    type Tab = Box<dyn Tab>;

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        tab.ui(self, ui);
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.title().into()
    }
}

pub fn default_ui() -> Tree<Box<dyn Tab>> {
    Tree::new(vec![Box::new(Client), Box::new(Server), Box::new(Udp), Box::new(About)])
}

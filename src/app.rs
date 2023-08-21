use std::sync::{Arc, RwLock};

use egui::{global_dark_light_mode_switch, Layout, Align, Context};
use egui_dock::{DockArea, Tree};
use flume::{Sender, Receiver, unbounded};

use crate::tabs::{default_ui, Tab};

#[derive(Debug)]
pub enum ClientMsg {
    StartConnection,
    StopConnection,
    ViewStandardPorts,
    SendMessage,
    SaveConversation,
    ClearConversation,
}

#[derive(Debug)]
pub enum ServerMsg {
    StartListening,
    StopListening,
    ViewStandardPorts,
    DisconnectClient,
    SendMessage,
    SaveConversation,
    ClearConversation,
}

pub struct App {
    ctx: egui::Context,
    tree: Arc<RwLock<Tree<Box<dyn Tab>>>>,

    client_msg_channel: (Sender<ClientMsg>, Receiver<ClientMsg>),
    server_msg_channel: (Sender<ServerMsg>, Receiver<ServerMsg>),

    pub client_ip_address: String,
    pub client_port: String,
    pub client_received_msg: String,
    pub client_send_msg: String,
    pub client_connected_to_host: Option<String>,

    pub server_ip_address: String,
    pub server_port: String,
    pub server_received_msg: String,
    pub server_send_msg: String,
    pub server_connected_client: Option<String>,
}

impl App {
    pub fn new(ctx: egui::Context) -> Self {
        App {
            ctx,
            tree: Arc::new(RwLock::new(default_ui())),

            client_msg_channel: unbounded(),
            server_msg_channel: unbounded(),

            client_ip_address: "127.0.0.1".to_string(),
            client_port: "8585".to_string(),
            client_received_msg: String::new(),
            client_send_msg: String::new(),
            client_connected_to_host: None,

            server_ip_address: "127.0.0.1".to_string(),
            server_port: "8585".to_string(),
            server_received_msg: String::new(),
            server_send_msg: String::new(),
            server_connected_client: None
        }
    }

    pub fn run() -> Result<(), eframe::Error> {
        let native_options = eframe::NativeOptions {
            initial_window_size: Some(egui::vec2(700f32, 500f32)),
            default_theme: eframe::Theme::Light,
            .. eframe::NativeOptions::default()
        };

        eframe::run_native(
            "RustySockets",
            native_options,
            Box::new(|cc| Box::new(App::new(cc.egui_ctx.clone()))))
    }

    pub fn post_client_msg(&mut self, msg: ClientMsg) {
        self.client_msg_channel.0.send(msg).unwrap();
        self.ctx.request_repaint();
    }

    fn handle_client_msg(&mut self) {
        while let Ok(msg) = self.client_msg_channel.1.try_recv() {
            println!("{:?}", msg);

            match msg {
                ClientMsg::StartConnection => {},
                ClientMsg::StopConnection => {},
                ClientMsg::ViewStandardPorts => {},
                ClientMsg::SendMessage => {},
                ClientMsg::SaveConversation => {},
                ClientMsg::ClearConversation => {},
            }
        }
    }

    pub fn post_server_msg(&mut self, msg: ServerMsg) {
        self.server_msg_channel.0.send(msg).unwrap();
        self.ctx.request_repaint();
    }

    fn handle_server_msg(&mut self) {
        while let Ok(msg) = self.server_msg_channel.1.try_recv() {
            println!("{:?}", msg);

            match msg {
                ServerMsg::StartListening => {},
                ServerMsg::StopListening => {},
                ServerMsg::ViewStandardPorts => {},
                ServerMsg::DisconnectClient => {},
                ServerMsg::SendMessage => {},
                ServerMsg::SaveConversation => {},
                ServerMsg::ClearConversation => {},
            }
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.handle_client_msg();
            self.handle_server_msg();

            ui.with_layout(Layout::bottom_up(Align::Max), |ui| {
                global_dark_light_mode_switch(ui);

                DockArea::new(self.tree.clone().write().as_deref_mut().unwrap())
                    .draggable_tabs(false)
                    .show_close_buttons(false)
                    .show_add_popup(false)
                    .show_inside(ui, self);
            })
        });

    }
}
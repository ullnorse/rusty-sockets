use crate::tabs::Tab;
use crate::app::{App, ServerMsg};

pub struct Server;

impl Tab for Server {
    fn ui(&self, app: &mut App, ui: &mut egui::Ui) {
        ui.label("Listen on");

        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.label("IP Address");
                ui.text_edit_singleline(&mut app.server_ip_address);
            });

            ui.horizontal(|ui| {
                ui.label("Port");
                ui.text_edit_singleline(&mut app.server_port);

                if ui.button("Port").on_hover_text("View Standard Ports").clicked() {
                    app.post_server_msg(ServerMsg::ViewStandardPorts);
                }

                if !app.server_listening {
                    if ui.button("Start Listening").clicked() {
                        app.post_server_msg(ServerMsg::StartListening);
                    }
                } else if ui.button("Stop Listening").clicked() {
                    app.post_server_msg(ServerMsg::StopListening);
                }
            });
        });

        ui.label("Connected Client < NONE >");

        ui.group(|ui| {
            ui.allocate_ui_with_layout(egui::Vec2 { x: ui.available_width(), y: ui.available_height() - 25f32 }, egui::Layout::bottom_up(egui::Align::Min), |ui| {
                ui.horizontal(|ui| {
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.label("Message");

                            ui.add_enabled_ui(app.client_connected_to_host.is_some(), |ui| {
                                ui.add_sized(egui::vec2(ui.available_width() - 178f32, 20f32), egui::TextEdit::singleline(&mut app.client_send_msg));
                                if ui.button("Send").on_hover_text("Send text to client").clicked() {
                                    app.post_server_msg(ServerMsg::SendMessage);
                                }

                                if ui.button("Disconnect").on_hover_text("Disconnect client").clicked() {
                                    app.post_server_msg(ServerMsg::DisconnectClient);
                                }
                            });
                        });
                    });

                    ui.vertical(|ui| {
                        if ui.add_sized(egui::vec2(48f32, 20f32), egui::Button::new("Save")).clicked() {
                            app.post_server_msg(ServerMsg::SaveConversation);
                        }

                        if ui.add_sized(egui::vec2(48f32, 20f32), egui::Button::new("Clear")).clicked() {
                            app.post_server_msg(ServerMsg::ClearConversation);
                        }
                    });
                });

                ui.label("Send");

                ui.vertical(|ui| {
                    ui.label("Conversation with Client");
                    ui.add_sized(ui.available_size(), egui::TextEdit::multiline(&mut app.server_received_msg).interactive(false));
                });
            })
        });
    }

    fn title(&self) -> &str {
        "Server"
    }
}
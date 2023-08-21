use crate::tabs::Tab;
use crate::app::{App, ClientMsg};

pub struct Client;

impl Tab for Client {
    fn ui(&self, app: &mut App, ui: &mut egui::Ui) {
        ui.label("Connect To");

        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.label("IP Address");
                ui.text_edit_singleline(&mut app.client_ip_address);
            });

            ui.horizontal(|ui| {
                ui.label("Port");
                ui.text_edit_singleline(&mut app.client_port);

                if ui.button("Port").on_hover_text("View standard ports").clicked() {
                    app.post_client_msg(ClientMsg::ViewStandardPorts);
                }

                if app.client_connected_to_host.is_none() {
                    if ui.button("Connect").on_hover_text("Start Connection").clicked() {
                        app.post_client_msg(ClientMsg::StartConnection);
                    }
                } else if ui.button("Disconnect").on_hover_text("Stop Connection").clicked() {
                    app.post_client_msg(ClientMsg::StopConnection);
                }

                let mut checked = false;
                ui.checkbox(&mut checked, "Secure").on_hover_text("Set Has Secure");
            });
        });

        ui.label(format!("Connected To < {} >", app.client_connected_to_host.clone().unwrap_or("NONE".to_string())));

        ui.group(|ui| {
            ui.allocate_ui_with_layout(egui::Vec2 { x: ui.available_width(), y: ui.available_height() - 25f32 }, egui::Layout::bottom_up(egui::Align::Min), |ui| {
                ui.horizontal(|ui| {
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.label("Message");

                            ui.add_enabled_ui(app.client_connected_to_host.is_some(), |ui| {
                                ui.add_sized(egui::vec2(ui.available_width() - 100f32, 20f32), egui::TextEdit::singleline(&mut app.client_send_msg));
                                if ui.button("Send").on_hover_text("Send text to host").clicked() {
                                    app.post_client_msg(ClientMsg::SendMessage);
                                }
                            });
                        });
                    });

                    ui.vertical(|ui| {
                        if ui.add_sized(egui::vec2(48f32, 20f32),egui::Button::new("Save"))
                            .on_hover_text("Save conversation with host to a file")
                            .clicked()
                        {
                            app.post_client_msg(ClientMsg::SaveConversation);
                        }

                        if ui.add_sized(egui::vec2(48f32, 20f32), egui::Button::new("Clear"))
                            .on_hover_text("Clear conversation with host")
                            .clicked()
                        {
                            app.post_client_msg(ClientMsg::ClearConversation);
                        }
                    });
                });

                ui.label("Send");

                ui.vertical(|ui| {
                    ui.label("Conversation with host");
                    ui.add_sized(ui.available_size(), egui::TextEdit::multiline(&mut app.client_received_msg).interactive(false));
                });
            })
        });
    }

    fn title(&self) -> &str {
        "Client"
    }
}
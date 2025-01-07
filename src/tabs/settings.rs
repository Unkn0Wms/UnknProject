use egui::{CursorIcon::PointingHand as Clickable, RichText};
use egui_modal::Modal;

use crate::{
    custom_widgets::{Button, CheckBox, TextEdit},
    hacks,
    utils::config::{default_api_endpoint, default_cdn_endpoint, default_cdn_fallback_endpoint},
    MyApp,
};

impl MyApp {
    pub fn render_settings_tab(&mut self, ctx: &egui::Context) -> () {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .drag_to_scroll(false)
                .show(ui, |ui| {
                    ui.heading("Settings");
                    ui.separator();

                    ui.add_space(5.0);

                    // MARK: - Display Options
                    ui.group(|ui| {
                        ui.label("Display Options:");
                        ui.add_space(5.0);

                        if ui
                            .ccheckbox(
                                &mut self.app.config.show_only_favorites,
                                "Show only favorite hacks",
                            )
                            .changed()
                        {
                            self.app.config.save();
                        }
                        if ui
                            .ccheckbox(
                                &mut self.app.config.lowercase_hacks,
                                "Lowercase hack names & descriptions",
                            )
                            .changed()
                        {
                            self.app.hacks = match hacks::Hack::fetch_hacks(
                                &self.app.config.api_endpoint,
                                self.app.config.lowercase_hacks,
                            ) {
                                Ok(hacks) => hacks,
                                Err(_err) => {
                                    self.ui.main_menu_message =
                                        "Failed to fetch hacks.".to_string();
                                    Vec::new()
                                }
                            };

                            self.toasts.info(format!(
                                "Hacks refreshed{}.",
                                if self.app.config.lowercase_hacks {
                                    " (with lowercase)"
                                } else {
                                    ""
                                }
                            ));
                            self.app.config.save();
                        };
                        if ui
                            .ccheckbox(
                                &mut self.app.config.hide_steam_account,
                                "Hide Steam account",
                            )
                            .changed()
                        {
                            self.app.config.save();
                        }
                        if ui
                            .ccheckbox(
                                &mut self.app.config.hide_statistics,
                                "Hide statistics",
                            )
                            .changed()
                        {
                            self.app.config.save();
                        };
                        if ui
                            .ccheckbox(
                                &mut self.app.config.disable_notifications,
                                "Disable notifications",
                            )
                            .changed()
                        {
                            self.app.config.save();
                        }
                        if ui
                            .ccheckbox(
                                &mut self.app.config.skip_injects_delay,
                                "Skip injects delay (visual)",
                            )
                            .changed()
                        {
                            self.app.config.save();
                        }
                        if ui
                            .ccheckbox(
                                &mut self.app.config.automatically_select_hack,
                                "Automatically select recently injected hack",
                            )
                            .changed()
                        {
                            self.app.config.save();
                        }

                        ui.horizontal(|ui| {
                            ui.label("Favorites Color:");
                            if ui
                                .color_edit_button_srgba(&mut self.app.config.favorites_color)
                                .on_hover_cursor(Clickable)
                                .changed()
                            {
                                self.app.config.save();
                            }
                        });
                    });

                    ui.add_space(5.0);

                    // MARK: - Injection/Delay Options
                    ui.group(|ui| {
                        ui.label("Injection Options:");

                        let modal_injector = Modal::new(ctx, "injector_confirm_dialog")
                            .with_close_on_outside_click(true);

                        modal_injector.show(|ui| {
                            ui.label("Select architecture to delete:");
                            ui.horizontal(|ui| {
                                if ui
                                    .cbutton(RichText::new("x64").color(egui::Color32::LIGHT_RED))
                                    .clicked()
                                {
                                    if let Err(err) = self.delete_injectors("x64") {
                                        self.toasts.error(err);
                                    } else {
                                        self.toasts.success("x64 injector deleted.");
                                        modal_injector.close();
                                    }
                                }

                                if ui
                                    .cbutton(RichText::new("x86").color(egui::Color32::LIGHT_RED))
                                    .clicked()
                                {
                                    if let Err(err) = self.delete_injectors("x86") {
                                        self.toasts.error(err);
                                    } else {
                                        self.toasts.success("x86 injector deleted.");
                                        modal_injector.close();
                                    }
                                    modal_injector.close();
                                }

                                if ui
                                    .cbutton(RichText::new("Both").color(egui::Color32::LIGHT_RED))
                                    .clicked()
                                {
                                    if let Err(err) = self.delete_injectors("both") {
                                        self.toasts.error(err);
                                    } else {
                                        self.toasts.success("Both injectors deleted.");
                                        modal_injector.close();
                                    }
                                }

                                if ui.cbutton("Cancel").clicked() {
                                    modal_injector.close();
                                }
                            });
                        });

                        if ui.cbutton("Delete injector").clicked() {
                            modal_injector.open();
                        }
                    });

                    ui.add_space(5.0);

                    ui.horizontal(|ui| {
                        if ui.cbutton("Open loader folder").clicked() {
                            let downloads_dir = dirs::config_dir()
                                .unwrap_or_else(|| std::path::PathBuf::from("."))
                                .join("unknproject");
                            let _ = opener::open(downloads_dir);
                        }

                        if ui.cbutton("Open log file").clicked() {
                            let log_file = dirs::config_dir()
                                .unwrap_or_else(|| std::path::PathBuf::from("."))
                                .join("unknproject")
                                .join("unknproject.log");
                            let _ = opener::open(log_file);
                        }

                        let modal_settings = Modal::new(ctx, "settings_reset_confirm_dialog")
                            .with_close_on_outside_click(true);

                        modal_settings.show(|ui| {
                            ui.label("Are you sure you want to reset the settings?");
                            ui.horizontal(|ui| {
                                if ui
                                    .cbutton(RichText::new("Reset").color(egui::Color32::LIGHT_RED))
                                    .clicked()
                                {
                                    self.app.config.reset();
                                    self.toasts.success("Settings reset.");
                                    modal_settings.close();
                                }

                                if ui.cbutton("Cancel").clicked() {
                                    modal_settings.close();
                                }
                            });
                        });

                        if ui
                            .cbutton(
                                RichText::new("Reset settings").color(egui::Color32::LIGHT_RED),
                            )
                            .clicked()
                        {
                            modal_settings.open();
                        }

                        let modal_statistics = Modal::new(ctx, "statistics_reset_confirm_dialog")
                            .with_close_on_outside_click(true);

                        modal_statistics.show(|ui| {
                            ui.label("Are you sure you want to reset the statistics?");
                            ui.horizontal(|ui| {
                                if ui
                                    .cbutton(RichText::new("Reset").color(egui::Color32::LIGHT_RED))
                                    .clicked()
                                {
                                    self.app.statistics.reset();
                                    self.toasts.success("Statistics reset.");
                                    modal_statistics.close();
                                }

                                if ui.cbutton("Cancel").clicked() {
                                    modal_statistics.close();
                                }
                            });
                        });

                        if ui.cbutton(RichText::new("Reset statistics")).clicked() {
                            modal_statistics.open();
                        }
                    });
                });
        });
    }
}

use egui::{RichText, Sense, TextStyle};

use crate::{
    custom_widgets::{Button, Hyperlink},
    MyApp,
};

impl MyApp {
    pub fn render_about_tab(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .drag_to_scroll(false)
                .show(ui, |ui| {
                    ui.heading("About");
                    ui.separator();
                    if ui
                        .add(
                            egui::Image::new(egui::include_image!("../../resources/img/icon.ico"))
                                .max_width(100.0)
                                .rounding(10.0)
                                .sense(Sense::click()),
                        )
                        .clicked()
                    {
                        self.toasts.info("Hello there!");
                    }
                    ui.label(RichText::new(format!("v{}", self.app_version)).size(15.0));
                    ui.add_space(5.0);
                    ui.label(RichText::new("UnknProject The best in the field of CS:GO Cheats!").size(16.0));
                    ui.label(format!("btw, you opened it {} times", self.app.statistics.opened_count));
                    ui.add_space(5.0);
                    ui.horizontal_wrapped(|ui| {
                        let width =
                            ui.fonts(|f| f.glyph_width(&TextStyle::Body.resolve(ui.style()), ' '));
                        ui.spacing_mut().item_spacing.x = width;

                        ui.clink("Made with egui by Unkn0Wn", "https://github.com/Unkn0Wms");
                    });

                    ui.add_space(5.0);
                    ui.heading("Socials:");
                    ui.add_space(5.0);

                    ui.horizontal(|ui| {
                        ui.link_button(
                            "Discord",
                            "https://discord.gg/Rgx6rnrbAX",
                            &mut self.toasts,
                        );
                        ui.link_button("Telegram", "https://t.me/unkn0wnrage", &mut self.toasts);
                    });

                    ui.add_space(5.0);

                    ui.heading("Keybinds:");

                    let keybinds = vec![
                        ("F5", "Refresh hacks"),
                        ("Enter", "Inject selected hack"),
                        ("Escape", "Deselect hack"),
                    ];

                    for (key, action) in &keybinds {
                        ui.label(format!("{}: {}", key, action));
                    }
                });
        });
    }
}

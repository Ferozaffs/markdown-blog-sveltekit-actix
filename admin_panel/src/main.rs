#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1280.0, 720.0)),
        ..Default::default()
    };
    eframe::run_native(
        "MB - Admin panel",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<AdminPanel>::default()
        }),
    )
}

struct AdminPanel {
    markdown: String,
}

impl Default for AdminPanel {
    fn default() -> Self {
        Self {
            markdown: "_".to_owned(),
        }
    }
}

impl eframe::App for AdminPanel {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Load").clicked() {
                    self.markdown = "Load".to_owned();
                }
                else if ui.button("Save").clicked() {
                    self.markdown = "Save".to_owned();
                }
                else if ui.button("Upload").clicked() {
                    self.markdown = "Upload".to_owned();
                }
                
            });
            let width = ui.available_width();
            let height = ui.available_height();

            ui.horizontal(|ui| {
                ui.set_min_height(height);
                egui::ScrollArea::vertical()
                    .id_source("source")
                    .show(ui, |ui| {
                        ui.add_sized(
                            [width * 0.4, height],
                            egui::TextEdit::multiline(&mut self.markdown),
                        );
                    });
                egui::ScrollArea::vertical()
                    .id_source("preview")
                    .show(ui, |ui| {
                        ui.add_sized(
                            [width * 0.4, height],
                            egui::Label::new(self.markdown.to_owned()),
                        );
                    });
                egui::ScrollArea::vertical()
                    .id_source("data")
                    .show(ui, |ui| {
                        ui.add_sized(
                            [width * 0.2, height],
                            egui::Label::new(self.markdown.to_owned()),
                        );
                    });
            });
            //ui.columns(3, |columns|{
            //    egui::ScrollArea::vertical()
            //        .id_source("source")
            //        .show(&mut columns[0], |ui| {
            //            ui.text_edit_multiline(&mut self.text);
            //        });
            //    egui::ScrollArea::vertical()
            //        .id_source("preview")
            //        .show(&mut columns[1], |ui| {
            //            ui.label(format!("{}", self.text));
            //        });
            //    egui::ScrollArea::vertical()
            //        .id_source("data")
            //        .show(&mut columns[2], |ui| {
            //            ui.label(format!("Data"));
            //        });     
            //});
       });
    }
}
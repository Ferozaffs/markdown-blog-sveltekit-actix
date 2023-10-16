#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::fs;

use eframe::egui;
use egui_file::FileDialog;
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};

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
    cache: CommonMarkCache,
    opened_file: String,
    open_file_dialog: Option<FileDialog>,
}

impl Default for AdminPanel {
    fn default() -> Self {
        Self {
            markdown: "_".to_string(),
            cache: CommonMarkCache::default(),
            opened_file: "_".to_string(),
            open_file_dialog: None
        }
    }
}

impl eframe::App for AdminPanel {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Load").clicked() {
                    let mut dialog = FileDialog::open_file(None);
                    dialog.open();
                    self.open_file_dialog = Some(dialog);
                }
                else if ui.button("Save").clicked() {
                    fs::write(self.opened_file.clone() + "_new", self.markdown.clone()).expect("Unable to write file");
                }
                else if ui.button("Upload").clicked() {
                    self.markdown = "Upload".to_owned();
                }
                
            });

            if let Some(dialog) = &mut self.open_file_dialog {
                if dialog.show(ctx).selected() {
                    if let Some(file) = dialog.path() {
                        self.opened_file = Some(file.to_path_buf()).unwrap().to_str().unwrap().to_string();
                        self.markdown = fs::read_to_string(self.opened_file.clone()).unwrap();  
                    }
                }
            }     

            let width = ui.available_width();
            let height = ui.available_height();

            ui.horizontal(|ui| {
                ui.set_min_height(height);
                egui::ScrollArea::vertical()
                    .id_source("data")
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            ui.add_sized(
                                [width * 0.2, height * 0.1],
                                egui::Button::new("Test1")
                            );
                            ui.add_sized(
                                [width * 0.2, height * 0.1],
                                egui::Button::new("Test2")
                            );
                            ui.add_sized(
                                [width * 0.2, height * 0.1],
                                egui::Button::new("Test3")
                            );
                            ui.add_sized(
                                [width * 0.2, height * 0.1],
                                egui::Button::new("Test4")
                            );
                        });
                    });
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
                        ui.with_layout(egui::Layout::top_down(eframe::emath::Align::Min), |ui| {
                            CommonMarkViewer::new("viewer").show(ui, &mut self.cache, &self.markdown);
                        });
                    });
            });
       });
    }
}
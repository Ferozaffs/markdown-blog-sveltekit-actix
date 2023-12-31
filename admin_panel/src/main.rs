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

enum FileLoadState {
    LoadMarkdown,
    LoadImage
}

struct MetaData {
    title: String,
    tags: Vec<String>
}

struct AdminPanel {
    markdown: String,
    prev_markdown: String,
    opened_markdown_file: String,
    cache: CommonMarkCache,
    file_load_state: Option<FileLoadState>,
    open_file_dialog: Option<FileDialog>,
    meta_data: Option<MetaData>,
    tag_field: String,
}

impl Default for AdminPanel {
    fn default() -> Self {
        Self {
            markdown: "".to_string(),
            prev_markdown: "".to_string(),
            cache: CommonMarkCache::default(),
            opened_markdown_file: "".to_string(),
            file_load_state: None,
            open_file_dialog: None,
            meta_data: None,
            tag_field: "".to_string(),
        }
    }
}

impl eframe::App for AdminPanel {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        self.check_for_macros();
        self.file_dialog_handler(ctx);
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Load").clicked() {
                    self.file_load_state = Some(FileLoadState::LoadMarkdown);
                }
                else if ui.button("Save").clicked() {
                    let meta_data = self.markdown.find("@@META");
                    if meta_data.is_none() {
                        self.genereate_meta_data();
                    }

                    fs::write(self.opened_markdown_file.clone() + "_new", self.markdown.clone()).expect("Unable to write file");
                }
                else if ui.button("Upload").clicked() {
                    self.markdown = "Upload".to_owned();
                }
                
            });

            let width = ui.available_width();
            let height = ui.available_height();

            ui.horizontal(|ui| {
                ui.set_min_height(height);
                //Options panel
                egui::ScrollArea::vertical()
                    .id_source("data")
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            ui.add_sized(
                                [width * 0.2, height * 0.1],
                                egui::Label::new("Options")
                            );

                            let mut regenerate = false;
                            if let Some(meta_data) = self.meta_data.as_mut() {                              
                                ui.label("Title");

                                let response = ui.add(egui::TextEdit::singleline(&mut meta_data.title));
                                if response.changed() {
                                    regenerate = true;
                                }

                                ui.label("Tags");
                                ui.horizontal(|ui| {
                                    ui.add(egui::TextEdit::singleline(&mut self.tag_field));
                                    let response = ui.button("Add");
                                    if response.clicked() {
                                        meta_data.tags.push(self.tag_field.clone());
                                        regenerate = true;
                                    }
                                });

                                let mut index_removal: Vec<usize> = Vec::new(); 
                                for (i, tag) in meta_data.tags.iter().enumerate() {
                                    ui.horizontal(|ui| {
                                        ui.label(tag);
                                        let response = ui.button("X");
                                        if response.clicked() {
                                            index_removal.push(i)
                                        }
                                    });                                  
                                }

                                if index_removal.len() > 0 {                                       
                                    for index in index_removal.iter() {
                                        meta_data.tags.remove(*index);
                                    }
                                    regenerate = true
                                }
                            }
                            else {
                                ui.label("No data");
                            }

                            if regenerate == true {
                                self.regenereate_meta_data(true);
                            }
                        });
                    });
                //Markdown panel
                egui::ScrollArea::vertical()
                    .id_source("source")
                    .show(ui, |ui| {
                        ui.add_sized(
                            [width * 0.4, height],
                            egui::TextEdit::multiline(&mut self.markdown),
                        );
                    });
                //Preview panel
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

impl AdminPanel {
    fn genereate_meta_data(&mut self) {
        self.meta_data = Some(MetaData{
            title: String::from("Title"),
            tags: vec![
                String::from("Tag1"),
                String::from("Tag2")
             ]
        });

        self.regenereate_meta_data(false);
    }

    fn regenereate_meta_data(&mut self, clean: bool) {
        let mut md_meta: String = "@@META\n".to_string();
        md_meta.push_str(format!("@TITLE: {}\n", self.meta_data.as_ref().unwrap().title).as_str());
        md_meta.push_str("@TAGS: ");
        for tag in self.meta_data.as_ref().unwrap().tags.iter() {
            md_meta.push_str(format!("{} ", tag).as_str());
        }

        if clean == true {
            let lines: Vec<&str> = self.markdown.lines().collect();
            let cleaned_lines: Vec<&str> = lines.iter().skip(4).cloned().collect();
            self.markdown = cleaned_lines.join("\n");
        }

        self.markdown = format!("{}\n\n{}", md_meta, self.markdown);
    }

    fn load_meta_data(&mut self){

    }

    fn check_for_macros(&mut self) {
        if self.prev_markdown != self.markdown {
            let meta_data = self.markdown.find("@@META");
            if meta_data.is_some() {
                self.load_meta_data();
            }
            
            let image = self.markdown.find("@@IMAGE");
            if image.is_some() {
                self.file_load_state = Some(FileLoadState::LoadImage);
            }

            self.prev_markdown = self.markdown.clone();
        }
    }
    
    fn file_dialog_handler(&mut self, ctx: &egui::Context) {   

        if self.file_load_state.is_some() && self.open_file_dialog.is_none() {
            let mut dialog = FileDialog::open_file(None);
            dialog.open();
            self.open_file_dialog = Some(dialog);
        }

        if let Some(dialog) = &mut self.open_file_dialog {
            if dialog.show(ctx).selected() {
                if let Some(file) = dialog.path() {
                    match self.file_load_state.as_ref().unwrap() {
                        FileLoadState::LoadMarkdown => {
                            self.opened_markdown_file = Some(file.to_path_buf()).unwrap().to_str().unwrap().to_string();
                            self.markdown = fs::read_to_string(self.opened_markdown_file.clone()).unwrap();                         
                        }
                        FileLoadState::LoadImage => {
                            let image_md = format!("![@IMAGE]({})", Some(file.to_path_buf()).unwrap().to_str().unwrap());
                            self.markdown = self.markdown.replace("@@IMAGE", &image_md);
                        }
                    } 

                    self.file_load_state = None;
                    self.open_file_dialog = None;
                }
            }
        }     
    }
}


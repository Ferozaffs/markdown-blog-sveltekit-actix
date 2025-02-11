#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

pub mod data;
pub mod webconnector;

use std::fs;

use data::ServerContentSummary;
use eframe::egui;
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use egui_file::FileDialog;
use shared;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let mut admin_panel = AdminPanel::default();
    admin_panel.get_server_content_summary();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 720.0]),
        ..Default::default()
    };
    eframe::run_native(
        "MB - Admin panel",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::new(admin_panel))
        }),
    )
}

enum FileLoadState {
    LoadMarkdown,
    LoadImage,
}

struct ProjectPopup {
    show_project_popup: bool,
    title: String,
    description: String,
}

impl Default for ProjectPopup {
    fn default() -> Self {
        Self {
            show_project_popup: false,
            title: "".to_string(),
            description: "".to_string(),
        }
    }
}

struct AdminPanel {
    markdown: String,
    prev_markdown: String,
    opened_markdown_file: String,
    cache: CommonMarkCache,
    file_load_state: Option<FileLoadState>,
    open_file_dialog: Option<FileDialog>,
    meta_data: shared::MetaData,
    tag_field: String,
    selected_download_post: shared::PostSummary,
    selected_download_project: shared::ProjectSummary,
    selected_upload_project: shared::ProjectSummary,
    selected_upload_category: shared::ProjectCategory,
    server_settings: data::ServerSettings,
    server_content_summary: data::ServerContentSummary,
    project_popup: ProjectPopup,
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
            meta_data: shared::MetaData::default(),
            tag_field: "".to_string(),
            selected_download_post: shared::PostSummary::default(),
            selected_download_project: shared::ProjectSummary::default(),
            selected_upload_project: shared::ProjectSummary::default(),
            selected_upload_category: shared::ProjectCategory::default(),
            server_settings: data::load_server_settings(),
            server_content_summary: ServerContentSummary::default(),
            project_popup: ProjectPopup::default(),
        }
    }
}

impl eframe::App for AdminPanel {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.check_for_macros();
        self.file_dialog_handler(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            let width = ui.available_width();
            let height = ui.available_height();

            ui.horizontal(|ui| {
                if ui.button("Load").clicked() {
                    self.file_load_state = Some(FileLoadState::LoadMarkdown);
                } else if ui.button("Save").clicked() {
                    fs::write(
                        self.opened_markdown_file.clone() + "_new",
                        self.markdown.clone(),
                    )
                    .expect("Unable to write file");
                } else if ui.button("Upload").clicked() {
                    self.load_meta_data();
                    match webconnector::upload_post(
                        self.markdown.clone(),
                        &self.server_settings.address,
                        &self.server_settings.api_token,
                    ) {
                        Ok(_) => (),
                        Err(e) => println!("ERROR: {}", e.to_string()),
                    }
                    self.get_server_content_summary();
                }
                ui.label("Posts");
                egui::ComboBox::from_id_salt("Download posts combo")
                    .selected_text(&self.selected_download_post.title)
                    .show_ui(ui, |ui| {
                        // Add items to the ComboBox
                        for option in &self.server_content_summary.posts {
                            let response = ui.selectable_value(
                                &mut self.selected_download_post,
                                option.clone(),
                                option.title.as_str(),
                            );
                            if response.changed() {
                                self.selected_download_project = shared::ProjectSummary::default();
                            }
                        }
                    });
                ui.label("Projects");
                egui::ComboBox::from_id_salt("Download projects combo")
                    .selected_text(&self.selected_download_project.title)
                    .show_ui(ui, |ui| {
                        // Add items to the ComboBox
                        for option in &self.server_content_summary.projects {
                            let response = ui.selectable_value(
                                &mut self.selected_download_project,
                                option.clone(),
                                option.title.as_str(),
                            );
                            if response.changed() {
                                self.selected_download_post = shared::PostSummary::default();
                            }
                        }
                    });

                if ui.button("Download").clicked() {
                    self.download_markdown();
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    if ui.button("Sync").clicked() {
                        self.get_server_content_summary();
                    }
                    ui.add_sized(
                        [width * 0.15, 5.0],
                        egui::TextEdit::singleline(&mut self.server_settings.api_token),
                    );
                    ui.label("Api token: ");

                    ui.add_sized(
                        [width * 0.15, 5.0],
                        egui::TextEdit::singleline(&mut self.server_settings.address),
                    );
                    ui.label("Server: ");
                });
            });

            ui.separator();

            ui.horizontal(|ui| {
                ui.set_min_height(height);
                //Options panel
                egui::ScrollArea::vertical().id_salt("data").show(ui, |ui| {
                    ui.vertical(|ui| {
                        let mut regenerate = false;
                        ui.label("Title");
                        let response =
                            ui.add(egui::TextEdit::singleline(&mut self.meta_data.title));
                        if response.changed() {
                            regenerate = true;
                        }

                        ui.label("Description");
                        let response =
                            ui.add(egui::TextEdit::singleline(&mut self.meta_data.description));
                        if response.changed() {
                            regenerate = true;
                        }

                        let options = vec!["Post", "Project"];
                        ui.label("Type");
                        for (index, option) in options.iter().enumerate() {
                            let response =
                                ui.radio_value(&mut self.meta_data.post_type, index, *option);
                            if response.changed() {
                                if self.meta_data.post_type == 1 {
                                    self.meta_data.project = uuid::Uuid::nil();
                                    self.selected_upload_project =
                                        shared::ProjectSummary::default();
                                }
                                regenerate = true;
                            }
                        }

                        if self.meta_data.post_type == 0 {
                            ui.label("Project");
                            egui::ComboBox::from_id_salt("Project combo")
                                .selected_text(&self.selected_upload_project.title)
                                .show_ui(ui, |ui| {
                                    // Add items to the ComboBox
                                    for option in &self.server_content_summary.projects {
                                        let response = ui.selectable_value(
                                            &mut self.selected_upload_project,
                                            option.clone(),
                                            option.title.as_str(),
                                        );
                                        if response.changed() {
                                            self.meta_data.project =
                                                self.selected_upload_project.id;
                                            regenerate = true;
                                        }
                                    }
                                });

                            ui.label("Tags");
                            ui.horizontal(|ui| {
                                ui.add(egui::TextEdit::singleline(&mut self.tag_field));
                                let response = ui.button("Add");
                                if response.clicked() {
                                    if self.tag_field.len() > 0
                                        && !self.meta_data.tags.contains(&self.tag_field)
                                    {
                                        self.meta_data.tags.push(self.tag_field.clone());
                                        regenerate = true;
                                    }
                                }
                            });
                        } else if self.meta_data.post_type == 1 {
                            let options = vec!["Ongoing", "Completed"];
                            ui.label("Status");
                            for (index, option) in options.iter().enumerate() {
                                let response =
                                    ui.radio_value(&mut self.meta_data.status, index, *option);
                                if response.changed() {
                                    regenerate = true;
                                }
                            }

                            ui.label("Category");
                            ui.horizontal(|ui| {
                                egui::ComboBox::from_id_salt("Category combo")
                                    .selected_text(&self.selected_upload_category.title)
                                    .show_ui(ui, |ui| {
                                        // Add items to the ComboBox
                                        for option in &self.server_content_summary.categories {
                                            let response = ui.selectable_value(
                                                &mut self.selected_upload_category,
                                                option.clone(),
                                                option.title.as_str(),
                                            );
                                            if response.changed() {
                                                self.meta_data.category =
                                                    self.selected_upload_category.id;
                                                regenerate = true;
                                            }
                                        }
                                    });
                                let response = ui.button("Add");
                                if response.clicked() {
                                    self.project_popup.show_project_popup = true;
                                }
                            });

                            if self.project_popup.show_project_popup {
                                egui::Window::new("Add project category")
                                    .collapsible(false)
                                    .resizable(false)
                                    .show(ctx, |ui| {
                                        ui.label("Title");
                                        ui.add(egui::TextEdit::singleline(
                                            &mut self.project_popup.title,
                                        ));
                                        ui.label("Description");
                                        ui.add(egui::TextEdit::singleline(
                                            &mut self.project_popup.description,
                                        ));

                                        if ui.button("Add & close").clicked() {
                                            self.add_project_category();
                                            self.get_server_content_summary();
                                            self.project_popup.show_project_popup = false;
                                        }

                                        if ui.button("Close").clicked() {
                                            self.project_popup.show_project_popup = false;
                                        }
                                    });
                            }
                        }

                        let mut index_removal: Vec<usize> = Vec::new();
                        for (i, tag) in self.meta_data.tags.iter().enumerate() {
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
                                self.meta_data.tags.remove(*index);
                            }
                            regenerate = true
                        }

                        if regenerate == true {
                            self.regenereate_meta_data();
                        }
                    });
                });
                //Markdown panel
                egui::ScrollArea::vertical()
                    .id_salt("source")
                    .show(ui, |ui| {
                        ui.add_sized(
                            [width * 0.4, height],
                            egui::TextEdit::multiline(&mut self.markdown),
                        );
                    });
                //Preview panel
                egui::ScrollArea::vertical()
                    .id_salt("preview")
                    .show(ui, |ui| {
                        ui.with_layout(egui::Layout::top_down(eframe::emath::Align::Min), |ui| {
                            CommonMarkViewer::new().show(ui, &mut self.cache, &self.markdown);
                        });
                    });
            });
        });
    }

    fn on_exit(&mut self, _: Option<&eframe::glow::Context>) {
        data::save_server_settings(&self.server_settings).expect("Failed to save state");
    }
}

impl AdminPanel {
    fn regenereate_meta_data(&mut self) {
        shared::store_meta_data(&mut self.markdown, self.meta_data.clone());
    }

    fn load_meta_data(&mut self) {
        self.meta_data = shared::load_meta_data(&self.markdown).0;

        self.regenereate_meta_data();
    }

    fn check_for_macros(&mut self) {
        if self.prev_markdown != self.markdown {
            let meta_data = self.markdown.find("@@META");
            if meta_data.is_some() {
                self.load_meta_data();
            }

            let image = self.markdown.find("@@IMAGE");
            if image.is_some() && self.file_load_state.is_none() {
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
                            self.opened_markdown_file = Some(file.to_path_buf())
                                .unwrap()
                                .to_str()
                                .unwrap()
                                .to_string();
                            self.markdown =
                                fs::read_to_string(self.opened_markdown_file.clone()).unwrap();

                            self.load_meta_data()
                        }
                        FileLoadState::LoadImage => {
                            let escaped_path = Some(file.to_path_buf())
                                .unwrap()
                                .to_str()
                                .unwrap()
                                .replace("\\", "\\\\");
                            let image_md = format!("![@IMAGE]({})", escaped_path);
                            self.markdown = self.markdown.replacen("@@IMAGE", &image_md, 1);
                        }
                    }

                    self.file_load_state = None;
                    self.open_file_dialog = None;
                }
            }
        }
    }

    fn get_server_content_summary(&mut self) {
        self.server_content_summary =
            webconnector::load_content_summary(self.server_settings.address.as_str());
    }

    fn download_markdown(&mut self) {
        let mut id = uuid::Uuid::nil();
        if self.selected_download_post.title.is_empty() == false {
            id = self.selected_download_post.id
        } else if self.selected_download_project.title.is_empty() == false {
            id = self.selected_download_project.id
        }
        self.markdown = webconnector::get_markdown(self.server_settings.address.as_str(), id);

        self.load_meta_data()
    }

    fn add_project_category(&mut self) {
        let _ = webconnector::upload_project_category(
            self.project_popup.title.as_str(),
            self.project_popup.description.as_str(),
            &self.server_settings.address,
            &self.server_settings.api_token,
        );
    }
}

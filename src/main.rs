#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![expect(rustdoc::missing_crate_level_docs)] // it's an example

use std::path::PathBuf;

use eframe::egui;

mod modals;
mod tabs;

use tabs::*;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 360.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Prospekt",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<Prospekt>::default())
        }),
    )
}

#[derive(Default)]
struct Modals {
    about: bool,
}

struct Prospekt {
    dock_state: egui_dock::DockState<Tab>,
    file_dialog: egui_file_dialog::FileDialog,
    modals: Modals,
}

impl Default for Prospekt {
    fn default() -> Self {
        Self {
            dock_state: egui_dock::DockState::new(vec![]),
            file_dialog: egui_file_dialog::FileDialog::new(),
            modals: Modals::default(),
        }
    }
}

impl eframe::App for Prospekt {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            egui::MenuBar::default().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open...").clicked() {
                        self.file_dialog.pick_file();
                    }

                    if ui.button("Quit").clicked() {
                        ui.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.menu_button("Help", |ui| {
                    if ui.button("Report a Bug").clicked() {
                        ui.ctx().open_url(egui::OpenUrl::new_tab(
                            "https://github.com/mikesk8r/prospekt/issues",
                        ));
                    }

                    if ui.button("About...").clicked() {
                        self.modals.about = true;
                    }
                })
            });

            modals::draw(&mut self.modals, ui);

            self.file_dialog.update(ui);
            if let Some(path) = self.file_dialog.take_picked() {
                let file = std::fs::read(&path);
                let string = format!("{}", &path.display());

                if string.ends_with(".vtf") {
                    let vtf = headcrab_vtf::VTF::from_bytes(file.unwrap().as_slice());

                    let mut i: usize = 0;
                    let mut buffer: Vec<egui::Color32> = vec![];
                    // while i < vtf.thumbnail.len() - 2 {
                    // TODO: add support for looking at mipmaps/frames
                    // also note to self: fix weird index bug :p
                    while i < vtf.texture.data[1][1].len() {
                        buffer.push(egui::Color32::from_rgb(
                            vtf.texture.data[1][1][i] as u8,
                            vtf.texture.data[1][1][i + 1] as u8,
                            vtf.texture.data[1][1][i + 2] as u8,
                        ));
                        i += 4;
                    }

                    let texture = ui.ctx().load_texture(
                        string.as_str(),
                        egui::ColorImage {
                            size: [vtf.width as usize, vtf.height as usize],
                            source_size: egui::vec2(vtf.width.into(), vtf.height.into()),
                            pixels: buffer,
                        },
                        egui::TextureOptions {
                            magnification: egui::TextureFilter::Nearest,
                            minification: egui::TextureFilter::Nearest,
                            wrap_mode: egui::TextureWrapMode::Repeat,
                            mipmap_mode: None,
                        },
                    );

                    self.dock_state.push_to_focused_leaf(Tab {
                        id: self.dock_state.surfaces_count() as u16,
                        vtf: Some(vtf),
                        texture: Some(texture),
                        view_zoom: 1.0,
                        filename: format!("{}", &path.file_name().unwrap().display()),
                    });
                }
            }

            egui_dock::DockArea::new(&mut self.dock_state)
                .show_leaf_collapse_buttons(false)
                .style(egui_dock::Style::from_egui(ui.style().as_ref()))
                .show_inside(ui, &mut MainTabViewer);

            if self.dock_state.main_surface().num_tabs() == 0 {
                ui.centered_and_justified(|ui| {
                    ui.heading("No files open");
                });
            }
        });
    }
}

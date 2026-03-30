#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![expect(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;

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

struct Prospekt {
    dock_state: egui_dock::DockState<Tab>,
}

impl Default for Prospekt {
    fn default() -> Self {
        Self {
            dock_state: egui_dock::DockState::new(vec![]),
        }
    }
}

impl eframe::App for Prospekt {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            egui::MenuBar::default().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open...").clicked() {
                        self.dock_state.push_to_focused_leaf(Tab {
                            id: self.dock_state.surfaces_count() as u16,
                        });
                    }

                    if ui.button("Quit").clicked() {
                        ui.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });

            egui_dock::DockArea::new(&mut self.dock_state)
                .show_leaf_collapse_buttons(false)
                .style(egui_dock::Style::from_egui(ui.style().as_ref()))
                .show_inside(ui, &mut MainTabViewer);
        });
    }
}

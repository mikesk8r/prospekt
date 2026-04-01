#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![expect(rustdoc::missing_crate_level_docs)] // it's an example

use discord_rich_presence::DiscordIpc;
use eframe::egui;

mod modals;
mod presence;
mod settings;
mod tabs;

use settings::Settings;
use tabs::*;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 360.0]),
        ..Default::default()
    };
    let mut rpc = discord_rich_presence::DiscordIpcClient::new("1488668810906046515");

    let args = std::env::args().collect::<Vec<String>>();
    #[cfg(windows)]
    let settings_path = format!(
        "{}\\prospekt.toml",
        std::path::Path::new(&args[0])
            .canonicalize()
            .unwrap()
            .parent()
            .unwrap()
            .as_os_str()
            .to_str()
            .unwrap()
    );
    #[cfg(not(windows))]
    let settings_path = format!(
        "{}/prospekt.toml",
        std::path::Path::new(&args[0])
            .canonicalize()
            .parent()
            .unwrap()
            .unwrap()
            .as_os_str()
            .to_str()
            .unwrap()
    );

    let mut prefs = match std::fs::read_to_string(&settings_path) {
        Ok(str) => toml::from_str(str.as_str()).expect("invalid settings"),
        Err(_) => Settings::default(),
    };

    let _ = rpc.connect();
    let result = eframe::run_native(
        "Prospekt",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            let mut application = Prospekt::default();
            application.settings = Some(&mut prefs);

            if application.settings.as_ref().clone().unwrap().rpc != settings::RPCSetting::None {
                let activity = discord_rich_presence::activity::Activity::new()
                    .buttons(vec![discord_rich_presence::activity::Button::new(
                        "GitHub",
                        "https://github.com/mikesk8r/prospekt",
                    )])
                    .state("No files open");
                let _ = rpc.set_activity(activity);
            }

            application.rpc = Some(&mut rpc);

            Ok(Box::from(application))
        }),
    );
    let _ = rpc.clear_activity();
    let _ = rpc.close();
    let _ = std::fs::write(
        settings_path,
        toml::to_string_pretty(&prefs).unwrap().as_bytes(),
    );
    result
}

#[derive(Default)]
struct Modals {
    about: bool,
    controls: bool,
    settings: bool,
}

struct Prospekt<'a> {
    dock_state: egui_dock::DockState<Tab>,
    file_dialog: egui_file_dialog::FileDialog,
    pub rpc: Option<&'a mut discord_rich_presence::DiscordIpcClient>,
    modals: Modals,
    pub settings: Option<&'a mut Settings>,
}

impl<'a> Default for Prospekt<'a> {
    fn default() -> Self {
        Self {
            dock_state: egui_dock::DockState::new(vec![]),
            file_dialog: egui_file_dialog::FileDialog::new(),
            modals: Modals::default(),
            rpc: None,
            settings: None,
        }
    }
}

impl<'a> eframe::App for Prospekt<'a> {
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
                ui.menu_button("Edit", |ui| {
                    if ui.button("Settings...").clicked() {
                        self.modals.settings = true;
                    }
                });
                ui.menu_button("Help", |ui| {
                    if ui.button("Report a Bug").clicked() {
                        ui.ctx().open_url(egui::OpenUrl::new_tab(
                            "https://github.com/mikesk8r/prospekt/issues",
                        ));
                    }

                    if ui.button("Controls...").clicked() {
                        self.modals.controls = true;
                    }

                    if ui.button("About...").clicked() {
                        self.modals.about = true;
                    }
                })
            });

            let unwrapped_settings = self.settings.as_mut().unwrap().clone();
            let previous_rpc = unwrapped_settings.rpc.clone();
            if let Some(mut settings) = self.settings.as_mut() {
                modals::draw(&mut self.modals, &mut settings, ui);
            }

            self.file_dialog.update(ui);
            if let Some(path) = self.file_dialog.take_picked() {
                let file = std::fs::read(&path);
                let string = format!("{}", &path.display());

                if string.ends_with(".vtf") {
                    let vtf = headcrab_vtf::VTF::from_bytes(file.unwrap().as_slice());

                    let mut i: usize = 0;
                    let mut buffer: Vec<egui::Color32> = vec![];
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

                    i = 0;
                    buffer = vec![];
                    while i < vtf.thumbnail.len() - 2 {
                        buffer.push(egui::Color32::from_rgb(
                            vtf.thumbnail[i] as u8,
                            vtf.thumbnail[i + 1] as u8,
                            vtf.thumbnail[i + 2] as u8,
                        ));
                        i += 4;
                    }

                    let thumbnail = ui.ctx().load_texture(
                        string.as_str(),
                        egui::ColorImage {
                            size: [vtf.thumbnail_width as usize, vtf.thumbnail_height as usize],
                            source_size: egui::vec2(
                                vtf.thumbnail_width.into(),
                                vtf.thumbnail_height.into(),
                            ),
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
                        // id: self.dock_state.surfaces_count() as u16,
                        vtf: Some(vtf),
                        texture: Some(texture),
                        thumbnail: Some(thumbnail),
                        view_zoom: 1.0,
                        filename: format!("{}", &path.file_name().unwrap().display()),
                    });
                }
            }

            if self.dock_state.main_surface().num_tabs() == 0 {
                ui.centered_and_justified(|ui| {
                    ui.heading("No files open");
                });
            }

            let mut tab_viewer = MainTabViewer { focused_tab: None };
            egui_dock::DockArea::new(&mut self.dock_state)
                .show_leaf_collapse_buttons(false)
                .style(egui_dock::Style::from_egui(ui.style().as_ref()))
                .show_inside(ui, &mut tab_viewer);

            if let Some(rpc) = &mut self.rpc {
                let num_tabs = self.dock_state.main_surface().num_tabs();

                if let Some(tab) = &tab_viewer.focused_tab
                    && num_tabs > 0
                {
                    if unwrapped_settings.rpc == settings::RPCSetting::Full {
                        let _ = rpc.set_activity(presence::status(&format!("Editing {}", &tab)));
                    } else if unwrapped_settings.rpc == settings::RPCSetting::HideFilename {
                        let _ = rpc.set_activity(presence::status(&"Editing a file".to_string()));
                    }
                }
                if num_tabs == 0 && unwrapped_settings.rpc != settings::RPCSetting::None {
                    let _ = rpc.set_activity(presence::status(&"No files open".to_string()));
                }

                if unwrapped_settings.rpc != previous_rpc {
                    match unwrapped_settings.rpc {
                        settings::RPCSetting::None => {
                            let _ = rpc.clear_activity();
                        }
                        settings::RPCSetting::HideFilename => {
                            let editing = "Editing a file".to_string();
                            let none = "No files open".to_string();

                            let _ = rpc.set_activity(presence::status(
                                match self.dock_state.main_surface().num_tabs() {
                                    0 => &none,
                                    _ => &editing,
                                },
                            ));
                        }
                        settings::RPCSetting::Full => {
                            let editing =
                                format!("Editing {}", &tab_viewer.focused_tab.clone().unwrap());
                            let none = "No files open".to_string();

                            let _ = rpc.set_activity(presence::status(
                                match self.dock_state.main_surface().num_tabs() {
                                    0 => &none,
                                    _ => &editing,
                                },
                            ));
                        }
                    }
                }
            }
        });
    }
}

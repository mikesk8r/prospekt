use eframe::egui::{self, Ui};
use egui_dock::TabViewer;
use headcrab_vtf::{VTF, VTFFlags};

/// Views a tab can have
enum View {
    VTF,
    // this will eventually include vpks, mdls, etc.
}

#[derive(Default)]
pub struct Tab {
    pub id: u16,
    pub filename: String,
    pub vtf: Option<VTF<u16>>,
    pub texture: Option<egui::TextureHandle>,
    pub view_zoom: f32,
}

pub struct MainTabViewer;

impl TabViewer for MainTabViewer {
    // This associated type is used to attach some data to each tab.
    type Tab = Tab;

    // Returns the current `tab`'s title.
    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        (&tab.filename).into()
    }

    // Defines the contents of a given `tab`.
    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        if let Some(vtf) = &tab.vtf {
            ui.ctx().input(|input| {
                // TODO
            });
            egui::Panel::left("VTF properties").show_inside(ui, |ui| {
                ui.label(format!("Version: {}.{}", vtf.version.0, vtf.version.1));
                ui.label(format!("Dimensions: {} px by {} px", vtf.width, vtf.height));
                ui.label(format!("Format: {:?}", vtf.texture_format));
                ui.collapsing("Flags", |ui| {
                    // should probably move flags to actual bools so this can work :p
                    ui.label("TODO");
                });
            });
            let texture = tab.texture.as_mut().unwrap();
            egui::CentralPanel::default().show_inside(ui, |ui| {
                let size = texture.size_vec2();
                let sized_texture = egui::load::SizedTexture::new(texture.id(), size);
                ui.centered_and_justified(|ui| {
                    ui.add(egui::Image::new(sized_texture).fit_to_exact_size(size * tab.view_zoom));
                })
            });
        }
    }
}

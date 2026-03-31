use eframe::egui::{self, Modifiers, Ui, Vec2};
use egui_dock::TabViewer;
use headcrab_vtf::VTF;

pub struct Tab {
    pub id: u16,
    pub filename: String,
    pub vtf: Option<VTF<u16>>,
    pub texture: Option<egui::TextureHandle>,
    pub thumbnail: Option<egui::TextureHandle>,
    pub view_zoom: f32,
}

pub struct MainTabViewer;

impl TabViewer for MainTabViewer {
    // This associated type is used to attach some data to each tab.
    type Tab = Tab;

    fn scroll_bars(&self, _tab: &Self::Tab) -> [bool; 2] {
        [false, false]
    }

    // Returns the current `tab`'s title.
    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        (&tab.filename).into()
    }

    // Defines the contents of a given `tab`.
    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        if let Some(vtf) = &mut tab.vtf {
            egui::Panel::left("VTF properties").show_inside(ui, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.label(format!("Version: {}.{}", vtf.version.0, vtf.version.1));
                    ui.label(format!("Dimensions: {} px by {} px", vtf.width, vtf.height));
                    ui.label(format!("Format: {:?}", vtf.texture_format));
                    ui.collapsing("Thumbnail", |ui| {
                        ui.horizontal(|ui| {
                            let texture = tab.thumbnail.as_mut().unwrap();
                            let size = texture.size_vec2();
                            let sized_texture = egui::load::SizedTexture::new(texture.id(), size);
                            ui.add(egui::Image::new(sized_texture).fit_to_exact_size(size * 2.0));
                            ui.vertical(|ui| {
                                ui.label(format!("Width: {} px", vtf.thumbnail_width));
                                ui.label(format!("Height: {} px", vtf.thumbnail_height));
                            })
                        });
                    });
                    ui.collapsing("Flags", |ui| {
                        ui.checkbox(&mut vtf.flags.all_mips, "All Mipmaps");
                        ui.checkbox(&mut vtf.flags.anisotropic, "Anisotropic");
                        ui.checkbox(&mut vtf.flags.border, "Border");
                        ui.checkbox(&mut vtf.flags.cubemap, "Cubemap (Envmap)");
                        ui.checkbox(&mut vtf.flags.clamp_s, "Clamp S coordinates");
                        ui.checkbox(&mut vtf.flags.clamp_t, "Clamp T coordinates");
                        ui.checkbox(&mut vtf.flags.clamp_u, "Clamp U coordinates");
                        ui.checkbox(&mut vtf.flags.depth_render_target, "Depth Render Target");
                        ui.add_enabled(
                            false,
                            egui::Checkbox::new(&mut vtf.flags.eight_bit_alpha, "Eight Bit Alpha"),
                        );
                        ui.checkbox(&mut vtf.flags.no_debug_override, "No Debug Override");
                        ui.checkbox(&mut vtf.flags.no_depth_buffer, "No Depth Buffer");
                        ui.checkbox(&mut vtf.flags.no_mips, "No Mipmaps");
                        ui.checkbox(&mut vtf.flags.no_lod, "No LOD");
                        ui.checkbox(&mut vtf.flags.normal_map, "Normal Map");
                        ui.add_enabled(
                            false,
                            egui::Checkbox::new(&mut vtf.flags.one_bit_alpha, "One Bit Alpha"),
                        );
                        ui.checkbox(&mut vtf.flags.pointsample, "Pointsampling");
                        ui.checkbox(&mut vtf.flags.pre_srgb, "Pre-SRGB");
                        ui.checkbox(&mut vtf.flags.procedural, "Procedural");
                        ui.checkbox(&mut vtf.flags.pwl_corrected, "PWL-corrected");
                        ui.checkbox(&mut vtf.flags.single_copy, "Single Copy");
                        ui.checkbox(&mut vtf.flags.ssbump, "SSBump");
                        ui.checkbox(&mut vtf.flags.trilinear, "Trilinear");
                        ui.checkbox(&mut vtf.flags.render_target, "Render Target");
                        ui.checkbox(&mut vtf.flags.vertex_texture, "Vertex Texture");
                    });
                });
            });

            let texture = tab.texture.as_mut().unwrap();
            let response = egui::ScrollArea::both()
                .content_margin(egui::Margin {
                    left: 127,
                    right: 127,
                    top: 63,
                    bottom: 63,
                })
                .show(ui, |ui| {
                    let size = texture.size_vec2();
                    let sized_texture = egui::load::SizedTexture::new(texture.id(), size);
                    ui.centered_and_justified(|ui| {
                        ui.add(
                            egui::Image::new(sized_texture).fit_to_exact_size(size * tab.view_zoom),
                        );
                    })
                })
                .inner
                .response;

            ui.ctx().input(|input| {
                if !response.hovered() {
                    return;
                }

                let events = &input.events;
                for event in events {
                    if let egui::Event::MouseWheel {
                        unit: _,
                        delta,
                        phase: _,
                        modifiers,
                    } = event
                    {
                        if modifiers.command {
                            if (tab.view_zoom <= 10.0 || delta.y <= 0.0)
                                && (tab.view_zoom >= 0.5 || delta.y >= 0.0)
                            {
                                tab.view_zoom += delta.y * 0.5;
                            }
                        }
                    }
                }
            });
        }
    }
}

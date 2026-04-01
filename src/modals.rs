use eframe::egui;

pub fn draw(
    modals: &mut super::Modals,
    settings: &mut super::settings::Settings,
    ui: &mut egui::Ui,
) {
    if modals.about {
        about_modal(modals, ui);
    }

    if modals.controls {
        controls_modal(modals, ui);
    }

    if modals.settings {
        settings_modal(modals, settings, ui);
    }
}

fn about_modal(modals: &mut super::Modals, ui: &mut egui::Ui) {
    egui::Modal::new("About".into()).show(ui, |ui| {
        ui.heading("About");
        ui.label("Prospekt v0.1.0");
        ui.add_space(15.0);
        if ui.button("OK").clicked() {
            modals.about = false;
        }
    });
}

fn controls_modal(modals: &mut super::Modals, ui: &mut egui::Ui) {
    egui::Modal::new("Controls".into()).show(ui, |ui| {
        egui::Grid::new("some_unique_id").show(ui, |ui| {
            ui.heading("Key");
            ui.heading("Action");
            ui.end_row();

            ui.label("Mouse Wheel");
            ui.label("Scroll Vertically");
            ui.end_row();

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("SHIFT").monospace());
                ui.label("+ Scroll Wheel");
            });
            ui.label("Scroll Horizontally");
            ui.end_row();

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("CTRL").monospace());
                ui.label("+ Scroll Wheel");
            });
            ui.label("Zoom");
            ui.end_row()
        });
        // ui.add_space(15.0);
        if ui.button("OK").clicked() {
            modals.controls = false;
        }
    });
}

fn settings_modal(
    modals: &mut super::Modals,
    settings: &mut super::settings::Settings,
    ui: &mut egui::Ui,
) {
    egui::Modal::new("Settings".into()).show(ui, |ui| {
        ui.heading("Settings");
        egui::ComboBox::from_label("Discord RPC Mode")
            .selected_text(format!("{:?}", settings.rpc))
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut settings.rpc,
                    super::settings::RPCSetting::None,
                    format!("{:?}", super::settings::RPCSetting::None),
                );
                ui.selectable_value(
                    &mut settings.rpc,
                    super::settings::RPCSetting::HideFilename,
                    format!("{:?}", super::settings::RPCSetting::HideFilename),
                );
                ui.selectable_value(
                    &mut settings.rpc,
                    super::settings::RPCSetting::Full,
                    format!("{:?}", super::settings::RPCSetting::Full),
                );
            });
        ui.add_space(15.0);
        if ui.button("Close").clicked() {
            modals.settings = false;
        }
    });
}

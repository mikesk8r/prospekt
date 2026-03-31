use eframe::egui;

pub fn draw(modals: &mut super::Modals, ui: &mut egui::Ui) {
    if modals.about {
        about_modal(modals, ui);
    }

    if modals.controls {
        controls_modal(modals, ui);
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

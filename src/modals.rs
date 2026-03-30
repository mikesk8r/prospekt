use eframe::egui;

pub fn draw(modals: &mut super::Modals, ui: &mut egui::Ui) {
    if modals.about {
        about_modal(modals, ui);
    }
}

fn about_modal(modals: &mut super::Modals, ui: &mut egui::Ui) {
    egui::Modal::new("About".into()).show(ui, |ui| {
        ui.heading("About");
        ui.label("Prospekt v0.1.0");
        ui.add_space(20.0);
        if ui.button("OK").clicked() {
            modals.about = false;
        }
    });
}

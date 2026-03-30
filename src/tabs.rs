use eframe::egui::{self, Ui};
use egui_dock::TabViewer;

/// Views a tab can have
enum View {
    VTF,
    // this will eventually include vpks, mdls, etc.
}

#[derive(Default)]
pub struct Tab {
    pub id: u16,
}

pub struct MainTabViewer;

impl TabViewer for MainTabViewer {
    // This associated type is used to attach some data to each tab.
    type Tab = Tab;

    // Returns the current `tab`'s title.
    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        format!("tab {}", tab.id).into()
    }

    // Defines the contents of a given `tab`.
    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        ui.label(format!("Content of {}", tab.id));
    }
}

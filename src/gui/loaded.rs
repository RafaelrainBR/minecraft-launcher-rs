use iced::{
    widget::{column, scrollable, text},
    Element,
    Length::Fill,
    Task,
};

use crate::launcher::Launcher;

use super::{App, Screen};

#[derive(Debug)]
pub struct LoadedVersionsState {
    pub launcher: Launcher,
    pub versions: Vec<String>,
    pub filter: String,
    pub is_loading: bool,
}

#[derive(Debug, Clone)]
pub enum LoadedVersionsMessage {
    FilterChanged(String),
    LaunchVersion(String),
    None,
}

impl LoadedVersionsState {
    pub fn new(launcher: Launcher, versions: Vec<String>) -> Self {
        return Self {
            launcher,
            versions,
            filter: "".to_string(),
            is_loading: false,
        };
    }

    pub fn title(&self) -> String {
        "Minecraft Launcher - Versions".to_string()
    }

    pub fn update(&mut self, message: LoadedVersionsMessage) -> Task<LoadedVersionsMessage> {
        match message {
            LoadedVersionsMessage::FilterChanged(str) => self.filter = str,
            LoadedVersionsMessage::LaunchVersion(version_id) => {
                self.is_loading = true;

                // self.launcher.select_version(version_id.clone());
            }
            LoadedVersionsMessage::None => {}
        }

        Task::none()
    }

    pub fn view(&self) -> iced::Element<LoadedVersionsMessage> {
        let filter = self.filter.to_lowercase();

        let mut versions = if filter.is_empty() {
            self.versions.clone()
        } else {
            self.versions
                .iter()
                .filter(|id| id.to_lowercase().contains(&filter))
                .cloned()
                .collect()
        };

        let buttons: Vec<Element<LoadedVersionsMessage>> = versions
            .into_iter()
            .map(|id| {
                iced::widget::button(text(id.clone()))
                    .on_press(LoadedVersionsMessage::LaunchVersion(id))
                    .into()
            })
            .collect();

        let scrollable_list = scrollable(column(buttons)).spacing(12).width(Fill);

        column![scrollable_list].height(Fill).width(Fill).into()
    }
}

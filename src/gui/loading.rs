use iced::{
    widget::{column, text},
    Length::Fill,
    Task,
};

use crate::{launcher::Launcher, model::VersionType};

use super::{loaded::LoadedVersionsState, App};

#[derive(Debug)]
pub struct LoadingVersionsState {
    pub error: Option<crate::Error>,
}

#[derive(Debug)]
pub enum LoadingVersionsMessage {
    Error(crate::Error),
    ChangeState(App),
}

impl LoadingVersionsState {
    pub fn new(launcher: Launcher) -> (Self, Task<LoadingVersionsMessage>) {
        (
            Self { error: None },
            Task::future(LoadingVersionsState::fetch_versions(launcher)),
        )
    }

    pub fn title(&self) -> String {
        "Minecraft Launcher - Loading versions...".to_string()
    }

    pub fn update(&mut self, message: LoadingVersionsMessage) {
        match message {
            LoadingVersionsMessage::Error(err) => self.error = Some(err),
            _ => {}
        }
    }

    pub fn view(&self) -> iced::Element<LoadingVersionsMessage> {
        if let Some(err) = &self.error {
            let error_text = text!("Erro: {}", err).center();
            return column![error_text].width(Fill).height(Fill).into();
        };

        let loading_text = text!("Carregando").center();
        column![loading_text].width(Fill).height(Fill).into()
    }

    async fn fetch_versions(launcher: Launcher) -> LoadingVersionsMessage {
        let versions = match launcher.list_versions(Some(VersionType::Release)) {
            Ok(versions) => versions,
            Err(err) => return LoadingVersionsMessage::Error(err),
        };

        let mapped_versions: Vec<String> = versions.iter().map(|ver| ver.id.clone()).collect();
        LoadingVersionsMessage::ChangeState(App::LoadedVersions(LoadedVersionsState::new(
            launcher,
            mapped_versions,
        )))
    }
}

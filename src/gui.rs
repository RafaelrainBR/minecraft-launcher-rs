use std::fmt::Debug;

use iced::{
    widget::{button, column, text, Button, Column, Text},
    Element,
};
use loaded::{LoadedVersionsMessage, LoadedVersionsState};
use loading::{LoadingVersionsMessage, LoadingVersionsState};

use crate::{launcher::Launcher, Result};

mod loaded;
mod loading;

pub trait Screen<T: Debug> {
    fn title(&self) -> String;

    fn update(&mut self, message: T);

    fn view(&self) -> Element<T>;
}

#[derive(Debug)]
pub enum App {
    LoadingVersions(LoadingVersionsState),
    LoadedVersions(LoadedVersionsState),
}

#[derive(Debug)]
pub enum Message {
    Loading(LoadingVersionsMessage),
    Loaded(LoadedVersionsMessage),
}

impl Screen<Message> for App {
    fn title(&self) -> String {
        match self {
            App::LoadingVersions(state) => state.title(),
            App::LoadedVersions(state) => state.title(),
        }
    }

    fn update(&mut self, message: Message) {
        match self {
            App::LoadingVersions(loading_state) => match message {
                Message::Loading(loading_message) => match loading_message {
                    LoadingVersionsMessage::ChangeState(app) => *self = app,
                    loading_message => loading_state.update(loading_message),
                },
                _ => eprintln!("Invalid message for loading state: {:?}", message),
            },
            App::LoadedVersions(loaded_state) => match message {
                Message::Loaded(loaded_message) => {
                    loaded_state.update(loaded_message);
                }
                _ => eprintln!("Invalid message for loaded state: {:?}", message),
            },
        }
    }

    fn view(&self) -> Element<Message> {
        match self {
            App::LoadingVersions(loading_state) => loading_state.view().map(Message::Loading),
            App::LoadedVersions(loaded_state) => loaded_state.view().map(Message::Loaded),
        }
    }
}

pub fn display_gui(launcher: Launcher) -> Result<()> {
    iced::application(App::title, App::update, App::view).run_with(|| {
        let (state, task) = LoadingVersionsState::new(launcher);
        (
            App::LoadingVersions(state),
            task.map(|message| Message::Loading(message)),
        )
    })?;

    Ok(())
}

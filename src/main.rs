use std::{env, fs};

pub use crate::error::Error;
use config::LauncherConfig;
use launcher::Launcher;
pub use launcher_paths::{LauncherPath, LauncherPaths};
use platform::{PlatformData, PlatformType};

mod config;
mod error;
mod files;
mod http_client;
mod launcher;
mod launcher_paths;
mod launcher_runner;
mod launcher_version;
mod model;
mod platform;

type Result<T> = std::result::Result<T, Error>;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();

    let version_id = args.get(1).cloned().expect("Informe uma versao");
    println!("Version ID: {:?}", version_id);

    let data_location = get_launcher_data_location();
    let launcher_paths = create_launcher_paths(data_location);

    let platform_data = load_platform_data();
    let mut launcher = start_launcher(platform_data, launcher_paths.clone()).await;

    let selected_version = launcher.select_version(version_id).unwrap();
    launcher.start_downloads().await.unwrap();

    println!("Selected version: {:?}", selected_version);

    launcher.launch_game().await.unwrap();

    launcher.persist_config().await.unwrap();
    ()
}

pub fn load_platform_data() -> PlatformData {
    let platform = PlatformData::new().unwrap();

    println!("Platform: {:?}", &platform);

    platform
}

pub fn get_launcher_data_location() -> String {
    let mut path = std::env::current_dir().unwrap();
    path.push("launcher_data");

    fs::create_dir_all(path.clone()).unwrap();

    path.as_path().to_str().unwrap().to_string()
}

fn create_launcher_paths(base_path: String) -> launcher_paths::LauncherPaths {
    launcher_paths::LauncherPaths::new(base_path)
}

async fn start_launcher(platform_data: PlatformData, launcher_paths: LauncherPaths) -> Launcher {
    let config = load_config(launcher_paths.clone()).await.unwrap();
    let mut launcher = Launcher::new(platform_data, launcher_paths, config);

    launcher.launcher_paths.create_folders().await.unwrap();

    launcher.load_version_manifest_or_download().await.unwrap();

    launcher
}

async fn load_config(launcher_paths: LauncherPaths) -> Result<LauncherConfig> {
    let config = LauncherConfig::load(launcher_paths)
        .await?
        .unwrap_or_default();
    Ok(config)
}

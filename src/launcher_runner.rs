use tokio::io::copy;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

use crate::{
    config::LauncherConfig,
    launcher_paths::join_paths,
    launcher_version::LauncherVersion,
    model::mojang::MojangVersion,
    platform::{PlatformData, PlatformType},
    LauncherPaths, Result,
};

pub async fn launch_game(
    launcher_paths: &LauncherPaths,
    platform_data: &PlatformData,
    launcher_version: &LauncherVersion,
    launcher_config: &LauncherConfig,
) -> Result<()> {
    let mut command_arguments = build_game_launch_command(
        launcher_paths,
        platform_data,
        launcher_version,
        launcher_config,
    )?;

    println!(
        "Launching game with command: {:?}",
        command_arguments.clone().join(" ")
    );

    let mut command_builder = {
        let mut builder = Command::new(command_arguments.remove(0));
        builder
            .args(&command_arguments)
            .stdin(std::process::Stdio::inherit())
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .current_dir(launcher_paths.get_path(crate::LauncherPath::GameDir))
            .stderr(std::process::Stdio::piped());
        builder
    };

    let mut child = command_builder.spawn()?;

    println!("Game launched with PID: {:?}", child.id());

    child.wait().await?;

    Ok(())
}
pub fn build_game_launch_command(
    launcher_paths: &LauncherPaths,
    platform_data: &PlatformData,
    launcher_version: &LauncherVersion,
    launcher_config: &LauncherConfig,
) -> Result<Vec<String>> {
    let mut command = Vec::new();

    let mojang_version = launcher_version
        .mojang_version
        .as_ref()
        .ok_or(crate::Error::VersionNotSelectedError)?;

    let runtime_name = mojang_version.java_version.component.clone();
    let runtime_path = launcher_paths.build_runtime_path(&runtime_name);

    command.push(get_java_executable_path(&platform_data, runtime_path));

    let jvm_arguments = mojang_version
        .arguments
        .clone()
        .map(|arguments| arguments.select_arguments(&platform_data));
    if let Some((_, jvm_arguments)) = jvm_arguments {
        command.extend(jvm_arguments);
    }

    command.push(
        "-Djava.library.path=".to_string()
            + launcher_paths
                .build_natives_dir_path(&launcher_version.manifest_version.id)
                .as_str(),
    );

    command.push("-cp".to_string());
    command.push(build_java_class_path(
        &launcher_paths,
        &platform_data,
        &launcher_version,
    ));

    command.push(mojang_version.main_class.clone());

    command.extend(build_minecraft_arguments(
        &launcher_paths,
        &platform_data,
        &mojang_version,
        &launcher_config,
    ));

    Ok(command)
}

fn get_java_executable_path(platform_data: &PlatformData, runtime_base_path: String) -> String {
    let java_executable_name = match platform_data.platform_type {
        PlatformType::Windows => "java.exe",
        _ => "java",
    };

    join_paths(runtime_base_path, vec!["bin", java_executable_name])
}

fn build_java_class_path(
    launcher_paths: &LauncherPaths,
    platform_data: &PlatformData,
    launcher_version: &LauncherVersion,
) -> String {
    let mut paths: Vec<String> = Vec::new();

    let client_file_path =
        launcher_paths.build_client_file_path(&launcher_version.manifest_version.id);
    paths.push(client_file_path);

    let libraries = launcher_version.libraries.as_ref().unwrap();
    libraries
        .iter()
        .filter(|library| !library.is_native())
        .map(|library| launcher_paths.build_library_path(&library.get_path()))
        .for_each(|path| paths.push(path));

    let separator = match platform_data.platform_type {
        PlatformType::Windows => ";",
        _ => ":",
    };

    paths.join(separator)
}

fn build_minecraft_arguments(
    launcher_paths: &LauncherPaths,
    platform_data: &PlatformData,
    mojang_version: &MojangVersion,
    launcher_config: &LauncherConfig,
) -> Vec<String> {
    // get from mojang_version.minecraft_arguments or mojang_version.arguments. Both are Option. Select the one that is Some
    let arguments = mojang_version
        .arguments
        .as_ref()
        .map(|arguments| arguments.select_arguments(&platform_data).0)
        .unwrap_or_else(|| {
            mojang_version
                .minecraft_arguments
                .as_ref()
                .map(|arguments| arguments.split(" ").map(|s| s.to_string()).collect())
                .unwrap_or_default()
        });

    /*
    for each argument, try to replace, like i did in kotlin:
            var replacedArguments =
        versionData.minecraftArguments
            .replace("\${auth_player_name}", username)
            .replace("\${version_name}", versionData.versionId)
            .replace("\${game_directory}", gameFolders.gameDir)
            .replace("\${assets_root}", gameFolders.assetsDir)
            .replace("\${assets_index_name}", versionData.assetIndexId)
            .replace("\${auth_uuid}", "abcde")
            .replace("\${auth_access_token}", "token")
            .replace("\${user_type}", "msa")
            .replace("\${user_properties}", "{}")
     */
    arguments
        .iter()
        .map(|argument| {
            argument
                .replace(
                    "${auth_player_name}",
                    launcher_config
                        .user_name
                        .as_ref()
                        .unwrap_or(&"".to_string()),
                )
                .replace("${version_name}", &mojang_version.id)
                .replace(
                    "${game_directory}",
                    launcher_paths
                        .get_path(crate::LauncherPath::GameDir)
                        .as_str(),
                )
                .replace(
                    "${assets_root}",
                    launcher_paths
                        .get_path(crate::LauncherPath::AssetsDir)
                        .as_str(),
                )
                .replace("${assets_index_name}", &mojang_version.asset_index.id)
                .replace("${auth_uuid}", "abcde")
                .replace("${auth_access_token}", "token")
                .replace("${user_type}", "msa")
                .replace("${user_properties}", "{}")
        })
        .collect()
}

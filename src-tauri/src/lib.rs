/*
    Copyright © 2025 俞妹妹宇宙唯一老公 (hippyandy@proton.me)
    Licensed under AGPL-3.0-or-later

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/agpl-3.0>.
*/

use std::process::Command;
use std::path::Path;
use std::env;


/*
Tauri Commands
*/

#[tauri::command]
fn install_tauri() {
    let args = ["-e", "bash", "-c", "sudo dnf update -y && sudo dnf install clang rustup rust cargo webkit2gtk4.1-devel openssl-devel curl wget file libappindicator-gtk3-devel librsvg2-devel -y && sudo dnf group install c-development -y && cargo install tauri-cli && cargo install create-tauri-app ; read -p '按回车键退出……'"];
    Command::new("alacritty").args(&args).spawn().unwrap();
}

#[tauri::command]
fn create_project(path_str: String) -> Option<String> {
    let mut path = Path::new(&path_str);

    if let Ok(path_exist) = path.try_exists() {
        if !path_exist {
            return Some(format!("路径'{}'不存在！", path_str));
        }
    } else {
        return Some(format!("无法访问路径'{}'，可能是没有足够的权限。", path_str));
    }

    if path.is_file() {
        path = path.parent().unwrap();
    }

    env::set_current_dir(path).unwrap();

    let args = ["-e", "bash", "-c", "cargo create-tauri-app"];
    Command::new("alacritty").args(&args).spawn().unwrap();

    None
}

#[tauri::command]
fn open_project(path: String) -> Option<String> {
    let mut path = Path::new(&path);

    if path.is_file() {
        path = path.parent().unwrap();
    }

    loop {
        let src_tauri_path = path.join("src-tauri");
        if src_tauri_path.exists() && src_tauri_path.is_dir() {
            env::set_current_dir(path).unwrap();
            return Some(path.to_str().unwrap().to_string());
        }
        path = if let Some(parent) = path.parent() {
            parent
        } else {
            break;
        }
    }

    None
}

#[tauri::command]
fn dev() {
    let args = ["-e", "bash", "-c", "cargo tauri dev ; read -p '按回车键退出……'"];
    Command::new("alacritty").args(&args).spawn().unwrap();
}

#[tauri::command]
fn build() {
    let args = ["-e", "bash", "-c", "cargo tauri build ; read -p '按回车键退出……'"];
    Command::new("alacritty").args(&args).spawn().unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![install_tauri, create_project, open_project, dev, build])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
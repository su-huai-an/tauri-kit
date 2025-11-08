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
use tauri::Manager;

/*
Consts & Statics
*/
const LANGUAGE: [&'static str;2] = ["Press Enter to exit...", "按回车键退出……"];
static mut LANGUAGE_INDEX: usize = 0;

const PATH_NOT_EXIST: u8 = 0;
const PATH_CANT_ACCESS: u8 = 1;

/*
Tauri Commands
*/
#[tauri::command]
fn change_language() {
    unsafe {LANGUAGE_INDEX = (LANGUAGE_INDEX+1)%2;}
}

#[tauri::command]
fn install_tauri() {
    let hold_str = unsafe{ LANGUAGE[LANGUAGE_INDEX] };
    let script_str = format!("sudo dnf update -y && sudo dnf install clang rustup webkit2gtk4.1-devel openssl-devel curl wget file libappindicator-gtk3-devel librsvg2-devel -y && sudo dnf group install c-development -y && rustup-init -y && . $HOME/.bashrc && cargo install tauri-cli && cargo install create-tauri-app ; read -p '{hold_str}'");
    let args = ["-e", "bash", "-c", &script_str];
    Command::new("alacritty")
        .args(&args)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

#[tauri::command]
fn create_project(path_str: String) -> Option<u8> {
    let mut path = Path::new(&path_str);

    if let Ok(path_exist) = path.try_exists() {
        if !path_exist {
            return Some(PATH_NOT_EXIST);
        }
    } else {
        return Some(PATH_CANT_ACCESS);
    }

    if path.is_file() {
        path = path.parent().unwrap();
    }

    env::set_current_dir(path).unwrap();

    let args = ["-e", "bash", "-c", "cargo create-tauri-app"];
    Command::new("alacritty")
        .args(&args)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

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
    let hold_str = unsafe{ LANGUAGE[LANGUAGE_INDEX] };
    let script_str = format!("cargo tauri dev ; read -p '{hold_str}'");
    let args = ["-e", "bash", "-c", &script_str];
    Command::new("alacritty")
        .args(&args)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

#[tauri::command]
fn build() {
    let hold_str = unsafe{ LANGUAGE[LANGUAGE_INDEX] };
    let script_str = format!("cargo tauri build ; read -p '{hold_str}'");
    let args = ["-e", "bash", "-c", &script_str];
    Command::new("alacritty")
        .args(&args)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

#[tauri::command]
fn info() {
    let hold_str = unsafe{ LANGUAGE[LANGUAGE_INDEX] };
    let script_str = format!("cargo tauri info ; read -p '{hold_str}'");
    let args = ["-e", "bash", "-c", &script_str];
    Command::new("alacritty")
        .args(&args)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![change_language, install_tauri, create_project, open_project, dev, build, info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
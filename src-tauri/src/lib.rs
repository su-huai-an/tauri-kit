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
use std::thread;
use std::path::Path;
use std::env;
use tauri::AppHandle;
use tauri::Emitter;

/**************************************************
 * Consts & Statics
**************************************************/
const LANGUAGE_STRS: [[&'static str; 2]; 2] = [
    [
        "Multiple sudo password prompts will appear during installation. Please stay nearby to prevent timeout issues.",
        "Press Enter to exit..."
    ],
    [
        "安装过程中需要多次输入sudo密码，请不要离开，以免安装因等待sudo密码超时而中断。",
        "按回车键退出……"
    ]
];
static mut LANGUAGE_INDEX: usize = 0;
const ERROR_PATH_NOT_EXIST: u8 = 0;
const ERROR_PATH_CANT_ACCESS: u8 = 0;


/**************************************************
 * Tauri Commands
**************************************************/
#[tauri::command]
fn change_language() {
  unsafe { LANGUAGE_INDEX = (LANGUAGE_INDEX + 1) % 2; }
}

#[tauri::command]
fn install_tauri(app: AppHandle) {
  let not_leave_str = unsafe{ LANGUAGE_STRS[LANGUAGE_INDEX][0] };
  let hold_str = unsafe{ LANGUAGE_STRS[LANGUAGE_INDEX][1] };
  let script_str = format!("echo '{not_leave_str}' ; sudo dnf update -y && sudo dnf install clang rustup webkit2gtk4.1-devel openssl-devel curl wget file libappindicator-gtk3-devel librsvg2-devel -y && sudo dnf group install c-development -y && rustup-init && . $HOME/.bashrc && cargo install tauri-cli && cargo install create-tauri-app ; read -p '{hold_str}'");
  thread::spawn(move || {
    Command::new("alacritty")
      .args(&["-e", "bash", "-c", &script_str])
      .spawn()
      .expect("install_tauri Command spawn fial")
      .wait()
      .expect("install_tauri Command wait fail");

    app.emit_to("main", "show-container", ())
      .expect("install_tauri show-container fail");
  });
}

#[tauri::command]
fn create_project(path_str: String) -> Option<u8> {
  let mut path = Path::new(&path_str);

  if let Ok(path_exist) = path.try_exists() {
    if !path_exist {
      return Some(ERROR_PATH_NOT_EXIST);
    }
  } else {
    return Some(ERROR_PATH_CANT_ACCESS);
  }

  if path.is_file() {
    path = path.parent().expect("create_project: path.parent fail");
  }

  env::set_current_dir(path).expect("create_project: set_current_dir fail");

  let hold_str = unsafe{ LANGUAGE_STRS[LANGUAGE_INDEX][1] };
  let script_str = format!("cargo create-tauri-app ; read -p '{hold_str}'");
  Command::new("alacritty")
    .args(&["-e", "bash", "-c", &script_str])
    .spawn()
    .expect("create_project Command spawn fail")
    .wait()
    .expect("create_project Command wait fail");

  None
}

#[tauri::command]
fn open_project(path_str: String) -> Option<String> {
  let mut path = Path::new(&path_str);

  if path.is_file() {
    path = path.parent().expect("open_project: path.parent fail");
  }

  loop {
    let src_tauri_path = path.join("src-tauri");
    if src_tauri_path.exists() && src_tauri_path.is_dir() {
      env::set_current_dir(path).expect("open_project: set_current_dir fail");
      return Some(path.to_str().expect("open_project: path.to_str fail").to_string());
    }

    path = if let Some(parent) = path.parent() {
      parent
    } else {
      return None;
    }
  }//loop
}

#[tauri::command]
fn dev(app: AppHandle) {
  let hold_str = unsafe{ LANGUAGE_STRS[LANGUAGE_INDEX][1] };
  let script_str = format!("cargo tauri dev ; read -p '{hold_str}'");
  thread::spawn(move || {
    Command::new("alacritty")
      .args(&["-e", "bash", "-c", &script_str])
      .spawn()
      .expect("dev Command spawn fial")
      .wait()
      .expect("dev Command wait fail");

    app.emit_to("main", "show-container", ())
      .expect("dev show-container fail");
  });
}

#[tauri::command]
fn build(app: AppHandle) {
  let hold_str = unsafe{ LANGUAGE_STRS[LANGUAGE_INDEX][1] };
  let script_str = format!("cargo tauri build; read -p '{hold_str}'");
  thread::spawn(move || {
    Command::new("alacritty")
      .args(&["-e", "bash", "-c", &script_str])
      .spawn()
      .expect("build Command spawn fial")
      .wait()
      .expect("build Command wait fail");

    app.emit_to("main", "show-container", ())
      .expect("build show-container fail");
  });
}

#[tauri::command]
fn info(app: AppHandle) {
  let hold_str = unsafe{ LANGUAGE_STRS[LANGUAGE_INDEX][1] };
  let script_str = format!("cargo tauri info ; read -p '{hold_str}'");
  thread::spawn(move || {
    Command::new("alacritty")
      .args(&["-e", "bash", "-c", &script_str])
      .spawn()
      .expect("info Command spawn fial")
      .wait()
      .expect("info Command wait fail");

    app.emit_to("main", "show-container", ())
      .expect("info show-container fail");
  });
}

/**************************************************
 * Tauri Init Works
**************************************************/
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_opener::init())
    .invoke_handler(tauri::generate_handler![change_language, install_tauri, create_project, open_project, dev, build, info])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
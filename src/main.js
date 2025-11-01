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

const { invoke } = window.__TAURI__.core;
const { open } = window.__TAURI__.dialog;

const LANGUAGE = [["切换至中文",
  "Set up the Tauri development environment",
  "Project Dir: ",
  "Create new project",
  "Open project",
  "Where would you like to create the new project?",
  "Project created successfully. Please open it manually.",
  "Not a Tauri project. Please choose a valid Tauri project directory.",
  "Please open a project first!"],
  ["Change language to English", "安装Tauri开发环境", "项目路径：", "创建新项目", "打开项目", "你想在哪里创建新项目？", "新项目创建完成，请手动打开它", "你所打开的路径不是Tauri项目路径，请重新选择！", "请先打开项目！"]];

let language = 0;
let inited = false;

let button_change_language;
let button_install_tauri;
let strong_project_dir;
let span_project_dir;
let button_create_project;
let button_open_project;

function change_language(language_strs) {
  button_change_language.value = LANGUAGE[language][0];
  button_install_tauri.value = LANGUAGE[language][1];
  strong_project_dir.textContent = LANGUAGE[language][2];
  button_create_project.value = LANGUAGE[language][3];
  button_open_project.value = LANGUAGE[language][4];
}

async function install_tauri() {
  invoke("install_tauri");
}

async function create_project() {
  let path = await open({
    multiple: false,
    title: LANGUAGE[language][5],
    directory: true
  });

  let ret = await invoke("create_project", { "pathStr": path });
  
  if (ret === null) {
    inited = false;
    span_project_dir.textContent = LANGUAGE[language][6];
  } else {
    alert(ret);
  }
}

async function open_project() {
  let path = await open({
    multiple: false,
    title: LANGUAGE[language][4],
    directory: true
  });

  let project_dir = await invoke("open_project", { "path": path });
  if(project_dir !== null) {
    span_project_dir.textContent = project_dir;
    inited = true;
  } else {
    alert(LANGUAGE[language][7]);
  }
}

async function dev() {
  if (inited) {
    invoke("dev");
  } else {
    alert(LANGUAGE[language][8]);
  }
}

async function build() {
  if (inited) {
    invoke("build");
  } else {
    alert(LANGUAGE[language][8]);
  }
}

window.addEventListener("DOMContentLoaded", () => {
  button_change_language = document.getElementById("change_language");
  button_change_language.addEventListener("click", (e) => {
    language = (language + 1) % 2;
    change_language(LANGUAGE[language]);
  });
  strong_project_dir = document.getElementById("project-dir-name");
  span_project_dir = document.getElementById("project-dir");
  button_install_tauri = document.getElementById("install-tauri-button");
  button_install_tauri.addEventListener("click", (e) => {
    install_tauri();
  });
  button_create_project = document.getElementById("create-project-button");
  button_create_project.addEventListener("click", (e) => {
    create_project();
  });
  button_open_project = document.getElementById("open-project-button");
  button_open_project.addEventListener("click", (e) => {
    open_project();
  });
  document.getElementById("dev-button").addEventListener("click", (e) => {
    dev();
  });
  document.getElementById("build-button").addEventListener("click", (e) => {
    build();
  });
});
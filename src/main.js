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
const { listen } = window.__TAURI__.event;

const LANGUAGE_STRS = [
  [
    "切换至中文",//0
    "Set up the Tauri development environment",//1
    "Project Dir: ",//2
    "Create new project",//3
    "Open project",//4
    "Where would you like to create the new project?",//5
    "Project created successfully. Please open it manually.",//6
    "Unable to find path: '",//7
    "'.",//8
    "Unable to access path: '",//9
    "'. You may not have the required permissions.",//10
    "Not a Tauri project. Please choose a valid Tauri project directory.",//11
    "Please open a project first!"//12
  ],
  [
    "Change language to English",//0
    "安装Tauri开发环境",//1
    "项目路径：",//2
    "创建新项目",//3
    "打开项目",//4
    "你想在哪里创建新项目？",//5
    "新项目创建完成，请手动打开它",//6
    "路径'",//7
    "'不存在！",//8
    "路径'",//9
    "'无法访问，可能是权限不足！",//10
    "你所打开的路径不是Tauri项目路径，请重新选择！",//11
    "请先打开项目！"//12
  ]
];
const ERROR_PATH_NOT_EXIST = 0;
const ERROR_PATH_CANT_ACCESS = 1;

let languageIndex = 0;
let projectOpened = false;

let containerMain;
let changeLanguageButton;
let installTauriButton;
let projectDirStrong;
let projectDirSpan;
let createProjectButton;
let openProjectButton;

async function changeLanguage() {
  invoke("change_language");
  languageIndex = (languageIndex + 1) % 2;
  changeLanguageButton.value = LANGUAGE_STRS[languageIndex][0];
  installTauriButton.value = LANGUAGE_STRS[languageIndex][1];
  projectDirStrong.textContent = LANGUAGE_STRS[languageIndex][2];
  createProjectButton.value = LANGUAGE_STRS[languageIndex][3];
  openProjectButton.value = LANGUAGE_STRS[languageIndex][4];
}

async function installTauri() {
  containerMain.style.display = "none";
  invoke("install_tauri");
}

async function createProject() {
  containerMain.style.display = "none";
  let path = await open({
    multiple: false,
    title: LANGUAGE_STRS[languageIndex][5],
    directory: true
  });
  if (path === null) {
    containerMain.style.display = "flex";
    return; 
  }

  let ret = await invoke("create_project", { "pathStr": path });
  if (ret === null) {
    projectOpened = false;
    projectDirSpan.textContent = LANGUAGE_STRS[languageIndex][6];
  } else {
    if (ret === ERROR_PATH_NOT_EXIST) {
      alert(LANGUAGE_STRS[languageIndex][7]+path+LANGUAGE_STRS[languageIndex][8]);
    } else if (ret === ERROR_PATH_CANT_ACCESS) {
      alert(LANGUAGE_STRS[languageIndex][9]+path+LANGUAGE_STRS[languageIndex][10]);
    }
  }

  containerMain.style.display = "flex";
}

async function openProject() {
  containerMain.style.display = "none";
  let path = await open({
    multiple: false,
    title: LANGUAGE_STRS[languageIndex][4],
    directory: true
  });
  if (path===null) { 
    containerMain.style.display = "flex";
    return; 
  }

  let projectDir = await invoke("open_project", { "pathStr": path });
  if (projectDir !== null) {
    projectDirSpan.textContent = projectDir;
    projectOpened = true;
  } else {
    alert(LANGUAGE_STRS[languageIndex][11]);
  }
  containerMain.style.display = "flex";
}

async function dev() {
  if (projectOpened) {
    containerMain.style.display = "none";
    invoke("dev");
  } else {
    alert(LANGUAGE_STRS[languageIndex][12]);
  }
}

async function build() {
  if(projectOpened) {
    containerMain.style.display = "none";
    invoke("build");
  } else {
    alert(LANGUAGE_STRS[languageIndex][12]);
  }
}

async function info() {
  containerMain.style.display = "none";
  invoke("info");
}

window.addEventListener("DOMContentLoaded", () => {
  containerMain = document.getElementById("container-main-id");
  changeLanguageButton = document.getElementById("change-language-button-id");
  changeLanguageButton.addEventListener("click", (e) => {
    changeLanguage();
  });

  projectDirStrong = document.getElementById("project-dir-strong-id");
  projectDirSpan = document.getElementById("project-dir-span-id");

  installTauriButton = document.getElementById("install-tauri-button-id");
  installTauriButton.addEventListener("click", (e) => {
    installTauri();
  });

  createProjectButton = document.getElementById("create-project-button-id");
  createProjectButton.addEventListener("click", (e) => {
    createProject();
  });

  openProjectButton = document.getElementById("open-project-button-id");
  openProjectButton.addEventListener("click", (e) => {
    openProject();
  });

  document.getElementById("dev-button-id").addEventListener("click", (e) => {
    dev();
  });
  document.getElementById("build-button-id").addEventListener("click", (e) => {
    build();
  });
  document.getElementById("info-button-id").addEventListener("click", (e) => {
    info();
  });

  listen("show-container", (event) => {
    containerMain.style.display = "flex";
  });
});
## Introduction  
tauri-kit is a basic GUI for Tauri development.  
It can be used to create new projects, open existing projects, and execute `dev`, `build`, `info` commands.  
  
## Compatibility  
This application runs good on Fedora 41.  
Compatibility with other Fedora versions or Linux distributions hasn't been verified.  
  
## Prerequisites  
### Set up the Tauri development environment  
1. Install rustup:  
  1.1. [Copy install script](https://rust-lang.org/tools/install/), then run the script in terminal.  
      When there comes up version choose prompt, choose the default version(the `1)` one).  
  1.2. or you can add ` -s -- -y` at the end of the script, then run the script in terminal, to automatically install the default version of rust toolchain.  
  1.3. After installation, you should restart the terminal to enable configurations.  
2. Install cargo Tauri tools:  
  2.1. Run `cargo install tauri-cli` to install `cargo tauri` **Command line Interface**.  
  2.2. Use `cargo install create-tauri-app` to install the command for creating new Tauri projects.  
3. Visit [this web](https://v2.tauri.app/start/prerequisites/) to learn how to install System Dependencies.  
### Build and Install tauri-kit  
1. Download and decode *tauri-kit* source code or `git clone` *tauri-kit* ropository, then `cd` into source code dir, run `cargo tauri build`.  
2. Find RPM in `tauri-kit/src-tauri/target/release/bundles/RPM/` and use `sudo dnf install <path to rpm>` command to install it.  
  
## License  
This project is licensed under the **GNU Affero General Public License v3.0 or later (AGPL-3.0-or-later)**.  
- 📜 Full license text: [LICENSE](LICENSE)  
- 🔗 [About AGPL: ](https://www.gnu.org/licenses/agpl-3.0)  
  
## My Social Media  
- [YouTube: ](https://www.youtube.com/@ChengAnXu-c4x)  
- [Bilibili: ](https://space.bilibili.com/166684807)  
  
## Contact Me  
email: hippyandy@proton.me  
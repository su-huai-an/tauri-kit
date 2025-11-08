Name:           tauri-kit
Version:        0.1.0
Release:        %autorelease
Summary:        A basic GUI for tauri development

License:        AGPL-3.0-or-later
URL:            https://github.com/su-huai-an/tauri-kit
Source0:        %{url}/archive/refs/tags/%{name}-%{version}.tar.gz

# 基础构建依赖
BuildRequires:  rustup
BuildRequires:  clang
BuildRequires:  pkgconfig
BuildRequires:  findutils

# GUI 依赖
BuildRequires:  webkit2gtk4.1-devel

# 系统运行时依赖
Requires:       alacritty
Requires:       bash
Requires:       gtk4
Requires:       webkit2gtk4.1
Requires:       libappindicator-gtk3

%description
tauri-kit is a basic GUI for Tauri development.
It can be used to install the Tauri development environment,
create new projects,
open existing projects,
run projects in development mode,
and build projects.

%prep
%autosetup -n %{name}-%{version}

%build
# 设置构建环境
export CARGO_HOME=%{_tmppath}/cargo
export RUSTUP_HOME=%{_tmppath}/rustup
mkdir -p ${CARGO_HOME} ${RUSTUP_HOME}

# 安全地设置 RUSTFLAGS（处理未定义的 _rustflags 宏）
%global rustflags %{?_rustflags:-C link-arg=-Wl,-z,relro,-z,now}
export RUSTFLAGS="%{rustflags}"

echo "RUSTFLAGS set to: $RUSTFLAGS"

# 安装 Rust 工具链
if [ ! -f "${CARGO_HOME}/bin/cargo" ]; then
    echo "Installing Rust toolchain..."
    rustup-init -y
    source "${CARGO_HOME}/env"
    export PATH="${CARGO_HOME}/bin:${PATH}"
    
    rustc --version
    cargo --version
fi

export PATH="${CARGO_HOME}/bin:${PATH}"

# 安装 Tauri CLI
echo "Installing Tauri CLI..."
cargo install tauri-cli --locked || cargo install tauri-cli

# 构建 Tauri 应用
echo "Building Tauri application..."
cargo tauri build --debug

%install
# 创建安装目录
mkdir -p %{buildroot}%{_bindir}
mkdir -p %{buildroot}%{_datadir}/applications
mkdir -p %{buildroot}%{_datadir}/icons/hicolor/scalable/apps
mkdir -p %{buildroot}%{_datadir}/icons/hicolor/256x256/apps

# 查找并安装二进制文件
if [ -f src-tauri/target/debug/%{name} ]; then
    echo "Installing binary from src-tauri/target/debug/"
    install -D -m 0755 src-tauri/target/debug/%{name} %{buildroot}%{_bindir}/%{name}
elif [ -f target/debug/%{name} ]; then
    echo "Installing binary from target/debug/"
    install -D -m 0755 target/debug/%{name} %{buildroot}%{_bindir}/%{name}
else
    echo "Searching for binary file..."
    binary=$(find . -name "%{name}" -type f -executable | head -1)

    if [ -n "$binary" ] && [ -f "$binary" ] && [ -x "$binary" ]; then
        echo "Installing binary from: $binary"
        install_dir=%{buildroot}%{_bindir}/%{name}
        install -D -m 0755 "$binary" $install_dir
        
        if [ -f "$install_dir" ] && [ -x "$install_dir" ]; then
            echo "Binary installed successfully"
        else
            echo "Error: Binary installation failed!" >&2
            exit 1
        fi
    else
        echo "Error: No executable binary found for %{name}" >&2
        exit 1
    fi
fi

# 安装桌面文件
for desktop_file in src-tauri/assets/linux/%{name}.desktop \
                    src-tauri/target/debug/bundle/rpm/*/usr/share/applications/%{name}.desktop; do
    if [ -f "$desktop_file" ]; then
        install -D -m 0644 "$desktop_file" %{buildroot}%{_datadir}/applications/%{name}.desktop
        break
    fi
done

# 安装图标
# 安装多尺寸图标
for size in 32 128; do
    size_icon_name=${size}x${size}
    for icon_file in "src-tauri/icons/$size_icon_name.png" \
                        "src-tauri/icons/icon.png" \
                        "src-tauri/target/debug/bundle/rpm/*/usr/share/icons/hicolor/$size_icon_name/apps/%{name}.png"; do
        if [ -f $icon_file ]; then
            install -D -m 0644 "$icon_file" "%{buildroot}%{_datadir}/icons/hicolor/$size_icon_name/apps/%{name}.png"
            echo "Installed $size_icon_name icon from: $icon_file"
            break
        fi
    done
done

icon_path="src-tauri/icons/128x128@2x.png"
if [ -f $icon_path ];then
    install -D -m 0644 $icon_path "%{buildroot}%{_datadir}/icons/hicolor/256x256/apps/%{name}.png"
    echo "Installed 256x256 icon from: $icon_path"
fi

# 安装SVG图标
for svg_icon_path in "src-tauri/icons/icon.svg" \
                        "src-tauri/target/debug/bundle/rpm/*/usr/share/icons/hicolor/scalable/apps/%{name}.svg"; do
    if [ -f $svg_icon_path ]; then
        install -D -m 0644 "$svg_icon_path" "%{buildroot}%{_datadir}/icons/hicolor/scalable/apps/%{name}.svg"
        echo "Installed scalable SVG icon"
    fi
done

# 安装许可证和文档
[ -f LICENSE ] && install -D -m 0644 LICENSE %{buildroot}%{_licensedir}/%{name}/LICENSE
[ -f README.md ] && install -D -m 0644 README.md %{buildroot}%{_docdir}/%{name}/README.md

%check
# 简单的二进制文件检查
if [ -f %{buildroot}%{_bindir}/%{name} ]; then
    echo "Binary installed successfully"
fi

%files
%license %{_licensedir}/%{name}/LICENSE
%doc %{_docdir}/%{name}/README.md
%{_bindir}/%{name}
%{_datadir}/applications/%{name}.desktop
%{_datadir}/icons/hicolor/*/apps/%{name}.*

%changelog
* Fri Nov 07 2025 俞妹妹宇宙唯一老公 <hippyandy@proton.me> - 0.2.2-1
- First Release of tauri-kit

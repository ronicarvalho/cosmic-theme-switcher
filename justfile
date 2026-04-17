# cosmic-theme-switcher — install tooling
#
# APP_ID must stay in sync with `Application::APP_ID` in src/app.rs.

set shell := ["bash", "-uc"]

APP_ID := "dev.encoders.CosmicThemeSwitcher"
BIN    := "cosmic-theme-switcher"
PREFIX := env_var_or_default("PREFIX", env_var("HOME") / ".local")

default:
    @just --list

build:
    cargo build --release

run:
    cargo run --release

install: build
    install -Dm755 target/release/{{BIN}} "{{PREFIX}}/bin/{{BIN}}"
    install -Dm644 data/{{APP_ID}}.desktop "{{PREFIX}}/share/applications/{{APP_ID}}.desktop"
    install -Dm644 data/icons/moon.svg "{{PREFIX}}/share/icons/hicolor/scalable/apps/{{APP_ID}}.svg"
    command -v update-desktop-database >/dev/null 2>&1 && update-desktop-database "{{PREFIX}}/share/applications" || true
    command -v gtk-update-icon-cache   >/dev/null 2>&1 && gtk-update-icon-cache   "{{PREFIX}}/share/icons/hicolor" || true
    @echo "Installed under {{PREFIX}}"

uninstall:
    rm -f "{{PREFIX}}/bin/{{BIN}}"
    rm -f "{{PREFIX}}/share/applications/{{APP_ID}}.desktop"
    rm -f "{{PREFIX}}/share/icons/hicolor/scalable/apps/{{APP_ID}}.svg"
    command -v update-desktop-database >/dev/null 2>&1 && update-desktop-database "{{PREFIX}}/share/applications" || true
    command -v gtk-update-icon-cache   >/dev/null 2>&1 && gtk-update-icon-cache   "{{PREFIX}}/share/icons/hicolor" || true
    @echo "Uninstalled from {{PREFIX}}"

reload-panel:
    pkill -x cosmic-panel || true

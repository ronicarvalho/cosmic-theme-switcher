# COSMIC Theme Switcher

COSMIC panel applet that toggles between light and dark theme with a single
click on a systray icon.

It works by flipping the content of
`$XDG_CONFIG_HOME/cosmic/com.system76.CosmicTheme.Mode/v1/is_dark`
(default: `$HOME/.config/...`) between `true` and `false`, which is what COSMIC
watches to switch modes.

## Build dependencies (Pop!_OS / Debian)

```sh
sudo apt install cargo cmake just libexpat1-dev libfontconfig-dev \
    libfreetype-dev libxkbcommon-dev pkgconf
```

## Install

From the repo root:

```sh
just build          # cargo build --release
just install        # copy binary + .desktop + icon into $HOME/.local
just reload-panel   # restart cosmic-panel so it rescans applets
```

Then open **COSMIC Settings → Desktop → Panel → Add applet** and add
"COSMIC Theme Switch".

To uninstall:

```sh
just uninstall
```

> Make sure `$HOME/.local/bin` is on your `PATH`; the `.desktop` file relies
> on that for its `Exec=cosmic-theme-switcher` entry.

To install somewhere else (e.g. system-wide), override `PREFIX`:

```sh
sudo just PREFIX=/usr/local install
```

## Run without installing (smoke test)

```sh
just run    # cargo run --release
```

## On the bar

![Dark Mode](dark-mode.png)

![Light Mode](light-mode.png)

## Layout

- `src/theme_io.rs` — read/write the `is_dark` file (unit-tested)
- `src/icons.rs` — embedded Sun/Moon SVGs
- `src/app.rs` — `cosmic::Application` implementation
- `src/main.rs` — entry point
- `data/` — panel metadata and icon assets

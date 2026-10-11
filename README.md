# Versync

Versync is a lightweight, minimalist spotify lyrics viewer for the terminal, built in Rust and made for Linux. 

i made this project on gentoo linux, but it works on any linux distribution :)

## features

- **live synchronization** — lyrics follow spotify playback automatically.
- **smooth transitions** — 600ms color transitions between lyric lines.
- **minimalist design** — no unnecessary buttons, artwork, or visual clutter.
- **context aware lyrics** — see previous, current, and upcoming lines.
- **automatic track detection** — lyrics update when the song changes.
- **lightweight TUI** — built with Rust and Ratatui.
- **cross distribution support** — designed for Linux without requiring a specific init system.
- **no spotify API keys** — uses MPRIS through `playerctl`.
 
## requirements

- Linux with a working D-Bus session
- spotify with MPRIS support
- `playerctl`
- Rust and Cargo (to build from source)
- internet connection for retrieving lyrics
- a terminal with true color support is recommended

## installation

clone the repository:

```bash
git clone https://github.com/cimohhh/versync.git
cd versync
```

run the installer:

```bash
./install.sh
```

the installer compiles Versync in release mode and installs it to `~/.local/bin/versync`.

ensure `~/.local/bin` is included in your `PATH`.

## usage

start spotify and play a song, then run:

```bash
versync
```

Versync automatically detects the currently playing track and retrieves synchronized lyrics when available.

| Key | Action |
|---|---|
| `q` | Quit |
| `Esc` | Quit |

Versync runs directly in your terminal. it does not require Kitty or a particular desktop environment.

## how it works

Versync communicates with spotify through the Linux MPRIS interface using `playerctl`.

it retrieves timestamped lyrics from [LRCLIB](https://lrclib.net/) and matches them to spotify's playback position.

lyrics are rendered using Ratatui, with cyan highlighting for the active line and smooth transitions between lines.

## technology

- **Rust** — core application
- **Ratatui** — terminal interface
- **Crossterm** — terminal rendering and input
- **Reqwest** — HTTP client
- **Serde** — JSON deserialization
- **LRCLIB** — synchronized lyrics
- **playerctl** — spotify playback integration

## compatibility

Versync is designed for Linux distributions with spotify MPRIS support, including Gentoo, Arch Linux, Fedora, Ubuntu, and Debian.

currently tested on Gentoo Linux. other distributions have not yet been verified.

no systemd dependency is required by Versync itself.

## limitations

- spotify is currently the only supported player.
- not every track has synchronized lyrics available.
- lyric accuracy depends on the timestamps provided by LRCLIB.
- an internet connection is required to fetch lyrics.

## contributing

issues, bug reports, suggestions, and pull requests are welcome.

## license

Versync is licensed under the [MIT License](LICENSE).

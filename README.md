versync is a spotify lyrics viewer built in rust and made for linux. 
i made this project on gentoo linux, but it works on any linux distribution :)

FEATURES - lyrics follow spotify playback automatically, 600ms color transitions between lyric lines, see previous/current/upcoming lines, lyrics update when the song changes, lightweight TUI built with rust and ratatui, cross distribution support, designed for linux without requiring a specific init system, no spotify API keys, uses MPRIS through playerctl.
 
REQUIREMENTS - versync requires linux with a working D Bus session, spotify with MPRIS support, playerctl, rust and cargo (to build from source), internet connection for retrieving lyrics, and a terminal with true color support is recommended.

INSTALLATION - clone the repository:

```bash
git clone https://github.com/cimohhh/versync.git
cd versync
```

run the installer:

```bash
./install.sh
```

the installer compiles versync in release mode and installs it to `~/.local/bin/versync`.

make sure `~/.local/bin` is included in your `PATH`.

USAGE - start spotify and play a song, then run:

```bash
versync
```

versync automatically detects the currently playing track and retrieves synchronized lyrics when available.

| Key | Action |
|---|---|
| `q` | Quit |
| `Esc` | Quit |

versync runs directly in your terminal. it does not require kitty or a specific desktop environment.

HOW IT WORKS - versync communicates with spotify through the linux MPRIS interface using playerctl. it retrieves timestamped lyrics from [LRCLIB](https://lrclib.net/) and matches them to spotify's playback position. lyrics are rendered using ratatui, with cyan highlighting for the active line and smooth transitions between lines.

COMPATIBILITY - versync is designed for linux distributions with spotify MPRIS support, including gentoo, arch, fedora, ubuntu, and debian.

## license

versync is licensed under the [MIT License](LICENSE).

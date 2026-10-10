mod lyrics;
mod ui;

use std::{
    io,
    process::Command,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use ratatui::{Terminal, backend::CrosstermBackend};

use ui::App;

const SPOTIFY_POLL: Duration = Duration::from_millis(250);
const ANIMATION_FRAME: Duration = Duration::from_millis(16);
const MAX_IDLE_WAIT: Duration = Duration::from_millis(100);

fn playerctl(args: &[&str]) -> Option<String> {
    let output = Command::new("playerctl")
        .arg("-p")
        .arg("spotify")
        .args(args)
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn current_track() -> Option<(String, String)> {
    let data = playerctl(&["metadata", "--format", "{{title}}\t{{artist}}"])?;

    let (title, artist) = data.split_once('\t')?;

    if title.is_empty() || artist.is_empty() {
        return None;
    }

    Some((title.to_string(), artist.to_string()))
}

struct PlaybackClock {
    position: f64,
    updated_at: Instant,
    playing: bool,
}

impl PlaybackClock {
    fn new() -> Self {
        Self {
            position: 0.0,
            updated_at: Instant::now(),
            playing: false,
        }
    }

    fn reset(&mut self) {
        self.position = 0.0;
        self.updated_at = Instant::now();
        self.playing = false;
    }

    fn update(&mut self, position: f64, playing: bool) {
        self.position = position;
        self.updated_at = Instant::now();
        self.playing = playing;
    }

    fn estimated_position(&self) -> f64 {
        if self.playing {
            self.position + self.updated_at.elapsed().as_secs_f64()
        } else {
            self.position
        }
    }
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    let (tx, rx) = mpsc::channel();

    let mut app = App::new();
    let mut clock = PlaybackClock::new();

    let mut previous_track: Option<(String, String)> = None;
    let mut last_poll = Instant::now() - SPOTIFY_POLL;
    let mut last_draw = Instant::now() - ANIMATION_FRAME;

    let mut request_id = 0u64;
    let mut needs_redraw = true;
    let mut last_animation_state = false;

    loop {
        if last_poll.elapsed() >= SPOTIFY_POLL {
            last_poll = Instant::now();

            let track = current_track();

            if track != previous_track {
                previous_track = track.clone();
                request_id += 1;

                app.reset_lyrics();
                clock.reset();

                if let Some((title, artist)) = track {
                    app.title = title.clone();
                    app.artist = artist.clone();
                    app.message = "Loading synchronized lyrics...".into();

                    let sender = tx.clone();
                    let id = request_id;

                    thread::spawn(move || {
                        let result = lyrics::fetch_lyrics(&title, &artist);

                        let _ = sender.send((id, result));
                    });
                } else {
                    app.title = "Waiting for Spotify...".into();
                    app.artist.clear();
                    app.message = "Play a song in Spotify".into();
                }

                needs_redraw = true;
            }

            if previous_track.is_some() {
                let playing = playerctl(&["status"]).as_deref() == Some("Playing");

                if let Some(position) = playerctl(&["position"]) {
                    if let Ok(seconds) = position.parse::<f64>() {
                        clock.update(seconds, playing);
                    }
                }
            }
        }

        while let Ok((id, result)) = rx.try_recv() {
            if id != request_id {
                continue;
            }

            match result {
                Ok(Some(raw)) => {
                    app.set_lyrics(&raw);

                    if app.lyrics.is_empty() {
                        app.message = "No timed lyrics available".into();
                    }
                }
                Ok(None) => {
                    app.message = "No synchronized lyrics found".into();
                }
                Err(error) => {
                    app.message = format!("Lyrics request failed: {error}");
                }
            }

            needs_redraw = true;
        }

        app.position = clock.estimated_position();

        let animating = clock.playing && app.animation_active();

        if animating || last_animation_state != animating {
            needs_redraw = true;
        }

        last_animation_state = animating;

        if needs_redraw && last_draw.elapsed() >= ANIMATION_FRAME {
            terminal.draw(|frame| ui::draw(frame, &app))?;
            last_draw = Instant::now();
            needs_redraw = animating;
        }

        // Wait for input without spinning the CPU.
        let wait = if animating {
            ANIMATION_FRAME
        } else {
            MAX_IDLE_WAIT
        };

        if event::poll(wait)? {
            match event::read()? {
                Event::Key(key)
                    if key.kind == KeyEventKind::Press
                        && matches!(key.code, KeyCode::Char('q') | KeyCode::Esc) =>
                {
                    break;
                }
                Event::Resize(_, _) => {
                    needs_redraw = true;
                }
                _ => {}
            }
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();

    if let Err(error) = execute!(stdout, EnterAlternateScreen) {
        disable_raw_mode()?;
        return Err(error);
    }

    let result = (|| {
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let result = run_app(&mut terminal);
        let _ = terminal.show_cursor();

        result
    })();

    let _ = disable_raw_mode();
    let _ = execute!(io::stdout(), LeaveAlternateScreen);

    result
}


use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

const TRANSITION: f64 = 0.600;
const EARLY_FADE: f64 = 0.300;

const CYAN: (u8, u8, u8) = (103, 232, 249);
const GRAY: (u8, u8, u8) = (148, 163, 184);
const DIM: (u8, u8, u8) = (71, 85, 105);

pub struct LyricLine {
    pub time: f64,
    pub text: String,
}

pub struct App {
    pub title: String,
    pub artist: String,
    pub lyrics: Vec<LyricLine>,
    pub position: f64,
    pub message: String,
}

impl App {
    pub fn new() -> Self {
        Self {
            title: "Waiting for Spotify...".into(),
            artist: String::new(),
            lyrics: Vec::new(),
            position: 0.0,
            message: "Play a song in Spotify".into(),
        }
    }

    pub fn reset_lyrics(&mut self) {
        self.lyrics.clear();
        self.position = 0.0;
    }

    pub fn set_lyrics(&mut self, raw: &str) {
        self.lyrics = parse_lyrics(raw);
    }

    pub fn animation_active(&self) -> bool {
        self.transition().is_some()
    }

    fn transition(&self) -> Option<(usize, f64)> {
        if self.lyrics.len() < 2 {
            return None;
        }

        let next = self
            .lyrics
            .iter()
            .position(|line| line.time > self.position)?;

        if next == 0 {
            return None;
        }

        let timestamp = self.lyrics[next].time;
        let start = timestamp - EARLY_FADE;
        let end = start + TRANSITION;

        if self.position < start || self.position >= end {
            return None;
        }

        let progress =
            ((self.position - start) / TRANSITION)
                .clamp(0.0, 1.0);

        Some((next, smoothstep(progress)))
    }
}

pub fn parse_lyrics(raw: &str) -> Vec<LyricLine> {
    let mut lyrics = Vec::new();

    for line in raw.lines() {
        if let Some((timestamp, text)) = line.split_once(']') {
            if let Some(timestamp) = timestamp.strip_prefix('[') {
                if let Some((minutes, seconds)) =
                    timestamp.split_once(':')
                {
                    if let (Ok(minutes), Ok(seconds)) = (
                        minutes.parse::<f64>(),
                        seconds.parse::<f64>(),
                    ) {
                        lyrics.push(LyricLine {
                            time: minutes * 60.0 + seconds,
                            text: text.trim().to_string(),
                        });
                    }
                }
            }
        }
    }

    lyrics.sort_by(|a, b| a.time.total_cmp(&b.time));
    lyrics
}

fn smoothstep(t: f64) -> f64 {
    t * t * (3.0 - 2.0 * t)
}

fn blend(
    from: (u8, u8, u8),
    to: (u8, u8, u8),
    progress: f64,
) -> Color {
    let p = progress.clamp(0.0, 1.0);

    let channel = |a: u8, b: u8| {
        (a as f64 + (b as f64 - a as f64) * p)
            .round() as u8
    };

    Color::Rgb(
        channel(from.0, to.0),
        channel(from.1, to.1),
        channel(from.2, to.2),
    )
}

fn lyric_color(distance: isize) -> (u8, u8, u8) {
    match distance.abs() {
        0 => CYAN,
        1 => GRAY,
        _ => DIM,
    }
}

fn render_line(
    frame: &mut Frame,
    area: Rect,
    text: &str,
    color: Color,
    bold: bool,
) {
    let mut style = Style::default().fg(color);

    if bold {
        style = style.add_modifier(Modifier::BOLD);
    }

    frame.render_widget(
        Paragraph::new(
            Line::from(Span::styled(text.to_owned(), style)),
        )
        .alignment(Alignment::Center),
        area,
    );
}

pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(5),
        ])
        .split(area);

    frame.render_widget(
        Paragraph::new(app.title.as_str())
            .style(
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(Alignment::Center),
        sections[0],
    );

    frame.render_widget(
        Paragraph::new(app.artist.as_str())
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center),
        sections[1],
    );

    frame.render_widget(
        Block::default()
            .borders(Borders::TOP)
            .border_style(
                Style::default().fg(Color::DarkGray),
            ),
        sections[2],
    );

    let lyrics_area = sections[3];

    if app.lyrics.is_empty() {
        frame.render_widget(
            Paragraph::new(app.message.as_str())
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::Cyan)),
            lyrics_area,
        );
        return;
    }

    if lyrics_area.height == 0 {
        return;
    }

    let active = app
        .lyrics
        .iter()
        .rposition(|line| line.time <= app.position)
        .map(|index| index as isize)
        .unwrap_or(-1);

    let transition = app.transition();

    let (base, progress) = match transition {
        Some((next, progress)) => {
            (next as isize - 1, progress)
        }
        None => (active, 0.0),
    };

    let center =
        lyrics_area.y as i32 + lyrics_area.height as i32 / 2;

    // Delay the physical row shift until the fade
    // has already made substantial progress.
    let shift = if transition.is_some() && progress >= 0.8 {
        1
    } else {
        0
    };

    for relative in -3isize..=4 {
        let index = base + relative;

        if index < 0 {
            continue;
        }

        let Some(lyric) = app.lyrics.get(index as usize) else {
            continue;
        };

        let y = center + relative as i32 - shift;

        if y < lyrics_area.y as i32
            || y >= lyrics_area.bottom() as i32
        {
            continue;
        }

        let color = if transition.is_some() {
            blend(
                lyric_color(index - base),
                lyric_color(index - (base + 1)),
                progress,
            )
        } else {
            let rgb = lyric_color(index - active);
            Color::Rgb(rgb.0, rgb.1, rgb.2)
        };

        let row = Rect {
            x: lyrics_area.x,
            y: y as u16,
            width: lyrics_area.width,
            height: 1,
        };

        render_line(
            frame,
            row,
            &lyric.text,
            color,
            transition.is_none() && index == active,
        );
    }
}

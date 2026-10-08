use std::env;
use std::io::IsTerminal;
use termimad::crossterm::style::Color;
use termimad::{terminal_size, MadSkin};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoxStyle {
    Hook,           // Red border
    TaskContent,    // Cyan border
    AdditionalInfo, // Yellow border
}

/// Determines whether colored markdown rendering should be enabled.
pub fn should_color() -> bool {
    if let Ok(val) = env::var("NO_COLOR") {
        if !val.is_empty() {
            return false;
        }
    }
    if let Ok(val) = env::var("DID_NO_COLOR") {
        if !val.is_empty() {
            return false;
        }
    }
    if let Ok(val) = env::var("DID_COLOR") {
        if val == "0" || val.eq_ignore_ascii_case("false") {
            return false;
        }
        if val == "1" || val.eq_ignore_ascii_case("true") {
            return true;
        }
    }
    if let Ok(val) = env::var("CLICOLOR") {
        if val == "0" {
            return false;
        }
    }
    if let Ok(val) = env::var("FORCE_COLOR") {
        if val == "1" || val.eq_ignore_ascii_case("true") {
            return true;
        }
    }

    std::io::stdout().is_terminal()
}

/// Renders markdown text directly (without borders) if colored rendering is enabled,
/// or outputs standard plain text otherwise.
pub fn render_markdown(md: &str) {
    if should_color() {
        let skin = make_skin();
        skin.print_text(md);
    } else if md.ends_with('\n') {
        print!("{}", md);
    } else {
        println!("{}", md);
    }
}

/// Renders a titled content block inside a colored border container.
///
/// - `Hook`: Red border
/// - `TaskContent`: Cyan border
/// - `AdditionalInfo`: Yellow border
pub fn draw_box(title: &str, content_md: &str, style: BoxStyle) {
    if !should_color() {
        if !title.is_empty() {
            println!("{}", title);
        }
        if content_md.ends_with('\n') {
            print!("{}", content_md);
        } else {
            println!("{}", content_md);
        }
        return;
    }

    let color_code = match style {
        BoxStyle::Hook => "\x1b[31m",
        BoxStyle::TaskContent => "\x1b[36m",
        BoxStyle::AdditionalInfo => "\x1b[33m",
    };
    let reset = "\x1b[0m";

    let term_width = (terminal_size().0 as usize).clamp(40, 100);
    let inner_width = term_width.saturating_sub(4);

    let skin = make_skin();
    let rendered_text = format!("{}", skin.text(content_md, Some(inner_width)));

    let lines: Vec<&str> = rendered_text.lines().collect();

    // Top border
    let mut top_line = String::new();
    top_line.push_str("╭─ ");
    if !title.is_empty() {
        top_line.push_str(title);
        top_line.push(' ');
    }
    let current_top_len = unicode_width::UnicodeWidthStr::width(top_line.as_str());
    if current_top_len < term_width.saturating_sub(1) {
        let fill_len = term_width.saturating_sub(1).saturating_sub(current_top_len);
        top_line.push_str(&"─".repeat(fill_len));
    }
    top_line.push('╮');

    println!("{}{}{}", color_code, top_line, reset);

    // Content lines
    for line in lines {
        let line_vis_width = visible_width(line);
        let padding = inner_width.saturating_sub(line_vis_width);
        println!(
            "{}│{} {} {}{}│{}",
            color_code,
            reset,
            line,
            " ".repeat(padding),
            color_code,
            reset
        );
    }

    // Bottom border
    let bottom_line = format!("╰{}╯", "─".repeat(term_width.saturating_sub(2)));
    println!("{}{}{}", color_code, bottom_line, reset);
}

/// Helper function to compute visible display width excluding ANSI escape sequences.
fn visible_width(s: &str) -> usize {
    let mut width = 0;
    let mut in_escape = false;
    for c in s.chars() {
        if c == '\x1b' {
            in_escape = true;
        } else if in_escape {
            if c == 'm' || c == 'K' {
                in_escape = false;
            }
        } else {
            width += unicode_width::UnicodeWidthChar::width(c).unwrap_or(0);
        }
    }
    width
}

/// Constructs a customized `MadSkin` with distinct, vibrant terminal colors.
fn make_skin() -> MadSkin {
    let mut skin = MadSkin::default_dark();
    skin.set_headers_fg(Color::Yellow);
    skin.bold.set_fg(Color::Cyan);
    skin.italic.set_fg(Color::Magenta);
    skin.code_block.set_fg(Color::Green);
    skin.inline_code.set_fg(Color::Green);
    skin.bullet.set_fg(Color::Yellow);
    skin.quote_mark.set_fg(Color::DarkCyan);
    skin
}

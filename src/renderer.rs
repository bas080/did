use std::env;
use std::io::IsTerminal;
use termimad::crossterm::style::Color;
use termimad::MadSkin;

/// Determines whether colored markdown rendering should be enabled.
///
/// Rendering is enabled by default when stdout is connected to a TTY, unless opted out
/// via standard environment variables (`NO_COLOR`, `DID_NO_COLOR`, `DID_COLOR=0`, or `CLICOLOR=0`).
/// Setting `DID_COLOR=1` or `FORCE_COLOR=1` forces colored rendering even when not on a TTY.
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

/// Renders markdown text to stdout using `termimad` if colored rendering is enabled,
/// or outputs standard plain text otherwise.
pub fn render_markdown(md: &str) {
    if should_color() {
        let skin = make_skin();
        skin.print_text(md);
    } else {
        if md.ends_with('\n') {
            print!("{}", md);
        } else {
            println!("{}", md);
        }
    }
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

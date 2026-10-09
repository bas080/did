use std::env;
use std::io::IsTerminal;
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use syntect::util::as_24_bit_terminal_escaped;
use termimad::crossterm::style::Color;
use termimad::{terminal_size, MadSkin};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoxStyle {
    Hook,           // Red border
    #[allow(dead_code)]
    TaskContent,    // Cyan border
    AdditionalInfo, // Yellow border
}

/// Determines whether colored output should be enabled.
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

/// Determines whether box containers should be drawn around output sections.
pub fn should_box() -> bool {
    if let Ok(val) = env::var("DID_NO_BOX") {
        if !val.is_empty() {
            return false;
        }
    }
    if let Ok(val) = env::var("NO_BOX") {
        if !val.is_empty() {
            return false;
        }
    }
    if let Ok(val) = env::var("DID_BOX") {
        if val == "0" || val.eq_ignore_ascii_case("false") {
            return false;
        }
        if val == "1" || val.eq_ignore_ascii_case("true") {
            return true;
        }
    }
    if let Ok(val) = env::var("DID_COLOR") {
        if val == "1" || val.eq_ignore_ascii_case("true") {
            return true;
        }
    }
    if let Ok(val) = env::var("FORCE_COLOR") {
        if val == "1" || val.eq_ignore_ascii_case("true") {
            return true;
        }
    }

    std::io::stdout().is_terminal()
}

/// Highlights syntax in codeblocks without border boxes, adding extra padding lines above and below.
pub fn highlight_and_box_codeblocks(md: &str, _max_width: usize) -> String {
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let theme_name = match env::var("DID_SYNTAX_THEME")
        .or_else(|_| env::var("DID_THEME"))
        .unwrap_or_default()
        .to_lowercase()
        .as_str()
    {
        "light" | "github" => "InspiredGitHub",
        "solarized" => "Solarized (dark)",
        "mocha" => "base16-mocha.dark",
        _ => "base16-ocean.dark",
    };

    let theme = ts
        .themes
        .get(theme_name)
        .or_else(|| ts.themes.get("base16-ocean.dark"))
        .or_else(|| ts.themes.values().next());

    let use_color = should_color();
    let mut result = String::new();
    let lines: Vec<&str> = md.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i];
        if line.trim_start().starts_with("```") {
            let lang_token = line.trim_start().trim_start_matches('`').trim();
            let mut code_lines = Vec::new();
            i += 1;
            while i < lines.len() && !lines[i].trim_start().starts_with("```") {
                code_lines.push(lines[i]);
                i += 1;
            }
            if i < lines.len() {
                i += 1;
            }

            let syntax = if !lang_token.is_empty() {
                ps.find_syntax_by_token(lang_token)
                    .or_else(|| ps.find_syntax_by_extension(lang_token))
                    .unwrap_or_else(|| ps.find_syntax_plain_text())
            } else {
                ps.find_syntax_plain_text()
            };

            let mut h = if use_color {
                theme.map(|t| HighlightLines::new(syntax, t))
            } else {
                None
            };

            // Padding line above codeblock
            result.push('\n');

            for code_line in code_lines {
                let formatted_line = if let Some(ref mut highlighter) = h {
                    let line_with_nl = format!("{}\n", code_line);
                    let ranges = highlighter
                        .highlight_line(&line_with_nl, &ps)
                        .unwrap_or_default();
                    let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
                    escaped.trim_end_matches('\n').to_string()
                } else {
                    code_line.to_string()
                };

                result.push_str("  ");
                result.push_str(&formatted_line);
                result.push('\n');
            }

            // Padding line below codeblock
            result.push('\n');
        } else {
            result.push_str(line);
            result.push('\n');
            i += 1;
        }
    }

    result
}

/// Renders markdown text directly if colored rendering is enabled,
/// or outputs standard plain text otherwise.
pub fn render_markdown(md: &str) {
    if should_color() {
        let term_width = (terminal_size().0 as usize).clamp(40, 100);
        let processed = highlight_and_box_codeblocks(md, term_width);
        let skin = make_skin();
        skin.print_text(&processed);
    } else if md.ends_with('\n') {
        print!("{}", md);
    } else {
        println!("{}", md);
    }
}

/// Prints a section separator (`---`).
pub fn render_section_break() {
    if should_color() {
        render_markdown("\n---\n");
    } else {
        println!("\n---");
    }
}

/// Renders a titled content block inside a border container.
pub fn draw_box(title: &str, content_md: &str, style: BoxStyle) {
    draw_box_to(&mut std::io::stdout(), title, content_md, style);
}

/// Renders a titled content block inside a border container to the specified writer.
pub fn draw_box_to<W: std::io::Write>(writer: &mut W, title: &str, content_md: &str, style: BoxStyle) {
    if !should_box() {
        if should_color() {
            render_section_break();
            if !title.is_empty() {
                let _ = writeln!(writer, "## {}", title);
            }
            let term_width = (terminal_size().0 as usize).clamp(40, 100);
            let processed = highlight_and_box_codeblocks(content_md, term_width);
            let skin = make_skin();
            let _ = write!(writer, "{}", skin.text(&processed, Some(term_width)));
        } else {
            if !title.is_empty() {
                let _ = writeln!(writer, "{}", title);
            }
            if content_md.ends_with('\n') {
                let _ = write!(writer, "{}", content_md);
            } else {
                let _ = writeln!(writer, "{}", content_md);
            }
        }
        return;
    }

    let use_color = should_color();
    let (color_code, reset) = if use_color {
        let code = match style {
            BoxStyle::Hook => "\x1b[31m",
            BoxStyle::TaskContent => "\x1b[36m",
            BoxStyle::AdditionalInfo => "\x1b[33m",
        };
        (code, "\x1b[0m")
    } else {
        ("", "")
    };

    let term_width = (terminal_size().0 as usize).clamp(40, 100);
    let inner_width = term_width.saturating_sub(4);

    let processed_md = highlight_and_box_codeblocks(content_md, inner_width);

    let skin = make_skin();
    let rendered_text = if use_color {
        format!("{}", skin.text(&processed_md, Some(inner_width)))
    } else {
        processed_md
    };

    let lines: Vec<&str> = rendered_text.lines().collect();

    // Top border
    let mut top_line = String::new();
    if !title.is_empty() {
        top_line.push_str("╭─ ");
        top_line.push_str(title);
        top_line.push(' ');
    } else {
        top_line.push_str("╭─");
    }
    let current_top_len = unicode_width::UnicodeWidthStr::width(top_line.as_str());
    if current_top_len < term_width.saturating_sub(1) {
        let fill_len = term_width.saturating_sub(1).saturating_sub(current_top_len);
        top_line.push_str(&"─".repeat(fill_len));
    }
    top_line.push('╮');

    let _ = writeln!(writer, "{}{}{}", color_code, top_line, reset);

    // Content lines
    for line in lines {
        let line_vis_width = visible_width(line);
        let padding = inner_width.saturating_sub(line_vis_width);
        let _ = writeln!(
            writer,
            "{}│{} {} {}{}│{}",
            color_code, reset, line, " ".repeat(padding), color_code, reset
        );
    }

    // Bottom border
    let bottom_line = format!("╰{}╯", "─".repeat(term_width.saturating_sub(2)));
    let _ = writeln!(writer, "{}{}{}", color_code, bottom_line, reset);
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

/// Constructs a customized `MadSkin` with a vibrant Rainbow Unicorn color palette.
fn make_skin() -> MadSkin {
    let theme_var = env::var("DID_THEME").unwrap_or_default().to_lowercase();
    let is_light = theme_var == "light" || theme_var == "github";

    let mut skin = if is_light {
        MadSkin::default_light()
    } else {
        MadSkin::default_dark()
    };

    // Ensure all headings are left-aligned
    for header in &mut skin.headers {
        header.align = termimad::Alignment::Left;
    }

    // Rainbow Unicorn Palette
    skin.headers[0].set_fg(Color::AnsiValue(205)); // # Header 1: Hot Pink
    skin.headers[1].set_fg(Color::AnsiValue(51));  // ## Header 2: Neon Cyan
    skin.headers[2].set_fg(Color::AnsiValue(226)); // ### Header 3: Bright Yellow
    skin.bold.set_fg(Color::AnsiValue(46));        // **Bold**: Neon Green
    skin.italic.set_fg(Color::AnsiValue(183));     // *Italic*: Lavender

    // High-contrast inline codeblock (no background, bright magenta foreground)
    skin.inline_code.set_fg(Color::AnsiValue(201)); // `Code`: Bright Magenta
    skin.inline_code.set_bg(Color::Reset);

    skin.bullet.set_fg(Color::AnsiValue(208));     // - Bullet: Pastel Coral
    skin.quote_mark.set_fg(Color::AnsiValue(37));  // > Quote: Dark Cyan
    skin
}

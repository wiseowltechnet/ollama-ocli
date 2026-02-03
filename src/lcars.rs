// LCARS authentic colors (hex multiples of 33)
pub const ORANGE: &str = "\x1b[38;2;255;153;0m";
pub const PURPLE: &str = "\x1b[38;2;204;153;255m";
pub const BLUE: &str = "\x1b[38;2;153;204;255m";
pub const PINK: &str = "\x1b[38;2;255;153;204m";
pub const RED: &str = "\x1b[38;2;255;102;102m";
pub const YELLOW: &str = "\x1b[38;2;255;204;0m";
pub const RESET: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";

pub fn header() -> String {
    format!(
        "{}╔═══════════════════════════════════════════════════════════════╗\n\
         ║  {}LCARS{}  {}OLLAMA CLI{}                                          ║\n\
         ╚═══════════════════════════════════════════════════════════════╝{}",
        ORANGE, BLUE, ORANGE, YELLOW, ORANGE, RESET
    )
}

pub fn prompt() -> String {
    format!("{}▶{} ", BLUE, RESET)
}

pub fn success(msg: &str) -> String {
    format!("{}●{} {}", BLUE, RESET, msg)
}

pub fn error(msg: &str) -> String {
    format!("{}●{} {}", RED, RESET, msg)
}

pub fn info(msg: &str) -> String {
    format!("{}●{} {}", PURPLE, RESET, msg)
}

pub fn warning(msg: &str) -> String {
    format!("{}●{} {}", YELLOW, RESET, msg)
}

pub fn status_bar(left: &str, right: &str) -> String {
    let width = 63;
    let left_len = left.chars().count();
    let right_len = right.chars().count();
    let padding = if width > left_len + right_len {
        width - left_len - right_len
    } else {
        1
    };
    format!(
        "{}{}{}{}{}",
        ORANGE,
        left,
        " ".repeat(padding),
        PURPLE,
        right
    )
}

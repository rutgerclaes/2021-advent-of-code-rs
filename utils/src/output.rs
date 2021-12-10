use crate::results::Result;
use ansi_term::Colour::Green;
use ansi_term::Colour::Red;
use ansi_term::Style;
use env_logger::Builder;
use env_logger::Env;
use std::fmt::Display;

pub fn display_result<I: Display>(result: Result<I>) -> String {
    let style = Style::new().bold();
    match result {
        Ok(outcome) => style.fg(Green).paint(format!("{}", outcome)).to_string(),
        Err(error) => Red
            .paint(format!(
                "{}: {}",
                "Failed to compute result",
                style.paint(format!("{}", error))
            ))
            .to_string(),
    }
}

pub fn init_env_log() {
    let env = Env::default().default_filter_or("info");
    Builder::from_env(env).format_timestamp(None).init();
}

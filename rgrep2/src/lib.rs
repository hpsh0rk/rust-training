use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::ops::Range;
use std::path::Path;

use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use regex::Regex;

mod error;
pub use error::GrepError;

/// A rgrep for grep string from files
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
#[command(next_line_help = true)]
pub struct GrepConfig {
    #[arg(short, long)]
    pattern: String,
    #[arg(short, long)]
    glob: String,
}

pub type StrategyFn = fn(&Path, &mut dyn BufRead, &Regex, &mut dyn Write) -> Result<(), GrepError>;

impl GrepConfig {
    pub fn match_with_default_strategy(&self) -> Result<(), GrepError> {
        self.match_with(default_strategy)
    }

    pub fn match_with(&self, strategy: StrategyFn) -> Result<(), GrepError> {
        let regex = Regex::new(&self.pattern)?;
        let files: Vec<_> = glob::glob(&self.glob)?.collect();
        files.iter().for_each(|v| {
            if let Ok(path) = v {
                if let Ok(file) = File::open(path) {
                    let mut reader = BufReader::new(file);
                    let mut stdout = io::stdout();
                    if let Err(e) = strategy(path.as_path(), &mut reader, &regex, &mut stdout) {
                        println!("Internal error:{:?}", e);
                    }
                }
            }
        });
        Ok(())
    }
}

fn default_strategy(
    path: &Path,
    reader: &mut dyn BufRead,
    pattern: &Regex,
    writer: &mut dyn Write,
) -> Result<(), GrepError> {
    let matches = reader
        .lines()
        .enumerate()
        .map(|(lineno, line)| {
            line.ok()
                .and_then(|line| {
                    pattern
                        .find(&line)
                        .map(|m| format_line(&line, lineno + 1, m.range()))
                })
        })
        .filter_map(|v| v.ok_or(()).ok())
        .collect::<Vec<_>>()
        .join("\n");

    if !matches.is_empty() {
        writer.write_all(path.display().to_string().green().as_bytes())?;
        writer.write_all(b"\n")?;
        writer.write_all(matches.as_bytes())?;
        writer.write_all(b"\n")?;
    }

    Ok(())
}

/// 格式化输出匹配的行，包含行号，列号和带有高亮的第一个匹配项
pub fn format_line(line: &str, lineno: usize, range: Range<usize>) -> String {
    let Range { start, end } = range;
    let prefix = &line[..start];
    format!(
        "{0: >6}:{1: <3} {2}{3}{4}",
        lineno.to_string().blue(),
        // 找到匹配项的起始位置，注意对汉字等非 ascii 字符，我们不能使用 prefix.len()
        // 这是一个 O(n) 的操作，会拖累效率，这里只是为了演示的效果
        (prefix.chars().count() + 1).to_string().cyan(),
        prefix,
        &line[start..end].red(),
        &line[end..]
    )
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn format_line_should_work() {
        let result = format_line("Hello, Tyr~", 1000, 7..10);
        let expected = format!(
            "{0: >6}:{1: <3} Hello, {2}~",
            "1000".blue(),
            "8".cyan(),
            "Tyr".red()
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn default_strategy_should_work() {
        let path = Path::new("src/main.rs");
        let input = b"hello world!\nhey Tyr!";
        let mut reader = BufReader::new(&input[..]);
        let pattern = Regex::new(r"he\w+").unwrap();
        let mut writer = Vec::new();
        default_strategy(path, &mut reader, &pattern, &mut writer).unwrap();
        let result = String::from_utf8(writer).unwrap();
        let expected = [
            String::from("src/main.rs"),
            format_line("hello world!", 1, 0..5),
            format_line("hey Tyr!\n", 2, 0..3),
        ];

        assert_eq!(result, expected.join("\n"));
    }
}

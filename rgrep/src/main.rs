/*
 * 首先是最简单的，给定一个字符串以及一个文件，打印出文件中所有包含该字符串的行：
 *
 * $ rgrep Hello a.txt
 * 55: Hello world. This is an exmaple text
 *
 * 然后放宽限制，允许用户提供一个正则表达式，来查找文件中所有包含该字符串的行：
 *
 * $ rgrep Hel[^\\s]+ a.txt
 * 55: Hello world. This is an exmaple text
 * 89: Help me! I need assistant!
 *
 * 如果这个也可以实现，那进一步放宽限制，允许用户提供一个正则表达式，来查找满足文件通配符的所有文件（你可以使用 globset 或者 glob 来处理通配符），比如：
 *
 * $ rgrep Hel[^\\s]+ a*.txt
 * a.txt
 *      55:1 Hello world. This is an exmaple text
 *      89:1 Help me! I need assistant!
 *      5:6 Use `Help` to get help.
 * abc.txt:
 *      100:1 Hello Tyr!
 *
 * 其中，冒号前面的数字是行号，后面的数字是字符在这一行的位置。
 *
 * 对于输出的结果，最好能把匹配的文字用不同颜色展示。
 *
 */

use std::fs;

use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use regex::Regex;

use glob::glob_with;
use glob::MatchOptions;

/// A rgrep for grep string from files
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
#[command(next_line_help = true)]
struct Cli {
    #[arg(short, long)]
    text: String,
    #[arg(short, long)]
    path: String,
}

// fn main() -> Result<()> {
//     let cli = Cli::parse();
//
//     let contents = fs::read_to_string(cli.path)?;
//
//     contents.lines().enumerate().for_each(|(row, line)| {
//         if let Some(col) = line.find(&cli.text) {
//             println!(
//                 "{}:{} {}{}{}",
//                 row,
//                 col,
//                 &line[..col],
//                 &line[col..col + cli.text.len()].red().bold(),
//                 &line[col + cli.text.len()..]
//             );
//         }
//     });
//     Ok(())
// }

// fn main() -> Result<()> {
//     let cli = Cli::parse();
//
//     let contents = fs::read_to_string(cli.path)?;
//
//     contents.lines().enumerate().for_each(|(row, line)| {
//         if let Some(re) = Regex::new(&cli.text.as_str()).unwrap().find(line){
//             let start = re.start();
//             let end = re.end();
//             println!(
//                 "{}:{} {}{}{}",
//                 row+1,
//                 start,
//                 &line[..start],
//                 &line[start..end].red().bold(),
//                 &line[end..]
//             );
//         };
//     });
//     Ok(())
// }

fn main() -> Result<()> {
    let cli = Cli::parse();

    let options = MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };
    glob_with(&cli.path, options).unwrap().for_each(|entry| {
        if let Ok(path) = entry {
            println!("{:?}", path.file_name().unwrap());
            let contents = fs::read_to_string(path).unwrap();
            contents.lines().enumerate().for_each(|(row, line)| {
                if let Some(re) = Regex::new(cli.text.as_str()).unwrap().find(line) {
                    let start = re.start();
                    let end = re.end();
                    println!(
                        "\t{}:{} {}{}{}",
                        (row + 1).to_string().green(),
                        start.to_string().blue(),
                        &line[..start],
                        &line[start..end].red().bold(),
                        &line[end..]
                    );
                };
            });
        }
    });
    Ok(())
}

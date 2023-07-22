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

use anyhow::Result;
use clap::Parser;
use rgrep::GrepConfig;

fn main() -> Result<()> {
    let cli = GrepConfig::parse();
    cli.match_with_default_strategy()?;

    Ok(())
}

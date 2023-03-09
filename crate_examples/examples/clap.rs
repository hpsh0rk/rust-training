use clap::{Parser, Subcommand, ValueEnum};

// 定义 httpie 的 CLI 的主入口，它包含若干个子命令
// 下面 /// 的注释是文档，clap 会将其作为 CLI 的帮助

/// A naive httpie implementation with Rust, can you imagine how easy it is?
#[derive(Parser, Debug)]
#[command(version = "1.0", author = "Sh0rk <sh0rk@qq.com>")]
// #[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
// #[command(next_line_help = true)]
struct Cli {
    #[arg(short, long, value_name = "Test_Name", default_value = "my_name")]
    name: Option<String>,
    #[command(subcommand)]
    command: Command,
}

// #[derive(Parser, Debug)]
#[derive(Subcommand, Debug)]
enum Command {
    Get(Get),
    Post(Post),
}

/// feed get with an url and we will retrieve the response for you
#[derive(Parser, Debug)]
struct Get {
    /// 对参数进行验证
    // #[arg(value_name="test_value")]
    #[arg(value_parser = my_value_parse)]
    value: String,
    /// Network port to use
    #[arg(value_parser = clap::value_parser!(u16).range(1..))]
    port: u16,

    /// HTTP 请求的 URL
    #[arg(short, long, value_name = "Test_URL")]
    url: Option<String>,
    #[arg(short, long, value_enum)]
    mode: Mode,
}

fn my_value_parse(s: &str) -> Result<String, String> {
    let error_pre = "start";
    if s.starts_with(error_pre) {
        Err(format!("Can't start with {}", error_pre))
    } else {
        Ok(s.to_string())
    }
}

#[derive(ValueEnum, Debug, Clone)]
enum Mode {
    Fast,
    Slow,
}

// post 子命令。需要输入一个 url，和若干个可选的 key=value，用于提供 json body

/// feed post with an url and optional key=value pairs. We will post the data
/// as JSON, and retrieve the response for you
#[derive(Parser, Debug)]
struct Post {
    /// HTTP 请求的 URL
    url: String,
    /// HTTP 请求的 body
    body: Vec<String>,
}

fn main() {
    let cli = Cli::parse();
    if let Some(name) = cli.name.as_deref() {
        println!("value of name: {name}")
    }
    println!("{:?}", cli);
}

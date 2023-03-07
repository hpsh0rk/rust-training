use std::{collections::HashMap, str::FromStr};

use anyhow::{anyhow, Result};
use clap::Parser;
use colored::Colorize;
use mime::Mime;
use reqwest::{
    header::{self, HeaderName, HeaderValue},
    Client, Response, Url,
};
use syntect::{
    easy::HighlightLines,
    highlighting::{Style, ThemeSet},
    parsing::SyntaxSet,
    util::{as_24_bit_terminal_escaped, LinesWithEndings},
};

// 定义 httpie 的 CLI 的主入口，它包含若干个子命令
// 下面 /// 的注释是文档，clap 会将其作为 CLI 的帮助

/// A naive httpie implementation with Rust, can you imagine how easy it is?
#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "Sh0rk <sh0rk@qq.com>")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    Get(Get),
    Post(Post),
}

/// feed get with an url and we will retrieve the response for you
#[derive(Parser, Debug)]
struct Get {
    /// HTTP 请求的 URL
    #[clap(parse(try_from_str = parse_url))]
    url: String,
    #[clap(parse(try_from_str = parse_custom_header))]
    headers: Vec<MyHeader>,
}

// post 子命令。需要输入一个 url，和若干个可选的 key=value，用于提供 json body

/// feed post with an url and optional key=value pairs. We will post the data
/// as JSON, and retrieve the response for you
#[derive(Parser, Debug)]
struct Post {
    /// HTTP 请求的 URL
    #[clap(parse(try_from_str = parse_url))]
    url: String,
    /// HTTP 请求的 body
    #[clap(parse(try_from_str = parse_kv_pair))]
    body: Vec<KvPair>,
}

#[derive(PartialEq, Debug)]
struct KvPair {
    k: String,
    v: String,
}

impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('=');
        let err = || anyhow!(format!("Failed to parse {}", s));
        Ok(Self {
            k: (split.next().ok_or_else(err)?).to_string(),
            v: (split.next().ok_or_else(err)?).to_string(),
        })
    }
}

#[derive(PartialEq, Debug)]
struct MyHeader {
    name: HeaderName,
    value: HeaderValue,
}

impl FromStr for MyHeader {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(':');
        let err = || anyhow!(format!("Failed to parse {}", s));
        let name = split.next().ok_or_else(err)?;
        let value = split.next().ok_or_else(err)?;

        Ok(Self {
            name: HeaderName::from_str(name)?,
            value: HeaderValue::from_str(value)?,
        })
    }
}

fn parse_url(s: &str) -> Result<String> {
    let _url: Url = s.parse()?;
    Ok(s.into())
}

fn parse_kv_pair(s: &str) -> Result<KvPair> {
    s.parse()
}

fn parse_custom_header(s: &str) -> Result<MyHeader> {
    s.parse()
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Opts::parse();
    let mut headers = header::HeaderMap::new();
    headers.insert("X-POWERED-BY", "Rust".parse().unwrap());
    headers.insert(header::USER_AGENT, "Rust Httpie".parse().unwrap());
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;
    let result = match opts.subcmd {
        SubCommand::Get(ref args) => get(client, args).await?,
        SubCommand::Post(ref args) => post(client, args).await?,
    };

    Ok(result)
}

async fn get(client: Client, args: &Get) -> Result<()> {
    // todo 实现什么trait能直接进行转换？
    let mut headers = header::HeaderMap::new();
    for pair in &args.headers {
        headers.insert(pair.name.clone(), pair.value.clone());
    }
    let resp = client.get(&args.url).headers(headers).send().await?;
    Ok(print_resp(resp).await?)
}
async fn post(client: Client, args: &Post) -> Result<()> {
    let mut body = HashMap::new();
    for pair in args.body.iter() {
        body.insert(&pair.k, &pair.v);
    }
    let resp = client.post(&args.url).json(&body).send().await?;
    Ok(print_resp(resp).await?)
}

async fn print_resp(resp: Response) -> Result<()> {
    let status = format!("{:?} {}", resp.version(), resp.status()).blue();
    println!("{}\n", status);

    for (name, value) in resp.headers() {
        println!("{}: {:?}", name.to_string().green(), value);
    }
    println!();

    let mime = get_content_type(&resp);
    let body = resp.text().await?;
    match mime {
        Some(v) if v == mime::APPLICATION_JSON => print_syntect(&body, "json"),
        Some(v) if v == mime::TEXT_HTML => print_syntect(&body, "html"),
        _ => println!("{}", body),
    }

    Ok(())
}

fn get_content_type(resp: &Response) -> Option<Mime> {
    resp.headers()
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().parse().unwrap())
}

fn print_syntect(s: &str, ext: &str) {
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let syntax = ps.find_syntax_by_extension(ext).unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);

    for line in LinesWithEndings::from(s) {
        let ranges: Vec<(Style, &str)> = h.highlight(line, &ps);
        let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
        print!("{}", escaped);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_url_works() {
        assert!(parse_url("abc").is_err());
        assert!(parse_url("http://abc.xyz").is_ok());
        assert!(parse_url("https://httpbin.org/post").is_ok());
    }

    #[test]
    fn parse_kv_pair_works() {
        assert!(parse_kv_pair("a").is_err());
        assert_eq!(
            parse_kv_pair("a=1").unwrap(),
            KvPair {
                k: "a".into(),
                v: "1".into(),
            }
        );
        assert_eq!(
            parse_kv_pair("b=").unwrap(),
            KvPair {
                k: "b".into(),
                v: "".into(),
            }
        );
    }

    #[test]
    fn parse_custom_header_works() {
        assert!(parse_custom_header("a").is_err());
        assert_eq!(
            parse_custom_header("a:1").unwrap(),
            MyHeader {
                name: HeaderName::from_str("a").unwrap(),
                value: HeaderValue::from_str("1").unwrap(),
            }
        );
        assert_eq!(
            parse_custom_header("b:").unwrap(),
            MyHeader {
                name: HeaderName::from_str("b").unwrap(),
                value: HeaderValue::from_str("").unwrap(),
            }
        );
    }
}

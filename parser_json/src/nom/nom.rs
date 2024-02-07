use std::collections::HashMap;

use nom::branch::alt;
use nom::bytes::complete::{escaped, tag, take_till1, take_while, take_while_m_n};
use nom::character::complete::{char, multispace0};
use nom::combinator::{cut, map, opt, peek, value};
use nom::error::context;
use nom::multi::separated_list0;
use nom::number::complete::double;
use nom::sequence::{delimited, preceded, separated_pair, terminated};
use nom::IResult;

#[derive(Debug, PartialEq)]
pub enum JsonValue<'a> {
    String(&'a str),
    Boolean(bool),
    Number(f64),
    Array(Vec<JsonValue<'a>>),
    Object(HashMap<&'a str, JsonValue<'a>>),
    Null,
}

// whitespace Json 空格解析（等价于 nom 内置函数 multispace0）
// fn whitespace(i: &str) -> IResult<&str, &str> {
//     let chars = " \t\r\n";
//     take_while(move |c| chars.contains(c))(i)
// }

// string 整个字符串解析
fn string(i: &str) -> IResult<&str, &str> {
    context(
        "string",
        preceded(char('\"'), cut(terminated(parse_str, char('\"')))),
    )(i)
}

// parse_str 单独字符串解析
fn parse_str(i: &str) -> IResult<&str, &str> {
    escaped(normal, '\\', escapable)(i)
}

// normal 普通字符解析
fn normal(i: &str) -> IResult<&str, &str> {
    take_till1(|c: char| c == '\\' || c == '"' || c.is_ascii_control())(i)
}

// escapable 转义字符解析
fn escapable(i: &str) -> IResult<&str, &str> {
    context(
        "escaped",
        alt((
            tag("\""),
            tag("\\"),
            tag("/"),
            tag("b"),
            tag("f"),
            tag("n"),
            tag("r"),
            tag("t"),
            hex,
        )),
    )(i)
}

// hex  十六进制字符解析
fn hex(i: &str) -> IResult<&str, &str> {
    context(
        "hex",
        preceded(
            peek(tag("u")),
            take_while_m_n(5, 5, |c: char| c.is_ascii_hexdigit() || c == 'u'),
        ),
    )(i)
}

// boolean 布尔数据类型解析
fn boolean(i: &str) -> IResult<&str, bool> {
    alt((value(true, tag("true")), value(false, tag("false"))))(i)
}

// null Null解析
fn null(i: &str) -> IResult<&str, JsonValue> {
    map(tag("null"), |_| JsonValue::Null)(i)
}

// array 数组解析
fn array(i: &str) -> IResult<&str, Vec<JsonValue>> {
    context(
        "array",
        delimited(
            tag("["),
            separated_list0(tag(","), delimited(multispace0, json_value, multispace0)),
            tag("]"),
        ),
    )(i)
}

// key_value kv格式解析
fn key_value(i: &str) -> IResult<&str, (&str, JsonValue)> {
    separated_pair(
        preceded(multispace0, string),
        cut(preceded(multispace0, char(':'))),
        json_value,
    )(i)
}

// object 对象格式解析
fn object(i: &str) -> IResult<&str, HashMap<&str, JsonValue>> {
    context(
        "object",
        preceded(
            char('{'),
            cut(terminated(
                map(
                    separated_list0(preceded(multispace0, char(',')), key_value),
                    |tuple_vec| tuple_vec.into_iter().map(|(k, v)| (k, v)).collect(),
                ),
                preceded(multispace0, char('}')),
            )),
        ),
    )(i)
}

// json_value JsonValue 解析
fn json_value(i: &str) -> IResult<&str, JsonValue> {
    context(
        "json value",
        delimited(
            multispace0,
            alt((
                map(string, |s| JsonValue::String(s)),
                map(double, JsonValue::Number),
                map(boolean, JsonValue::Boolean),
                null,
                map(array, JsonValue::Array),
                map(object, JsonValue::Object),
            )),
            multispace0,
        ),
    )(i)
}

fn root(i: &str) -> IResult<&str, JsonValue> {
    delimited(
        multispace0,
        alt((map(object, JsonValue::Object), map(array, JsonValue::Array))),
        opt(multispace0),
    )(i)
}

#[cfg(test)]
mod test_json {
    use super::*;

    #[test]
    fn test_parse_json() {
        let data = r#"
        {
            "a"  : 42,
            "b": [ "x", "y", 12 ] ,
            "c": { "hello" : "world"}
        } "#;

        println!(
            "will try to parse valid JSON data:\n\n**********\n{}\n**********\n",
            data
        );

        println!("parsing a valid file:\n{:#?}\n", root(data));

        assert_eq!(
            root(data),
            Ok((
                "",
                JsonValue::Object(HashMap::from([
                    ("a", JsonValue::Number(42.0)),
                    (
                        "b",
                        JsonValue::Array(vec![
                            JsonValue::String("x"),
                            JsonValue::String("y"),
                            JsonValue::Number(12.0)
                        ])
                    ),
                    (
                        "c",
                        JsonValue::Object(HashMap::from([("hello", JsonValue::String("world"))]))
                    )
                ]))
            ))
        )
    }
}

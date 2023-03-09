fn main() {
    let mut fmt = jsonxf::Formatter::pretty_printer();
    fmt.line_separator = String::from("\r\n");
    let a = fmt.format("{\"a\":1,\"b\":2}").unwrap();
    println!("{}", a);
}

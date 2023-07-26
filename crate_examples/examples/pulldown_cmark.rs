use pulldown_cmark::{html, Options, Parser};
// use termion::{color, style};

fn main() {
    // Markdown 文本
    let markdown_input = "
# Heading One
## Another Heading
Hello, this is **bold** and *italic* text.
    ";

    // 创建解析器
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(&markdown_input, options);

    // 输出 HTML
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    // 在终端上渲染 HTML
    // println!("{}{}{}",
    //          color::Fg(color::LightGreen),
    //          &html_output,
    //          style::Reset);

    // println!("{}", &html_output,);
    termimad::print_inline(&html_output);
}

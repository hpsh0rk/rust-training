use sqlparser::{dialect::GenericDialect, parser::Parser};

fn main() {
    let dialect = GenericDialect {}; // or AnsiDialect

    let sql = "SELECT a a1, b, 123, myfunc(b), * \
    FROM data_source \
    WHERE a > b AND b < 100 AND c BETWEEN 10 AND 20 \
    ORDER BY a DESC, b \
    LIMIT 50 OFFSET 10";

    let ast = Parser::parse_sql(&dialect, sql).unwrap();

    println!("{:#?}", ast);
}

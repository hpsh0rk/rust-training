use queryer::{TyrDialect, example_sql};
use sqlparser::parser::Parser;

fn main() {
    println!("{:#?}", Parser::parse_sql(&TyrDialect::default(), &example_sql()));
}

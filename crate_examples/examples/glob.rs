use glob::glob_with;
use glob::MatchOptions;

fn main() {
    let options = MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };
    for entry in glob_with("examples/*a*", options).unwrap() {
        if let Ok(path) = entry {
            println!("{:?}", path.display())
        }
    }
}

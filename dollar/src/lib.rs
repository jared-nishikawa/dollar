pub mod scanner;
pub mod parser;


pub fn validate(s: &str) -> std::result::Result<parser::Parser, parser::Error> {
    let p = parser::Parser::new(s)?;
    p.parse()
}

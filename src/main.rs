
fn main() {
    match dollar::validate("abc $def $$ some $exp $$ other exp $ another exp \\$ $$ \\$$ $$") {
        Ok(p) => println!("{:?}", p.nodes),
        Err(e) => println!("{}", e),
    }
}

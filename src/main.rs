use heraclitus_compiler::prelude::*;
mod rules;

fn main() {
    let code = "1 + 2";
    let rules = rules::get_rules();
    let mut cc = Compiler::new("Amber", rules);
    cc.load(code);
    if let Ok(tokens) = cc.tokenize() {
        println!("{tokens:?}");
    }
}

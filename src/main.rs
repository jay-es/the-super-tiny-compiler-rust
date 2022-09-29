mod parser;
mod tokenizer;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let tokens = tokenizer::tokenizer(input.trim()).unwrap();
    let _ast = parser::parser(tokens);
}

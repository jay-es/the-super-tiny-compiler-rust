mod parser;
mod tokenizer;
mod transformer;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let tokens = tokenizer::tokenizer(input.trim()).unwrap();
    let ast = parser::parser(tokens).unwrap();
    let new_ast = transformer::transformer(ast).unwrap();

    println!("{:?}", new_ast);
}

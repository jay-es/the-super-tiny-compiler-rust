mod code_generator;
mod parser;
mod tokenizer;
mod transformer;

fn compiler(input: &str) -> String {
    let tokens = tokenizer::tokenizer(input).unwrap();
    let ast = parser::parser(tokens).unwrap();
    let new_ast = transformer::transformer(ast).unwrap();
    code_generator::code_generator(new_ast)
}

#[test]
fn compiler_test() {
    let input = "(add 2 (subtract 4 2))";
    let excepted = "add(2, subtract(4, 2));".to_string();

    assert_eq!(compiler(input), excepted);
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    println!("{}", compiler(&input));
}

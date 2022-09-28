#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    ParenOpen,
    ParenClose,
    Name(String),
    String(String),
    Number(String),
}

pub fn tokenizer(input: &str) -> Result<Vec<Token>, String> {
    let chars = input.chars().collect::<Vec<char>>();
    let mut current = 0;
    let mut tokens: Vec<Token> = vec![];

    while current < input.len() {
        let char = chars[current];

        if char == '(' {
            tokens.push(Token::ParenOpen);
            current += 1;
            continue;
        }
        if char == ')' {
            tokens.push(Token::ParenClose);
            current += 1;
            continue;
        }

        if char.is_whitespace() {
            current += 1;
            continue;
        }

        if char.is_numeric() {
            let mut value = String::new();

            while chars[current].is_numeric() {
                value.push(chars[current]);
                current += 1;
            }

            tokens.push(Token::Number(value));
            continue;
        }

        if char == '"' {
            let mut value = String::new();
            current += 1;

            while chars[current] != '"' {
                value.push(chars[current]);
                current += 1;
            }

            tokens.push(Token::String(value));
            continue;
        }

        if char.is_alphabetic() {
            let mut value = String::new();

            while chars[current].is_alphabetic() {
                value.push(chars[current]);
                current += 1;
            }

            tokens.push(Token::Name(value));
            continue;
        }

        return Err(format!("I dont know what this character is: {}", char));
    }

    Ok(tokens)
}

#[cfg(test)]
mod tokenizer_test {
    use super::*;

    #[test]
    fn サンプル() {
        let input = "(add 2 (subtract 4 2))";
        let excepted = vec![
            Token::ParenOpen,
            Token::Name("add".to_string()),
            Token::Number("2".to_string()),
            Token::ParenOpen,
            Token::Name("subtract".to_string()),
            Token::Number("4".to_string()),
            Token::Number("2".to_string()),
            Token::ParenClose,
            Token::ParenClose,
        ];

        assert_eq!(tokenizer(input).unwrap(), excepted);
    }

    #[test]
    fn エラー() {
        let input = "(add 2 (subtract 4 * 2))";

        assert!(tokenizer(input).is_err());
    }
}

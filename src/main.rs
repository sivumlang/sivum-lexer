#![allow(unused, non_camel_case_types)]

fn main() {
    //From the arguments, I need a path
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        return close_program("Expected a path to a .sivum file.");
    }
    let path = args[1].clone();
    if !path.ends_with(".sivum") {
        return close_program("Expected a .sivum file.");
    }

    //I need to read a file
    let text = std::fs::read_to_string(path);
    let Ok(text) = text else {
        eprintln!("{}", text.unwrap_err());
        return close_program("Failed to read file");
    };

    //parse it into tokens
    let mut failures: Vec<Failure> = Vec::new();
    let mut tokens: Vec<Token> = Vec::new();
    let mut skip_point = 0;
    let chars: Vec<char> = text.chars().collect();

    for i in 0..chars.len() {
        if i < skip_point {
            continue;
        }
        let current_char = chars[i];
        if current_char.is_whitespace() {
            continue;
        }

        if let Some(sep) = Token::get_separator(current_char) {
            tokens.push(sep);
        } else {
            for j in (i + 1)..chars.len() {
                let next_char = chars[j];
                let word = &chars[i..=j];

                if Token::get_separator(next_char).is_some() {
                    skip_point = j;
                    let word: String = word.into_iter().collect();
                    //Parse word
                    //Here is where I need to determine between the following
                    //ID_VAR ID_FUN and all the LITS
                    //Check if whole word is made up of 0..9

                    if word.chars().all(|x| x.is_numeric()) {
                        let num = word.parse();
                        if let Ok(num) = num {
                            tokens.push(Token::LIT_NUM(num));
                        } else {
                            failures.push(Failure {
                                char_index: i,
                                message: String::from("Could not parse numeric."),
                            })
                        }
                        break;
                    }

                    //Check if word starts with " and ends with "
                    if word.starts_with('"') && word.ends_with('"') {
                        tokens.push(Token::LIT_STR(word));
                        break;
                    }

                    //Otherwise it is some kind of ID
                    //If it is not preceeded with a dot, it is a variable
                    //Otherwise it is a method

                } else {
                    //Check if word is keyword
                    if let Some(kw) = Token::get_keyword(word) {
                        tokens.push(kw);
                        skip_point = j + 1;
                        break;
                    }
                }
            }
        }
    }

    //return a list of parsed tokens
    println!("Hello, world!");
}

fn close_program(message: &str) {
    eprintln!("{}", message);
}

struct Failure {
    char_index: usize,
    message: String,
}

enum Token {
    BAD,

    ID_VAR(String),
    ID_FUN(String),

    LIT_NUM(u64),
    LIT_STR(String),

    DOT,

    SEP_OP,
    SEP_CP,
    SEP_OCB,
    SEP_CCB,
    SEP_SEMI,
    SEP_COL,

    KW_FOR,
    KW_IF,
    KW_ELSE,
    KW_CONT,
    KW_RET,
    KW_FUN,
}

impl Token {
    fn get_separator(c: char) -> Option<Token> {
        match c {
            '.' => Some(Token::DOT),
            '(' => Some(Token::SEP_OP),
            ')' => Some(Token::SEP_CP),
            '{' => Some(Token::SEP_OCB),
            '}' => Some(Token::SEP_CCB),
            ';' => Some(Token::SEP_SEMI),
            ':' => Some(Token::SEP_COL),
            _ => None,
        }
    }

    fn get_keyword(chars: &[char]) -> Option<Token> {
        let s: String = chars.iter().collect();
        match s.as_str() {
            "for" => Some(Token::KW_FOR),
            "if" => Some(Token::KW_IF),
            "else" => Some(Token::KW_ELSE),
            "continue" => Some(Token::KW_CONT),
            "return" => Some(Token::KW_RET),
            "fun" => Some(Token::KW_FUN),
            _ => None,
        }
    }
}

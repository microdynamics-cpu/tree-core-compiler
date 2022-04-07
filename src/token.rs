use std::process::exit;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Num,
    Plus,
    Minus,
    Mul,
    Div,
}

impl From<char> for TokenType {
    fn from(c: char) -> Self {
        match c {
            '+' => TokenType::Plus,
            '-' => TokenType::Minus,
            '*' => TokenType::Mul,
            '/' => TokenType::Div,
            e => panic!("unknow Token type: {}", e),
        }
    }
}

impl Default for TokenType {
    fn default() -> Self {
        TokenType::Num
    }
}

#[derive(Default, Debug)]
pub struct Token {
    pub ty: TokenType, // token type
    pub val: i32,      // number literal
    pub input: String, // token string (for error reporting)
}

pub fn tokenize(mut p: String) -> Vec<Token> {
    // tokenized input is stored to this vec
    let mut tokens: Vec<Token> = vec![];

    let org = p.clone();
    while let Some(c) = p.chars().nth(0) {
        // skip whitespce
        if c.is_whitespace() {
            p = p.split_off(1); // p++
            continue;
        }

        match c {
            '+' | '-' | '*' | '/' => {
                let token = Token {
                    ty: TokenType::from(c),
                    input: org.clone(),
                    ..Default::default()
                };
                p = p.split_off(1); // p++
                tokens.push(token);
                continue;
            }
            _ => (),
        }

        if c.is_ascii_digit() {
            let (n, remaining) = strtol(&p);
            p = remaining;
            let token = Token {
                ty: TokenType::Num,
                input: org.clone(),
                val: n.unwrap() as i32,
            };
            tokens.push(token);
            continue;
        }

        eprint!("cannot tokenize: {}\n", p);
        exit(1);
    }

    // for v in tokens.iter() {
    //     println!("[tk] type: {:?}, val: {}, input: {}", v.ty, v.val, v.input);
    // }

    return tokens;
}

pub fn strtol(s: &String) -> (Option<i64>, String) {
    if s.is_empty() {
        return (None, s.clone());
    }

    let mut pos = 0;
    let mut remaining = s.clone();
    let len = s.len();

    while len != pos {
        if !s.chars().nth(pos).unwrap().is_ascii_digit() {
            break;
        }
        pos += 1;
    }

    if len == pos {
        (Some(remaining.parse::<i64>().unwrap()), "".into())
    } else {
        let t: String = remaining.drain(..pos).collect();
        (Some(t.parse::<i64>().unwrap()), remaining)
    }
}

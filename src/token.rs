#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Num(i32),
    Ident(String),
    Plus,
    Minus,
    Mul,
    Div,
    Return,
    Semicolon,
    Equal,
}

impl From<char> for TokenType {
    fn from(c: char) -> Self {
        match c {
            '+' => TokenType::Plus,
            '-' => TokenType::Minus,
            '*' => TokenType::Mul,
            '/' => TokenType::Div,
            ';' => TokenType::Semicolon,
            '=' => TokenType::Equal,
            e => panic!("unknow Token type: {}", e),
        }
    }
}

impl From<String> for TokenType {
    fn from(s: String) -> Self {
        match &*s {
            "return" => TokenType::Return,
            name => TokenType::Ident(name.to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub ty: TokenType, // token type
    pub input: String, // token string (for error reporting)
}

pub fn scan(mut p: String) -> Vec<Token> {
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
            '+' | '-' | '*' | '/' | ';' | '=' => {
                let token = Token {
                    ty: TokenType::from(c),
                    input: org.clone(),
                };
                p = p.split_off(1); // p++
                tokens.push(token);
                continue;
            }
            _ => (),
        }

        // keyword or identifier
        if c.is_alphabetic() || c == '_' {
            let mut name = String::new();
            while let Some(c2) = p.chars().nth(0) {
                if c2.is_alphabetic() || c2.is_ascii_digit() || c2 == '_' {
                    name.push(c2);
                    p = p.split_off(1); // p++
                    continue;
                }
                break;
            }
            let token = Token {
                ty: TokenType::from(name),
                input: org.clone(),
            };
            tokens.push(token);
            continue;
        }


        if c.is_ascii_digit() {
            let n = strtol(&mut p);
            let token = Token {
                ty: TokenType::Num(n.unwrap() as i32),
                input: org.clone(),
            };
            tokens.push(token);
            continue;
        }

        panic!("cannot tokenize: {}\n", p);
    }

    // for v in tokens.iter() {
    //     println!("[tk] type: {:?}, input: {}", v.ty, v.input);
    // }

    tokens
}

pub fn tokenize(p: String) -> Vec<Token> {
    scan(p)
}

pub fn strtol(s: &mut String) -> Option<i64> {
    if s.is_empty() {
        return None;
    }

    let mut pos = 0;
    for c in s.chars() {
        if !c.is_ascii_digit() {
            break;
        }
        pos += 1;
    }

    let t: String = s.drain(..pos).collect();
    Some(t.parse::<i64>().unwrap())
}

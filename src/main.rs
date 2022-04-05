use std::env;

use zodiac::strtol;

use std::process::exit;

enum TokenType {
    Num, // number literal
}

#[derive(Default, Debug)]
struct Token {
    ty: i32, // token type
    val: i32, // number literal
    input: String, // token string (for error reporting)
}

fn tokenize(mut p: String) -> Vec<Token> {
    // tokenized input is stored to this vec.
    let mut tokens: Vec<Token> = vec![];

    let org = p.clone();
    while let Some(c) = p.chars().nth(0) {
        // skip whitespce
        if c.is_whitespace() {
            p = p.split_off(1); // p++
            continue;
        }

        // + or -
        if c == '+' || c == '-' {
            let token = Token {
                ty: c as i32,
                input: org.clone(),
                ..Default::default()
            };
            p = p.split_off(1); // p++
            tokens.push(token);
            continue;
        }

        // Number
        if c.is_ascii_digit() {
            let (n, mut remaining) = strtol(&p);
            p = remaining;
            let token = Token {
                ty: TokenType::Num as i32,
                input: org.clone(),
                val: n.unwrap() as i32,
            };
            tokens.push(token);
            continue;
        }

        eprint!("cannot tokenize: {}\n", p);
        exit(1);
    }
    return tokens;
}


fn fail(tokens: &Vec<Token>, i: usize) {
    eprint!("unexpected character: {:?}\n", tokens[i]);
    exit(1);
}

fn main() {
    let mut args = env::args();
    if args.len() != 2 {
        eprint!("Usage: 9cc <code>\n");
        return;
    }

    let tokens = tokenize(args.nth(1).unwrap());

    // Print the prologue
    print!(".intel_syntax noprefix\n");
    print!(".global main\n");
    print!("main:\n");


    // verify that the given expression starts with a number,
    // and then emit the first `mov` instruction.
    if tokens[0].ty != TokenType::Num as i32 {
        fail(&tokens, 0);
    }
    print!("  mov rax, {}\n", tokens[0].val);

    // emit assembly as we consume the sequence of `+ <number>`
    // or `- <number>`.
    let mut i = 1;
    while i != tokens.len() {
        if tokens[i].ty == '+' as i32 {
            i += 1;
            if tokens[i].ty != TokenType::Num as i32 {
                fail(&tokens, i);
            }
            print!("  add rax, {}\n", tokens[i].val);
            i += 1;
            continue;
        }

        if tokens[i].ty == '-' as i32 {
            i += 1;
            if tokens[i].ty != TokenType::Num as i32 {
                fail(&tokens, i);
            }
            print!("  sub rax, {}\n", tokens[i].val);
            i += 1;
            continue;
        }

        fail(&tokens, i);
    }

    print!("  ret\n");
    return;
}

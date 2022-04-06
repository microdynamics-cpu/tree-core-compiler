use std::env;

// extern crate zodiac;
use zodiac::codegen::gen_x86;
use zodiac::ir::gen_ir;
use zodiac::parse::Node;
use zodiac::regalloc::alloc_regs;
use zodiac::token::tokenize;

fn main() {
    let mut args = env::args();
    if args.len() != 2 {
        eprint!("Usage: 9cc <code>\n");
        return;
    }

    // tokenize and parse
    let tokens = tokenize(args.nth(1).unwrap());
    let node = Node::expr(tokens);
    let irv = gen_ir(node);
    let irv_alloced = alloc_regs(irv);
    // Print the prologue
    print!(".intel_syntax noprefix\n");
    print!(".global main\n");
    print!("main:\n");

    gen_x86(irv_alloced);
    return;
}

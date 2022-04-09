use std::env;

use zodiac::codegen::{gen_x86, gen_riscv};
use zodiac::ir::gen_ir;
use zodiac::parse::Node;
use zodiac::regalloc::alloc_regs;
use zodiac::token::tokenize;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprint!("Usage: zodiac -[x|r] <code>\n");
        return;
    }

    let data = &args[1];
    let arch = &args[2];
    // println!("arch: {}, data:{}", arch, data);
    let tokens = tokenize(data.to_string());
    let node = Node::parse(&tokens);
    let mut irv = gen_ir(node);
    alloc_regs(&mut irv);

    if arch == "x" {    
        gen_x86(irv);
    } else if arch == "r" {
        gen_riscv(irv);
    } else {
        eprint!("error arch param\n");
        return;
    }

    return;
}

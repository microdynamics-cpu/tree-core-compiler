use crate::ir::{IRType, IR};
use crate::REGS;

pub fn gen_x86(irv: Vec<IR>) {
    use self::IRType::*;

    // print the prologue
    print!(".intel_syntax noprefix\n");
    print!(".global main\n");
    print!("main:\n");
    for ir in irv.clone() {
        match ir.op {
            IMM => print!("  mov {}, {}\n", REGS[ir.lhs], ir.rhs),
            MOV => print!("  mov {}, {}\n", REGS[ir.lhs], REGS[ir.rhs]),
            RETURN => {
                print!("  mov rax, {}\n", REGS[ir.lhs]);
                print!("  ret\n");
            }
            ADD => print!("  add {}, {}\n", REGS[ir.lhs], REGS[ir.rhs]),
            SUB => print!("  sub {}, {}\n", REGS[ir.lhs], REGS[ir.rhs]),
            MUL => {
                print!("  mov rax, {}\n", REGS[ir.rhs]);
                print!("  mul {}\n", REGS[ir.lhs]);
                print!("  mov {}, rax\n", REGS[ir.lhs]);
            }
            DIV => {
                print!("  mov rax, {}\n", REGS[ir.lhs]);
                print!("  cqo\n");
                print!("  div {}\n", REGS[ir.rhs]);
                print!("  mov {}, rax\n", REGS[ir.lhs]);
            }
            NOP | KILL => (),
        }
    }
}

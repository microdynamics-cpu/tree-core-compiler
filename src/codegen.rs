use crate::ir::{IRType, IR};
use crate::{REGS_X86, REGS_RISCV};

pub fn gen_x86(irv: Vec<IR>) {
    use self::IRType::*;

    // print the prologue
    print!(".intel_syntax noprefix\n");
    print!(".global main\n");
    print!("main:\n");
    for ir in irv.clone() {
        match ir.op {
            IMM => print!("  mov {}, {}\n", REGS_X86[ir.lhs], ir.rhs),
            MOV => print!("  mov {}, {}\n", REGS_X86[ir.lhs], REGS_X86[ir.rhs]),
            RETURN => {
                print!("  mov rax, {}\n", REGS_X86[ir.lhs]);
                print!("  ret\n");
            }
            ADD => print!("  add {}, {}\n", REGS_X86[ir.lhs], REGS_X86[ir.rhs]),
            SUB => print!("  sub {}, {}\n", REGS_X86[ir.lhs], REGS_X86[ir.rhs]),
            MUL => {
                print!("  mov rax, {}\n", REGS_X86[ir.rhs]);
                print!("  mul {}\n", REGS_X86[ir.lhs]);
                print!("  mov {}, rax\n", REGS_X86[ir.lhs]);
            }
            DIV => {
                print!("  mov rax, {}\n", REGS_X86[ir.lhs]);
                print!("  cqo\n");
                print!("  div {}\n", REGS_X86[ir.rhs]);
                print!("  mov {}, rax\n", REGS_X86[ir.lhs]);
            }
            NOP | KILL => (),
        }
    }
}

pub fn gen_riscv(irv: Vec<IR>) {
    use self::IRType::*;

    println!(".text");
    println!(".global main");
    println!("main:");
    for ir in irv.clone() {
        match ir.op {
            IMM => print!("  li {}, {}\n", REGS_RISCV[ir.lhs], ir.rhs),
            MOV => print!("  mv {}, {}\n", REGS_RISCV[ir.lhs], REGS_RISCV[ir.rhs]),
            RETURN => {
                print!("  mv a0, {}\n", REGS_RISCV[ir.lhs]);
                print!("  ret\n");
            }
            ADD => print!("  add {}, {}, {}\n", REGS_RISCV[ir.lhs], REGS_RISCV[ir.lhs], REGS_RISCV[ir.rhs]),
            SUB => print!("  sub {}, {}, {}\n", REGS_RISCV[ir.lhs], REGS_RISCV[ir.lhs], REGS_RISCV[ir.rhs]),
            MUL => print!("  mul {}, {}, {}\n", REGS_RISCV[ir.lhs], REGS_RISCV[ir.lhs], REGS_RISCV[ir.rhs]),
            DIV => print!("  div {}, {}, {}\n", REGS_RISCV[ir.lhs], REGS_RISCV[ir.lhs], REGS_RISCV[ir.rhs]),
            NOP | KILL => (),
        }
    }
}

use std::sync::Mutex;
use crate::ir::{IRType, IR};
use crate::{REGS_X86, REGS_RISCV};

lazy_static!{
    static ref LAB_NUM: Mutex<usize> = Mutex::new(0);
}

fn gen_label() -> String {
    let label = format!(".L{}", *LAB_NUM.lock().unwrap());
    *LAB_NUM.lock().unwrap() += 1;
    return label;
}

pub fn gen_x86(irv: Vec<IR>) {
    use self::IRType::*;
    let ret = gen_label();

    // print the prologue
    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");
    println!("  push rbp");
    println!("  mov rbp, rsp");
    for ir in irv.clone() {
        let lhs = ir.lhs.unwrap();
        match ir.op {
            IMM => println!("  mov {}, {}", REGS_X86[lhs], ir.rhs.unwrap()),
            MOV => println!("  mov {}, {}", REGS_X86[lhs], REGS_X86[ir.rhs.unwrap()]),
            RETURN => {
                println!("  mov rax, {}", REGS_X86[lhs]);
                println!("  jmp {}", ret);
            }
            ALLOCA => {
                if ir.rhs.is_some() {
                    println!("  sub rsp, {}", ir.rhs.unwrap());
                }
                println!("  mov {}, rsp", REGS_X86[lhs]);
            }
            LOAD => println!("  mov {}, [{}]", REGS_X86[lhs], REGS_X86[ir.rhs.unwrap()]),
            STORE => println!("  mov [{}], {}", REGS_X86[lhs], REGS_X86[ir.rhs.unwrap()]),
            ADD => println!("  add {}, {}", REGS_X86[lhs], REGS_X86[ir.rhs.unwrap()]),
            SUB => println!("  sub {}, {}", REGS_X86[lhs], REGS_X86[ir.rhs.unwrap()]),
            MUL => {
                println!("  mov rax, {}", REGS_X86[ir.rhs.unwrap()]);
                println!("  mul {}", REGS_X86[lhs]);
                println!("  mov {}, rax", REGS_X86[lhs]);
            }
            DIV => {
                println!("  mov rax, {}", REGS_X86[lhs]);
                println!("  cqo");
                println!("  div {}", REGS_X86[ir.rhs.unwrap()]);
                println!("  mov {}, rax", REGS_X86[lhs]);
            }
            NOP | KILL => (),
        }
    }

    println!("{}:", ret);
    println!("  mov rsp, rbp");
    println!("  mov rsp, rbp");
    println!("  pop rbp");
    println!("  ret");
}

pub fn gen_riscv(irv: Vec<IR>) {
    use self::IRType::*;

    println!(".text");
    println!(".global main");
    println!("main:");
    for ir in irv.clone() {
        let lhs = ir.lhs.unwrap();
        match ir.op {
            IMM => println!("  li {}, {}", REGS_RISCV[lhs], ir.rhs.unwrap()),
            MOV => println!("  mv {}, {}", REGS_RISCV[lhs], REGS_RISCV[ir.rhs.unwrap()]),
            RETURN => {
                println!("  mv a0, {}", REGS_RISCV[lhs]);
                println!("  ret");
            }
            ALLOCA | LOAD | STORE => (),
            ADD => println!("  add {}, {}, {}", REGS_RISCV[lhs], REGS_RISCV[lhs], REGS_RISCV[ir.rhs.unwrap()]),
            SUB => println!("  sub {}, {}, {}", REGS_RISCV[lhs], REGS_RISCV[lhs], REGS_RISCV[ir.rhs.unwrap()]),
            MUL => println!("  mul {}, {}, {}", REGS_RISCV[lhs], REGS_RISCV[lhs], REGS_RISCV[ir.rhs.unwrap()]),
            DIV => println!("  div {}, {}, {}", REGS_RISCV[lhs], REGS_RISCV[lhs], REGS_RISCV[ir.rhs.unwrap()]),
            NOP | KILL => (),
        }
    }
}

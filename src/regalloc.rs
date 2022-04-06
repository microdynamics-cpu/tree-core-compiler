use ir::{IRType, IR};
use REGS_N;

use std::sync::Mutex;

lazy_static! {
    static ref USED: Mutex<[bool; REGS_N]> = Mutex::new([false; REGS_N]);
    static ref REG_MAP: Mutex<Vec<Option<usize>>> = Mutex::new(vec![None]);
}

fn used_get(i: usize) -> bool {
    USED.lock().unwrap()[i]
}

fn used_set(i: usize, val: bool) {
    USED.lock().unwrap()[i] = val;
}

fn reg_map_get(i: usize) -> Option<usize> {
    REG_MAP.lock().unwrap().get(i).cloned().unwrap()
}

fn reg_map_set(i: usize, val: usize) {
    REG_MAP.lock().unwrap()[i] = Some(val);
}

fn alloc(ir_reg: usize) -> usize {
    if let Some(r) = reg_map_get(ir_reg) {
        assert!(used_get(r));
        return r;
    }

    for i in 0..REGS_N {
        if used_get(i) {
            continue;
        }
        used_set(i, true);
        reg_map_set(ir_reg, i);
        return i;
    }
    panic!("register exhauseted");
}

fn kill(r: usize) {
    assert!(used_get(r));
    used_set(r, false);
}

pub fn alloc_regs(irv: Vec<IR>) -> Vec<IR> {
    use self::IRType::*;
    let mut new: Vec<IR> = vec![];
    let irv_len = irv.len();

    unsafe {
        REG_MAP.lock().unwrap().set_len(irv_len);
    }

    for i in 0..irv_len {
        let mut ir = irv[i].clone();
        match ir.op {
            IMM => ir.lhs = alloc(ir.lhs),
            RETURN => kill(reg_map_get(ir.lhs).unwrap()),
            KILL => {
                kill(reg_map_get(ir.lhs).unwrap());
                ir.op = NOP;
            }
            ADD | SUB | MOV => {
                ir.lhs = alloc(ir.lhs);
                ir.rhs = alloc(ir.rhs);
            }
            op => panic!("unknow operator: {:?}", op),
        }
        new.push(ir);
    }
    return new;
}
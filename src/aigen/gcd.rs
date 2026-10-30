//! Euclid 减法版 GCD：输入两个正整数，输出最大公约数。
//!
//! 这是“真实控制流”的例子。按现有后端，`JUMP(..., Breg::Label(...))`
//! 会被 `build_single` 拒绝，所以这个程序目前生成不出来；留着它正好暴露
//! 标签跳转这个最影响可用性的缺口。

use crate::aigen::dsl::*;
use crate::backends::symphony::AluOp::Sub;
use crate::backends::symphony::CondOp::{Jb, Je};
use crate::backends::symphony::Regs::*;
use crate::Label;

const GCD_LOOP: Label = Label("gcd_loop");
const GCD_LESS: Label = Label("gcd_less");
const GCD_DONE: Label = Label("gcd_done");
const GCD_END: Label = Label("gcd_end");

pub fn build() -> Result<String, &'static str> {
    crate::asm_program! {
        gcd_main: [
            input(R1),
            input(R2),
            jmp(GCD_LOOP),
        ],
        gcd_loop: [
            cmp(R1, reg(R2)),
            branch(Je, GCD_DONE),
            branch(Jb, GCD_LESS),
            alu(Sub, R1, R1, reg(R2)),
            jmp(GCD_LOOP),
        ],
        gcd_less: [
            alu(Sub, R2, R2, reg(R1)),
            jmp(GCD_LOOP),
        ],
        gcd_done: [
            out(R1),
        ],
        gcd_end: [
            jmp(GCD_END),
        ],
    }
}

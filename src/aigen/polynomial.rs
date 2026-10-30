//! Horner 形式求多项式：3x^3 + 2x^2 + 5x + 7，取 x = 2。
//!
//! 没有乘法指令，所以用 `lsl` 乘 2。顺便测试立即数 ALU 和寄存器移位。

use crate::aigen::dsl::*;
use crate::backends::symphony::AluOp::{Add, Lsl};
use crate::backends::symphony::Regs::*;

pub fn build() -> Result<String, &'static str> {
    let body = vec![
        li(R1, 1), // 移位量：x = 2，所以每次左移 1 位
        li(R2, 3), // ((3 * x + 2) * x + 5) * x + 7
        alu(Lsl, R2, R2, reg(R1)),
        alu(Add, R2, R2, imm(2)),
        alu(Lsl, R2, R2, reg(R1)),
        alu(Add, R2, R2, imm(5)),
        alu(Lsl, R2, R2, reg(R1)),
        alu(Add, R2, R2, imm(7)),
        out(R2),
    ];

    build_blocks(vec![block("horner_shift", body)])
}

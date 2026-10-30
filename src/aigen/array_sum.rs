//! 数组求和：先用寄存器地址 store，再用立即数地址 load。
//!
//! 这个例子主要观察 Load/Store 的寄存器模式与立即数模式在生成器里长什么样。

use crate::aigen::dsl::*;
use crate::backends::symphony::AluOp::Add;
use crate::backends::symphony::Regs::*;

pub fn build() -> Result<String, &'static str> {
    const BASE: u16 = 256;
    let values = [10u16, 20, 30, 40, 50];

    let mut body = vec![li(R10, BASE), li(R3, 0)];

    for (i, value) in values.iter().enumerate() {
        body.push(alu(Add, R1, R10, imm(i as u16))); // r1 = BASE + i
        body.push(li(R4, *value));
        body.push(store8(R1, R4)); // store8 [r1], r4
    }

    for i in 0..values.len() {
        body.push(load8(R2, imm(BASE + i as u16))); // load8i r2, BASE+i
        body.push(alu(Add, R3, R3, reg(R2))); // sum += value
    }

    body.push(out(R3));

    build_blocks(vec![block("sum_array", body)])
}

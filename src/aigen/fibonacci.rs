//! 不依赖跳转的 Fibonacci：用 Rust 的 for 循环把 10 项展开成直线指令。
//!
//! 这是一个“最小可运行”示例，用来观察 ALU 立即数、寄存器运算和 out 的生成效果。

use crate::aigen::dsl::*;
use crate::backends::symphony::AluOp::Add;
use crate::backends::symphony::Regs::*;

pub fn build() -> Result<String, &'static str> {
    let mut body = vec![li(R1, 0), li(R2, 1)];

    for _ in 0..10 {
        body.push(out(R1));
        body.push(alu(Add, R3, R1, reg(R2)));
        body.push(mov(R1, R2));
        body.push(mov(R2, R3));
    }

    build_blocks(vec![block("fib_unrolled", body)])
}

//! 冒泡排序：初始化内存、排序、再输出。
//!
//! 这个例子把内存读写、嵌套循环、条件分支都用到了。和 gcd 一样，它目前
//! 会卡在 `Breg::Label` 上，所以属于“想这么写，但生成器还没支持”的示例。

use crate::aigen::dsl::*;
use crate::backends::symphony::AluOp::{Add, Sub};
use crate::backends::symphony::CondOp::{Jae, Jbe};
use crate::backends::symphony::Regs::*;
use crate::Label;

const BS_OUTER: Label = Label("bs_outer");
const BS_INNER: Label = Label("bs_inner");
const BS_NO_SWAP: Label = Label("bs_no_swap");
const BS_NEXT_OUTER: Label = Label("bs_next_outer");
const BS_SORTED: Label = Label("bs_sorted");
const BS_PRINT: Label = Label("bs_print");
const BS_END: Label = Label("bs_end");

pub fn build() -> Result<String, &'static str> {
    const BASE: u16 = 256;
    let data = [5u16, 3, 8, 1, 9, 2];

    let mut init = vec![li(R10, BASE), li(R11, data.len() as u16)];
    for (i, value) in data.iter().enumerate() {
        init.push(alu(Add, R3, R10, imm(i as u16)));
        init.push(li(R4, *value));
        init.push(store8(R3, R4));
    }
    init.push(li(R1, 0)); // i = 0
    init.push(jmp(BS_OUTER));

    let blocks = vec![
        block("bs_main", init),
        block(
            "bs_outer",
            vec![
                // if i >= n - 1: 排序结束
                alu(Sub, R6, R11, imm(1)),
                cmp(R1, reg(R6)),
                branch(Jae, BS_SORTED),
                // j = 0，然后直接落到 bs_inner
                li(R2, 0),
            ],
        ),
        block(
            "bs_inner",
            vec![
                // limit = n - 1 - i
                alu(Sub, R6, R11, imm(1)),
                alu(Sub, R6, R6, reg(R1)),
                cmp(R2, reg(R6)),
                branch(Jae, BS_NEXT_OUTER),
                // addr = BASE + j
                alu(Add, R3, R10, reg(R2)),
                load8(R4, reg(R3)),
                alu(Add, R8, R3, imm(1)),
                load8(R5, reg(R8)),
                cmp(R4, reg(R5)),
                branch(Jbe, BS_NO_SWAP),
                // data[j] > data[j + 1]，交换
                store8(R3, R5),
                store8(R8, R4),
            ],
        ),
        block(
            "bs_no_swap",
            vec![alu(Add, R2, R2, imm(1)), jmp(BS_INNER)],
        ),
        block(
            "bs_next_outer",
            vec![alu(Add, R1, R1, imm(1)), jmp(BS_OUTER)],
        ),
        block("bs_sorted", vec![li(R1, 0)]),
        block(
            "bs_print",
            vec![
                cmp(R1, reg(R11)),
                branch(Jae, BS_END),
                alu(Add, R3, R10, reg(R1)),
                load8(R4, reg(R3)),
                out(R4),
                alu(Add, R1, R1, imm(1)),
                jmp(BS_PRINT),
            ],
        ),
        block("bs_end", vec![jmp(BS_END)]),
    ];

    build_blocks(blocks)
}

//! 一层很薄的 Rust-DSL 辅助函数。
//!
//! `symphony_playground.rs` 里的写法是直接构造 `SymphonyISA` 变体；这里把
//! 常用的“立即数/寄存器/跳转/读写内存”包装成小函数，方便在 aigen 里写程序。
//! 注意：这只是调用点上的语法糖，没有改变 `backends::symphony` 的任何行为。

use crate::backends::symphony::{
    AluOp, Breg, CondOp, ImdNumber, LoadOp, Regs, StoreOp, Symphony, SymphonyISA,
};
use crate::{Block, Label};

pub fn imm(value: u16) -> Breg {
    Breg::Imd(ImdNumber(value))
}

pub fn reg(register: Regs) -> Breg {
    Breg::Reg(register)
}

pub fn lbl(label: Label) -> Breg {
    Breg::Label(label)
}

/// `addi dst, zr, value`：把立即数装进寄存器。
pub fn li(dst: Regs, value: u16) -> SymphonyISA {
    SymphonyISA::ALU(AluOp::Add, dst, Regs::Zr, imm(value))
}

/// `add dst, src, zr`：寄存器之间搬移。
pub fn mov(dst: Regs, src: Regs) -> SymphonyISA {
    SymphonyISA::ALU(AluOp::Add, dst, src, reg(Regs::Zr))
}

pub fn alu(op: AluOp, dst: Regs, a: Regs, b: Breg) -> SymphonyISA {
    SymphonyISA::ALU(op, dst, a, b)
}

/// `cmp` 的 D 槽固定是 flags，A 是第一个操作数，B 是第二个操作数。
pub fn cmp(a: Regs, b: Breg) -> SymphonyISA {
    SymphonyISA::ALU(AluOp::Cmp, Regs::Flags, a, b)
}

pub fn input(register: Regs) -> SymphonyISA {
    SymphonyISA::Input(register)
}

pub fn out(register: Regs) -> SymphonyISA {
    SymphonyISA::Output(reg(register))
}

pub fn outi(value: u16) -> SymphonyISA {
    SymphonyISA::Output(imm(value))
}

pub fn jmp(target: Label) -> SymphonyISA {
    SymphonyISA::JUMP(CondOp::Jmp, lbl(target))
}

pub fn branch(op: CondOp, target: Label) -> SymphonyISA {
    SymphonyISA::JUMP(op, lbl(target))
}

pub fn load8(dst: Regs, addr: Breg) -> SymphonyISA {
    SymphonyISA::Load(LoadOp::Load8, dst, addr)
}

/// 现有 `Store(StoreOp, Regs, Breg)` 的签名是“地址寄存器 + 值”。
pub fn store8(addr: Regs, value: Regs) -> SymphonyISA {
    SymphonyISA::Store(StoreOp::Store8, addr, reg(value))
}

pub fn block(name: &'static str, body: Vec<SymphonyISA>) -> Block<Symphony> {
    Block {
        label: Label(name),
        ir: Symphony(body),
    }
}

pub fn build_blocks(blocks: Vec<Block<Symphony>>) -> Result<String, &'static str> {
    let mut asm = String::new();
    for block in blocks {
        let text = block.build().map_err(|_| "backend returned Err")?;
        asm.push_str(&text);
        if !asm.ends_with('\n') {
            asm.push('\n');
        }
        asm.push('\n');
    }
    Ok(asm)
}

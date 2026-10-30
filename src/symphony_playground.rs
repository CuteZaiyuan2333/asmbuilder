use crate::*;
use crate::backends::symphony::*;
use crate::backends::symphony::AluOp::*;
use crate::backends::symphony::Breg::*;
use crate::backends::symphony::LoadOp::*;
use crate::backends::symphony::Regs::*;
use crate::backends::symphony::StoreOp::*;
use crate::backends::symphony::SymphonyISA::*;

pub fn test(){
    let tree = vec![
        ALU(Add, R1, Zr, Imd(ImdNumber(1))),
        ALU(Add, R2, Zr, Imd(ImdNumber(1))),
        ALU(Cmp, Flags, R1, Reg(R2)),
    ];

    let block = Block{
        label: "main".to_string(),
        ir: Symphony(tree),
    };

    let asm = block.build().unwrap();

    println!("{}", asm);
}
//-------------------------------------------------------

const TABLE_BASE: u16 = 0x2000;
const MESSAGE_BASE: u16 = 0x2100;
const RESULT_BASE: u16 = 0x2200;

fn load_imm(dst: Regs, value: u16) -> SymphonyISA {
    ALU(Add, dst, Zr, Imd(ImdNumber(value)))
}

fn store_imm(op: StoreOp, src: Regs, addr: u16) -> SymphonyISA {
    Store(op, src, Imd(ImdNumber(addr)))
}

/// One nibble of a table-driven CRC-16/CCITT update.
/// crc = (crc << 4) ^ table[((crc >> 12) ^ nibble) & 0xF]
fn crc_nibble(ir: &mut Vec<SymphonyISA>, nibble: Regs) {
    ir.push(ALU(Lsr, R7, R1, Imd(ImdNumber(12)))); // index = crc >> 12
    ir.push(ALU(Xor, R7, R7, Reg(nibble)));        // index ^= nibble
    ir.push(ALU(And, R7, R7, Imd(ImdNumber(0xF)))); // index &= 0xF
    ir.push(ALU(Lsl, R7, R7, Imd(ImdNumber(1))));  // index *= 2 (16-bit table)
    ir.push(ALU(Add, R7, R7, Reg(R2)));            // index += table base
    ir.push(Load(Load16, R8, Reg(R7)));            // value = table[index]
    ir.push(ALU(Lsl, R1, R1, Imd(ImdNumber(4))));  // crc <<= 4
    ir.push(ALU(Xor, R1, R1, Reg(R8)));            // crc ^= value
}

pub fn test2(){
    let mut ir = Vec::new();

    // ---------------------------------------------------------------------
    // Data section: build a 16-entry CRC-16/CCITT lookup table in RAM, and
    // copy a fixed message next to it.
    // ---------------------------------------------------------------------
    for i in 0..16u16 {
        let mut crc = i << 12;
        for _ in 0..4 {
            crc = if crc & 0x8000 != 0 {
                (crc << 1) ^ 0x1021
            } else {
                crc << 1
            };
        }
        ir.push(load_imm(R5, crc));
        ir.push(store_imm(Store16, R5, TABLE_BASE + i * 2));
    }

    let message = b"SYMPHONY!";
    for (i, byte) in message.iter().enumerate() {
        ir.push(load_imm(R5, *byte as u16));
        ir.push(store_imm(Store8, R5, MESSAGE_BASE + i as u16));
    }

    // Load the first four message bytes as one word, then write it back as a
    // 32-bit value. This also exercises the wide load/store forms.
    ir.push(Load(Load32, R3, Imd(ImdNumber(MESSAGE_BASE))));
    ir.push(store_imm(Store32, R3, RESULT_BASE));

    // ---------------------------------------------------------------------
    // Stage 1: CRC-16 over the 9-byte message, fully unrolled.
    //   r1 = crc, r2 = table base
    // ---------------------------------------------------------------------
    ir.push(Input(R1));                             // seed from input
    ir.push(ALU(Xor, R1, R1, Imd(ImdNumber(0xFFFF))));
    ir.push(load_imm(R2, TABLE_BASE));

    for (i, _) in message.iter().enumerate() {
        ir.push(Load(Load8, R5, Imd(ImdNumber(MESSAGE_BASE + i as u16))));

        // High nibble.
        ir.push(ALU(Lsr, R6, R5, Imd(ImdNumber(4))));
        ir.push(ALU(And, R6, R6, Imd(ImdNumber(0xF))));
        crc_nibble(&mut ir, R6);

        // Low nibble.
        ir.push(ALU(And, R6, R5, Imd(ImdNumber(0xF))));
        crc_nibble(&mut ir, R6);
    }

    // ---------------------------------------------------------------------
    // Stage 2: byte-wise reduction using every remaining ALU operation.
    //   r9 = sum, r10 = xor, r11 = and, r12 = or, r13 = nand
    // ---------------------------------------------------------------------
    ir.push(load_imm(R9, 0));
    ir.push(load_imm(R10, 0));
    ir.push(load_imm(R11, 0xFFFF));
    ir.push(load_imm(R12, 0));

    for (i, _) in message.iter().enumerate() {
        ir.push(Load(Load8, R5, Imd(ImdNumber(MESSAGE_BASE + i as u16))));
        ir.push(ALU(Add, R9, R9, Reg(R5)));
        ir.push(ALU(Xor, R10, R10, Reg(R5)));
        ir.push(ALU(And, R11, R11, Reg(R5)));
        ir.push(ALU(Or, R12, R12, Reg(R5)));
    }

    ir.push(ALU(Nand, R13, R11, Reg(R12)));

    // ---------------------------------------------------------------------
    // Stage 3: fold the CRC and the reductions into one result.
    // ---------------------------------------------------------------------
    ir.push(ALU(Lsl, R3, R9, Imd(ImdNumber(3))));   // sum << 3
    ir.push(ALU(Xor, R3, R3, Reg(R10)));            // ^= xor
    ir.push(ALU(Lsr, R4, R11, Imd(ImdNumber(2))));  // and >> 2
    ir.push(ALU(Asr, R4, R4, Imd(ImdNumber(1))));   // arithmetic shift
    ir.push(ALU(Add, R3, R3, Reg(R4)));             // += ...
    ir.push(ALU(Nor, R4, R12, Reg(R13)));           // ~(or | nand)
    ir.push(ALU(Sub, R4, R4, Reg(R9)));             // -= sum

    ir.push(ALU(Xor, R1, R1, Reg(R3)));
    ir.push(ALU(Xor, R1, R1, Reg(R4)));

    ir.push(store_imm(Store32, R1, RESULT_BASE + 4));
    ir.push(store_imm(Store16, R9, RESULT_BASE + 8));
    ir.push(store_imm(Store8, R10, RESULT_BASE + 10));

    // ---------------------------------------------------------------------
    // Stage 4: publish the results.
    // ---------------------------------------------------------------------
    ir.push(Output(Reg(R1)));   // final hash
    ir.push(Output(Reg(R9)));   // sum
    ir.push(Output(Reg(R10)));  // xor
    ir.push(Output(Reg(R11)));  // and
    ir.push(Output(Reg(R12)));  // or
    ir.push(Output(Reg(R13)));  // nand
    ir.push(Output(Reg(R3)));   // mixed
    ir.push(Output(Imd(ImdNumber(0xCAFE))));

    let block = Block{
        label: "crc_kernel".to_string(),
        ir: Symphony(ir),
    };

    let asm = block.build().unwrap();

    println!("{}", asm);
}

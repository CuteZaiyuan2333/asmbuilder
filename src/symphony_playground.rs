use crate::backends::symphony::CondOp::Jmp;
use crate::*;
use crate::backends::symphony::*;
use crate::backends::symphony::AluOp::*;
use crate::backends::symphony::Breg::*;
use crate::backends::symphony::Regs::*;
use crate::backends::symphony::SymphonyISA::*;

const MAIN:crate::Label = crate::Label("main");
const ABCDE:crate::Label = crate::Label("abcde");

pub fn test(){
    
    let main_block = Block{
        label: MAIN,
        ir: Symphony(vec![
            ALU(Add, R1, Zr, Imd(ImdNumber(1))),
            ALU(Add, R2, Zr, Imd(ImdNumber(1))),
            ALU(Cmp, Flags, R1, Reg(R2)),
            JUMP(Jmp, Breg::Label(ABCDE)),
        ]),
    };

    let abcde_block = Block{
        label: ABCDE,
        ir: Symphony(vec![
            Output(Breg::Reg(Flags)),
            JUMP(Jmp, Breg::Label(MAIN)),
        ])
    };

    let asm = main_block.build().unwrap() + "\n" + &abcde_block.build().unwrap();

    println!("{}", asm);
}
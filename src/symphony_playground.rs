use crate::*;
use crate::backends::symphony::*;
use crate::backends::symphony::AluOp::*;
use crate::backends::symphony::Breg::*;
use crate::backends::symphony::Regs::*;
use crate::backends::symphony::SymphonyISA::*;

pub fn test(){
    let main_label = crate::Label("main".to_string());
    
    let block = Block{
        label: main_label,
        ir: Symphony(vec![
            ALU(Add, R1, Zr, Imd(ImdNumber(1))),
            ALU(Add, R2, Zr, Imd(ImdNumber(1))),
            ALU(Cmp, Flags, R1, Reg(R2)),
        ]),
    };

    let asm = block.build().unwrap();

    println!("{}", asm);
}
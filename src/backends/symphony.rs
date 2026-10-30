use crate::{Backend, Label};

pub struct Symphony(pub Vec<SymphonyISA>);

#[derive(Clone)]
pub enum SymphonyISA{
    Input(Regs),
    Output(Breg),
    ALU(AluOp, Regs, Regs, Breg),
    JUMP(CondOp, Breg),
    Load(LoadOp, Regs, Breg),
    Store(StoreOp, Regs, Breg),
}

#[derive(Clone)]
pub struct ImdNumber(pub u16);

#[derive(Clone)]
pub enum Regs{
    R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11, R12, R13, Zr, Sp, Flags
}
impl Regs{
    pub fn build(self) -> String{
        match self {
            Self::R1 => "r1",
            Self::R2 => "r2",
            Self::R3 => "r3",
            Self::R4 => "r4",
            Self::R5 => "r5",
            Self::R6 => "r6",
            Self::R7 => "r7",
            Self::R8 => "r8",
            Self::R9 => "r9",
            Self::R10 => "r10",
            Self::R11 => "r11",
            Self::R12 => "r12",
            Self::R13 => "r13",
            Self::Zr => "zr",
            Self::Sp => "sp",
            Self::Flags => "flags",
        }.to_string()
    }
}

#[derive(Clone)]
pub enum Breg{
    Reg(Regs),
    Imd(ImdNumber),
    Label(Label)
}
impl Breg {
    pub fn build(self) -> String{
        match self {
            Self::Reg(reg) => reg.build(),
            Self::Imd(imd) => imd.0.to_string(),
            Self::Label(label) => label.0.to_string(),
        }
    }
}

#[derive(Clone)]
pub enum AluOp{
    Nand, Or, And, Nor, Add, Sub, Xor, Lsl, Lsr, Asr, Cmp,
}
impl AluOp{
    pub fn build(self, is_imd: bool) -> String{
        let name = match self {
            Self::Nand => "nand",
            Self::Or => "or",
            Self::And => "and",
            Self::Nor => "nor",
            Self::Add => "add",
            Self::Sub => "sub",
            Self::Xor => "xor",
            Self::Lsl => "lsl",
            Self::Lsr => "lsr",
            Self::Asr => "asr",
            Self::Cmp => "cmp",
        }.to_string();
        if is_imd{
            name + "i"
        }else{
            name
        }
    }
}

#[derive(Clone)]
pub enum CondOp{
    Jmp, Je, Jne, Jb, Jae, Jbe, Ja, Jl, Jge, Jle, Jg,
}
impl CondOp{
    pub fn build(self, is_imd: bool) -> String{
        let name = match self {
            Self::Jmp => "jmp",
            Self::Je => "je",
            Self::Jne => "jne",
            Self::Jb => "jb",
            Self::Jae => "jae",
            Self::Jbe => "jbe",
            Self::Ja => "ja",
            Self::Jl => "jl",
            Self::Jge => "jge",
            Self::Jle => "jle",
            Self::Jg => "jg"
        }.to_string();
        if is_imd{
            name + "i"
        }else{
            name
        }
    }
}

#[derive(Clone)]
pub enum LoadOp{
    Load8, Load16, Load32, Pload,
}
impl LoadOp{
    fn build(self, is_imd: bool) -> String{
        let name = match self {
            Self::Load8 => "load8",
            Self::Load16 => "load16",
            Self::Load32 => "load32",
            Self::Pload => "pload",
        }.to_string();
        if is_imd{
            name + "i"
        }else{
            name
        }
    }
}

#[derive(Clone)]
pub enum StoreOp{
    Store8, Store16, Store32, Pstore,
}
impl StoreOp{
    fn build(self, is_imd: bool) -> String{
        let name = match self {
            Self::Store8 => "store8",
            Self::Store16 => "store16",
            Self::Store32 => "store32",
            Self::Pstore => "pstore",
        }.to_string();
        if is_imd{
            name + "i"
        }else{
            name
        }
    }
}

impl Symphony{
    fn build_single(single: &SymphonyISA) -> Result<String, ()> {
        let temp: SymphonyISA = (*single).clone();
        return match temp {
            SymphonyISA::Input(reg) =>{
                Ok("in ".to_string() + &reg.build())
            }
            SymphonyISA::Output(breg) =>{
                Ok(match breg{
                    Breg::Reg(_) => "out ",
                    Breg::Imd(_) => "outi ",
                    _ => return Err(()),
                }.to_string() + &breg.build())
            }
            SymphonyISA::ALU(op, dreg, areg, breg) =>{
                Ok(match breg {
                    Breg::Reg(_) => op.build(false),
                    Breg::Imd(_) => op.build(true),
                    _ => return Err(()),
                } + " " + &dreg.build() + ", " + &areg.build() + ", " + &breg.build())
            }
            SymphonyISA::JUMP(op, breg) =>{
                Ok(match breg {
                    Breg::Reg(_) => op.build(false),
                    Breg::Imd(_) => op.build(true),
                    Breg::Label(_) => op.build(true),
                } + " " + &breg.build())
            }
            SymphonyISA::Load(op, dreg, addr_reg) =>{
                Ok(match addr_reg {
                    Breg::Reg(_) => op.build(false),
                    Breg::Imd(_) => op.build(true),
                    _ => return Err(()),
                } + " " + &dreg.build() + ", " + &addr_reg.build())
            }
            SymphonyISA::Store(op, addr_reg, value_reg) =>{
                Ok(match value_reg {
                        Breg::Reg(_) => op.build(false),
                        Breg::Imd(_) => op.build(true),
                        _ => return Err(()),
                } + " " + &addr_reg.build() + ", " + &value_reg.build())
            }
        };
    }
}

impl Backend for Symphony{
    fn build_asm_body(self) -> Result<String, ()> {
        let mut body = "".to_string();
        for item in self.0.iter(){
            body = body + "\t" + &Self::build_single(item).unwrap() + "\n"
        }
        Ok(body)
    }
}

impl Default for Symphony {
    fn default() -> Self {
        Self(Vec::new())
    }
}
use crate::isa::Op;
use std::fmt::Write;

pub fn disassemble(program: &[Op]) -> String {
    let mut source = String::new();

    for instruction in program{
        match instruction {
            Op::Push(value) => {
                let _ = write!(source, "PUSH {}\n",value);
                // source.push_str(&format!("PUSH {}\n",value));
            },
            Op::Load(index) => {
                let _ = write!(source,"LOAD {}\n",index);
            },
            Op::Store(index) => {
                let _ = write!(source,"STORE {}\n", index);
            },
             Op::Add => {
                let _ = write!(source, "ADD\n");
            }
            Op::Pop => {
                let _ = write!(source, "POP\n");
            }
            Op::Dup => {
                let _ = write!(source, "DUP\n");
            }
            Op::Swap => {
                let _ = write!(source, "SWAP\n");
            }
            Op::Sub => {
                let _ = write!(source, "SUB\n");
            }
            Op::Div => {
                let _ = write!(source, "DIV\n");
            }
            Op::Mul => {
                let _ = write!(source, "MUL\n");
            }
            Op::Mod => {
                let _ = write!(source, "MOD\n");
            }
            Op::Neg => {
                let _ = write!(source, "NEG\n");
            }
            Op::Print => {
                let _ = write!(source, "PRINT\n");
            }
            Op::Halt => {
                let _ = write!(source, "HALT\n");
            }
        }
    }

    source
}
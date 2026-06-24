mod isa;
mod vm;

use vm::VM;
use isa::Op;

fn main(){
    let mut vm = VM::new();

    let program = vec![
        Op::Push(7),
        Op::Push(3),
        Op::Add,
        Op::Print,
        Op::Halt,
    ];

    for Instruction in program {
        vm.execute(Instruction);
    }

}
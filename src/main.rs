mod isa;
mod vm;

use vm::VM;
use isa::Op;

fn main(){
    let mut vm = VM::new();

    let program = vec![
    Op::Push(2),
    Op::Push(5),
    Op::Swap,
    Op::Print,
    Op::Print,
    Op::Halt,
];

    for Instruction in program {
        if let Err(error) = vm.execute(Instruction) {
            println!("VM Error: {}", error);
            break;
        }
    }

}
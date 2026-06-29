mod isa;
mod vm;

use vm::VM;
use isa::Op;

use std::error::Error;

fn decode_program(bytes: &[u8]) -> Result<Vec<Op>, String> {
    let mut pc = 0;
    let mut operations = Vec::new();

    while pc < bytes.len() {
        let rb = &bytes[pc..];
        let (op, consumed) = Op::decode(rb)?;
        operations.push(op);
        pc += consumed;
        // match Op::decode(rb) {
        //     Ok((operation, consumed)) => {
        //         operations.push(operation);
        //         pc += consumed;
        //     }

        //     Err(e) => {
        //         return Err(e);
        //     }
        // }
    }

    Ok(operations)
}

fn encode_program(program: &[Op]) -> Vec<u8> {
    let mut bytecode = Vec::new();
    for instruction in program {
        let bytes = instruction.encode();
        bytecode.extend(bytes);
    }
    bytecode
}

fn main() -> Result<(), Box<dyn Error>>{
    let mut vm = VM::new();

    let program = vec![
    Op::Push(100),
    Op::Push(20),
    Op::Add,
    Op::Print,
    Op::Halt,
];

    let bytecode = encode_program(&program);

    let instructions = decode_program(&bytecode)?;
    for instruction in instructions {
        if let Err(error) = vm.execute(instruction) {
            println!("VM Error: {}",error);
            break;
        }
    }

    // println!("{:?}",bytecode);
    // for Instruction in program {
    //     if let Err(error) = vm.execute(Instruction) {
    //         println!("VM Error: {}", error);
    //         break;
    //     }
    // }

//     let bytes = Op::Push(10).encode();

// println!("{:?}", bytes);
Ok(())

}
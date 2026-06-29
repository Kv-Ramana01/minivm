use std::env;

mod assembler;
mod disassembler;
mod fileio;
mod isa;
mod vm;

use assembler::assemble;
use disassembler::disassemble;
use fileio::*;
use isa::Op;
use vm::VM;

use std::error::Error;

// use std::thread;
// use std::time::Duration;

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

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage Help:");
        println!("  asm <input.tasm> <output.tbc>");
        println!("  run <program.tbc>");
        println!("  dis <program.tbc>");
        return Ok(());
    }

    let mut vm = VM::new();

    match args[1].as_str() {
        "asm" => {
            if args.len() != 4 {
                println!("Usage: asm <input.tasm> <output.tbc>");
                return Ok(());
            }
            let extn1 = args[2].as_str().split('.').last().unwrap_or("");
            if extn1 != "tasm" {
                println!("Could not find a .tasm file.");
                return Ok(());
            }
            let extn2 = args[3].as_str().split('.').last().unwrap_or("");
            if extn2 != "tbc" {
                println!("Expected a tbc extension.");
                return Ok(());
            }
            let source = read_source(args[2].as_str())?;
            let program = assemble(&source)?;
            let bytecode = encode_program(&program);
            write_bytecode(args[3].as_str(), &bytecode)?;
            println!("{} file generated successfully", args[3]);
        }
        "run" => {
            let ext = args[2].split('.').last().unwrap_or("");

            if ext != "tbc" {
                println!("Expected a .tbc file.");
                return Ok(());
            }
            let trace = args.len() == 4 && args[3] == "--trace";
            let bytes = read_bytecode(args[2].as_str())?;
            let instructions = decode_program(&bytes)?;
            for instruction in instructions {
                if trace {
                    println!("Instruction: {:?}", instruction);
                    println!("Stack: {:?}", vm.stack());
                }
                if let Err(error) = vm.execute(instruction) {
                    println!("VM Error: {}", error);
                    break;
                }
                if trace {
                    println!("Stack after: {:?}", vm.stack());
                    println!("---------------------");
                    // thread::sleep(Duration::from_millis(500));
                }
            }
        }
        "dis" => {
            if args.len() != 3 {
                println!("Usage: dis <program.tbc>");
                return Ok(());
            }
            let ext = args[2].split('.').last().unwrap_or("");

            if ext != "tbc" {
                println!("Expected a .tbc file.");
                return Ok(());
            }
            let bytes = read_bytecode(args[2].as_str())?;
            let program = decode_program(&bytes)?;
            let text = disassemble(&program);
            for line in text.lines() {
                println!("{}", line);
            }
        }
        _ => {
            println!("Unknown Command.");
        }
    }
    //     let program = vec![
    //     Op::Push(100),
    //     Op::Push(20),
    //     Op::Add,
    //     Op::Print,
    //     Op::Halt,
    // ];

    // println!("{:?}", source);

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

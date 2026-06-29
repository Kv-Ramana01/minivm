
use crate::isa::Op;

pub fn assemble(source: &str) -> Result<Vec<Op>, String> {
    let mut program = Vec::new();

    for line in source.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
                    continue;
                }
        match parts[0] {
            "PUSH" => {
                if parts.len() != 2 {
                    return Err("PUSH operation needs one operand.".to_string());
                }
                let value = parts[1].parse::<i64>().map_err(|_| format!("Invalid number: {}", parts[1]))?;
                program.push(Op::Push(value));
            },
            "LOAD" => {
                if parts.len() != 2 {
                    return Err("LOAD operation needs one index.".to_string());
                }
                let value = parts[1].parse::<u8>().
                map_err(|_| format!("Invalid index: {}",parts[1]))?;
                program.push(Op::Load(value));
            },
            "STORE" => {
                if parts.len() != 2 {
                    return Err("STORE operation needs one index.".to_string());
                }
                let value = parts[1].parse::<u8>().
                map_err(|_| format!("Invalid index: {}",parts[1]))?;
                program.push(Op::Store(value));
            },
            "ADD" => program.push(Op::Add),
            "POP" => program.push(Op::Pop),
            "DUP" => program.push(Op::Dup),
            "SWAP" => program.push(Op::Swap),
            "SUB" => program.push(Op::Sub),
            "DIV" => program.push(Op::Div),
            "MUL" => program.push(Op::Mul),
            "MOD" => program.push(Op::Mod),
            "NEG" => program.push(Op::Neg),
            "PRINT" => program.push(Op::Print),
            "HALT" => program.push(Op::Halt),
            _ => {
                return Err(format!("Unknown instruction: {}", parts[0]));
            }
        }
    }

    Ok(program)
}
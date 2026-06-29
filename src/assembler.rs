

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
            "ADD" => program.push(Op::Add),
            "PRINT" => program.push(Op::Print),
            "HALT" => program.push(Op::Halt),
            _ => {
                return Err(format!("Unknown instruction: {}", parts[0]));
            }
        }
    }

    Ok(program)
}
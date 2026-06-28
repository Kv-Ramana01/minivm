#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op{
    Push(i64),

    Pop,
    Dup,
    Swap,

    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Neg,

    Load(u8),
    Store(u8),

    Print,
    Halt,
}

impl Op {
    pub fn encode(&self) -> Vec<u8> {
        match self{
            Op::Add => {
                vec![0x10]
            },
            Op::Push(val) => {
                let mut bytes = Vec::with_capacity(9);

                bytes.push(0x01);
                let ser_bytes = val.to_le_bytes();
                bytes.extend(ser_bytes);
                bytes
            },
            Op::Pop  => vec![0x02],
            Op::Dup  => vec![0x03],
            Op::Swap => vec![0x04],

            Op::Sub => vec![0x11],
            Op::Mul => vec![0x12],
            Op::Div => vec![0x13],
            Op::Mod => vec![0x14],
            Op::Neg => vec![0x15],

            Op::Load(index)  => {
                let mut bytes = Vec::with_capacity(2);
                bytes.push(0x20);
                let ser_bytes = index.to_le_bytes();
                bytes.extend(ser_bytes);
                bytes
            },
            Op::Store(index) => {
                 let mut bytes = Vec::with_capacity(2);

                bytes.push(0x21);
                let ser_bytes = index.to_le_bytes();
                bytes.extend(ser_bytes);
                bytes
            },

            // System Operations
            Op::Print => vec![0x30],
            Op::Halt  => vec![0x00],
        }
    }

    pub fn decode(bytes: &[u8]) -> Result<(Op, usize), String> {
        if bytes.is_empty() {
            return Err("No bytes to decode".to_string());
        }

        let opcode = bytes[0];

        match opcode {
            0x02 => Ok((Op::Pop, 1)),
            0x03 => Ok((Op::Dup, 1)),
            0x04 => Ok((Op::Swap, 1)),

            0x10 => Ok((Op::Add, 1)),
            0x11 => Ok((Op::Sub, 1)),
            0x12 => Ok((Op::Mul, 1)),
            0x13 => Ok((Op::Div, 1)),
            0x14 => Ok((Op::Mod, 1)),
            0x15 => Ok((Op::Neg, 1)),

            0x30 => Ok((Op::Print, 1)),
            0x00 => Ok((Op::Halt, 1)),
            0x01 => { 
                if bytes.len() < 9 {
                    return Err("Truncated bytecode: Missing 8-byte value for the push operation".to_string());
                }
                let raw_bytes: [u8; 8] = bytes[1..9].try_into();
                let value = i64::from_le_bytes(raw_bytes);
                Ok((Op::Push(value), 9))
            }
            0x20 => { 
                if bytes.len() < 2 {
                    return Err("Truncated bytecode: Missing slot index for LOAD oepration".to_string());
                }
                Ok((Op::Load(bytes[1]),2))
             }
            0x21 => { 
                if bytes.len() < 2 {
                    return Err("Truncated bytecode: Missing slot index for STORE oepration".to_string());
                }
                Ok((Op::Store(bytes[1]), 2))

             }

    _        => Err(format!("Unknown opcode: {:#04X}", opcode)),
        }
    }
}
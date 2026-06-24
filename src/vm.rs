use crate::isa::Op;

pub struct VM {
    stack: Vec<i64>,
    globals: [i64;256],
}

impl VM {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            globals: [0; 256],
        }
    }

    pub fn execute(&mut self, op:Op) {
        match op {
            Op::Push(value) => {
                self.stack.push(value);
            }

            Op::Add => {
                let b = self.stack.pop().unwrap();
                let a = self.stack.pop().unwrap();

                self.stack.push(a+b);
            }
            
            Op::Print => {
                let value = self.stack.pop().unwrap();
                println!("{}",value);
            }

            Op::Halt => {
                println!("Halt");
            }
            _=> {
                println!("Instruction not implemented yet");
            }
        }
    }
}
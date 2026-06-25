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

    fn pop_stack(&mut self) -> Result<i64, String> {
        match self.stack.pop() {
            Some(value) => Ok(value),
            None => Err("Stack underflow".to_string()),
        }
    }

    pub fn execute(&mut self, op:Op) -> Result<(), String> {
        match op {
            Op::Push(value) => {
                self.stack.push(value);
                Ok(())
            }

            Op::Pop => {
                self.pop_stack()?;
                Ok(())
            }

            Op::Dup => {
                let val = self.pop_stack()?;
                self.stack.push(val);
                self.stack.push(val);

                Ok(())
            }

            Op::Swap => {
                let val1 = self.pop_stack()?;
                let val2 = self.pop_stack()?;

                self.stack.push(val1);
                self.stack.push(val2);

                Ok(())
            }

            Op::Add => {
                let b = self.pop_stack()?;
                let a = self.pop_stack()?;

                self.stack.push(a+b);
                Ok(())
            }
            
            Op::Print => {
                let value = self.pop_stack()?;
                println!("{}",value);
                Ok(())
            }

            Op::Halt => {
                println!("Halt");
                Ok(())
            }
            _=> {
                println!("Instruction not implemented yet");
                Ok(())
            }
        }
    }
}
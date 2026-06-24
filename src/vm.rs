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
}
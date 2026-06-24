mod isa;
mod vm;

use vm::VM;

fn main(){
    let _vm = VM::new();

    println!("MiniVM started!!");
}
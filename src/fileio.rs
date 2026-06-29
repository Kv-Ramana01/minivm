use std::fs;
use std::io;

pub fn write_bytecode(path: &str, bytecode: &[u8],) -> io::Result<()>{
    fs::write(path, bytecode)
}

pub fn read_bytecode(path: &str,) -> io::Result<Vec<u8>> {
    fs::read(path)
}
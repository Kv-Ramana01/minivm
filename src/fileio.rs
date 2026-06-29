use std::fs;
use std::io;

pub fn write_bytecode(path: &str, bytecode: &[u8],) -> io::Result<()>{
    let mut file = Vec::new();
    file.push(0x4D); //M
    file.push(0x56); //V
    file.push(0x4D); //M
    file.push(0x00); //\0
    
    //version
    file.push(0x01);

    //length
    let length = bytecode.len() as u32;
    let length_bytes = length.to_le_bytes();
    file.extend(length_bytes);

    //bytecode
    file.extend(bytecode);
    fs::write(path, file)

}

fn validate_signature(file : &[u8]) -> bool {
    if file.len() >= 5 && file[0] == 0x4D &&  file[1] == 0x56 && file[2] == 0x4D && file[3] == 0x00 && file[4] == 0x01 {
        return true;
    }
    
    false
} 

pub fn read_bytecode(path: &str,) -> io::Result<Vec<u8>> {
    let file = fs::read(path)?;

    if !validate_signature(&file) {
        return Err(io::Error::new(io::ErrorKind::InvalidData,"Invalid Signature!"));
    }

    if file.len() < 9 {
        return Err(io::Error::new(io::ErrorKind::UnexpectedEof,"Not Enough data!"));
    }

    let raw_bytes: [u8;4]= file[5..9].try_into().unwrap();
    let length = u32::from_le_bytes(raw_bytes) as usize;

    if length != file.len() - 9 {
        return Err(io::Error::new(io::ErrorKind::InvalidData,"Size of the bytecode does not match with length provided."));
    }

    let bytecode = file[9..].to_vec();

    Ok(bytecode)
}

pub fn read_source(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}
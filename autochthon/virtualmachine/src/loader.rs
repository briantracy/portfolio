
use crate::bits;


pub enum LoaderError {
    InvalidHeader,
    InvalidSymbolTable
}

struct BinaryHeader {
    symbol_table_location: i32,
    data_section_location: i32,
    code_section_location: i32
}

pub struct LoadedProgram {
    symbol_table: std::collections::HashMap<String, i32>,
    code: Vec<u8>,
    data: Vec<u8>
}

impl BinaryHeader {
    pub fn extract_from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < std::mem::size_of::<BinaryHeader>() { 
            None 
        } else {
            Some(BinaryHeader {
                symbol_table_location: bits::interpret_big_endian_slice(&bytes[0..4]),
                data_section_location: bits::interpret_big_endian_slice(&bytes[4..8]),
                code_section_location: bits::interpret_big_endian_slice(&bytes[8..12])
            })   
        }     
    }
}

impl LoadedProgram {

    pub fn from_binary_bytes(bytes: Vec<u8>) -> Result<Self, LoaderError> {
        let header = BinaryHeader::extract_from_bytes(&bytes[0..]).ok_or(LoaderError::InvalidHeader)?;

    }
}
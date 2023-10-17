use std::fs;
use std::io;

pub enum Filetype {
    Asm(&'static str),
    PlaintextBinary(&'static str),
    EncodedBinary(&'static str),
}

impl Filetype {
    pub fn parse_to_word_array(&self) -> io::Result<[i16; 65536]> {
        match self {
            Filetype::EncodedBinary(s) => {
                let input_bytes = fs::read(s).unwrap();
                let mut return_bytes = [0; 65536];
                if input_bytes.len() % 2 != 0 {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Malformed input: input byte array does not have an even number of bytes (was {})", input_bytes.len())));
                }
                if input_bytes.len() > (0xFE00 - 0x3000) * 2 {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Malformed input: input byte array is longer than the maximum allowed length {} (was {})", 0xFE00 - 0x3000, input_bytes.len() / 2)));
                }
                for i in 0..(input_bytes.len() / 2) {
                    return_bytes[i + 0x3000] = (input_bytes[2 * i] as u16 * 2u16.pow(8) + input_bytes[2 * i + 1] as u16) as i16;
                }
                Ok(return_bytes)
            }
            Filetype::PlaintextBinary(s) => {
                let input_bytes = fs::read_to_string(s).unwrap().trim().split_whitespace().collect::<String>();
                let mut return_bytes = [0; 65536];
                if input_bytes.len() % 16 != 0 {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Malformed input: number of input bytes is not divisible by 16 (was {})", input_bytes.len())));
                }
                if input_bytes.len() > (0xFE00 - 0x3000) * 16 {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Malformed input: input byte array is longer than the maximum allowed length {} (was {})", 0xFE00 - 0x3000, input_bytes.len() / 16)));
                }
                for i in 0..(input_bytes.len() / 16) {
                    return_bytes[i + 0x3000] = (u16::from_str_radix(&input_bytes[i * 16..(i + 1) * 16], 2).unwrap()) as i16;
                }
                Ok(return_bytes)
            }
            _ => Ok([0; 65536]),
        }
    }
}

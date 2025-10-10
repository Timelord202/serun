use std::{fmt, fs};

const NES_HEADER_PREFIX: [u8; 4] = [0x4E, 0x45, 0x53, 0x1A];

#[derive(Debug)]
pub struct Cartidge {
    prg_size: u8,
    chr_size: u8,
    prg: Vec<u8>,
    chr: Vec<u8>
}

#[derive(Debug)]
pub enum CartidgeError {
    MissingHeaderPrefix,
    Io(String)
}

impl fmt::Display for CartidgeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingHeaderPrefix => write!(f, "File is missing NES header prefix, is this a .nes file?"),
            Self::Io(msg) => write!(f, "Error while attempting to parse .nes file: {msg}")
        }
    }
}

impl Cartidge {
    fn new(prg_size: u8, chr_size: u8, prg: Vec<u8>, chr: Vec<u8>) -> Self {
        Cartidge {
            prg_size,
            chr_size,
            prg,
            chr
        }
    }

    pub fn from_path(path: &str) -> Result<Cartidge, CartidgeError> {
        let bytes = fs::read(path);
        match bytes {
            Ok(data) => Self::from_bytes(data),
            Err(error) => Err(CartidgeError::Io(error.to_string()))
        }
    }

    fn from_bytes(data: Vec<u8>) -> Result<Cartidge, CartidgeError> {
        if data[..4] != NES_HEADER_PREFIX {
            return Err(CartidgeError::MissingHeaderPrefix);
        }
        println!("{:x?}", &data[0..4]);
        Ok(Self::new(1, 1, vec![1], vec![1]))
    }
}
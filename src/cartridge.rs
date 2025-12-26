use std::{fmt, fs};

const NES_HEADER_PREFIX: [u8; 4] = [0x4E, 0x45, 0x53, 0x1A];
const PRG_ROM_UNIT_SIZE: usize = 16384;
const CHR_ROM_UNIT_SIZE: usize = 8192;

#[derive(Debug)]
pub struct Cartidge {
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>
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
    fn new(prg_rom: Vec<u8>, chr_rom: Vec<u8>) -> Self {
        Cartidge {
            prg_rom,
            chr_rom
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
        let prg_rom_size = (data[4] as usize) * PRG_ROM_UNIT_SIZE;
        let chr_rom_size = (data[5] as usize) * CHR_ROM_UNIT_SIZE;
        let skip_trainer = data[6] & 0b100 == 0;
        let prg_start = if skip_trainer { 0x10 } else { 0x10 + 0x200 };
        let prg_end = prg_start + prg_rom_size;

        Ok(Self::new(
            data[prg_start..prg_end].to_vec(), 
            data[prg_end..(prg_end + chr_rom_size)].to_vec()
        ))
    }
}
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Memory {
    memory: Vec<u8>,
}

impl Default for Memory {
    fn default() -> Self {
        Memory {
            memory: vec![0; 0xFFFF]
        }
    }
}

impl Memory {
    pub fn read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    pub fn read_u16(&mut self, pos: u16) -> u16 {
        let lo = self.read(pos) as u16;
        let hi = self.read(pos.wrapping_add(1)) as u16;
        (hi << 8) | lo
    }

    pub fn write_u16(&mut self, pos: u16, data: u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & 0xFF) as u8;
        self.write(pos, lo);
        self.write(pos + 1, hi);
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(&program[..]);
        self.write_u16(0xFFFC, 0x8000);
    }
}
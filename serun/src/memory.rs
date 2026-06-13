// Implemented for both Ram and Bus
pub trait Memory {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, data: u8);
    fn load(&mut self, program: Vec<u8>);

    fn read_u16(&mut self, pos: u16) -> u16 {
        let lo = self.read(pos) as u16;
        let hi = self.read(pos.wrapping_add(1)) as u16;
        (hi << 8) | lo
    }

    fn read_u16_jmp_bug(&mut self, pos: u16) -> u16 {
        let lo = self.read(pos) as u16;
        let hi = self.read(pos & 0xFF00) as u16;
        (hi << 8) | lo
    }

    fn write_u16(&mut self, pos: u16, data: u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & 0xFF) as u8;
        self.write(pos, lo);
        self.write(pos + 1, hi);
    }
}


// Ram is used under the hood for the Bus.
// It's also used for debugging the cpu, as the tests requires
// raw reading/writing to ram
pub struct Ram {
    pub raw_memory: Vec<u8>,
}

impl Default for Ram {
    fn default() -> Self {
        Ram {
            raw_memory: vec![0; 0x10000]
        }
    }
}

impl Memory for Ram {
    fn read(&self, addr: u16) -> u8 {
        self.raw_memory[addr as usize]
    }

    fn write(&mut self, addr: u16, data: u8) {
        self.raw_memory[addr as usize] = data;
    }

    fn load(&mut self, program: Vec<u8>) {
        self.raw_memory[0x8000..(0x8000 + program.len())].copy_from_slice(&program[..]);
        self.write_u16(0xFFFC, 0x8000);
    }
}

#[derive(Default)]
pub struct Bus {
    ram: Ram,
}

impl Memory for Bus {
    fn read(&self, addr: u16) -> u8 { 
        self.ram.read(addr)
    }
    fn write(&mut self, addr: u16, data: u8) {
        self.ram.write(addr, data)
    }
    fn load(&mut self, program: Vec<u8>) {
        self.ram.load(program);
    }
}

impl Bus {
    pub fn block(&self, start: u16, end: u16) -> &[u8] {
        &self.ram.raw_memory[start as usize..end as usize]
    }
}
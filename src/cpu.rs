use crate::memory::Memory;
use crate::opcodes::{CPU_OPCODES, AddressingMode};

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub stack_pointer: u8,
    pub program_counter: u16,
    pub status: u8,
    memory: Memory,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            stack_pointer: 0,
            status: 0,
            program_counter: 0,
            memory: Memory::new(),
        }
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.status = 0;

        self.program_counter = self.memory.read_u16(0xFFFC);
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.memory.load(program);
        self.reset();
        self.run()
    }

    fn get_operand_address(&mut self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.program_counter,

            AddressingMode::ZeroPage => self.memory.read(self.program_counter) as u16,

            AddressingMode::Absolute => self.memory.read_u16(self.program_counter),

            AddressingMode::ZeroPage_X => {
                let pos = self.memory.read(self.program_counter);
                pos.wrapping_add(self.register_x) as u16
            }
            AddressingMode::ZeroPage_Y => {
                let pos = self.memory.read(self.program_counter);
                pos.wrapping_add(self.register_y) as u16
            }

            AddressingMode::Absolute_X => {
                let base = self.memory.read_u16(self.program_counter);
                base.wrapping_add(self.register_x as u16)
            }
            AddressingMode::Absolute_Y => {
                let base = self.memory.read_u16(self.program_counter);
                base.wrapping_add(self.register_y as u16)
            }

            AddressingMode::Indirect_X => {
                let base = self.memory.read(self.program_counter);

                let ptr: u8 = base.wrapping_add(self.register_x);
                let lo = self.memory.read(ptr as u16);
                let hi = self.memory.read(ptr.wrapping_add(1) as u16);
                (hi as u16) << 8 | (lo as u16)
            }
            AddressingMode::Indirect_Y => {
                let base = self.memory.read(self.program_counter);

                let lo = self.memory.read(base as u16);
                let hi = self.memory.read(base.wrapping_add(1) as u16);
                let deref_base = (hi as u16) << 8 | (lo as u16);
                deref_base.wrapping_add(self.register_y as u16)
            }

            AddressingMode::NoneAddressing => {
                panic!("mode {mode:?} is not supported");
            }
        }
    }

    fn lda(&mut self, value: u8) {
        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn iny(&mut self) {
        self.register_y = self.register_y.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_y);
    }

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            self.status |= 0b0000_0010;
        } else {
            self.status &= 0b1111_1101;
        }

        if result & 0b1000_0000 != 0 {
            self.status |= 0b1000_0000;
        } else {
            self.status &= 0b0111_1111;
        }
    }

    pub fn run(&mut self) {
        loop {
            let opscode_hex = self.memory.read(self.program_counter);
            let opcode = CPU_OPCODES.get(&opscode_hex).unwrap_or_else(|| panic!("Failed to retrieve opcode!"));
            println!("opcode: {opcode:?}");
            self.program_counter += 1;

            match opcode.name {
                "ADC" => todo!(),
                "AND" => todo!(),
                "ASL" => todo!(),
                "BCC" => todo!(),
                "BCS" => todo!(),
                "BEQ" => todo!(),
                "BIT" => todo!(),
                "BMI" => todo!(),
                "BNE" => todo!(),
                "BPL" => todo!(),
                "BRK" => return,
                "BVC" => todo!(),
                "BVS" => todo!(),
                "CLC" => todo!(),
                "CLD" => todo!(),
                "CLI" => todo!(),
                "CLV" => todo!(),
                "CMP" => todo!(),
                "CPX" => todo!(),
                "CPY" => todo!(),
                "DEC" => todo!(),
                "DEX" => todo!(),
                "DEY" => todo!(),
                "EOR" => todo!(),
                "INC" => todo!(),
                "INX" => self.inx(),
                "INY" => self.iny(),
                "JMP" => todo!(),
                "JSR" => todo!(),
                "LDA" => {
                    let operand = self.get_operand_address(&opcode.addressing_mode);
                    let param = self.memory.read(operand);

                    self.lda(param);
                },
                "LDX" => todo!(),
                "LDY" => todo!(),
                "LSR" => todo!(),
                "NOP" => {},
                "ORA" => todo!(),
                "PHA" => todo!(),
                "PHP" => todo!(),
                "PLA" => todo!(),
                "PLP" => todo!(),
                "ROL" => todo!(),
                "ROR" => todo!(),
                "RTI" => todo!(),
                "RTS" => todo!(),
                "SBC" => todo!(),
                "SEC" => todo!(),
                "SED" => todo!(),
                "SEI" => todo!(),
                "STA" => todo!(),
                "STX" => todo!(),
                "STY" => todo!(),
                "TAX" => self.tax(),
                "TAY" => todo!(),
                "TSX" => todo!(),
                "TXA" => todo!(),
                "TXS" => todo!(),
                "TYA" => todo!(),
                _ => panic!("Found unknown opcode: {}", opcode.name),
            }
            self.program_counter += (opcode.bytes - 1) as u16;
        }
    }
}

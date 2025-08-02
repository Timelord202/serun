use crate::memory::Memory;
use crate::opcodes::{CPU_OPCODES, AddressingMode, Instruction, Opcode};

pub mod instructions;

const SP_INITIAL_ADDR: u8 = 0xFD;
const STACK_START_ADDR: u16 = 0x01FF;

pub enum StatusFlag {
    C,
    Z,
    I,
    D,
    B,
    V,
    N
}

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
        self.stack_pointer = SP_INITIAL_ADDR;

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
            _ => {
                panic!("mode {mode:?} is not supported");
            }
        }
    }

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            self.set_status_flag(StatusFlag::Z);
        } else {
            self.status &= 0b1111_1101;
        }

        if result & 0b1000_0000 != 0 {
            self.set_status_flag(StatusFlag::N);
        } else {
            self.status &= 0b0111_1111;
        }
    }

    fn get_operand(&mut self, instruction: &Instruction) -> u8 {
        let operand_address = self.get_operand_address(&instruction.addressing_mode);
        self.memory.read(operand_address)
    }

    pub fn push_stack(&mut self, value: u8) {
        let stack_write_addr = STACK_START_ADDR - (SP_INITIAL_ADDR- self.stack_pointer) as u16;
        self.memory.write(stack_write_addr, value);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }

    pub fn pop_stack(&mut self) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        let stack_read_addr = STACK_START_ADDR - (SP_INITIAL_ADDR- self.stack_pointer) as u16;
        self.memory.read(stack_read_addr)
    }

    pub fn pop_stack_u16(&mut self) -> u16 {
        let lo = self.pop_stack() as u16;
        let hi = self.pop_stack() as u16;
        hi << 8 | lo
    }

    pub fn get_status_flag(&mut self, flag: StatusFlag) -> u8 {
        match flag {
            StatusFlag::C => self.status & 0b0000_0001,
            StatusFlag::Z => (self.status & 0b0000_0010) >> 1,
            StatusFlag::I => (self.status & 0b0000_0100) >> 2,
            StatusFlag::D => (self.status & 0b0000_1000) >> 3,
            StatusFlag::B => (self.status & 0b0001_0000) >> 4,
            StatusFlag::V => (self.status & 0b0100_0000) >> 6,
            StatusFlag::N => (self.status & 0b1000_0000) >> 7,
        }
    }

    pub fn set_status_flag(&mut self, flag: StatusFlag) {
        match flag {
            StatusFlag::C => self.status |= 0b0000_0001,
            StatusFlag::Z => self.status |= 0b0000_0010,
            StatusFlag::I => self.status |= 0b0000_0100,
            StatusFlag::D => self.status |= 0b0000_1000,
            StatusFlag::B => self.status |= 0b0001_0000,
            StatusFlag::V => self.status |= 0b0100_0000,
            StatusFlag::N => self.status |= 0b1000_0000,
        };
    }

    pub fn clear_status_flag(&mut self, flag: StatusFlag) {
        match flag {
            StatusFlag::C => self.status &= 0b1111_1110,
            StatusFlag::Z => self.status &= 0b1111_1101,
            StatusFlag::I => self.status &= 0b1111_1011,
            StatusFlag::D => self.status &= 0b1111_0111,
            StatusFlag::B => self.status &= 0b1110_1111,
            StatusFlag::V => self.status &= 0b1011_1111,
            StatusFlag::N => self.status &= 0b0111_1111,
        };
    }

    pub fn run(&mut self) {
        loop {
            let instruction_hex = self.memory.read(self.program_counter);
            let instruction = CPU_OPCODES.get(&instruction_hex).unwrap_or_else(|| panic!("Failed to retrieve opcode!"));
            self.program_counter += 1;

            match instruction.opcode {
                Opcode::ADC => todo!(),
                Opcode::AND => self.and(instruction),
                Opcode::ASL => self.asl(instruction),
                Opcode::BCC => self.bcc(),
                Opcode::BCS => self.bcs(),
                Opcode::BEQ => self.beq(),
                Opcode::BIT => self.bit(instruction),
                Opcode::BMI => self.bmi(),
                Opcode::BNE => self.bne(),
                Opcode::BPL => self.bpl(),
                Opcode::BRK => return,
                Opcode::BVC => todo!(),
                Opcode::BVS => todo!(),
                Opcode::CLC => self.clc(),
                Opcode::CLD => {},
                Opcode::CLI => self.cli(),
                Opcode::CLV => self.clv(),
                Opcode::CMP => self.cmp(instruction),
                Opcode::CPX => self.cpx(instruction),
                Opcode::CPY => self.cpy(instruction),
                Opcode::DEC => self.dec(instruction),
                Opcode::DEX => self.dex(),
                Opcode::DEY => self.dey(),
                Opcode::EOR => self.eor(instruction),
                Opcode::INC => self.inc(instruction),
                Opcode::INX => self.inx(),
                Opcode::INY => self.iny(),
                Opcode::JMP => todo!(),
                Opcode::JSR => todo!(),
                Opcode::LDA => self.lda(instruction),
                Opcode::LDX => self.ldx(instruction),
                Opcode::LDY => self.ldy(instruction),
                Opcode::LSR => todo!(),
                Opcode::NOP => {},
                Opcode::ORA => self.ora(instruction),
                Opcode::PHA => self.pha(),
                Opcode::PHP => self.php(),
                Opcode::PLA => self.pla(),
                Opcode::PLP => self.plp(),
                Opcode::ROL => self.rol(instruction),
                Opcode::ROR => self.ror(instruction),
                Opcode::RTI => self.rti(),
                Opcode::RTS => self.rts(),
                Opcode::SBC => todo!(),
                Opcode::SEC => self.sec(),
                Opcode::SED => self.sed(),
                Opcode::SEI => self.sei(),
                Opcode::STA => self.sta(instruction),
                Opcode::STX => self.stx(instruction),
                Opcode::STY => self.sty(instruction),
                Opcode::TAX => self.tax(),
                Opcode::TAY => self.tay(),
                Opcode::TSX => self.tsx(),
                Opcode::TXA => self.txa(),
                Opcode::TXS => self.txs(),
                Opcode::TYA => self.tya(),
            }
            self.program_counter += (instruction.bytes - 1) as u16;
        }
    }
}
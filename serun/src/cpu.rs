use crate::memory::Memory;
use crate::opcodes::{CPU_OPCODES, AddressingMode, Instruction, Opcode};

pub mod instructions;

const SP_BASE_ADDR: u16 = 0x0100;
const SP_INITIAL_ADDR: u8 = 0xFD;

pub enum StatusFlag {
    C,
    Z,
    I,
    D,
    B,
    V,
    N,
    Unused,
}

#[derive(Default)]
pub struct CPU<'a> {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub sp: u8,
    pub pc: u16,
    pub status: u8,
    // TODO: This won't work for testing when a Bus is implemented.
    // Will need to fix so that this can be used with a Bus or Ram
    pub memory: Memory,
    curr_instr: Option<&'a Instruction>,
}

impl CPU<'_> {
    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.status = 0;
        self.sp = SP_INITIAL_ADDR;
        self.pc = self.memory.read_u16(0xFFFC);
    }

    pub fn load_program(&mut self, program: Vec<u8>) {
        self.memory.load(program);
        self.reset();
    }

    fn get_operand_address(&mut self) -> u16 {
        let instruction = self.curr_instr.unwrap();
        let mode = &instruction.addressing_mode;
        // PC points to the opcode, need to increment in order to get operand
        let oper_addr = self.pc.wrapping_sub(self.curr_instr.unwrap().bytes as u16 - 1);

        match mode {
            AddressingMode::Immediate => oper_addr,

            AddressingMode::ZeroPage => self.memory.read(oper_addr) as u16,

            AddressingMode::Absolute => self.memory.read_u16(oper_addr),

            AddressingMode::ZeroPage_X => {
                let pos = self.memory.read(oper_addr);
                pos.wrapping_add(self.register_x) as u16
            }

            AddressingMode::ZeroPage_Y => {
                let pos = self.memory.read(oper_addr);
                pos.wrapping_add(self.register_y) as u16
            }

            AddressingMode::Absolute_X => {
                let base = self.memory.read_u16(oper_addr);
                base.wrapping_add(self.register_x as u16)
            }

            AddressingMode::Absolute_Y => {
                let base = self.memory.read_u16(oper_addr);
                base.wrapping_add(self.register_y as u16)
            }

            AddressingMode::Indirect => {
                let operand_address = self.memory.read_u16(oper_addr);
                self.memory.read_u16(operand_address)
            }

            AddressingMode::Indirect_X => {
                let base = self.memory.read(oper_addr);
                let ptr = base.wrapping_add(self.register_x);
                let lo = self.memory.read(ptr as u16);
                let hi = self.memory.read(ptr.wrapping_add(1) as u16);
                (hi as u16) << 8 | (lo as u16)
            }

            AddressingMode::Indirect_Y => {
                let base = self.memory.read(oper_addr);
                let lo = self.memory.read(base as u16);
                let hi = self.memory.read(base.wrapping_add(1) as u16);
                let deref_base = (hi as u16) << 8 | (lo as u16);
                deref_base.wrapping_add(self.register_y as u16)
            }

            AddressingMode::Relative => {
                let displacement = self.memory.read(oper_addr) as i8;
                // TODO: Add instr length instead of just 2
                self.pc.wrapping_add_signed(displacement as i16)
            }

            _ => {
                panic!("addressing mode {mode:?} is not supported");
            }
        }
    }

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            self.set_status_flag(StatusFlag::Z);
        } else {
            self.clear_status_flag(StatusFlag::Z);
        }

        if result & 0b1000_0000 != 0 {
            self.set_status_flag(StatusFlag::N);
        } else {
            self.clear_status_flag(StatusFlag::N);
        }
    }

    fn get_operand(&mut self) -> u8 {
        let operand_address = self.get_operand_address();
        self.memory.read(operand_address)
    }

    pub fn push_stack(&mut self, value: u8) {
        self.memory.write(SP_BASE_ADDR + u16::from(self.sp), value);
        self.sp = self.sp.wrapping_sub(1);
    }

    pub fn push_stack_u16(&mut self, value: u16) {
        let hi = (value >> 8) as u8;
        let lo = (value & 0xFF) as u8;
        self.push_stack(hi);
        self.push_stack(lo);
    }

    pub fn pop_stack(&mut self) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        self.memory.read(SP_BASE_ADDR + u16::from(self.sp))
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
            StatusFlag::Unused => (self.status & 0b0010_0000) >> 5,
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
            StatusFlag::Unused => self.status |= 0b0010_0000,
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
            StatusFlag::Unused => self.status &= 0b1101_1111,
            StatusFlag::V => self.status &= 0b1011_1111,
            StatusFlag::N => self.status &= 0b0111_1111,
        };
    }

    pub fn execute_instruction(&mut self) {
        let instruction_hex = self.memory.read(self.pc);
        let instruction = CPU_OPCODES.get(&instruction_hex);
        self.curr_instr = instruction;
        let instruction = instruction.unwrap_or_else(|| panic!("Failed to retrieve instruction: {:x}", instruction_hex));
        self.pc = self.pc.wrapping_add(instruction.bytes as u16);

        match &instruction.opcode {
            Opcode::ADC => self.adc(),
            Opcode::AND => self.and(),
            Opcode::ASL => self.asl(),
            Opcode::BCC => self.bcc(),
            Opcode::BCS => self.bcs(),
            Opcode::BEQ => self.beq(),
            Opcode::BIT => self.bit(),
            Opcode::BMI => self.bmi(),
            Opcode::BNE => self.bne(),
            Opcode::BPL => self.bpl(),
            Opcode::BRK => self.brk(),
            Opcode::BVC => self.bvc(),
            Opcode::BVS => self.bvs(),
            Opcode::CLC => self.clc(),
            Opcode::CLD => {},
            Opcode::CLI => self.cli(),
            Opcode::CLV => self.clv(),
            Opcode::CMP => self.cmp(),
            Opcode::CPX => self.cpx(),
            Opcode::CPY => self.cpy(),
            Opcode::DEC => self.dec(),
            Opcode::DEX => self.dex(),
            Opcode::DEY => self.dey(),
            Opcode::EOR => self.eor(),
            Opcode::INC => self.inc(),
            Opcode::INX => self.inx(),
            Opcode::INY => self.iny(),
            Opcode::JMP => self.jmp(),
            Opcode::JSR => self.jsr(),
            Opcode::LDA => self.lda(),
            Opcode::LDX => self.ldx(),
            Opcode::LDY => self.ldy(),
            Opcode::LSR => self.lsr(),
            Opcode::NOP => {},
            Opcode::ORA => self.ora(),
            Opcode::PHA => self.pha(),
            Opcode::PHP => self.php(),
            Opcode::PLA => self.pla(),
            Opcode::PLP => self.plp(),
            Opcode::ROL => self.rol(),
            Opcode::ROR => self.ror(),
            Opcode::RTI => self.rti(),
            Opcode::RTS => self.rts(),
            Opcode::SBC => self.sbc(),
            Opcode::SEC => self.sec(),
            Opcode::SED => self.sed(),
            Opcode::SEI => self.sei(),
            Opcode::STA => self.sta(),
            Opcode::STX => self.stx(),
            Opcode::STY => self.sty(),
            Opcode::TAX => self.tax(),
            Opcode::TAY => self.tay(),
            Opcode::TSX => self.tsx(),
            Opcode::TXA => self.txa(),
            Opcode::TXS => self.txs(),
            Opcode::TYA => self.tya(),
        }
    }
}
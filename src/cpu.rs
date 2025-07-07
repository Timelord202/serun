use crate::memory::Memory;
use crate::opcodes::{CPU_OPCODES, AddressingMode, Opcode};

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

            AddressingMode::NoneAddressing => {
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

    fn get_address_value(&mut self, opcode: &Opcode) -> u8 {
        let operand = self.get_operand_address(&opcode.addressing_mode);
        self.memory.read(operand)
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
        let lo = self.pop_stack();
        let high = self.pop_stack();
        (high << 8 | lo) as u16 
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

    pub fn run(&mut self) {
        loop {
            let opscode_hex = self.memory.read(self.program_counter);
            let opcode = CPU_OPCODES.get(&opscode_hex).unwrap_or_else(|| panic!("Failed to retrieve opcode!"));
            self.program_counter += 1;

            match opcode.name {
                "ADC" => todo!(),
                "AND" => {
                    let param = self.get_address_value(&opcode);
                    self.and(param);
                },
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
                "CLC" => self.clc(),
                "CLD" => {},
                "CLI" => self.cli(),
                "CLV" => self.clv(),
                "CMP" => {
                    let param = self.get_address_value(&opcode);
                    self.cmp(param);
                },
                "CPX" => {
                    let param = self.get_address_value(&opcode);
                    self.cpx(param);
                },
                "CPY" => {
                    let param = self.get_address_value(&opcode);
                    self.cpy(param);
                },
                "DEC" => {
                    let address = self.get_operand_address(&opcode.addressing_mode);
                    self.dec(address);
                },
                "DEX" => self.dex(),
                "DEY" => self.dey(),
                "EOR" => {
                    let param = self.get_address_value(&opcode);
                    self.eor(param);
                },
                "INC" => {
                    let address = self.get_operand_address(&opcode.addressing_mode);
                    self.inc(address);
                },
                "INX" => self.inx(),
                "INY" => self.iny(),
                "JMP" => todo!(),
                "JSR" => todo!(),
                "LDA" => {
                    let param = self.get_address_value(&opcode);
                    self.lda(param);
                },
                "LDX" => {
                    let param = self.get_address_value(&opcode);
                    self.ldx(param);
                },
                "LDY" => {
                    let param = self.get_address_value(&opcode);
                    self.ldy(param);
                },
                "LSR" => todo!(),
                "NOP" => {},
                "ORA" => {
                    let param = self.get_address_value(&opcode);
                    self.ora(param);
                },
                "PHA" => self.pha(),
                "PHP" => self.php(),
                "PLA" => self.pla(),
                "PLP" => self.plp(),
                "ROL" => {
                    if let AddressingMode::NoneAddressing = opcode.addressing_mode {
                        self.rol(true, 0x0000);
                    }
                    else {
                        let address = self.get_operand_address(&opcode.addressing_mode);
                        self.rol(false, address);
                    }
                },
                "ROR" => todo!(),
                "RTI" => self.rti(),
                "RTS" => self.rts(),
                "SBC" => todo!(),
                "SEC" => self.sec(),
                "SED" => self.sed(),
                "SEI" => self.sei(),
                "STA" => {
                    let address = self.get_operand_address(&opcode.addressing_mode);
                    self.sta(address);
                },
                "STX" => {
                    let address = self.get_operand_address(&opcode.addressing_mode);
                    self.stx(address);
                },
                "STY" => {
                    let address = self.get_operand_address(&opcode.addressing_mode);
                    self.sty(address);
                },
                "TAX" => self.tax(),
                "TAY" => self.tay(),
                "TSX" => self.tsx(),
                "TXA" => self.txa(),
                "TXS" => self.txs(),
                "TYA" => self.tya(),
                _ => panic!("Found unknown opcode: {}", opcode.name),
            }
            self.program_counter += (opcode.bytes - 1) as u16;
        }
    }
}
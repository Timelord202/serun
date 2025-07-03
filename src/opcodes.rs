use std::collections::HashMap;
use std::sync::LazyLock;

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,
    Absolute,
    Absolute_X,
    Absolute_Y,
    Indirect_X,
    Indirect_Y,
    NoneAddressing,
}

pub struct Opcode<'a> {
    hex: u8,
    name: &'a str,
    bytes: u8,
    addressing_mode: AddressingMode
}

impl Opcode<'_> {
    fn new(hex: u8, name: &str, bytes: u8, addressing_mode: AddressingMode) -> Opcode<'_> {
        Opcode {
            hex,
            name,
            bytes,
            addressing_mode
        }
    }
}

pub static CPU_OPCODES: LazyLock<HashMap<u8, Opcode<'_>>> = LazyLock::new(|| {
    HashMap::from([

        // ADC
        (0x69, Opcode::new(0x69, "ADC", 2, AddressingMode::Immediate)),
        (0x65, Opcode::new(0x65, "ADC", 2, AddressingMode::ZeroPage)),
        (0x75, Opcode::new(0x75, "ADC", 2, AddressingMode::ZeroPage_X)),
        (0x6D, Opcode::new(0x6D, "ADC", 3, AddressingMode::Absolute)),
        (0x7D, Opcode::new(0x7D, "ADC", 3, AddressingMode::Absolute_X)),
        (0x79, Opcode::new(0x79, "ADC", 3, AddressingMode::Absolute_Y)),
        (0x61, Opcode::new(0x61, "ADC", 2, AddressingMode::Indirect_X)),
        (0x71, Opcode::new(0x71, "ADC", 2, AddressingMode::Indirect_Y)),

        // AND
        
    ])
});
use phf::phf_map;


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

pub enum Opcode {
    ADC { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
    AND { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
    ASL { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
    BCC { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
    BCS { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
    BEQ { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
    BIT { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
    BMI { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
    BNE { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
    BPL { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
    BRK { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
    BVC { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
    BVS { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
    CLC { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
    CLD { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
    CLI { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
    CLV { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
    CMP { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
    CPX { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
    CPY { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
    DEC { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
    DEX { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
    DEY { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
    EOR { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
    INC { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
    INX { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
    INY { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
    JMP { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
    JSR { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
    LDA { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
    LDX { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
    LDY { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
    LSR { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
    NOP { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
    ORA { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
    PHA { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
    PHP { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
    PLA { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
    PLP { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
    ROL { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
    ROR { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
    RIT { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
    RTS { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
    SBC { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
    SEC { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
    SED { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
    SEI { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
    STA { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
    STX { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
    STY { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
    TAX { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
    TAY { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
    TSX { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
    TXA { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
    TXS { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
    TYA { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
}

pub static CPU_OPCODES: phf::Map<u8, Opcode> = phf_map! {

        // ADC
        0x69u8 => Opcode::ADC { bytes: 2, cycles: 2, addressing_mode: AddressingMode::Immediate },
        0x65u8 => Opcode::ADC { bytes: 2, cycles: 3, addressing_mode: AddressingMode::ZeroPage },
        0x75u8 => Opcode::ADC { bytes: 2, cycles: 4, addressing_mode: AddressingMode::ZeroPage_X },
        0x6Du8 => Opcode::ADC { bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute },
        0x7Du8 => Opcode::ADC { bytes: 3, cycles: 4,addressing_mode: AddressingMode::Absolute_X },      // (+1 if page crossed)
        0x79u8 => Opcode::ADC { bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute_Y },     // (+1 if page crossed)
        0x61u8 => Opcode::ADC { bytes: 2, cycles: 6, addressing_mode: AddressingMode::Indirect_X },
        0x71u8 => Opcode::ADC { bytes: 2, cycles: 5, addressing_mode: AddressingMode::Indirect_Y },     // (+1 if page crossed)

        // AND
        0x29u8 => Opcode::AND { bytes: 2, cycles: 2, addressing_mode: AddressingMode::Immediate },
        0x25u8 => Opcode::AND { bytes: 2, cycles: 3, addressing_mode: AddressingMode::ZeroPage },
        0x35u8 => Opcode::AND { bytes: 2, cycles: 4, addressing_mode: AddressingMode::ZeroPage_X },
        0x2Du8 => Opcode::AND { bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute },
        0x3Du8 => Opcode::AND { bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute_X },     // (+1 if page crossed)
        0x39u8 => Opcode::AND { bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute_Y },     // (+1 if page crossed)
        0x21u8 => Opcode::AND { bytes: 2, cycles: 6, addressing_mode: AddressingMode::Indirect_X },
        0x31u8 => Opcode::AND { bytes: 2, cycles: 5, addressing_mode: AddressingMode::Indirect_Y },     // (+1 if page crossed)

        // ASL
        0x0Au8 => Opcode::ASL { bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },
        0x06u8 => Opcode::ASL { bytes: 2, cycles: 5, addressing_mode: AddressingMode::ZeroPage },
        0x16u8 => Opcode::ASL { bytes: 2, cycles: 6, addressing_mode: AddressingMode::ZeroPage_X },
        0x0Eu8 => Opcode::ASL { bytes: 3, cycles: 6, addressing_mode: AddressingMode::Absolute },
        0x1Eu8 => Opcode::ASL { bytes: 3, cycles: 7, addressing_mode: AddressingMode::Absolute_X }
};
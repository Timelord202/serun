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
    Accumulator,
    NoneAddressing,
}

#[derive(Debug)]
pub enum Opcode {
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    JMP,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SED,
    SEI,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA
}

#[derive(Debug)]
pub struct Instruction {
    pub opcode: Opcode,
    pub bytes: u8,
    pub cycles: u8,
    pub addressing_mode: AddressingMode
}

pub static CPU_OPCODES: phf::Map<u8, Instruction> = phf_map! {

        // ADC
        0x69u8 => Instruction { opcode: Opcode::ADC, bytes: 2, cycles: 2, addressing_mode: AddressingMode::Immediate },
        0x65u8 => Instruction { opcode: Opcode::ADC, bytes: 2, cycles: 3, addressing_mode: AddressingMode::ZeroPage },
        0x75u8 => Instruction { opcode: Opcode::ADC, bytes: 2, cycles: 4, addressing_mode: AddressingMode::ZeroPage_X },
        0x6Du8 => Instruction { opcode: Opcode::ADC, bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute },
        0x7Du8 => Instruction { opcode: Opcode::ADC, bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute_X },         // (+1 if page crossed)
        0x79u8 => Instruction { opcode: Opcode::ADC, bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute_Y },         // (+1 if page crossed)
        0x61u8 => Instruction { opcode: Opcode::ADC, bytes: 2, cycles: 6, addressing_mode: AddressingMode::Indirect_X },
        0x71u8 => Instruction { opcode: Opcode::ADC, bytes: 2, cycles: 5, addressing_mode: AddressingMode::Indirect_Y },         // (+1 if page crossed)

        // AND
        0x29u8 => Instruction { opcode: Opcode::AND, bytes: 2, cycles: 2, addressing_mode: AddressingMode::Immediate },
        0x25u8 => Instruction { opcode: Opcode::AND, bytes: 2, cycles: 3, addressing_mode: AddressingMode::ZeroPage },
        0x35u8 => Instruction { opcode: Opcode::AND, bytes: 2, cycles: 4, addressing_mode: AddressingMode::ZeroPage_X },
        0x2Du8 => Instruction { opcode: Opcode::AND, bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute },
        0x3Du8 => Instruction { opcode: Opcode::AND, bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute_X },         // (+1 if page crossed)
        0x39u8 => Instruction { opcode: Opcode::AND, bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute_Y },         // (+1 if page crossed)
        0x21u8 => Instruction { opcode: Opcode::AND, bytes: 2, cycles: 6, addressing_mode: AddressingMode::Indirect_X },
        0x31u8 => Instruction { opcode: Opcode::AND, bytes: 2, cycles: 5, addressing_mode: AddressingMode::Indirect_Y },         // (+1 if page crossed)

        // ASL
        0x0Au8 => Instruction { opcode: Opcode::ASL, bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },
        0x06u8 => Instruction { opcode: Opcode::ASL, bytes: 2, cycles: 5, addressing_mode: AddressingMode::ZeroPage },
        0x16u8 => Instruction { opcode: Opcode::ASL, bytes: 2, cycles: 6, addressing_mode: AddressingMode::ZeroPage_X },
        0x0Eu8 => Instruction { opcode: Opcode::ASL, bytes: 3, cycles: 6, addressing_mode: AddressingMode::Absolute },
        0x1Eu8 => Instruction { opcode: Opcode::ASL, bytes: 3, cycles: 7, addressing_mode: AddressingMode::Absolute_X },

        // BCC
        0x90u8 => Instruction { opcode: Opcode::BCC, bytes: 2, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },     // (+1 if branch succeeds, +2 if to a new page)

        // BCS
        0xB0u8 => Instruction { opcode: Opcode::BCS, bytes: 2, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },     // (+1 if branch succeeds, +2 if to a new page)

        // BEQ
        0xF0u8 => Instruction { opcode: Opcode::BEQ, bytes: 2, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },     // (+1 if branch succeeds, +2 if to a new page)

        // BIT
        0x24u8 => Instruction { opcode: Opcode::BIT, bytes: 2, cycles: 3, addressing_mode: AddressingMode::ZeroPage },
        0x2Cu8 => Instruction { opcode: Opcode::BIT, bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute },

        // BMI
        0x30u8 => Instruction { opcode: Opcode::BMI, bytes: 2, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },     // (+1 if branch succeeds, +2 if to a new page)

        // BNE
        0xD0u8 => Instruction { opcode: Opcode::BNE, bytes: 2, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },     // (+1 if branch succeeds, +2 if to a new page)

        // BPL
        0x10u8 => Instruction { opcode: Opcode::BPL, bytes: 2, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },     // (+1 if branch succeeds, +2 if to a new page)

        // BRK
        0x00u8 => Instruction { opcode: Opcode::BRK, bytes: 1, cycles: 7, addressing_mode: AddressingMode::NoneAddressing },

        // BVC
        0x50u8 => Instruction { opcode: Opcode::BVC, bytes: 2, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },     // (+1 if branch succeeds, +2 if to a new page)

        // BVS
        0x70u8 => Instruction { opcode: Opcode::BVS, bytes: 2, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },     // (+1 if branch succeeds, +2 if to a new page)

        // CLC
        0x18u8 => Instruction { opcode: Opcode::CLC, bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },

        // CLD
        0xD8u8 => Instruction { opcode: Opcode::CLD, bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },

        // CLI
        0x58u8 => Instruction { opcode: Opcode::CLI, bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },
        
        // CLV
        0xB8u8 => Instruction { opcode: Opcode::CLV, bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },

        // CMP
        0xC9u8 => Instruction { opcode: Opcode::CMP, bytes: 2, cycles: 2, addressing_mode: AddressingMode::Immediate },
        0xC5u8 => Instruction { opcode: Opcode::CMP, bytes: 2, cycles: 3, addressing_mode: AddressingMode::ZeroPage },
        0xD5u8 => Instruction { opcode: Opcode::CMP, bytes: 2, cycles: 4, addressing_mode: AddressingMode::ZeroPage_X },
        0xCDu8 => Instruction { opcode: Opcode::CMP, bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute },
        0xDDu8 => Instruction { opcode: Opcode::CMP, bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute_X },         // (+1 if page crossed)
        0xD9u8 => Instruction { opcode: Opcode::CMP, bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute_Y },         // (+1 if page crossed)
        0xC1u8 => Instruction { opcode: Opcode::CMP, bytes: 2, cycles: 6, addressing_mode: AddressingMode::Indirect_X },
        0xD1u8 => Instruction { opcode: Opcode::CMP, bytes: 2, cycles: 5, addressing_mode: AddressingMode::Indirect_Y },         // (+1 if page crossed)

        // CPX
        0xE0u8 => Instruction { opcode: Opcode::CPX, bytes: 2, cycles: 2, addressing_mode: AddressingMode::Immediate },
        0xE4u8 => Instruction { opcode: Opcode::CPX, bytes: 2, cycles: 3, addressing_mode: AddressingMode::ZeroPage },
        0xECu8 => Instruction { opcode: Opcode::CPX, bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute },

        // CPY
        0xC0u8 => Instruction { opcode: Opcode::CPY, bytes: 2, cycles: 2, addressing_mode: AddressingMode::Immediate },
        0xC4u8 => Instruction { opcode: Opcode::CPY, bytes: 2, cycles: 3, addressing_mode: AddressingMode::ZeroPage },
        0xCCu8 => Instruction { opcode: Opcode::CPY, bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute },

        // DEC
        0xC6u8 => Instruction { opcode: Opcode::DEC, bytes: 2, cycles: 5, addressing_mode: AddressingMode::ZeroPage },
        0xD6u8 => Instruction { opcode: Opcode::DEC, bytes: 2, cycles: 6, addressing_mode: AddressingMode::ZeroPage_X },
        0xCEu8 => Instruction { opcode: Opcode::DEC, bytes: 3, cycles: 6, addressing_mode: AddressingMode::Absolute },
        0xDEu8 => Instruction { opcode: Opcode::DEC, bytes: 3, cycles: 7, addressing_mode: AddressingMode::Absolute_X },

        // DEX
        0xCAu8 => Instruction { opcode: Opcode::DEX, bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },

        // DEY
        0x88u8 => Instruction { opcode: Opcode::DEY, bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },

        // EOR
        0x49u8 => Instruction { opcode: Opcode::EOR, bytes: 2, cycles: 2, addressing_mode: AddressingMode::Immediate },
        0x45u8 => Instruction { opcode: Opcode::EOR, bytes: 2, cycles: 3, addressing_mode: AddressingMode::ZeroPage },
        0x55u8 => Instruction { opcode: Opcode::EOR, bytes: 2, cycles: 4, addressing_mode: AddressingMode::ZeroPage_X },
        0x4Du8 => Instruction { opcode: Opcode::EOR, bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute },
        0x5Du8 => Instruction { opcode: Opcode::EOR, bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute_X },         // (+1 if page crossed)
        0x59u8 => Instruction { opcode: Opcode::EOR, bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute_Y },         // (+1 if page crossed)
        0x41u8 => Instruction { opcode: Opcode::EOR, bytes: 2, cycles: 6, addressing_mode: AddressingMode::Indirect_X },
        0x51u8 => Instruction { opcode: Opcode::EOR, bytes: 2, cycles: 5, addressing_mode: AddressingMode::Indirect_Y },         // (+1 if page crossed)

        // INC
        0xE6u8 => Instruction { opcode: Opcode::INC, bytes: 2, cycles: 5, addressing_mode: AddressingMode::ZeroPage },
        0xF6u8 => Instruction { opcode: Opcode::INC, bytes: 2, cycles: 6, addressing_mode: AddressingMode::ZeroPage_X },
        0xEEu8 => Instruction { opcode: Opcode::INC, bytes: 3, cycles: 6, addressing_mode: AddressingMode::Absolute },
        0xFEu8 => Instruction { opcode: Opcode::INC, bytes: 3, cycles: 7, addressing_mode: AddressingMode::Absolute_X },

        // INX
        0xE8u8 => Instruction { opcode: Opcode::INX, bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },

        // INY
        0xC8u8 => Instruction { opcode: Opcode::INY, bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },

        // JMP
        0x4Cu8 => Instruction { opcode: Opcode::JMP, bytes: 3, cycles: 3, addressing_mode: AddressingMode::Absolute },
        0x6Cu8 => Instruction { opcode: Opcode::JMP, bytes: 3, cycles: 5, addressing_mode: AddressingMode::NoneAddressing },

        // JSR
        0x20u8 => Instruction { opcode: Opcode::JSR, bytes: 3, cycles: 6, addressing_mode: AddressingMode::Absolute },

        // LDA
        0xA9u8 => Instruction { opcode: Opcode::LDA, bytes: 2, cycles: 2, addressing_mode: AddressingMode::Immediate },
        0xA5u8 => Instruction { opcode: Opcode::LDA, bytes: 2, cycles: 3, addressing_mode: AddressingMode::ZeroPage },
        0xB5u8 => Instruction { opcode: Opcode::LDA, bytes: 2, cycles: 4, addressing_mode: AddressingMode::ZeroPage_X },
        0xADu8 => Instruction { opcode: Opcode::LDA, bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute },
        0xBDu8 => Instruction { opcode: Opcode::LDA, bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute_X },         // (+1 if page crossed)
        0xB9u8 => Instruction { opcode: Opcode::LDA, bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute_Y },         // (+1 if page crossed)
        0xA1u8 => Instruction { opcode: Opcode::LDA, bytes: 2, cycles: 6, addressing_mode: AddressingMode::Indirect_X },
        0xB1u8 => Instruction { opcode: Opcode::LDA, bytes: 2, cycles: 5, addressing_mode: AddressingMode::Indirect_Y },         // (+1 if page crossed)

        // LDX
        0xA2u8 => Instruction { opcode: Opcode::LDX, bytes: 2, cycles: 2, addressing_mode: AddressingMode::Immediate },
        0xA6u8 => Instruction { opcode: Opcode::LDX, bytes: 2, cycles: 3, addressing_mode: AddressingMode::ZeroPage },
        0xB6u8 => Instruction { opcode: Opcode::LDX, bytes: 2, cycles: 4, addressing_mode: AddressingMode::ZeroPage_Y },
        0xAEu8 => Instruction { opcode: Opcode::LDX, bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute },
        0xBEu8 => Instruction { opcode: Opcode::LDX, bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute_Y },          // (+1 if page crossed)

        // LDY
        0xA0u8 => Instruction { opcode: Opcode::LDY, bytes: 2, cycles: 2, addressing_mode: AddressingMode::Immediate },
        0xA4u8 => Instruction { opcode: Opcode::LDY, bytes: 2, cycles: 3, addressing_mode: AddressingMode::ZeroPage },
        0xB4u8 => Instruction { opcode: Opcode::LDY, bytes: 2, cycles: 4, addressing_mode: AddressingMode::ZeroPage_Y },
        0xACu8 => Instruction { opcode: Opcode::LDY, bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute },
        0xBCu8 => Instruction { opcode: Opcode::LDY, bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute_Y },          // (+1 if page crossed)

        // LSR
        0x4Au8 => Instruction { opcode: Opcode::LSR, bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },
        0x46u8 => Instruction { opcode: Opcode::LSR, bytes: 2, cycles: 5, addressing_mode: AddressingMode::ZeroPage },
        0x56u8 => Instruction { opcode: Opcode::LSR, bytes: 2, cycles: 6, addressing_mode: AddressingMode::ZeroPage_X },
        0x4Eu8 => Instruction { opcode: Opcode::LSR, bytes: 3, cycles: 6, addressing_mode: AddressingMode::Absolute },
        0x5Eu8 => Instruction { opcode: Opcode::LSR, bytes: 3, cycles: 7, addressing_mode: AddressingMode::Absolute_X },

        // NOP
        0xEAu8 => Instruction { opcode: Opcode::NOP, bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },

        // ORA
        0x09u8 => Instruction { opcode: Opcode::ORA, bytes: 2, cycles: 2, addressing_mode: AddressingMode::Immediate },
        0x05u8 => Instruction { opcode: Opcode::ORA, bytes: 2, cycles: 3, addressing_mode: AddressingMode::ZeroPage },
        0x15u8 => Instruction { opcode: Opcode::ORA, bytes: 2, cycles: 4, addressing_mode: AddressingMode::ZeroPage_X },
        0x0Du8 => Instruction { opcode: Opcode::ORA, bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute },
        0x1Du8 => Instruction { opcode: Opcode::ORA, bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute_X },         // (+1 if page crossed)
        0x19u8 => Instruction { opcode: Opcode::ORA, bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute_Y },         // (+1 if page crossed)
        0x01u8 => Instruction { opcode: Opcode::ORA, bytes: 2, cycles: 6, addressing_mode: AddressingMode::Indirect_X },
        0x11u8 => Instruction { opcode: Opcode::ORA, bytes: 2, cycles: 5, addressing_mode: AddressingMode::Indirect_Y },         // (+1 if page crossed)

        // PHA
        0x48u8 => Instruction { opcode: Opcode::PHA, bytes: 1, cycles: 3, addressing_mode: AddressingMode::NoneAddressing },

        // PHP
        0x08u8 => Instruction { opcode: Opcode::PHP, bytes: 1, cycles: 3, addressing_mode: AddressingMode::NoneAddressing },

        // PLA
        0x68u8 => Instruction { opcode: Opcode::PLA, bytes: 1, cycles: 4, addressing_mode: AddressingMode::NoneAddressing },

        // PLP
        0x28u8 => Instruction { opcode: Opcode::PLP, bytes: 1, cycles: 4, addressing_mode: AddressingMode::NoneAddressing },

        // ROL
        0x2Au8 => Instruction { opcode: Opcode::ROL, bytes: 1, cycles: 2, addressing_mode: AddressingMode::Accumulator },
        0x26u8 => Instruction { opcode: Opcode::ROL, bytes: 2, cycles: 5, addressing_mode: AddressingMode::ZeroPage },
        0x36u8 => Instruction { opcode: Opcode::ROL, bytes: 2, cycles: 6, addressing_mode: AddressingMode::ZeroPage_X },
        0x2Eu8 => Instruction { opcode: Opcode::ROL, bytes: 3, cycles: 6, addressing_mode: AddressingMode::Absolute },
        0x3Eu8 => Instruction { opcode: Opcode::ROL, bytes: 3, cycles: 7, addressing_mode: AddressingMode::Absolute_X },

        // ROR
        0x6Au8 => Instruction { opcode: Opcode::ROR, bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },
        0x66u8 => Instruction { opcode: Opcode::ROR, bytes: 2, cycles: 5, addressing_mode: AddressingMode::ZeroPage },
        0x76u8 => Instruction { opcode: Opcode::ROR, bytes: 2, cycles: 6, addressing_mode: AddressingMode::ZeroPage_X },
        0x6Eu8 => Instruction { opcode: Opcode::ROR, bytes: 3, cycles: 6, addressing_mode: AddressingMode::Absolute },
        0x7Eu8 => Instruction { opcode: Opcode::ROR, bytes: 3, cycles: 7, addressing_mode: AddressingMode::Absolute_X },

        // RTI
        0x40u8 => Instruction { opcode: Opcode::RTI, bytes: 1, cycles: 6, addressing_mode: AddressingMode::NoneAddressing },

        // RTS
        0x60u8 => Instruction { opcode: Opcode::RTS, bytes: 1, cycles: 6, addressing_mode: AddressingMode::NoneAddressing },

        // SBC
        0xE9u8 => Instruction { opcode: Opcode::SBC, bytes: 2, cycles: 2, addressing_mode: AddressingMode::Immediate },
        0xE5u8 => Instruction { opcode: Opcode::SBC, bytes: 2, cycles: 3, addressing_mode: AddressingMode::ZeroPage },
        0xF5u8 => Instruction { opcode: Opcode::SBC, bytes: 2, cycles: 4, addressing_mode: AddressingMode::ZeroPage_X },
        0xEDu8 => Instruction { opcode: Opcode::SBC, bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute },
        0xFDu8 => Instruction { opcode: Opcode::SBC, bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute_X },         // (+1 if page crossed)
        0xF9u8 => Instruction { opcode: Opcode::SBC, bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute_Y },         // (+1 if page crossed)
        0xE1u8 => Instruction { opcode: Opcode::SBC, bytes: 2, cycles: 6, addressing_mode: AddressingMode::Indirect_X },
        0xF1u8 => Instruction { opcode: Opcode::SBC, bytes: 2, cycles: 5, addressing_mode: AddressingMode::Indirect_Y },         // (+1 if page crossed)

        // SEC
        0x38u8 => Instruction { opcode: Opcode::SEC, bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },

        // SED
        0xF8u8 => Instruction { opcode: Opcode::SED, bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },

        // SEI
        0x78u8 => Instruction { opcode: Opcode::SEI, bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },

        // STA
        0x85u8 => Instruction { opcode: Opcode::STA, bytes: 2, cycles: 3, addressing_mode: AddressingMode::ZeroPage },
        0x95u8 => Instruction { opcode: Opcode::STA, bytes: 2, cycles: 4, addressing_mode: AddressingMode::ZeroPage_X },
        0x8Du8 => Instruction { opcode: Opcode::STA, bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute },
        0x9Du8 => Instruction { opcode: Opcode::STA, bytes: 3, cycles: 5, addressing_mode: AddressingMode::Absolute_X },
        0x99u8 => Instruction { opcode: Opcode::STA, bytes: 3, cycles: 5, addressing_mode: AddressingMode::Absolute_Y },
        0x81u8 => Instruction { opcode: Opcode::STA, bytes: 2, cycles: 6, addressing_mode: AddressingMode::Indirect_X },
        0x91u8 => Instruction { opcode: Opcode::STA, bytes: 2, cycles: 6, addressing_mode: AddressingMode::Indirect_Y },

        // STX
        0x86u8 => Instruction { opcode: Opcode::STX, bytes: 2, cycles: 3, addressing_mode: AddressingMode::ZeroPage },
        0x96u8 => Instruction { opcode: Opcode::STX, bytes: 2, cycles: 4, addressing_mode: AddressingMode::ZeroPage_Y },
        0x8Eu8 => Instruction { opcode: Opcode::STX, bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute },

        // STY
        0x84u8 => Instruction { opcode: Opcode::STY, bytes: 2, cycles: 3, addressing_mode: AddressingMode::ZeroPage },
        0x94u8 => Instruction { opcode: Opcode::STY, bytes: 2, cycles: 4, addressing_mode: AddressingMode::ZeroPage_Y },
        0x8Cu8 => Instruction { opcode: Opcode::STY, bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute },

        // TAX
        0xAAu8 => Instruction { opcode: Opcode::TAX, bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },

        // TAY
        0xA8u8 => Instruction { opcode: Opcode::TAY, bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },

        // TSX
        0xBAu8 => Instruction { opcode: Opcode::TSX, bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },

        // TXA
        0x8Au8 => Instruction { opcode: Opcode::TXA, bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },

        // TXS
        0x9Au8 => Instruction { opcode: Opcode::TSX, bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },

        // TYA
        0x98u8 => Instruction { opcode: Opcode::TYA, bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },
    };
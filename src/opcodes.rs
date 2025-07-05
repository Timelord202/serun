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
    RTI { bytes: u8, cycles: u8, addressing_mode: AddressingMode },
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
        0x7Du8 => Opcode::ADC { bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute_X },         // (+1 if page crossed)
        0x79u8 => Opcode::ADC { bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute_Y },         // (+1 if page crossed)
        0x61u8 => Opcode::ADC { bytes: 2, cycles: 6, addressing_mode: AddressingMode::Indirect_X },
        0x71u8 => Opcode::ADC { bytes: 2, cycles: 5, addressing_mode: AddressingMode::Indirect_Y },         // (+1 if page crossed)

        // AND
        0x29u8 => Opcode::AND { bytes: 2, cycles: 2, addressing_mode: AddressingMode::Immediate },
        0x25u8 => Opcode::AND { bytes: 2, cycles: 3, addressing_mode: AddressingMode::ZeroPage },
        0x35u8 => Opcode::AND { bytes: 2, cycles: 4, addressing_mode: AddressingMode::ZeroPage_X },
        0x2Du8 => Opcode::AND { bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute },
        0x3Du8 => Opcode::AND { bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute_X },         // (+1 if page crossed)
        0x39u8 => Opcode::AND { bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute_Y },         // (+1 if page crossed)
        0x21u8 => Opcode::AND { bytes: 2, cycles: 6, addressing_mode: AddressingMode::Indirect_X },
        0x31u8 => Opcode::AND { bytes: 2, cycles: 5, addressing_mode: AddressingMode::Indirect_Y },         // (+1 if page crossed)

        // ASL
        0x0Au8 => Opcode::ASL { bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },
        0x06u8 => Opcode::ASL { bytes: 2, cycles: 5, addressing_mode: AddressingMode::ZeroPage },
        0x16u8 => Opcode::ASL { bytes: 2, cycles: 6, addressing_mode: AddressingMode::ZeroPage_X },
        0x0Eu8 => Opcode::ASL { bytes: 3, cycles: 6, addressing_mode: AddressingMode::Absolute },
        0x1Eu8 => Opcode::ASL { bytes: 3, cycles: 7, addressing_mode: AddressingMode::Absolute_X },

        // BCC
        0x90u8 => Opcode::BCC { bytes: 2, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },     // (+1 if branch succeeds, +2 if to a new page)

        // BCS
        0xB0u8 => Opcode::BCS { bytes: 2, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },     // (+1 if branch succeeds, +2 if to a new page)

        // BEQ
        0xF0u8 => Opcode::BEQ { bytes: 2, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },     // (+1 if branch succeeds, +2 if to a new page)

        // BIT
        0x24u8 => Opcode::BIT { bytes: 2, cycles: 3, addressing_mode: AddressingMode::ZeroPage },
        0x2Cu8 => Opcode::BIT { bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute },

        // BMI
        0x30u8 => Opcode::BMI { bytes: 2, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },     // (+1 if branch succeeds, +2 if to a new page)

        // BNE
        0xD0u8 => Opcode::BNE { bytes: 2, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },     // (+1 if branch succeeds, +2 if to a new page)

        // BPL
        0x10u8 => Opcode::BNE { bytes: 2, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },     // (+1 if branch succeeds, +2 if to a new page)

        // BRK
        0x00u8 => Opcode::BRK { bytes: 1, cycles: 7, addressing_mode: AddressingMode::NoneAddressing },

        // BVC
        0x50u8 => Opcode::BVC { bytes: 2, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },     // (+1 if branch succeeds, +2 if to a new page)

        // BVS
        0x70u8 => Opcode::BVS { bytes: 2, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },     // (+1 if branch succeeds, +2 if to a new page)

        // CLC
        0x18u8 => Opcode::CLC { bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },

        // CLD
        0xD8u8 => Opcode::CLD { bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },

        // CLI
        0x58u8 => Opcode::CLI { bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },
        
        // CLV
        0xB8u8 => Opcode::CLV { bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },

        // CMP
        0xC9u8 => Opcode::CMP { bytes: 2, cycles: 2, addressing_mode: AddressingMode::Immediate },
        0xC5u8 => Opcode::CMP { bytes: 2, cycles: 3, addressing_mode: AddressingMode::ZeroPage },
        0xD5u8 => Opcode::CMP { bytes: 2, cycles: 4, addressing_mode: AddressingMode::ZeroPage_X },
        0xCDu8 => Opcode::CMP { bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute },
        0xDDu8 => Opcode::CMP { bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute_X },         // (+1 if page crossed)
        0xD9u8 => Opcode::CMP { bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute_Y },         // (+1 if page crossed)
        0xC1u8 => Opcode::CMP { bytes: 2, cycles: 6, addressing_mode: AddressingMode::Indirect_X },
        0xD1u8 => Opcode::CMP { bytes: 2, cycles: 5, addressing_mode: AddressingMode::Indirect_Y },         // (+1 if page crossed)

        // CPX
        0xE0u8 => Opcode::CPX { bytes: 2, cycles: 2, addressing_mode: AddressingMode::Immediate },
        0xE4u8 => Opcode::CPX { bytes: 2, cycles: 3, addressing_mode: AddressingMode::ZeroPage },
        0xECu8 => Opcode::CPX { bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute },

        // CPY
        0xC0u8 => Opcode::CPX { bytes: 2, cycles: 2, addressing_mode: AddressingMode::Immediate },
        0xC4u8 => Opcode::CPX { bytes: 2, cycles: 3, addressing_mode: AddressingMode::ZeroPage },
        0xCCu8 => Opcode::CPX { bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute },

        // DEC
        0xC6u8 => Opcode::DEC { bytes: 2, cycles: 5, addressing_mode: AddressingMode::ZeroPage },
        0xD6u8 => Opcode::DEC { bytes: 2, cycles: 6, addressing_mode: AddressingMode::ZeroPage_X },
        0xCEu8 => Opcode::DEC { bytes: 3, cycles: 6, addressing_mode: AddressingMode::Absolute },
        0xDEu8 => Opcode::DEC { bytes: 3, cycles: 7, addressing_mode: AddressingMode::Absolute_X },

        // DEX
        0xCAu8 => Opcode::DEX { bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },

        // DEY
        0x88u8 => Opcode::DEX { bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },

        // EOR
        0x49u8 => Opcode::EOR { bytes: 2, cycles: 2, addressing_mode: AddressingMode::Immediate },
        0x45u8 => Opcode::EOR { bytes: 2, cycles: 3, addressing_mode: AddressingMode::ZeroPage },
        0x55u8 => Opcode::EOR { bytes: 2, cycles: 4, addressing_mode: AddressingMode::ZeroPage_X },
        0x4Du8 => Opcode::EOR { bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute },
        0x5Du8 => Opcode::EOR { bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute_X },         // (+1 if page crossed)
        0x59u8 => Opcode::EOR { bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute_Y },         // (+1 if page crossed)
        0x41u8 => Opcode::EOR { bytes: 2, cycles: 6, addressing_mode: AddressingMode::Indirect_X },
        0x51u8 => Opcode::EOR { bytes: 2, cycles: 5, addressing_mode: AddressingMode::Indirect_Y },         // (+1 if page crossed)

        // INC
        0xE6u8 => Opcode::INC { bytes: 2, cycles: 5, addressing_mode: AddressingMode::ZeroPage },
        0xF6u8 => Opcode::INC { bytes: 2, cycles: 6, addressing_mode: AddressingMode::ZeroPage_X },
        0xEEu8 => Opcode::INC { bytes: 3, cycles: 6, addressing_mode: AddressingMode::Absolute },
        0xFEu8 => Opcode::INC { bytes: 3, cycles: 7, addressing_mode: AddressingMode::Absolute_X },

        // INX
        0xE8u8 => Opcode::INX { bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },

        // INY
        0xC8u8 => Opcode::INY { bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },

        // JMP
        0x4Cu8 => Opcode::JMP { bytes: 3, cycles: 3, addressing_mode: AddressingMode::Absolute },
        0x6Cu8 => Opcode::JMP { bytes: 3, cycles: 5, addressing_mode: AddressingMode::NoneAddressing },

        // JSR
        0x20u8 => Opcode::JSR { bytes: 3, cycles: 6, addressing_mode: AddressingMode::Absolute },

        // LDA
        0xA9u8 => Opcode::LDA { bytes: 2, cycles: 2, addressing_mode: AddressingMode::Immediate },
        0xA5u8 => Opcode::LDA { bytes: 2, cycles: 3, addressing_mode: AddressingMode::ZeroPage },
        0xB5u8 => Opcode::LDA { bytes: 2, cycles: 4, addressing_mode: AddressingMode::ZeroPage_X },
        0xADu8 => Opcode::LDA { bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute },
        0xBDu8 => Opcode::LDA { bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute_X },         // (+1 if page crossed)
        0xB9u8 => Opcode::LDA { bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute_Y },         // (+1 if page crossed)
        0xA1u8 => Opcode::LDA { bytes: 2, cycles: 6, addressing_mode: AddressingMode::Indirect_X },
        0xB1u8 => Opcode::LDA { bytes: 2, cycles: 5, addressing_mode: AddressingMode::Indirect_Y },         // (+1 if page crossed)

        // LDX
        0xA2u8 => Opcode::LDX { bytes: 2, cycles: 2, addressing_mode: AddressingMode::Immediate },
        0xA6u8 => Opcode::LDX { bytes: 2, cycles: 3, addressing_mode: AddressingMode::ZeroPage },
        0xB6u8 => Opcode::LDX { bytes: 2, cycles: 4, addressing_mode: AddressingMode::ZeroPage_Y },
        0xAEu8 => Opcode::LDX { bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute },
        0xBEu8 => Opcode::LDX { bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute_Y },          // (+1 if page crossed)

        // LDY
        0xA0u8 => Opcode::LDY { bytes: 2, cycles: 2, addressing_mode: AddressingMode::Immediate },
        0xA4u8 => Opcode::LDY { bytes: 2, cycles: 3, addressing_mode: AddressingMode::ZeroPage },
        0xB4u8 => Opcode::LDY { bytes: 2, cycles: 4, addressing_mode: AddressingMode::ZeroPage_Y },
        0xACu8 => Opcode::LDY { bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute },
        0xBCu8 => Opcode::LDY { bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute_Y },          // (+1 if page crossed)

        // LSR
        0x4Au8 => Opcode::LSR { bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },
        0x46u8 => Opcode::LSR { bytes: 2, cycles: 5, addressing_mode: AddressingMode::ZeroPage },
        0x56u8 => Opcode::LSR { bytes: 2, cycles: 6, addressing_mode: AddressingMode::ZeroPage_X },
        0x4Eu8 => Opcode::LSR { bytes: 3, cycles: 6, addressing_mode: AddressingMode::Absolute },
        0x5Eu8 => Opcode::LSR { bytes: 3, cycles: 7, addressing_mode: AddressingMode::Absolute_X },

        // NOP
        0xEAu8 => Opcode::NOP { bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },

        // ORA
        0x09u8 => Opcode::ADC { bytes: 2, cycles: 2, addressing_mode: AddressingMode::Immediate },
        0x05u8 => Opcode::ADC { bytes: 2, cycles: 3, addressing_mode: AddressingMode::ZeroPage },
        0x15u8 => Opcode::ADC { bytes: 2, cycles: 4, addressing_mode: AddressingMode::ZeroPage_X },
        0x0Du8 => Opcode::ADC { bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute },
        0x1Du8 => Opcode::ADC { bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute_X },         // (+1 if page crossed)
        0x19u8 => Opcode::ADC { bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute_Y },         // (+1 if page crossed)
        0x01u8 => Opcode::ADC { bytes: 2, cycles: 6, addressing_mode: AddressingMode::Indirect_X },
        0x11u8 => Opcode::ADC { bytes: 2, cycles: 5, addressing_mode: AddressingMode::Indirect_Y },         // (+1 if page crossed)

        // PHA
        0x48u8 => Opcode::PHA { bytes: 1, cycles: 3, addressing_mode: AddressingMode::NoneAddressing },

        // PHP
        0x08u8 => Opcode::PHP { bytes: 1, cycles: 3, addressing_mode: AddressingMode::NoneAddressing },

        // PLA
        0x68u8 => Opcode::PLA { bytes: 1, cycles: 4, addressing_mode: AddressingMode::NoneAddressing },

        // PLP
        0x28u8 => Opcode::PLA { bytes: 1, cycles: 4, addressing_mode: AddressingMode::NoneAddressing },

        // ROL
        0x2Au8 => Opcode::ROL { bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },
        0x26u8 => Opcode::ROL { bytes: 2, cycles: 5, addressing_mode: AddressingMode::ZeroPage },
        0x36u8 => Opcode::ROL { bytes: 2, cycles: 6, addressing_mode: AddressingMode::ZeroPage_X },
        0x2Eu8 => Opcode::ROL { bytes: 3, cycles: 6, addressing_mode: AddressingMode::Absolute },
        0x3Eu8 => Opcode::ROL { bytes: 3, cycles: 7, addressing_mode: AddressingMode::Absolute_X },

        // ROR
        0x6Au8 => Opcode::ROR { bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },
        0x66u8 => Opcode::ROR { bytes: 2, cycles: 5, addressing_mode: AddressingMode::ZeroPage },
        0x76u8 => Opcode::ROR { bytes: 2, cycles: 6, addressing_mode: AddressingMode::ZeroPage_X },
        0x6Eu8 => Opcode::ROR { bytes: 3, cycles: 6, addressing_mode: AddressingMode::Absolute },
        0x7Eu8 => Opcode::ROR { bytes: 3, cycles: 7, addressing_mode: AddressingMode::Absolute_X },

        // RTI
        0x40u8 => Opcode::RTI { bytes: 1, cycles: 6, addressing_mode: AddressingMode::NoneAddressing },

        // RTS
        0x60u8 => Opcode::RTS { bytes: 1, cycles: 6, addressing_mode: AddressingMode::NoneAddressing },

        // SBC
        0xE9u8 => Opcode::SBC { bytes: 2, cycles: 2, addressing_mode: AddressingMode::Immediate },
        0xE5u8 => Opcode::SBC { bytes: 2, cycles: 3, addressing_mode: AddressingMode::ZeroPage },
        0xF5u8 => Opcode::SBC { bytes: 2, cycles: 4, addressing_mode: AddressingMode::ZeroPage_X },
        0xEDu8 => Opcode::SBC { bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute },
        0xFDu8 => Opcode::SBC { bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute_X },         // (+1 if page crossed)
        0xF9u8 => Opcode::SBC { bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute_Y },         // (+1 if page crossed)
        0xE1u8 => Opcode::SBC { bytes: 2, cycles: 6, addressing_mode: AddressingMode::Indirect_X },
        0xF1u8 => Opcode::SBC { bytes: 2, cycles: 5, addressing_mode: AddressingMode::Indirect_Y },         // (+1 if page crossed)

        // SEC
        0x38u8 => Opcode::SEC { bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },

        // SED
        0xF8u8 => Opcode::SED { bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },

        // SEI
        0x78u8 => Opcode::SEI { bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },

        // STA
        0x85u8 => Opcode::STA { bytes: 2, cycles: 3, addressing_mode: AddressingMode::ZeroPage },
        0x95u8 => Opcode::STA { bytes: 2, cycles: 4, addressing_mode: AddressingMode::ZeroPage_X },
        0x8Du8 => Opcode::STA { bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute },
        0x9Du8 => Opcode::STA { bytes: 3, cycles: 5, addressing_mode: AddressingMode::Absolute_X },
        0x99u8 => Opcode::STA { bytes: 3, cycles: 5, addressing_mode: AddressingMode::Absolute_Y },
        0x81u8 => Opcode::STA { bytes: 2, cycles: 6, addressing_mode: AddressingMode::Indirect_X },
        0x91u8 => Opcode::STA { bytes: 2, cycles: 6, addressing_mode: AddressingMode::Indirect_Y },

        // STX
        0x86u8 => Opcode::STX { bytes: 2, cycles: 3, addressing_mode: AddressingMode::ZeroPage },
        0x96u8 => Opcode::STX { bytes: 2, cycles: 4, addressing_mode: AddressingMode::ZeroPage_Y },
        0x8Eu8 => Opcode::STX { bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute },

        // STY
        0x84u8 => Opcode::STY { bytes: 2, cycles: 3, addressing_mode: AddressingMode::ZeroPage },
        0x94u8 => Opcode::STY { bytes: 2, cycles: 4, addressing_mode: AddressingMode::ZeroPage_Y },
        0x8Cu8 => Opcode::STY { bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute },

        // TAX
        0xAAu8 => Opcode::TAX { bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },

        // TAY
        0xA8u8 => Opcode::TAY { bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },

        // TSX
        0xBAu8 => Opcode::TSX { bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },

        // TXA
        0x8Au8 => Opcode::TXA { bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },

        // TXS
        0x9Au8 => Opcode::TXS { bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },

        // TYA
        0x98u8 => Opcode::TYA { bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },
    };
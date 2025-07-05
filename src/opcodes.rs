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

#[derive(Debug)]
pub struct Opcode<'a> {
    pub name: &'a str,
    pub bytes: u8,
    pub cycles: u8,
    pub addressing_mode: AddressingMode
}

pub static CPU_OPCODES: phf::Map<u8, Opcode> = phf_map! {

        // ADC
        0x69u8 => Opcode { name: "ADC", bytes: 2, cycles: 2, addressing_mode: AddressingMode::Immediate },
        0x65u8 => Opcode { name: "ADC", bytes: 2, cycles: 3, addressing_mode: AddressingMode::ZeroPage },
        0x75u8 => Opcode { name: "ADC", bytes: 2, cycles: 4, addressing_mode: AddressingMode::ZeroPage_X },
        0x6Du8 => Opcode { name: "ADC", bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute },
        0x7Du8 => Opcode { name: "ADC", bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute_X },         // (+1 if page crossed)
        0x79u8 => Opcode { name: "ADC", bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute_Y },         // (+1 if page crossed)
        0x61u8 => Opcode { name: "ADC", bytes: 2, cycles: 6, addressing_mode: AddressingMode::Indirect_X },
        0x71u8 => Opcode { name: "ADC", bytes: 2, cycles: 5, addressing_mode: AddressingMode::Indirect_Y },         // (+1 if page crossed)

        // AND
        0x29u8 => Opcode { name: "AND", bytes: 2, cycles: 2, addressing_mode: AddressingMode::Immediate },
        0x25u8 => Opcode { name: "AND", bytes: 2, cycles: 3, addressing_mode: AddressingMode::ZeroPage },
        0x35u8 => Opcode { name: "AND", bytes: 2, cycles: 4, addressing_mode: AddressingMode::ZeroPage_X },
        0x2Du8 => Opcode { name: "AND", bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute },
        0x3Du8 => Opcode { name: "AND", bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute_X },         // (+1 if page crossed)
        0x39u8 => Opcode { name: "AND", bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute_Y },         // (+1 if page crossed)
        0x21u8 => Opcode { name: "AND", bytes: 2, cycles: 6, addressing_mode: AddressingMode::Indirect_X },
        0x31u8 => Opcode { name: "AND", bytes: 2, cycles: 5, addressing_mode: AddressingMode::Indirect_Y },         // (+1 if page crossed)

        // ASL
        0x0Au8 => Opcode { name: "ASL", bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },
        0x06u8 => Opcode { name: "ASL", bytes: 2, cycles: 5, addressing_mode: AddressingMode::ZeroPage },
        0x16u8 => Opcode { name: "ASL", bytes: 2, cycles: 6, addressing_mode: AddressingMode::ZeroPage_X },
        0x0Eu8 => Opcode { name: "ASL", bytes: 3, cycles: 6, addressing_mode: AddressingMode::Absolute },
        0x1Eu8 => Opcode { name: "ASL", bytes: 3, cycles: 7, addressing_mode: AddressingMode::Absolute_X },

        // BCC
        0x90u8 => Opcode { name: "BCC", bytes: 2, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },     // (+1 if branch succeeds, +2 if to a new page)

        // BCS
        0xB0u8 => Opcode { name: "BCS", bytes: 2, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },     // (+1 if branch succeeds, +2 if to a new page)

        // BEQ
        0xF0u8 => Opcode { name: "BEQ", bytes: 2, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },     // (+1 if branch succeeds, +2 if to a new page)

        // BIT
        0x24u8 => Opcode { name: "BIT", bytes: 2, cycles: 3, addressing_mode: AddressingMode::ZeroPage },
        0x2Cu8 => Opcode { name: "BIT", bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute },

        // BMI
        0x30u8 => Opcode { name: "BMI", bytes: 2, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },     // (+1 if branch succeeds, +2 if to a new page)

        // BNE
        0xD0u8 => Opcode { name: "BNE", bytes: 2, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },     // (+1 if branch succeeds, +2 if to a new page)

        // BPL
        0x10u8 => Opcode { name: "BPL", bytes: 2, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },     // (+1 if branch succeeds, +2 if to a new page)

        // BRK
        0x00u8 => Opcode { name: "BRK", bytes: 1, cycles: 7, addressing_mode: AddressingMode::NoneAddressing },

        // BVC
        0x50u8 => Opcode { name: "BVC", bytes: 2, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },     // (+1 if branch succeeds, +2 if to a new page)

        // BVS
        0x70u8 => Opcode { name: "BVS", bytes: 2, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },     // (+1 if branch succeeds, +2 if to a new page)

        // CLC
        0x18u8 => Opcode { name: "CLC", bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },

        // CLD
        0xD8u8 => Opcode { name: "CLD", bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },

        // CLI
        0x58u8 => Opcode { name: "CLI", bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },
        
        // CLV
        0xB8u8 => Opcode { name: "CLV", bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },

        // CMP
        0xC9u8 => Opcode { name: "CMP", bytes: 2, cycles: 2, addressing_mode: AddressingMode::Immediate },
        0xC5u8 => Opcode { name: "CMP", bytes: 2, cycles: 3, addressing_mode: AddressingMode::ZeroPage },
        0xD5u8 => Opcode { name: "CMP", bytes: 2, cycles: 4, addressing_mode: AddressingMode::ZeroPage_X },
        0xCDu8 => Opcode { name: "CMP", bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute },
        0xDDu8 => Opcode { name: "CMP", bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute_X },         // (+1 if page crossed)
        0xD9u8 => Opcode { name: "CMP", bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute_Y },         // (+1 if page crossed)
        0xC1u8 => Opcode { name: "CMP", bytes: 2, cycles: 6, addressing_mode: AddressingMode::Indirect_X },
        0xD1u8 => Opcode { name: "CMP", bytes: 2, cycles: 5, addressing_mode: AddressingMode::Indirect_Y },         // (+1 if page crossed)

        // CPX
        0xE0u8 => Opcode { name: "CPX", bytes: 2, cycles: 2, addressing_mode: AddressingMode::Immediate },
        0xE4u8 => Opcode { name: "CPX", bytes: 2, cycles: 3, addressing_mode: AddressingMode::ZeroPage },
        0xECu8 => Opcode { name: "CPX", bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute },

        // CPY
        0xC0u8 => Opcode { name: "CPY", bytes: 2, cycles: 2, addressing_mode: AddressingMode::Immediate },
        0xC4u8 => Opcode { name: "CPY", bytes: 2, cycles: 3, addressing_mode: AddressingMode::ZeroPage },
        0xCCu8 => Opcode { name: "CPY", bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute },

        // DEC
        0xC6u8 => Opcode { name: "DEC", bytes: 2, cycles: 5, addressing_mode: AddressingMode::ZeroPage },
        0xD6u8 => Opcode { name: "DEC", bytes: 2, cycles: 6, addressing_mode: AddressingMode::ZeroPage_X },
        0xCEu8 => Opcode { name: "DEC", bytes: 3, cycles: 6, addressing_mode: AddressingMode::Absolute },
        0xDEu8 => Opcode { name: "DEC", bytes: 3, cycles: 7, addressing_mode: AddressingMode::Absolute_X },

        // DEX
        0xCAu8 => Opcode { name: "DEX", bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },

        // DEY
        0x88u8 => Opcode { name: "DEY", bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },

        // EOR
        0x49u8 => Opcode { name: "EOR", bytes: 2, cycles: 2, addressing_mode: AddressingMode::Immediate },
        0x45u8 => Opcode { name: "EOR", bytes: 2, cycles: 3, addressing_mode: AddressingMode::ZeroPage },
        0x55u8 => Opcode { name: "EOR", bytes: 2, cycles: 4, addressing_mode: AddressingMode::ZeroPage_X },
        0x4Du8 => Opcode { name: "EOR", bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute },
        0x5Du8 => Opcode { name: "EOR", bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute_X },         // (+1 if page crossed)
        0x59u8 => Opcode { name: "EOR", bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute_Y },         // (+1 if page crossed)
        0x41u8 => Opcode { name: "EOR", bytes: 2, cycles: 6, addressing_mode: AddressingMode::Indirect_X },
        0x51u8 => Opcode { name: "EOR", bytes: 2, cycles: 5, addressing_mode: AddressingMode::Indirect_Y },         // (+1 if page crossed)

        // INC
        0xE6u8 => Opcode { name: "INC", bytes: 2, cycles: 5, addressing_mode: AddressingMode::ZeroPage },
        0xF6u8 => Opcode { name: "INC", bytes: 2, cycles: 6, addressing_mode: AddressingMode::ZeroPage_X },
        0xEEu8 => Opcode { name: "INC", bytes: 3, cycles: 6, addressing_mode: AddressingMode::Absolute },
        0xFEu8 => Opcode { name: "INC", bytes: 3, cycles: 7, addressing_mode: AddressingMode::Absolute_X },

        // INX
        0xE8u8 => Opcode { name: "INX", bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },

        // INY
        0xC8u8 => Opcode { name: "INY", bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },

        // JMP
        0x4Cu8 => Opcode { name: "JMP", bytes: 3, cycles: 3, addressing_mode: AddressingMode::Absolute },
        0x6Cu8 => Opcode { name: "JMP", bytes: 3, cycles: 5, addressing_mode: AddressingMode::NoneAddressing },

        // JSR
        0x20u8 => Opcode { name: "JSR", bytes: 3, cycles: 6, addressing_mode: AddressingMode::Absolute },

        // LDA
        0xA9u8 => Opcode { name: "LDA", bytes: 2, cycles: 2, addressing_mode: AddressingMode::Immediate },
        0xA5u8 => Opcode { name: "LDA", bytes: 2, cycles: 3, addressing_mode: AddressingMode::ZeroPage },
        0xB5u8 => Opcode { name: "LDA", bytes: 2, cycles: 4, addressing_mode: AddressingMode::ZeroPage_X },
        0xADu8 => Opcode { name: "LDA", bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute },
        0xBDu8 => Opcode { name: "LDA", bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute_X },         // (+1 if page crossed)
        0xB9u8 => Opcode { name: "LDA", bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute_Y },         // (+1 if page crossed)
        0xA1u8 => Opcode { name: "LDA", bytes: 2, cycles: 6, addressing_mode: AddressingMode::Indirect_X },
        0xB1u8 => Opcode { name: "LDA", bytes: 2, cycles: 5, addressing_mode: AddressingMode::Indirect_Y },         // (+1 if page crossed)

        // LDX
        0xA2u8 => Opcode { name: "LDX", bytes: 2, cycles: 2, addressing_mode: AddressingMode::Immediate },
        0xA6u8 => Opcode { name: "LDX", bytes: 2, cycles: 3, addressing_mode: AddressingMode::ZeroPage },
        0xB6u8 => Opcode { name: "LDX", bytes: 2, cycles: 4, addressing_mode: AddressingMode::ZeroPage_Y },
        0xAEu8 => Opcode { name: "LDX", bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute },
        0xBEu8 => Opcode { name: "LDX", bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute_Y },          // (+1 if page crossed)

        // LDY
        0xA0u8 => Opcode { name: "LDY", bytes: 2, cycles: 2, addressing_mode: AddressingMode::Immediate },
        0xA4u8 => Opcode { name: "LDY", bytes: 2, cycles: 3, addressing_mode: AddressingMode::ZeroPage },
        0xB4u8 => Opcode { name: "LDY", bytes: 2, cycles: 4, addressing_mode: AddressingMode::ZeroPage_Y },
        0xACu8 => Opcode { name: "LDY", bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute },
        0xBCu8 => Opcode { name: "LDY", bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute_Y },          // (+1 if page crossed)

        // LSR
        0x4Au8 => Opcode { name: "LSR", bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },
        0x46u8 => Opcode { name: "LSR", bytes: 2, cycles: 5, addressing_mode: AddressingMode::ZeroPage },
        0x56u8 => Opcode { name: "LSR", bytes: 2, cycles: 6, addressing_mode: AddressingMode::ZeroPage_X },
        0x4Eu8 => Opcode { name: "LSR", bytes: 3, cycles: 6, addressing_mode: AddressingMode::Absolute },
        0x5Eu8 => Opcode { name: "LSR", bytes: 3, cycles: 7, addressing_mode: AddressingMode::Absolute_X },

        // NOP
        0xEAu8 => Opcode { name: "NOP", bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },

        // ORA
        0x09u8 => Opcode { name: "ORA", bytes: 2, cycles: 2, addressing_mode: AddressingMode::Immediate },
        0x05u8 => Opcode { name: "ORA", bytes: 2, cycles: 3, addressing_mode: AddressingMode::ZeroPage },
        0x15u8 => Opcode { name: "ORA", bytes: 2, cycles: 4, addressing_mode: AddressingMode::ZeroPage_X },
        0x0Du8 => Opcode { name: "ORA", bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute },
        0x1Du8 => Opcode { name: "ORA", bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute_X },         // (+1 if page crossed)
        0x19u8 => Opcode { name: "ORA", bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute_Y },         // (+1 if page crossed)
        0x01u8 => Opcode { name: "ORA", bytes: 2, cycles: 6, addressing_mode: AddressingMode::Indirect_X },
        0x11u8 => Opcode { name: "ORA", bytes: 2, cycles: 5, addressing_mode: AddressingMode::Indirect_Y },         // (+1 if page crossed)

        // PHA
        0x48u8 => Opcode { name: "PHA", bytes: 1, cycles: 3, addressing_mode: AddressingMode::NoneAddressing },

        // PHP
        0x08u8 => Opcode { name: "PHP", bytes: 1, cycles: 3, addressing_mode: AddressingMode::NoneAddressing },

        // PLA
        0x68u8 => Opcode { name: "PLA", bytes: 1, cycles: 4, addressing_mode: AddressingMode::NoneAddressing },

        // PLP
        0x28u8 => Opcode { name: "PLP", bytes: 1, cycles: 4, addressing_mode: AddressingMode::NoneAddressing },

        // ROL
        0x2Au8 => Opcode { name: "ROL", bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },
        0x26u8 => Opcode { name: "ROL", bytes: 2, cycles: 5, addressing_mode: AddressingMode::ZeroPage },
        0x36u8 => Opcode { name: "ROL", bytes: 2, cycles: 6, addressing_mode: AddressingMode::ZeroPage_X },
        0x2Eu8 => Opcode { name: "ROL", bytes: 3, cycles: 6, addressing_mode: AddressingMode::Absolute },
        0x3Eu8 => Opcode { name: "ROL", bytes: 3, cycles: 7, addressing_mode: AddressingMode::Absolute_X },

        // ROR
        0x6Au8 => Opcode { name: "ROR", bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },
        0x66u8 => Opcode { name: "ROR", bytes: 2, cycles: 5, addressing_mode: AddressingMode::ZeroPage },
        0x76u8 => Opcode { name: "ROR", bytes: 2, cycles: 6, addressing_mode: AddressingMode::ZeroPage_X },
        0x6Eu8 => Opcode { name: "ROR", bytes: 3, cycles: 6, addressing_mode: AddressingMode::Absolute },
        0x7Eu8 => Opcode { name: "ROR", bytes: 3, cycles: 7, addressing_mode: AddressingMode::Absolute_X },

        // RTI
        0x40u8 => Opcode { name: "RTI", bytes: 1, cycles: 6, addressing_mode: AddressingMode::NoneAddressing },

        // RTS
        0x60u8 => Opcode { name: "RTS", bytes: 1, cycles: 6, addressing_mode: AddressingMode::NoneAddressing },

        // SBC
        0xE9u8 => Opcode { name: "SBC", bytes: 2, cycles: 2, addressing_mode: AddressingMode::Immediate },
        0xE5u8 => Opcode { name: "SBC", bytes: 2, cycles: 3, addressing_mode: AddressingMode::ZeroPage },
        0xF5u8 => Opcode { name: "SBC", bytes: 2, cycles: 4, addressing_mode: AddressingMode::ZeroPage_X },
        0xEDu8 => Opcode { name: "SBC", bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute },
        0xFDu8 => Opcode { name: "SBC", bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute_X },         // (+1 if page crossed)
        0xF9u8 => Opcode { name: "SBC", bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute_Y },         // (+1 if page crossed)
        0xE1u8 => Opcode { name: "SBC", bytes: 2, cycles: 6, addressing_mode: AddressingMode::Indirect_X },
        0xF1u8 => Opcode { name: "SBC", bytes: 2, cycles: 5, addressing_mode: AddressingMode::Indirect_Y },         // (+1 if page crossed)

        // SEC
        0x38u8 => Opcode { name: "SEC", bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },

        // SED
        0xF8u8 => Opcode { name: "SED", bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },

        // SEI
        0x78u8 => Opcode { name: "SEI", bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },

        // STA
        0x85u8 => Opcode { name: "STA", bytes: 2, cycles: 3, addressing_mode: AddressingMode::ZeroPage },
        0x95u8 => Opcode { name: "STA", bytes: 2, cycles: 4, addressing_mode: AddressingMode::ZeroPage_X },
        0x8Du8 => Opcode { name: "STA", bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute },
        0x9Du8 => Opcode { name: "STA", bytes: 3, cycles: 5, addressing_mode: AddressingMode::Absolute_X },
        0x99u8 => Opcode { name: "STA", bytes: 3, cycles: 5, addressing_mode: AddressingMode::Absolute_Y },
        0x81u8 => Opcode { name: "STA", bytes: 2, cycles: 6, addressing_mode: AddressingMode::Indirect_X },
        0x91u8 => Opcode { name: "STA", bytes: 2, cycles: 6, addressing_mode: AddressingMode::Indirect_Y },

        // STX
        0x86u8 => Opcode { name: "STX", bytes: 2, cycles: 3, addressing_mode: AddressingMode::ZeroPage },
        0x96u8 => Opcode { name: "STX", bytes: 2, cycles: 4, addressing_mode: AddressingMode::ZeroPage_Y },
        0x8Eu8 => Opcode { name: "STX", bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute },

        // STY
        0x84u8 => Opcode { name: "STY", bytes: 2, cycles: 3, addressing_mode: AddressingMode::ZeroPage },
        0x94u8 => Opcode { name: "STY", bytes: 2, cycles: 4, addressing_mode: AddressingMode::ZeroPage_Y },
        0x8Cu8 => Opcode { name: "STY", bytes: 3, cycles: 4, addressing_mode: AddressingMode::Absolute },

        // TAX
        0xAAu8 => Opcode { name: "TAX", bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },

        // TAY
        0xA8u8 => Opcode { name: "TAY", bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },

        // TSX
        0xBAu8 => Opcode { name: "TSX", bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },

        // TXA
        0x8Au8 => Opcode { name: "TXA", bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },

        // TXS
        0x9Au8 => Opcode { name: "TSX", bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },

        // TYA
        0x98u8 => Opcode { name: "TYA", bytes: 1, cycles: 2, addressing_mode: AddressingMode::NoneAddressing },
    };
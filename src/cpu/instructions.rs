use std::ops::Add;

use crate::cpu::{CPU, StatusFlag};
use crate::opcodes::{AddressingMode, Instruction};

impl CPU {

    pub fn and(&mut self, instruction: &Instruction) {
        self.register_a &= self.get_operand(instruction);
        self.update_zero_and_negative_flags(self.register_a);
    }

    pub fn asl(&mut self, instruction: &Instruction) {
        match &instruction.addressing_mode {
            AddressingMode::Accumulator => {
                let old_bit_seven = (self.register_a & 0b1000_0000) >> 7;
                self.register_a <<= 1;
                self.clear_status_flag(StatusFlag::C);
                self.status |= old_bit_seven;
                self.update_zero_and_negative_flags(self.register_a);
            },
            AddressingMode::ZeroPage | AddressingMode::ZeroPage_X | AddressingMode::Absolute | AddressingMode::Absolute_X => {
                let operand_address = self.get_operand_address(&instruction.addressing_mode);
                let mut operand = self.get_operand(instruction);
                let old_bit_seven = (operand & 0b1000_0000) >> 7;
                operand <<= 1;
                self.clear_status_flag(StatusFlag::C);
                self.status |= old_bit_seven;
                self.memory.write(operand_address, operand);
                self.update_zero_and_negative_flags(operand);
            },
            _ => {

            }
        }
    }

    pub fn clc(&mut self) {
        self.clear_status_flag(StatusFlag::C);
    }

    pub fn cli(&mut self) {
        self.clear_status_flag(StatusFlag::I);
    }

    pub fn clv(&mut self) {
        self.clear_status_flag(StatusFlag::V);
    }

    // Compare contents of a register to a given value.
    // Used in cmp, cpx and cpy instructions
    fn compare_register(&mut self, register: u8, value: u8) {
        if register >= value {
            self.set_status_flag(StatusFlag::C);
        }
        else {
            self.clear_status_flag(StatusFlag::C);
        }
        if register == value {
            self.set_status_flag(StatusFlag::Z);
        }
        else {
            self.clear_status_flag(StatusFlag::Z);
        }

        let result = register.wrapping_sub(value);
        if result & 0b1000_0000 != 0 {
            self.set_status_flag(StatusFlag::N);
        } else {
            self.clear_status_flag(StatusFlag::N);
        }
    }

    pub fn cmp(&mut self, instruction: &Instruction) {
        let value = self.get_operand(instruction);
        self.compare_register(self.register_a, value);
    }

    pub fn cpx(&mut self, instruction: &Instruction) {
        let value = self.get_operand(instruction);
        self.compare_register(self.register_x, value);
    }

    pub fn cpy(&mut self, instruction: &Instruction) {
        let value = self.get_operand(instruction);
        self.compare_register(self.register_y, value);
    }

    pub fn dec(&mut self, instruction: &Instruction) {
        let address = self.get_operand_address(&instruction.addressing_mode);
        let mem_value = self.memory.read(address);
        let result = mem_value.wrapping_sub(1);
        self.memory.write(address, result);
        self.update_zero_and_negative_flags(result);
    }

    pub fn dex(&mut self) {
        self.register_x = self.register_x.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.register_x);
    }

    pub fn dey(&mut self) {
        self.register_y = self.register_y.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.register_y);
    }

    pub fn eor(&mut self, instruction: &Instruction) {
        self.register_a ^= self.get_operand(instruction);
        self.update_zero_and_negative_flags(self.register_a);
    }

    pub fn inc(&mut self, instruction: &Instruction) {
        let address = self.get_operand_address(&instruction.addressing_mode);
        let mem_value = self.memory.read(address);
        let result = mem_value.wrapping_add(1);
        self.memory.write(address, result);
        self.update_zero_and_negative_flags(result);
    }

    pub fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_x);
    }

    pub fn iny(&mut self) {
        self.register_y = self.register_y.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_y);
    }

    pub fn lda(&mut self, instruction: &Instruction) {
        self.register_a = self.get_operand(instruction);
        self.update_zero_and_negative_flags(self.register_a);
    }

    pub fn ldx(&mut self, instruction: &Instruction) {
        self.register_x = self.get_operand(instruction);
        self.update_zero_and_negative_flags(self.register_x);
    }

    pub fn ldy(&mut self, instruction: &Instruction) {
        self.register_y = self.get_operand(instruction);
        self.update_zero_and_negative_flags(self.register_y);
    }

    pub fn ora(&mut self, instruction: &Instruction) {
        self.register_a |= self.get_operand(instruction);
        self.update_zero_and_negative_flags(self.register_a);
    }

    pub fn pha(&mut self) {
        self.push_stack(self.register_a);
    }

    pub fn php(&mut self) {
        self.push_stack(self.status);
    }

    pub fn pla(&mut self) {
        self.register_a = self.pop_stack();
        self.update_zero_and_negative_flags(self.register_a);
    }

    pub fn plp(&mut self) {
        self.status = self.pop_stack();
    }

    pub fn rti(&mut self) {
        self.status = self.pop_stack();
        self.program_counter = self.pop_stack_u16();
    }

    pub fn rts(&mut self) {
        self.program_counter = self.pop_stack_u16() - 1;
    }

    pub fn sec(&mut self) {
        self.set_status_flag(StatusFlag::C);
    }

    pub fn sed(&mut self) {
        self.set_status_flag(StatusFlag::D);
    }

    pub fn sei(&mut self) {
        self.set_status_flag(StatusFlag::I);
    }

    pub fn sta(&mut self, instruction: &Instruction) {
        let address = self.get_operand_address(&instruction.addressing_mode);
        self.memory.write(address, self.register_a);
    }

    pub fn stx(&mut self, instruction: &Instruction) {
        let address = self.get_operand_address(&instruction.addressing_mode);
        self.memory.write(address, self.register_x);
    }

    pub fn sty(&mut self, instruction: &Instruction) {
        let address = self.get_operand_address(&instruction.addressing_mode);
        self.memory.write(address, self.register_y);
    }

    pub fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }

    pub fn tay(&mut self) {
        self.register_y = self.register_a;
        self.update_zero_and_negative_flags(self.register_y);
    }

    pub fn tsx(&mut self) {
        self.register_x = self.stack_pointer;
        self.update_zero_and_negative_flags(self.register_x);
    }

    pub fn txa(&mut self) {
        self.register_a = self.register_x;
        self.update_zero_and_negative_flags(self.register_a);
    }

    pub fn txs(&mut self) {
        self.stack_pointer = self.register_x;
    }

    pub fn tya(&mut self) {
        self.register_a = self.register_y;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn rotate_left(&mut self, mut operand: u8) -> u8 {
        let old_bit_seven = (operand & 0b1000_0000) >> 7;
        operand <<= 1;
        operand |= self.status & 1;
        self.clear_status_flag(StatusFlag::C);
        self.status |= old_bit_seven;
        operand
    }

    pub fn rol(&mut self, instruction: &Instruction) {
        match &instruction.addressing_mode {
            AddressingMode::Accumulator => {
                self.register_a = self.rotate_left(self.register_a);
                self.update_zero_and_negative_flags(self.register_a);
            },
            AddressingMode::ZeroPage | AddressingMode::ZeroPage_X | AddressingMode::Absolute | AddressingMode::Absolute_X => {
                let operand_address = self.get_operand_address(&instruction.addressing_mode);
                let operand = self.get_operand(instruction);
                let rotated_operand = self.rotate_left(operand);
                self.memory.write(operand_address, rotated_operand);
                self.update_zero_and_negative_flags(rotated_operand);
            },
            _ => {
                panic!("Recieved unexpected address mode while performing rol instruction");
            }
        }
    }

    fn rotate_right(&mut self, mut operand: u8) -> u8 {
        let old_bit_zero = operand & 0b0000_0001;
        operand >>= 1;
        operand |= (self.status & 1) << 7;
        self.clear_status_flag(StatusFlag::C);
        self.status |= old_bit_zero;
        operand
    }

    pub fn ror(&mut self, instruction: &Instruction) {
        match &instruction.addressing_mode {
            AddressingMode::Accumulator => {
                self.register_a = self.rotate_right(self.register_a);
                self.update_zero_and_negative_flags(self.register_a);
            },
            AddressingMode::ZeroPage | AddressingMode::ZeroPage_X | AddressingMode::Absolute | AddressingMode::Absolute_X => {
                let operand_address = self.get_operand_address(&instruction.addressing_mode);
                let operand = self.get_operand(instruction);
                let rotated_operand = self.rotate_right(operand);
                self.memory.write(operand_address, rotated_operand);
                self.update_zero_and_negative_flags(rotated_operand);
            },
            _ => {
                panic!("Recieved unexpected address mode while performing rol instruction");
            }
        }
    }
}
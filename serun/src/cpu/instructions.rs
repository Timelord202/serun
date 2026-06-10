use crate::cpu::{CPU, StatusFlag};
use crate::opcodes::AddressingMode;

impl CPU<'_> {
    fn modify_accumulator(&mut self, new_accumulator_value: u16, operand: u8) {
        if new_accumulator_value > 0xFF {
            self.set_status_flag(StatusFlag::C);
        }
        else {
            self.clear_status_flag(StatusFlag::C);
        }
        if (self.register_a ^ new_accumulator_value as u8) & (operand ^ new_accumulator_value as u8) & 0x80 != 0 {
            self.set_status_flag(StatusFlag::V);
        }
        else {
            self.clear_status_flag(StatusFlag::V);
        }

        self.register_a = new_accumulator_value as u8;
        self.update_zero_and_negative_flags(self.register_a);
    }

    // TODO: Create wrapping add for new accumulator values
    pub fn adc (&mut self) {
        let operand = self.get_operand();
        let new_accumulator_value = self.register_a as u16 + operand as u16 + self.get_status_flag(StatusFlag::C) as u16;
        self.modify_accumulator(new_accumulator_value, operand);
    }

    pub fn sbc (&mut self) {
        let operand = self.get_operand();
        let new_accumulator_value = self.register_a as u16 - operand as u16 - (1 - self.get_status_flag(StatusFlag::C) as u16);
        self.modify_accumulator(new_accumulator_value, operand);
    }

    pub fn and(&mut self) {
        self.register_a &= self.get_operand();
        self.update_zero_and_negative_flags(self.register_a);
    }

    pub fn asl(&mut self) {
        let instruction = self.curr_instr.unwrap();

        match &instruction.addressing_mode {
            AddressingMode::Accumulator => {
                let old_bit_seven = (self.register_a & 0b1000_0000) >> 7;
                self.register_a <<= 1;
                self.clear_status_flag(StatusFlag::C);
                self.status |= old_bit_seven;
                self.update_zero_and_negative_flags(self.register_a);
            },
            AddressingMode::ZeroPage | AddressingMode::ZeroPage_X | AddressingMode::Absolute | AddressingMode::Absolute_X => {
                let operand_address = self.get_operand_address();
                let mut operand = self.get_operand();
                let old_bit_seven = (operand & 0b1000_0000) >> 7;
                operand <<= 1;
                self.clear_status_flag(StatusFlag::C);
                self.status |= old_bit_seven;
                self.memory.write(operand_address, operand);
                self.update_zero_and_negative_flags(operand);
            },
            _ => {}
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

    pub fn cmp(&mut self) {
        let value = self.get_operand();
        self.compare_register(self.register_a, value);
    }

    pub fn cpx(&mut self) {
        let value = self.get_operand();
        self.compare_register(self.register_x, value);
    }

    pub fn cpy(&mut self) {
        let value = self.get_operand();
        self.compare_register(self.register_y, value);
    }

    pub fn dec(&mut self) {
        let address = self.get_operand_address();
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

    pub fn eor(&mut self) {
        self.register_a ^= self.get_operand();
        self.update_zero_and_negative_flags(self.register_a);
    }

    pub fn inc(&mut self) {
        let address = self.get_operand_address();
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

    pub fn lda(&mut self) {
        self.register_a = self.get_operand();
        self.update_zero_and_negative_flags(self.register_a);
    }

    pub fn ldx(&mut self) {
        self.register_x = self.get_operand();
        self.update_zero_and_negative_flags(self.register_x);
    }

    pub fn ldy(&mut self) {
        self.register_y = self.get_operand();
        self.update_zero_and_negative_flags(self.register_y);
    }

    pub fn ora(&mut self) {
        self.register_a |= self.get_operand();
        self.update_zero_and_negative_flags(self.register_a);
    }

    pub fn pha(&mut self) {
        self.push_stack(self.register_a);
    }

    pub fn php(&mut self) {
        self.push_stack(self.status | 0b_0011_0000);
    }

    pub fn pla(&mut self) {
        self.register_a = self.pop_stack();
        self.update_zero_and_negative_flags(self.register_a);
    }

    pub fn plp(&mut self) {
        self.status = self.pop_stack();
        self.clear_status_flag(StatusFlag::B);
        self.set_status_flag(StatusFlag::Unused);
    }

    pub fn rti(&mut self) {
        self.status = self.pop_stack();
        self.clear_status_flag(StatusFlag::B);
        self.set_status_flag(StatusFlag::Unused);
        self.pc = self.pop_stack_u16();
    }

    pub fn rts(&mut self) {
        let stack_val = self.pop_stack_u16();
        self.pc = stack_val.wrapping_add(1);
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

    pub fn sta(&mut self) {
        let address = self.get_operand_address();
        self.memory.write(address, self.register_a);
    }

    pub fn stx(&mut self) {
        let address = self.get_operand_address();
        self.memory.write(address, self.register_x);
    }

    pub fn sty(&mut self) {
        let address = self.get_operand_address();
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
        self.register_x = self.sp;
        self.update_zero_and_negative_flags(self.register_x);
    }

    pub fn txa(&mut self) {
        self.register_a = self.register_x;
        self.update_zero_and_negative_flags(self.register_a);
    }

    pub fn txs(&mut self) {
        self.sp = self.register_x;
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

    pub fn rol(&mut self) {
        let instruction = self.curr_instr.unwrap();

        match &instruction.addressing_mode {
            AddressingMode::Accumulator => {
                self.register_a = self.rotate_left(self.register_a);
                self.update_zero_and_negative_flags(self.register_a);
            },
            AddressingMode::ZeroPage | AddressingMode::ZeroPage_X | AddressingMode::Absolute | AddressingMode::Absolute_X => {
                let operand_address = self.get_operand_address();
                let operand = self.get_operand();
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

    pub fn ror(&mut self) {
        let instruction = self.curr_instr.unwrap();

        match &instruction.addressing_mode {
            AddressingMode::Accumulator => {
                self.register_a = self.rotate_right(self.register_a);
                self.update_zero_and_negative_flags(self.register_a);
            },
            AddressingMode::ZeroPage | AddressingMode::ZeroPage_X | AddressingMode::Absolute | AddressingMode::Absolute_X => {
                let operand_address = self.get_operand_address();
                let operand = self.get_operand();
                let rotated_operand = self.rotate_right(operand);
                self.memory.write(operand_address, rotated_operand);
                self.update_zero_and_negative_flags(rotated_operand);
            },
            _ => {
                panic!("Recieved unexpected address mode while performing rol instruction");
            }
        }
    }

    // Helper function for branch instructions
    fn branch(&mut self, flag: StatusFlag, require_flag_is_set: bool) {
        let flag_is_set = self.get_status_flag(flag) == 1;
        let target = self.get_operand_address();

        if flag_is_set == require_flag_is_set {
            self.pc = target;
        }
    }

    pub fn bcc(&mut self) {
        self.branch(StatusFlag::C, false);
    }

    pub fn bcs(&mut self) {
        self.branch(StatusFlag::C, true);
    }

    pub fn beq(&mut self) {
        self.branch(StatusFlag::Z, true);
    }

    pub fn bit(&mut self) {
        let operand = self.get_operand();
        let bit_seven = (operand & 0b1000_0000) >> 7;
        let bit_six = (operand & 0b0100_0000) >> 6;
        let result = self.register_a & operand;

        // Set negative flag
        if bit_seven == 1 {
            self.set_status_flag(StatusFlag::N);
        }
        else {
            self.clear_status_flag(StatusFlag::N);
        }

        // Set overflow flag
        if bit_six == 1 {
            self.set_status_flag(StatusFlag::V);
        }
        else {
            self.clear_status_flag(StatusFlag::V);
        }

        // Set zero flag
        if result == 0 {
            self.set_status_flag(StatusFlag::Z);
        }
        else {
            self.clear_status_flag(StatusFlag::Z);
        }
    }

    pub fn bmi(&mut self) {
        self.branch(StatusFlag::N, true);
    }

    pub fn bne(&mut self) {
        self.branch(StatusFlag::Z, false);
    }

    pub fn bpl(&mut self) {
        self.branch(StatusFlag::N, false);
    }

    pub fn brk(&mut self) {
        self.push_stack_u16(self.pc);
        self.push_stack(self.status | 0b0011_0000);
        self.set_status_flag(StatusFlag::I);
        self.pc = self.memory.read_u16(0xFFFE);
    }

    pub fn bvc(&mut self) {
        self.branch(StatusFlag::V, false);
    }

    pub fn bvs(&mut self) {
        self.branch(StatusFlag::V, true);
    }

    // TODO: Need to account for 6502 bug
    pub fn jmp(&mut self) {
        self.pc = self.get_operand_address();
    }

    pub fn jsr(&mut self) {
        self.push_stack_u16(self.pc.wrapping_sub(1));
        self.pc = self.get_operand_address();
    }

    // TODO: This and asl should be more generalized
    pub fn lsr(&mut self) {
        let instruction = self.curr_instr.unwrap();

        match &instruction.addressing_mode {
            AddressingMode::Accumulator => {
                let old_bit_zero = self.register_a & 1;
                self.register_a >>= 1;
                self.clear_status_flag(StatusFlag::C);
                self.status |= old_bit_zero;
                self.update_zero_and_negative_flags(self.register_a);
            },
            AddressingMode::ZeroPage | AddressingMode::ZeroPage_X | AddressingMode::Absolute | AddressingMode::Absolute_X => {
                let operand_address = self.get_operand_address();
                let mut operand = self.get_operand();
                let old_bit_zero = operand & 1;
                operand >>= 1;
                self.clear_status_flag(StatusFlag::C);
                self.status |= old_bit_zero;
                self.memory.write(operand_address, operand);
                self.update_zero_and_negative_flags(operand);
            },
            _ => {}
        }
    }
}
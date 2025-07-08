pub mod cpu;
pub mod memory;
pub mod opcodes;

#[cfg(test)]
mod test {
    use crate::opcodes::{Instruction, Opcode, AddressingMode};
    use super::cpu::CPU;

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.register_a =0;
        cpu.load_and_run(vec![0xaa, 0x00]);

        assert_eq!(cpu.register_x, 0)
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.register_x = 0xff;
        cpu.load_and_run(vec![0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 2)
    }

    #[test]
    fn test_push_and_pop_stack() {
        let mut cpu = CPU::new();
        cpu.reset();
        cpu.push_stack(0x01);
        cpu.push_stack(0x02);
        cpu.push_stack(0x03);
        assert_eq!(cpu.stack_pointer, 0xFA);
        let val1 = cpu.pop_stack();
        let val2 = cpu.pop_stack();
        let val3 = cpu.pop_stack();
        assert_eq!(val1, 0x03);
        assert_eq!(val2, 0x02);
        assert_eq!(val3, 0x01);
        assert_eq!(cpu.stack_pointer, 0xFD);
    }

    #[test]
    fn test_rol() {
        let mut cpu = CPU::new();
        let instruction = Instruction { opcode: Opcode::ROL, bytes: 1, cycles: 2, addressing_mode: AddressingMode::Accumulator };
        cpu.reset();
        cpu.status = 0b1010_0111;
        cpu.register_a = 0b0011_1100;
        cpu.rol(&instruction);
        assert_eq!(cpu.register_a, 0b0111_1001);
        assert_eq!(cpu.status, 0b0010_0100);
    }
}

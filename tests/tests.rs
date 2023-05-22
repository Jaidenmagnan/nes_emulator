#[cfg(test)]
mod test {
    use super::*;
    use nes_emulator::CPU;

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status & 0x02 == 0x0);
        assert!(cpu.status & 0x80 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status & 0x02 == 0x2)
    }

    #[test]
    fn test_0xaa_tax_move_a_to_a() {
        let mut cpu = CPU::new();
        cpu.register_a = 10;
        cpu.load(vec![0xaa, 0x00]);
        cpu.run();

        assert_eq!(cpu.register_x, 10)
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 0xc1);
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xe8, 0xe8, 0x00]);
        cpu.register_x = 0xff;
        cpu.run();

        assert_eq!(cpu.register_x, 1);
    }
}
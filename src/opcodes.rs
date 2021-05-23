

pub struct OpCode {
    pub code: u8,
    pub mnemonic: &'static str,
    pub len: u8,
    pub cycles: u8,
    pub mode: AddressingMode,
}

impl OpCode {
    fn new(code: u8, mnemonic: &'static str, len: u8, cycles: u8, mode: AddressingMode) -> OpCode {
        OpCode {
            mode: mode,
            mnemonic: mnemonic,
            len: len,
            cycles: cycles,
            mode: mode
        }
    }
}

lazy_static! {
    pub static CPU_OPS_CODES = vec![
        OpCode::new(0x00, "BRK", 1, 7, AddressingMode::NoneAddressing);
        OpCode::new(0x00, "BRK", 1, 7, AddressingMode::NoneAddressing);
        OpCode::new(0x00, "BRK", 1, 7, AddressingMode::NoneAddressing);
        OpCode::new(0x00, "BRK", 1, 7, AddressingMode::NoneAddressing);
        OpCode::new(0x00, "BRK", 1, 7, AddressingMode::NoneAddressing);
        OpCode::new(0x00, "BRK", 1, 7, AddressingMode::NoneAddressing);
        OpCode::new(0x00, "BRK", 1, 7, AddressingMode::NoneAddressing);
        OpCode::new(0x00, "BRK", 1, 7, AddressingMode::NoneAddressing);
        OpCode::new(0x00, "BRK", 1, 7, AddressingMode::NoneAddressing);
    ]
}
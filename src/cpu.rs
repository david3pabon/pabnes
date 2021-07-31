use std::{collections::HashMap, usize};
use crate::opcodes;

bitflags! {
    pub struct CpuFlags: u8 {
        const CARRY = 0b0000_0001;
        const ZERO = 0b0000_0010;
        const INTERRUPT_DISABLE = 0b0000_0100;
        const DECIMAL_MODE = 0b0000_1000;
        const BREAK = 0b0001_0000;
        const BREAK2 = 0b0010_0000;
        const OVERFLOW = 0b0100_0000;
        const NEGATIVE = 0b1000_0000;
    }
}

const STACK: u16 = 0x0010;
const STACK_RESET: u8 = 0xFD;

pub struct CPU {
    pub reg_a: u8,
    pub reg_x: u8,
    pub reg_y: u8,
    pub status: CpuFlags,
    pub pc: u16,
    pub sp: u8,
    memory: [u8; 0xFFFF]
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
    NoneAddressing,
}

trait Mem {
    fn mem_read(&self, addr: u16) -> u8;

    fn mem_write(&mut self, addr: u16, data: u8);

    fn mem_read_u16(&self, pos: u16) -> u16 {
        let low = self.mem_read(pos) as u16;
        let high = self.mem_read(pos + 1) as u16;
        (high << 8) | low
    }

    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let low = (0x00ff & data) as u8;
        let high = (data >> 8) as u8;
        self.mem_write(pos, low);
        self.mem_write(pos + 1, high);
    }
}

impl Mem for CPU {
    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            reg_a: 0,
            reg_x: 0,
            reg_y: 0,
            status: CpuFlags::from_bits_truncate(0b100100),
            pc: 0,
            sp: STACK_RESET,
            memory: [0; 0xFFFF]
        }
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.memory = [0; 0xFFFF];
        self.load(program);
        self.reset();
        self.run();
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000..(0x8000 + program.len())].clone_from_slice(&program[..]);
        self.mem_write_u16(0xFFFC, 0x8000);
    }

    pub fn reset(&mut self) {
        self.reg_a = 0;
        self.reg_x = 0;
        self.reg_y = 0;
        self.sp = STACK_RESET;
        self.status = CpuFlags::from_bits_truncate(0b100100);
        self.pc = self.mem_read_u16(0xFFFC);
    }

    pub fn run(&mut self) {
        let codes: &HashMap<u8, &'static opcodes::OpCode> = &*opcodes::OPCODES_MAP;
        
        loop {
            let code = self.mem_read(self.pc);
            self.pc += 1;
            let pc_state = self.pc;

            let operand = codes.get(&code).expect(&format!("Opcode {:x} is not recognized", code));

            match code {
                0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => {
                    self.lda(&operand.mode);
                }

                0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => {
                    self.sta(&operand.mode);
                }
                
                0xAA => self.tax(),

                0xE8 => self.inx(),

                0x00 => return,

                _ => todo!(),
            }

            if pc_state == self.pc {
                self.pc += (operand.len - 1) as u16;
            }
        }
    }

    fn lda(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.reg_a = value;
        self.update_zero_and_negative_flags(self.reg_a);
    }
  
    fn sta(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.reg_a);
    }
  
    fn tax(&mut self) {
        self.reg_x = self.reg_a;
        self.update_zero_and_negative_flags(self.reg_x);
    }
  
    fn inx(&mut self) {
        if self.reg_x < 0xff {
            self.reg_x = self.reg_x + 1;
        } else {
            self.reg_x = 0;
        }
        
        self.update_zero_and_negative_flags(self.reg_x);
    }
   
    fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            self.status.insert(CpuFlags::ZERO);
        } else {
            self.status.remove(CpuFlags::ZERO);
        }

        if result & 0b1000_0000 != 0 {
            self.status.insert(CpuFlags::NEGATIVE);
        } else {
            self.status.remove(CpuFlags::NEGATIVE);
        }
    }

    fn get_operand_address(&mut self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.pc,

            AddressingMode::ZeroPage => self.mem_read(self.pc) as u16,

            AddressingMode::Absolute => self.mem_read_u16(self.pc),
            
            AddressingMode::ZeroPageX => {
                let pos = self.mem_read(self.pc);
                let addr = pos.wrapping_add(self.reg_x) as u16;
                addr
            },
            AddressingMode::ZeroPageY => {
                let pos = self.mem_read(self.pc);
                let addr = pos.wrapping_add(self.reg_y) as u16;
                addr
            },
            
            AddressingMode::AbsoluteX => {
                let pos = self.mem_read_u16(self.pc);
                let addr = pos.wrapping_add(self.reg_x as u16);
                addr
            },
            AddressingMode::AbsoluteY => {
                let pos = self.mem_read_u16(self.pc);
                let addr = pos.wrapping_add(self.reg_y as u16);
                addr
            },
            
            AddressingMode::IndirectX => {
                let base = self.mem_read(self.pc);

                let ptr: u8 = (base as u8).wrapping_add(self.reg_x);
                let hi = self.mem_read(ptr as u16);
                let lo = self.mem_read(ptr.wrapping_add(1) as u16);
                (hi as u16) << 8 | (lo as u16)
            },
            AddressingMode::IndirectY => {
                let base = self.mem_read(self.pc);

                let lo = self.mem_read(base as u16);
                let hi = self.mem_read((base as u8).wrapping_add(1) as u16);
                let deref_base = (hi as u16) << 8 | (lo as u16);
                let deref = deref_base.wrapping_add(self.reg_y as u16);
                deref
            },
            
            AddressingMode::NoneAddressing => {
                panic!("mode {:?} is not supported :(", mode)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lda_immidiate_load_data() {
       let mut cpu = CPU::new();
       
       cpu.load_and_run(vec![0xA9, 0x05, 0x00]);

       assert_eq!(cpu.reg_a, 5);
       assert!(cpu.status.bits() & 0b0000_0010 == 0b00);
       assert!(cpu.status.bits() & 0b1000_0000 == 0);
    }

    #[test]
    fn test_lda_zero_flag() {
        let mut cpu = CPU::new();
        
        cpu.load_and_run(vec![0xA9, 0x00, 0x00]);
        
        assert!(cpu.status.bits() & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_lta_to_memory() {
        let mut cpu = CPU::new();
        cpu.load(vec![0x85, 0x10, 0x00]);
        cpu.reset();
        cpu.reg_a = 0x55;
        
        cpu.run();
        
        assert_eq!(cpu.mem_read(0x10 as u16), 0x55);
    }
    
    #[test]
    fn test_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xaa, 0x00]);
        cpu.reset();
        cpu.reg_a = 10;

        cpu.run();

        assert_eq!(cpu.reg_x, 10)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.load(vec![0xe8, 0xe8, 0x00]);
        cpu.reset();
        cpu.reg_x = 0xff;
        
        cpu.run();

        assert_eq!(cpu.reg_x, 1)
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.reg_x, 0xc1)
    }
}
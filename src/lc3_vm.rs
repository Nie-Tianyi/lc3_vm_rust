mod config;
mod util;
mod instructions;

use byteorder::{BigEndian, ReadBytesExt};
use config::*;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::Path;
use std::{io, process};
use util::*;

pub struct LC3VM {
    memory: [u16; MEMORY_SIZE], // total memory size
    reg: [u16; REG_COUNT],      // 8 REG + 1 PC + 1 FLAG
}

impl LC3VM {
    pub fn new() -> LC3VM {
        let memory = [0; MEMORY_SIZE];
        let mut reg = [0; REG_COUNT];
        reg[R_PC] = PC_START;

        LC3VM { memory, reg }
    }

    pub fn load(&mut self, path: impl AsRef<Path>) {
        let file = File::open(path).expect("Cannot Open File");
        let mut file = BufReader::new(file);

        let mut addr = file.read_u16::<BigEndian>().expect("Read File Error");

        loop {
            match file.read_u16::<BigEndian>() {
                Ok(ins) => {
                    self.write_address(addr, ins);
                    addr += 1;
                }
                Err(e) => {
                    if e.kind() == io::ErrorKind::UnexpectedEof {
                        println!("OK")
                    } else {
                        println!("failed: {}", e);
                    }
                    break;
                }
            }
        }
    }

    pub fn execute(&mut self) {
        loop {
            let ins = self.read_address(self.pc());
            self.inc_pc(1);
            let opcode = ins >> 12;

            match opcode {
                OP_BR => self.br(ins),
                OP_ADD => self.add(ins),
                OP_LD => self.ld(ins),
                OP_ST => self.st(ins),
                OP_JSR => self.jsr(ins),
                OP_AND => self.and(ins),
                OP_LDR => self.ldr(ins),
                OP_STR => self.str(ins),
                OP_NOT => self.not(ins),
                OP_LDI => self.ldi(ins),
                OP_STI => self.sti(ins),
                OP_JMP => self.jmp(ins),
                OP_LEA => self.lea(ins),
                OP_TRAP => self.trap(ins),
                OP_RES | OP_RTI => panic!("Abandoned Instructions"),
                _ => panic!("Unknown Instruction {opcode}"),
            }
        }
    }
}

impl LC3VM {
    #[inline]
    fn register(&self, reg: u16) -> u16 {
        let reg = reg as usize;
        if reg >= REG_COUNT {
            panic!("Error: Trying to access a illegal register")
        }
        self.reg[reg]
    }
    /// write a register
    #[inline]
    fn write_reg(&mut self, reg: u16, val: u16) {
        let reg = reg as usize;
        if reg >= REG_COUNT {
            panic!("Error: Trying to write a illegal register")
        }
        self.reg[reg] = val;
    }

    #[inline]
    fn pc(&self) -> u16 {
        self.reg[R_PC]
    }

    #[inline]
    fn set_pc(&mut self, val: u16) {
        self.reg[R_PC] = val
    }

    #[inline]
    fn inc_pc(&mut self, n: u16) {
        self.reg[R_PC] = self.reg[R_PC].wrapping_add(n);
    }

    #[inline]
    fn cond(&self) -> u16 {
        self.reg[R_COND]
    }

    #[inline]
    fn read_address(&mut self, address: u16) -> u16 {
        if address == MR_KBSR {
            let mut buffer = [0; 1];
            io::stdin().read_exact(&mut buffer).unwrap();
            if buffer[0] != 0 {
                self.write_address(MR_KBSR, 1 << 15);
                self.write_address(MR_KBDR, buffer[0] as u16);
            } else {
                self.write_address(MR_KBSR, 0);
            }
        }
        self.memory[address as usize]
    }

    #[inline]
    fn write_address(&mut self, address: u16, val: u16) {
        self.memory[address as usize] = val;
    }

    fn update_flag(&mut self, reg: u16) {
        let reg = reg as usize;
        if reg >= REG_COUNT {
            panic!("Error: trying to update an illegal register")
        }

        if self.reg[reg] == 0 {
            self.reg[R_COND] = FL_ZRO;
        } else if self.reg[reg] >> 15 != 0 {
            self.reg[R_COND] = FL_NEG;
        } else {
            self.reg[R_COND] = FL_POS;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let vm = LC3VM::new();
    }
}

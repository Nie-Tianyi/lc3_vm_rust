use byteorder::{BigEndian, ReadBytesExt};
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::Path;
use std::{io, process};

const MEMORY_SIZE: usize = u16::MAX as usize;
const REG_COUNT: usize = 10;
const PC_START: u16 = 0x3000;
const R0: usize = 0;
#[allow(dead_code)]
const R1: usize = 1;
#[allow(dead_code)]
const R2: usize = 2;
#[allow(dead_code)]
const R3: usize = 3;
#[allow(dead_code)]
const R4: usize = 4;
#[allow(dead_code)]
const R5: usize = 5;
#[allow(dead_code)]
const R6: usize = 6;
const R7: usize = 7;
const R_PC: usize = 8;
const R_COND: usize = 9;
const FL_POS: u16 = 1 << 0; // 1，P
const FL_ZRO: u16 = 1 << 1; // 2，Z
const FL_NEG: u16 = 1 << 2; // 4，N
const OP_BR: u16 = 0; // 0000
const OP_ADD: u16 = 1; // 0001
const OP_LD: u16 = 2; // 0010
const OP_ST: u16 = 3; // 0011
const OP_JSR: u16 = 4; // 0100
const OP_AND: u16 = 5; // 0101
const OP_LDR: u16 = 6; // 0110
const OP_STR: u16 = 7; // 0111
const OP_RTI: u16 = 8; // 1000
const OP_NOT: u16 = 9; // 1001
const OP_LDI: u16 = 10; // 1010
const OP_STI: u16 = 11; // 1011
const OP_JMP: u16 = 12; // 1100
const OP_RES: u16 = 13; // 1101
const OP_LEA: u16 = 14; // 1110
const OP_TRAP: u16 = 15; // 1111
const P_1: u16 = 0x1; // 0000 0000 0000 0001
const P_3: u16 = 0x7; // 0000 0000 0000 0111
const P_5: u16 = 0x1F; // 0000 0000 0001 1111
const P_6: u16 = 0x3F; // 0000 0000 0011 1111
const P_8: u16 = 0xFF; // 0000 0000 1111 1111
const P_9: u16 = 0x1FF; // 0000 0001 1111 1111
const P_11: u16 = 0x7FF; // 0000 0111 1111 1111
#[allow(dead_code)]
const P_16: u16 = 0xFFFF; // 1111 1111 1111 1111
const MR_KBSR: u16 = 0xFE00; // 键盘状态，是否按下
const MR_KBDR: u16 = 0xFE02; // 键盘数据存储
const TRAP_GETC: u16 = 0x20;
const TRAP_OUT: u16 = 0x21;
const TRAP_PUTS: u16 = 0x22;
const TRAP_IN: u16 = 0x23;
const TRAP_PUTSP: u16 = 0x24;
const TRAP_HALT: u16 = 0x25;

/// extend a 5-bit signed integer to 16-bit signed integer
#[inline]
pub(crate) fn sign_extend(mut num: u16, bits: usize) -> u16 {
    if (num >> (bits - 1)) & 1 != 0 {
        num |= 0xFFFF << bits
    }
    num
}

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

impl LC3VM {
    fn add(&mut self, ins: u16) {
        // `& P_3` means only retain last 3 bits
        let r0 = (ins >> 9) & P_3; // destination register
        let r1 = (ins >> 6) & P_3;
        let imm_flag = (ins >> 5) & P_1;

        if imm_flag == 1 {
            let imm5 = sign_extend(ins & P_5, 5);
            self.write_reg(r0, self.register(r1).wrapping_add(imm5));
        } else {
            let r2 = ins & P_3;
            self.write_reg(r0, self.register(r1).wrapping_add(self.register(r2)))
        }

        self.update_flag(r0);
    }
    // load indirect
    fn ldi(&mut self, ins: u16) {
        let r0 = (ins >> 9) & P_3;
        let pc_offset = sign_extend(ins & P_9, 9);

        let address = self.read_address(self.pc().wrapping_add(pc_offset));
        let val = self.read_address(address);
        self.write_reg(r0, val);
        self.update_flag(r0);
    }
    // bitwise and operation
    fn and(&mut self, ins: u16) {
        let r0 = (ins >> 9) & P_3;
        let r1 = (ins >> 6) & P_3;
        let imm_flag = (ins >> 5) & P_1;

        if imm_flag == 1 {
            let imm5 = sign_extend(ins & P_5, 5);
            self.write_reg(r0, self.register(r1) & imm5)
        } else {
            let r2 = ins & P_3;
            self.write_reg(r0, self.register(r1) & self.register(r2))
        }

        self.update_flag(r0);
    }
    // bitwise not operation
    fn not(&mut self, ins: u16) {
        let r0 = (ins >> 9) & P_3;
        let r1 = (ins >> 6) & P_3;

        self.write_reg(r0, !self.register(r1));
        self.update_flag(r0);
    }
    // branch
    fn br(&mut self, ins: u16) {
        let pc_offset = sign_extend(ins & P_9, 9);
        let cond_flag = (ins >> 9) & P_3;

        if cond_flag & self.cond() != 0 {
            self.inc_pc(pc_offset);
        }
    }
    // jump
    fn jmp(&mut self, ins: u16) {
        let r1 = (ins >> 6) & P_3;
        self.set_pc(self.register(r1));
    }
    // jump register
    fn jsr(&mut self, ins: u16) {
        let long_flag = (ins >> 11) & P_1;
        self.write_reg(R7 as u16, self.pc());

        if long_flag == 1 {
            let long_pc_offset = sign_extend(ins & P_11, 11);
            self.inc_pc(long_pc_offset);
        } else {
            let r1 = (ins >> 6) & P_3;
            self.set_pc(self.register(r1));
        }
    }
    // load
    fn ld(&mut self, ins: u16) {
        let r0 = (ins >> 9) & P_3;
        let pc_offset = sign_extend(ins & P_9, 9);
        let val = self.read_address(self.pc().wrapping_add(pc_offset));
        self.write_reg(r0, val);
        self.update_flag(r0);
    }
    // load register
    fn ldr(&mut self, ins: u16) {
        let r0 = (ins >> 9) & P_3;
        let r1 = (ins >> 6) & P_3;

        let offset = sign_extend(ins & P_6, 6);

        let val = self.read_address(self.register(r1).wrapping_add(offset));
        self.write_reg(r0, val);
        self.update_flag(r0);
    }
    // load effective address
    fn lea(&mut self, ins: u16) {
        let r0 = (ins >> 9) & P_3;
        let pc_offset = sign_extend(ins & P_9, 9);
        self.write_reg(r0, self.pc().wrapping_add(pc_offset));
        self.update_flag(r0);
    }
    // store
    fn st(&mut self, ins: u16) {
        let r0 = (ins >> 9) & P_3;
        let pc_offset = sign_extend(ins & P_9, 9);
        self.write_address(self.pc().wrapping_add(pc_offset), self.register(r0));
    }
    // store indirect
    fn sti(&mut self, ins: u16) {
        let r0 = (ins >> 9) & P_3;
        let pc_offset = sign_extend(ins & P_9, 9);
        let reg = self.read_address(self.pc().wrapping_add(pc_offset));
        self.write_address(reg, self.register(r0));
    }

    fn str(&mut self, ins: u16) {
        let r0 = (ins >> 9) & P_3;
        let r1 = (ins >> 6) & P_3;
        let offset = sign_extend(ins & P_6, 6);
        let addr = self.register(r1).wrapping_add(offset);
        self.write_address(addr, self.register(r0));
    }

    fn trap(&mut self, ins: u16) {
        match ins & P_8 {
            TRAP_GETC => {
                // get char
                let mut buffer = [0; 1];
                io::stdin().read_exact(&mut buffer).unwrap();
                self.write_reg(R0 as u16, buffer[0] as u16);
            }
            TRAP_OUT => {
                let c = self.register(R0 as u16) as u8;
                print!("{}", c as char);
            }
            TRAP_PUTS => {
                let mut index = self.register(R0 as u16);
                let mut c = self.read_address(index);
                while c != 0x0000 {
                    let chr = (c as u8) as char;
                    print!("{}", chr);
                    index += 1;
                    c = self.read_address(index);
                }
                io::stdout().flush().expect("Failed to Flush");
            }
            TRAP_IN => {
                print!("Enter a character : ");
                io::stdout().flush().expect("failed to flush");
                let char = io::stdin()
                    .bytes()
                    .next()
                    .and_then(|result| result.ok())
                    .map(|byte| byte as u16)
                    .unwrap();
                self.write_reg(R0 as u16, char);
            }
            TRAP_PUTSP => {
                let mut index = self.register(R0 as u16);
                let mut c = self.read_address(index);
                while c != 0x0000 {
                    let c1 = ((c & P_8) as u8) as char;
                    print!("{}", c1);
                    let c2 = ((c >> 8) as u8) as char;
                    if c2 != '\0' {
                        print!("{}", c2);
                    }
                    index += 1;
                    c = self.read_address(index);
                }
                io::stdout().flush().expect("Fail to Flush");
            }
            TRAP_HALT => {
                println!("HALT detected");
                io::stdout().flush().expect("Fail to Flush");
                process::exit(1);
            }
            _ => panic!("Unknown Trap Code"),
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

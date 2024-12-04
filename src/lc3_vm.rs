mod config;
mod util;

use std::io::{Read, Write};
use config::*;
use util::*;

struct LC3VM {
    memory: [u16; MEMORY_SIZE], // total memory size
    reg: [u16; REG_COUNT],      // 8 REG + 1 PC + 1 FLAG
    debug: bool,
}

impl LC3VM {
    pub fn new(debug: bool) -> LC3VM {
        let memory = [0; MEMORY_SIZE];
        let mut reg = [0; REG_COUNT];
        reg[R_PC] = PC_START;
        reg[R_COND] = FL_ZRO;

        LC3VM { memory, reg, debug }
    }

    pub fn execute(&mut self) {
        loop {
            let ins = self.read_address(self.reg[R_PC]);
            self.inc_pc(1);
            let opcode = ins >> 12;

            match opcode {
                OP_ADD => self.add(ins),
                OP_LD => self.ld(ins),
                OP_ST => self.st(ins),
                OP_JSR => self.str(ins),
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
    fn is_debug(&self) -> bool {
        self.debug
    }

    #[inline]
    fn register(&self, reg: u16) -> u16 {
        let reg = reg as usize;
        if reg >= REG_COUNT {
            panic!("Error: Trying to access a illegal register")
        }
        self.reg[reg]
    }

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
        self.reg[R_PC] += n;
    }

    #[inline]
    fn cond(&self) -> u16 {
        self.reg[R_COND]
    }

    #[inline]
    fn read_address(&self, address: u16) -> u16 {
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
        } else if self.reg[reg] >> 15 == 1 {
            self.reg[R_COND] = FL_NEG;
        } else {
            self.reg[R_COND] = FL_POS;
        }
    }
}

impl LC3VM {
    fn add(&mut self, ins: u16) {
        // & P_3 means only retain last 3 bits
        let r0 = (ins >> 9) & P_3; // destination register
        let r1 = (ins >> 6) & P_3;
        let imm_flag = (ins >> 6) & P_1;

        if imm_flag == 1 {
            let imm5 = sign_extend(ins & P_5, 5);
            self.write_reg(r0, self.register(r1) + imm5);
        } else {
            let r2 = ins & P_3;
            self.write_reg(r0, self.register(r1) + self.register(r2))
        }

        self.update_flag(r0);
    }
    // load indirect
    fn ldi(&mut self, ins: u16) {
        let r0 = (ins >> 9) & P_3;
        let pc_offset = sign_extend(ins & P_9, 9);

        let address = self.read_address(self.pc() + pc_offset);

        self.write_reg(r0, self.read_address(address));
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
        self.write_reg(r0, self.read_address(self.pc() + pc_offset));
        self.update_flag(r0);
    }
    // load register
    fn ldr(&mut self, ins: u16) {
        let r0 = (ins >> 9) & P_3;
        let r1 = (ins >> 6) & P_3;

        let offset = sign_extend(ins & P_6, 6);
        self.write_reg(r0, self.read_address(self.register(r1) + offset));
        self.update_flag(r0);
    }
    // load effective address
    fn lea(&mut self, ins: u16) {
        let r0 = (ins >> 9) & P_3;
        let pc_offset = sign_extend(ins & P_9, 9);
        self.write_reg(r0, self.pc() + pc_offset);
        self.update_flag(r0);
    }
    // store
    fn st(&mut self, ins: u16) {
        let r0 = (ins >> 9) & P_3;
        let pc_offset = sign_extend(ins & P_9, 9);
        self.write_address(self.pc() + pc_offset, self.register(r0));
    }
    // store indirect
    fn sti(&mut self, ins: u16) {
        let r0 = (ins >> 9) & P_3;
        let pc_offset = sign_extend(ins & P_9, 9);
        self.write_address(self.read_address(self.pc() + pc_offset), self.register(r0));
    }

    fn str(&mut self, ins: u16) {
        let r0 = (ins >> 9) & P_3;
        let r1 = (ins >> 6) & P_3;
        let offset = sign_extend(ins & P_6, 6);
        self.write_address(self.register(r1) + offset, self.register(r0));
    }

    fn trap(&mut self, ins: u16) {
        self.write_reg(R7 as u16, self.pc());
        match ins & P_8 {
            TRAP_GETC => { // get char
                let mut buffer = [0;1];
                std::io::stdin().read_exact(&mut buffer).unwrap();
                self.write_reg(R0 as u16, buffer[0] as u16);
                self.update_flag(R0 as u16);
            }
            TRAP_OUT => {
                let c = self.register(R0 as u16) as u8;
                print!("{}", c as char);
            }
            TRAP_PUTS => {
                let mut index = self.register(R0 as u16);
                let mut c = self.read_address(index);
                while c != 0x0 {
                    print!("{}", c as u8 as char);
                    index += 1;
                    c = self.read_address(index);
                }
                std::io::stdout().flush().expect("Failed to Flush");
            }
            TRAP_IN => {
                print!("Enter a  character : ");
                std::io::stdout().flush().expect("failed to flush");
                let char = std::io::stdin()
                    .bytes()
                    .next()
                    .and_then(|result| result.ok())
                    .map(|byte| byte as u16)
                    .unwrap();
                self.write_reg(R0 as u16, char);
                self.update_flag(R0 as u16);
            }
            TRAP_PUTSP => {
                let mut index = self.register(R0 as u16);
                let mut c = self.read_address(index);
                while c != 0x0 {
                    let c1 = ((c & 0xFF) as u8) as char;
                    print!("{}", c1);
                    let c2 = ((c >> 8) as u8) as char;
                    if c2 != '\0' {
                        print!("{}", c2);
                    }
                    index += 1;
                    c = self.read_address(index);
                }
                std::io::stdout().flush().expect("Fail to Flush");
            }
            TRAP_HALT => {
                println!("HALT detected");
                std::io::stdout().flush().expect("Fail to Flush");
                std::process::exit(1);
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
        let vm = LC3VM::new(true);
    }
}

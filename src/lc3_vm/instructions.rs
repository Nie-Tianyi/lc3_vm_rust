use crate::lc3_vm::config::*;
use crate::lc3_vm::util::sign_extend;
use crate::lc3_vm::LC3VM;
use std::io::{Read, Write};
use std::{io, process};

impl LC3VM {
    pub(crate) fn add(&mut self, ins: u16) {
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
    pub(crate) fn ldi(&mut self, ins: u16) {
        let r0 = (ins >> 9) & P_3;
        let pc_offset = sign_extend(ins & P_9, 9);

        let address = self.read_address(self.pc().wrapping_add(pc_offset));
        let val = self.read_address(address);
        self.write_reg(r0, val);
        self.update_flag(r0);
    }
    // bitwise and operation
    pub(crate) fn and(&mut self, ins: u16) {
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
    pub(crate) fn not(&mut self, ins: u16) {
        let r0 = (ins >> 9) & P_3;
        let r1 = (ins >> 6) & P_3;

        self.write_reg(r0, !self.register(r1));
        self.update_flag(r0);
    }
    // branch
    pub(crate) fn br(&mut self, ins: u16) {
        let pc_offset = sign_extend(ins & P_9, 9);
        let cond_flag = (ins >> 9) & P_3;

        if cond_flag & self.cond() != 0 {
            self.inc_pc(pc_offset);
        }
    }
    // jump
    pub(crate) fn jmp(&mut self, ins: u16) {
        let r1 = (ins >> 6) & P_3;
        self.set_pc(self.register(r1));
    }
    // jump register
    pub(crate) fn jsr(&mut self, ins: u16) {
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
    pub(crate) fn ld(&mut self, ins: u16) {
        let r0 = (ins >> 9) & P_3;
        let pc_offset = sign_extend(ins & P_9, 9);
        let val = self.read_address(self.pc().wrapping_add(pc_offset));
        self.write_reg(r0, val);
        self.update_flag(r0);
    }
    // load register
    pub(crate) fn ldr(&mut self, ins: u16) {
        let r0 = (ins >> 9) & P_3;
        let r1 = (ins >> 6) & P_3;

        let offset = sign_extend(ins & P_6, 6);

        let val = self.read_address(self.register(r1).wrapping_add(offset));
        self.write_reg(r0, val);
        self.update_flag(r0);
    }
    // load effective address
    pub(crate) fn lea(&mut self, ins: u16) {
        let r0 = (ins >> 9) & P_3;
        let pc_offset = sign_extend(ins & P_9, 9);
        self.write_reg(r0, self.pc().wrapping_add(pc_offset));
        self.update_flag(r0);
    }
    // store
    pub(crate) fn st(&mut self, ins: u16) {
        let r0 = (ins >> 9) & P_3;
        let pc_offset = sign_extend(ins & P_9, 9);
        self.write_address(self.pc().wrapping_add(pc_offset), self.register(r0));
    }
    // store indirect
    pub(crate) fn sti(&mut self, ins: u16) {
        let r0 = (ins >> 9) & P_3;
        let pc_offset = sign_extend(ins & P_9, 9);
        let reg = self.read_address(self.pc().wrapping_add(pc_offset));
        self.write_address(reg, self.register(r0));
    }

    pub(crate) fn str(&mut self, ins: u16) {
        let r0 = (ins >> 9) & P_3;
        let r1 = (ins >> 6) & P_3;
        let offset = sign_extend(ins & P_6, 6);
        let addr = self.register(r1).wrapping_add(offset);
        self.write_address(addr, self.register(r0));
    }

    pub(crate) fn trap(&mut self, ins: u16) {
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

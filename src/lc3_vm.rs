mod config;
mod operand;
use config::*;

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

        LC3VM { memory, reg, debug }
    }
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
    fn inc_pc(&mut self) {
        self.reg[R_PC] += 1;
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

    #[inline]
    fn stdin(&self) -> std::io::Stdin {
        std::io::stdin()
    }
    #[inline]
    fn stdout(&self, content: impl std::fmt::Display) {
        println!("{content}")
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let vm = LC3VM::new(true);
    }
}

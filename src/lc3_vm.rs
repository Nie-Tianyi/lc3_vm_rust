mod config;
use config::*;

struct LC3VM {
    memory: [u16; MEMORY_SIZE as usize], // total memory size
    reg: [u16; REG_COUNT as usize], // 8 REG + 1 PC + 1 FLAG
    debug: bool,
}

impl LC3VM {
    pub fn new(debug: bool) -> LC3VM {
        let memory = [0; MEMORY_SIZE];
        let mut reg = [0; REG_COUNT];
        reg[R_PC] = PC_START;

        LC3VM {
            memory,
            reg,
            debug,
        }
    }
    #[inline]
    pub fn is_debug(&self) -> bool {
        self.debug
    }
    #[inline]
    pub fn register(&self, reg: u16) -> u16 {
        if reg >= REG_COUNT {
            panic!("Error: Trying to access a illegal register")
        }
        self.reg[reg as usize]
    }
    #[inline]
    pub fn write_reg(&mut self, reg: u16, val: u16) {
        if reg >= REG_COUNT {
            panic!("Error: Trying to write a illegal register")
        }
        self.reg[reg as usize] = val;
    }
    #[inline]
    pub fn pc(&self) -> u16 {
        self.reg[R_PC]
    }
    #[inline]
    pub fn set_pc(&mut self, val: u16) {
        self.reg[R_PC] = val
    }
    #[inline]
    pub fn inc_pc(&mut self) {
        self.reg[R_PC] += 1;
    }
    #[inline]
    pub fn cond(&self) -> u16 {
        self.reg[R_COND]
    }
    #[inline]
    pub fn read_address(&self, address: u16) -> u16 {
        self.memory[address as usize]
    }
    #[inline]
    pub fn write_address(&mut self, address: u16, val: u16) {
        self.memory[address as usize] = val;
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
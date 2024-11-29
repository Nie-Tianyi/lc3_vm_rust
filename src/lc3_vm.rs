mod config;
use config::*;

struct LC3VM {
    memory: [u16; MEMORY_SIZE], // total memory size
    reg: [u16; REG_COUNT], // 8 REG + 1 PC + 1 FLAG
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
    pub fn register(&self, reg: usize) -> u16 {
        if reg > 10 {
            panic!("Error: Trying to access a illegal register")
        }
        self.reg[reg]
    }
    #[inline]
    pub fn write_reg(&mut self, reg: usize, val: u16) {
        if reg > 10 {
            panic!("Error: Trying to write a illegal register")
        }
        self.reg[reg] = val;
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let vm = LC3VM::new(true);
    }
}
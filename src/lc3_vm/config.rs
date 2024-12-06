pub const MEMORY_SIZE: usize = u16::MAX as usize;
pub const REG_COUNT: usize = 10;
pub const PC_START: u16 = 0x3000;
pub const R0: usize = 0;
#[allow(dead_code)]
pub const R1: usize = 1;
#[allow(dead_code)]
pub const R2: usize = 2;
#[allow(dead_code)]
pub const R3: usize = 3;
#[allow(dead_code)]
pub const R4: usize = 4;
#[allow(dead_code)]
pub const R5: usize = 5;
#[allow(dead_code)]
pub const R6: usize = 6;
pub const R7: usize = 7;
pub const R_PC: usize = 8;
pub const R_COND: usize = 9;
pub const FL_POS: u16 = 1 << 0; // 1，P
pub const FL_ZRO: u16 = 1 << 1; // 2，Z
pub const FL_NEG: u16 = 1 << 2; // 4，N
pub const OP_BR: u16 = 0; // 0000
pub const OP_ADD: u16 = 1; // 0001
pub const OP_LD: u16 = 2; // 0010
pub const OP_ST: u16 = 3; // 0011
pub const OP_JSR: u16 = 4; // 0100
pub const OP_AND: u16 = 5; // 0101
pub const OP_LDR: u16 = 6; // 0110
pub const OP_STR: u16 = 7; // 0111
pub const OP_RTI: u16 = 8; // 1000
pub const OP_NOT: u16 = 9; // 1001
pub const OP_LDI: u16 = 10; // 1010
pub const OP_STI: u16 = 11; // 1011
pub const OP_JMP: u16 = 12; // 1100
pub const OP_RES: u16 = 13; // 1101
pub const OP_LEA: u16 = 14; // 1110
pub const OP_TRAP: u16 = 15; // 1111
pub const P_1: u16 = 0x1; // 0000 0000 0000 0001
pub const P_3: u16 = 0x7; // 0000 0000 0000 0111
pub const P_5: u16 = 0x1F; // 0000 0000 0001 1111
pub const P_6: u16 = 0x3F; // 0000 0000 0011 1111
pub const P_8: u16 = 0xFF; // 0000 0000 1111 1111
pub const P_9: u16 = 0x1FF; // 0000 0001 1111 1111
pub const P_11: u16 = 0x7FF; // 0000 0111 1111 1111
#[allow(dead_code)]
pub const P_16: u16 = 0xFFFF; // 1111 1111 1111 1111
pub const MR_KBSR: u16 = 0xFE00; // 键盘状态，是否按下
pub const MR_KBDR: u16 = 0xFE02; // 键盘数据存储
pub const TRAP_GETC: u16 = 0x20;
pub const TRAP_OUT: u16 = 0x21;
pub const TRAP_PUTS: u16 = 0x22;
pub const TRAP_IN: u16 = 0x23;
pub const TRAP_PUTSP: u16 = 0x24;
pub const TRAP_HALT: u16 = 0x25;

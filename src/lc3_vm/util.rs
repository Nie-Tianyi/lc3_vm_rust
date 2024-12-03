/// extend a 5-bit signed integer to 16-bit signed integer
#[inline]
pub(crate) fn sign_extend(mut num: u16, bits: usize) -> u16 {
    if num >> (bits - 1) != 0 {
        num |= 0xFFFF << bits
    }
    num
}
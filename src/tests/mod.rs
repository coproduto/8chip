use super::*;

quickcheck! {
    fn reverse_concat(x: u16) -> bool {
        let highbit: u8 = (x >> 8) as u8;
        let lowbit: u8 = x as u8;
        let slice = &[highbit, lowbit];
        chip8::read_from_slice(slice) == x
    }
}

#[derive(Debug)]
pub struct ELF;

impl ELF {
    pub fn is_valid(input: &[u8]) -> bool {
        if &input[0x00..0x04] == [0x7F, 0x45, 0x4C, 0x46] {
            return true;
        }

        false
    }
}
#[derive(Debug)]
pub struct MachO;

impl MachO {
    pub fn is_valid(input: &[u8]) -> bool {
        if &input[0x00..0x04] == [0xCF, 0xFA, 0xED, 0xFE] {
            return true;
        } else if &input[0x00..0x04] == [0xCE, 0xFA, 0xED, 0xFE] {
            return true;
        }

        false
    }
}
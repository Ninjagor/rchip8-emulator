#[cfg(test)]
pub mod tests {
    use crate::{
        chip8::Chip8,
        constants::{FONTSET_SIZE, FONT_SET},
    };

    use super::*;

    #[test]
    fn font_load() {
        let mut chip8 = Chip8::new();

        chip8.load_fonts();

        assert_eq!(chip8.ram[0..FONTSET_SIZE], FONT_SET[0..FONTSET_SIZE]);
    }
}

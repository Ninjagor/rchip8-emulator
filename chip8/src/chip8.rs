use crate::constants::*;

pub struct Chip8 {
    pub ram: [u8; RAM_SIZE],
    pub vregs: [u8; VREG_SIZE],
    pub stack: [u16; STACK_SIZE],
    pub keyboard: [bool; KEYBOARD_MAP_SIZE],
    pub display: [bool; DISPLAY_WIDTH * DISPLAY_HEIGHT],

    pub ireg: u16,
    pub program_counter: u16,
    pub stack_pointer: u16,

    pub delay_timer: u8,
    pub sound_timer: u8,

    pub clear_flag: bool,
    pub display_flag: bool,
}

impl Chip8 {
    pub fn new() -> Self {
        let chip8 = Chip8 {
            ram: [0; RAM_SIZE],
            vregs: [0; VREG_SIZE],
            stack: [0; STACK_SIZE],
            keyboard: [false; KEYBOARD_MAP_SIZE],
            display: [false; DISPLAY_WIDTH * DISPLAY_HEIGHT],

            ireg: 0,
            program_counter: 0,
            stack_pointer: 0,

            delay_timer: 0,
            sound_timer: 0,

            clear_flag: false,
            display_flag: false,
        };

        chip8
    }

    pub fn load_rom(&mut self, data: &[u8]) {
        let start = 0x200 as usize;
        let end = (0x200 as usize) + data.len();
        self.ram[start..end].copy_from_slice(data);
    }

    pub fn load_fonts(&mut self) {
        for i in 0..FONTSET_SIZE {
            self.ram[i] = FONT_SET[i];
        }
    }

    pub fn get_display(&self) -> &[bool] {
        &self.display
    }

    pub fn tick(&mut self) {
        // print!("{}[2J", 27 as char);
        // for i in 0..self.display.len() {
        //     print!("{} ", (if self.display[i] == true { 1 } else { 0 }));
        // }
        let opcode = self.fetch_opcode();
        self.execute(opcode);
    }

    pub fn tick_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                // BEEP
            }
            self.sound_timer -= 1;
        }
    }

    pub fn fetch_opcode(&mut self) -> u16 {
        let higher_byte = self.ram[self.program_counter as usize] as u16;
        let lower_byte = self.ram[(self.program_counter + 1) as usize] as u16;
        let opcode = (higher_byte << 8) | lower_byte;
        self.program_counter += 2;
        opcode
    }
}

/*
RAM FORMATTING (for reference):


 ADDRESS                                         CONTENT
 ~~~~~~~                                         ~~~~~~~

   0x000  --------------------------------  <--  Start of RAM
          |                              |
          |  Interpreter code, fonts     |
          |                              |
   0x200  --------------------------------  <--  Start of user programs
          |                              |
          |                              |
          |      User programs and       |
          |        data go here          |
          |                              |
          |                              |
   0x600  ................................  <--  Start of user programs (ETI 660)
          |                              |
          |                              |
          |                              |
          |                              |
          |      User programs and       |
          |        data go here          |
          |                              |
          |                              |
          |                              |
          |                              |
   0xFFF  --------------------------------  <--  End of RAM
*/
pub const RAM_SIZE: usize = 4096;

pub const VREG_SIZE: usize = 16;
pub const STACK_SIZE: usize = 16;

pub const FONTSET_SIZE: usize = 16;
pub const KEYBOARD_MAP_SIZE: usize = 16;

pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;

pub const FONT_SET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

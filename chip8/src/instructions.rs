use std::usize;

use rand::Rng;

use crate::{
    chip8::Chip8,
    constants::{DISPLAY_HEIGHT, DISPLAY_WIDTH},
};

impl Chip8 {
    pub fn execute(&mut self, opcode: u16) {
        let first_digit = (opcode & 0xF000) >> 12;
        let second_digit = (opcode & 0x0F00) >> 8;
        let third_digit = (opcode & 0x00F0) >> 4;
        let fourth_digit = opcode & 0x000F;

        match (first_digit, second_digit, third_digit, fourth_digit) {
            // clear screen
            (0x0, 0x0, 0xE, 0) => {
                for i in 0..self.display.len() {
                    self.display[i] = false;
                }
                self.clear_flag = true;
            }
            // 00EE - return from subroutine
            (0, 0, 0xE, 0xE) => {
                self.stack_pointer -= 1;
                let last_addr = self.stack[self.stack_pointer as usize];

                self.program_counter = last_addr;
            }
            (0, 0, _, _) => {
                // unsupported 0x0 0x0 opcode
            }
            // set pc to addr
            (1, _, _, _) => {
                let address = opcode & 0xFFF;
                self.program_counter = address;
            }
            // 2NNN - subroutine
            (2, _, _, _) => {
                let address = opcode & 0xFFF;

                self.stack[self.stack_pointer as usize] = self.program_counter;
                self.stack_pointer += 1;

                self.program_counter = address;
            }
            // 3XNN
            (3, _, _, _) => {
                let x = second_digit as usize;
                let nn = (opcode & 0xFF) as u8;

                if self.vregs[x] == nn {
                    self.program_counter += 2;
                }
            }
            // 4XNN
            (4, _, _, _) => {
                let x = second_digit as usize;
                let nn = (opcode & 0xFF) as u8;

                if self.vregs[x] != nn {
                    self.program_counter += 2;
                }
            }
            // 5XY0
            (5, _, _, 0) => {
                let x = second_digit as usize;
                let y = third_digit as usize;

                if self.vregs[x] == self.vregs[y] {
                    self.program_counter += 2;
                }
            }
            // 5XY0
            (9, _, _, 0) => {
                let x = second_digit as usize;
                let y = third_digit as usize;

                if self.vregs[x] != self.vregs[y] {
                    self.program_counter += 2;
                }
            }
            // set vreg NN to X
            (6, _, _, _) => {
                let value = opcode & 0xFF;
                self.vregs[second_digit as usize] = value as u8;
            }
            // add vreg NN by X
            (7, _, _, _) => {
                let x = second_digit as usize;
                let nn = (opcode & 0xFF) as u8;
                self.vregs[x] = self.vregs[x].wrapping_add(nn);
            }
            // 8XY0
            (8, _, _, 0) => {
                let x = second_digit as usize;
                let y = third_digit as usize;
                self.vregs[x] = self.vregs[y];
            }
            // 8XY1
            (8, _, _, 1) => {
                let x = second_digit as usize;
                let y = third_digit as usize;
                self.vregs[x] = self.vregs[x] | self.vregs[y];
            }
            // 8XY2
            (8, _, _, 2) => {
                let x = second_digit as usize;
                let y = third_digit as usize;
                self.vregs[x] = self.vregs[x] & self.vregs[y];
            }
            // 8XY3
            (8, _, _, 3) => {
                let x = second_digit as usize;
                let y = third_digit as usize;
                self.vregs[x] = self.vregs[x] ^ self.vregs[y];
            }
            // 8XY4
            (8, _, _, 4) => {
                let x = second_digit as usize;
                let y = third_digit as usize;
                let sum = self.vregs[x] as u16 + self.vregs[y] as u16;
                if sum > 0xFF {
                    self.vregs[0xF] = 1;
                } else {
                    self.vregs[0xF] = 0;
                }
                self.vregs[x] = self.vregs[x].wrapping_add(self.vregs[y]);
            }
            // 8XY5
            (8, _, _, 5) => {
                let x = second_digit as usize;
                let y = third_digit as usize;

                // its opposite than usual but thats what its supposed to be
                if self.vregs[x] > self.vregs[y] {
                    self.vregs[0xF] = 1;
                } else {
                    self.vregs[0xF] = 0;
                }
                self.vregs[x] = self.vregs[x].wrapping_sub(self.vregs[y]);
            }
            // 8XY7
            (8, _, _, 7) => {
                let x = second_digit as usize;
                let y = third_digit as usize;

                // its opposite than usual but thats what its supposed to be
                if self.vregs[y] > self.vregs[x] {
                    self.vregs[0xF] = 1;
                } else {
                    self.vregs[0xF] = 0;
                }
                self.vregs[x] = self.vregs[y].wrapping_sub(self.vregs[x]);
            }
            // 8XY6
            (8, _, _, 6) => {
                let x = second_digit as usize;
                let mut value = self.vregs[x];
                self.vregs[0xF] = value & 0x1;
                value >>= 1;
                self.vregs[x] = value;
            }
            // 8XYE
            (8, _, _, 0xE) => {
                let x = second_digit as usize;
                let mut value = self.vregs[x];
                self.vregs[0xF] = value & 0x1;
                value <<= 1;
                self.vregs[x] = value;
            }
            // set ireg
            (0xA, _, _, _) => {
                let value = opcode & 0xFFF;
                self.ireg = value;
            }
            // BNNN variation (not BXNN - so it wont support those ROM's)
            (0xB, _, _, _) => {
                let nnn = opcode & 0xFFF;
                self.program_counter = (self.vregs[0] as u16) + nnn;
            }
            // CXNN
            (0xC, _, _, _) => {
                let x = second_digit as usize;
                let nn = (opcode & 0xFF) as u8;
                let rng: u8 = rand::thread_rng().gen();
                self.vregs[x] = rng & nn;
            }
            // display dxyn
            (0xD, _, _, _) => {
                let x = second_digit as usize;
                let y = third_digit as usize;
                let n = fourth_digit as u8;

                // set coords
                let x_coord = self.vregs[x] % 64;
                let y_coord = self.vregs[y] % 32;

                // set f register to 0 and display flag to false
                self.vregs[0xF] = 0;
                self.display_flag = false;

                let mut flipped: bool = false;

                for _y in 0..n {
                    let addr = self.ireg + _y as u16;
                    let pixels = self.ram[addr as usize];

                    for _x in 0..8 {
                        // fetch current pixel's bit. only flip on 1
                        if (pixels & (0b1000_0000 >> _x)) != 0 {
                            let x = (x_coord + _x) as usize % DISPLAY_WIDTH;
                            let y = (y_coord + _y) as usize % DISPLAY_HEIGHT;

                            let idx = x + DISPLAY_WIDTH * y;

                            flipped |= self.display[idx];
                            self.display[idx] ^= true;
                        }
                    }
                }

                if flipped {
                    self.vregs[0xF] = 1;
                    self.display_flag = true;
                } else {
                    self.vregs[0xF] = 0;
                    self.display_flag = false;
                }
            }
            (0xE, _, 9, 0xE) => {
                let x = second_digit as usize;
                let vx = self.vregs[x];
                let key = self.keyboard[vx as usize];
                if key {
                    self.program_counter += 2;
                }
            }
            // SKIP KEY RELEASE
            (0xE, _, 0xA, 1) => {
                let x = second_digit as usize;
                let vx = self.vregs[x];
                let key = self.keyboard[vx as usize];
                if !key {
                    self.program_counter += 2;
                }
            }
            // FX07
            (0xF, _, 0, 7) => {
                let x = second_digit as usize;

                self.vregs[x] = self.delay_timer;
            }
            //  FX0A
            (0xF, _, 0, 0xA) => {
                let x = second_digit as usize;
                let mut pressed = false;
                for i in 0..self.keyboard.len() {
                    if self.keyboard[i] {
                        self.vregs[x] = i as u8;
                        pressed = true;
                        break;
                    }
                }

                if !pressed {
                    self.program_counter -= 2;
                }
            }
            // FX15
            (0xF, _, 1, 5) => {
                let x = second_digit as usize;

                self.delay_timer = self.vregs[x];
            }
            // FX18
            (0xF, _, 1, 8) => {
                let x = second_digit as usize;

                self.sound_timer = self.vregs[x];
            }
            // FX1E
            (0xF, _, 1, 0xE) => {
                let x = second_digit as usize;

                self.ireg = self.ireg.wrapping_add(self.vregs[x].into());
            }
            // FX28
            (0xF, _, 2, 9) => {
                let x = second_digit as usize;
                let c = self.vregs[x] as u16;
                self.ireg = c * 5;
            }
            // FX33
            (0xF, _, 3, 3) => {
                let x = second_digit as usize;
                let mut split: [u8; 3] = [0, 0, 0];
                let vx = self.vregs[x];
                println!("VX: {}", vx);
                let mut vx_modifiable = vx;
                if vx >= 200 {
                    split[0] = 2;
                    vx_modifiable -= 200;
                } else if vx >= 100 {
                    split[0] = 1;
                    vx_modifiable -= 100;
                } else {
                    split[0] = 0;
                }
                let last_digit = vx_modifiable % 10;
                split[2] = last_digit;
                vx_modifiable -= last_digit;
                split[1] = vx_modifiable / 10;

                let ireg = self.ireg as usize;
                self.ram[ireg] = split[0];
                self.ram[ireg + 1] = split[1];
                self.ram[ireg + 2] = split[2];
            }
            // FX55
            (0xF, _, 5, 5) => {
                let x = second_digit as usize;
                let i = self.ireg as usize;
                for idx in 0..=x {
                    self.ram[i + idx] = self.vregs[idx];
                }
            }
            // FX65
            (0xF, _, 6, 5) => {
                let x = second_digit as usize;
                let i = self.ireg as usize;
                for idx in 0..=x {
                    self.vregs[idx] = self.ram[i + idx];
                }
            }
            (_, _, _, _) => {
                println!("Unimplemented opcode: {:#04x}", opcode)
                // unimplemented!("Unimplemented opcode: {:#04x}", opcode)
            }
        }
    }
}

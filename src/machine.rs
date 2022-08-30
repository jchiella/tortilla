use rand::Rng;

const MEMORY_SIZE: usize = 4096;
const STACK_SIZE: usize = 16;
const NUM_REGISTERS: usize = 16;
pub const CLOCK_SPEED: u64 = 700; // Hz (instructions/second)
pub const SCREEN_SIZE_X: u32 = 64;
pub const SCREEN_SIZE_Y: u32 = 32;

const MODERN_SHIFT: bool = true;
const MODERN_JUMP_WITH_OFFSET: bool = false;
const MODERN_LOAD_STORE: bool = true;

pub struct CHIP8 {
    memory: [u8; MEMORY_SIZE],
    pub screen: [bool; (SCREEN_SIZE_X * SCREEN_SIZE_Y) as usize],
    pub pc: u16,
    i: u16,
    stack: [u16; STACK_SIZE],
    stack_pointer: usize,
    delay_timer: u8,
    sound_timer: u8,
    registers: [u8; NUM_REGISTERS],
}

impl CHIP8 {
    pub fn new() -> Self {
        let font = [
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

        let mut chip8 = CHIP8 {
            memory: [0; MEMORY_SIZE],
            screen: [false; (SCREEN_SIZE_X * SCREEN_SIZE_Y) as usize],
            pc: 0x0200,
            i: 0,
            stack: [0; STACK_SIZE],
            stack_pointer: 0,
            delay_timer: 0,
            sound_timer: 0,
            registers: [0; NUM_REGISTERS],
        };

        chip8.memory[0x0050..0x00A0].copy_from_slice(&font);

        chip8
    }

    pub fn load_program(&mut self, program: &[u8]) {
        self.memory[0x0200..(0x0200 + program.len())].copy_from_slice(program);
    }

    pub fn fetch(&mut self) -> u16 {
        let instruction = ((self.memory[self.pc as usize] as u16) << 8)
            | self.memory[self.pc as usize + 1] as u16;
        self.pc += 2;
        instruction
    }

    pub fn decode_and_execute(&mut self, instruction: u16) -> bool {
        let mut redraw_required = false;

        let opcode = (instruction & 0xf000) >> 12; // the 1st nibble
        let x = (instruction & 0x0f00) >> 8; // the 2nd nibble
        let y = (instruction & 0x00f0) >> 4; // the 3rd nibble
        let n = instruction & 0x000f; // the 4th nibble
        let nn = instruction & 0x00ff; // the 2nd byte (3rd & 4th nibbles)
        let nnn = instruction & 0x0fff; // the 2nd-4th nibbles (12 bits)

        match opcode {
            0x0 => match nn {
                0xE0 => self.clear_screen(),
                0xEE => self.return_from_sub(),
                _ => unimplemented!(),
            },
            0x1 => self.jump(nnn),
            0x2 => self.call_sub(nnn),
            0x3 => self.skip_if_equal_literal(x, nn),
            0x4 => self.skip_if_not_equal_literal(x, nn),
            0x5 => self.skip_if_equal(x, y),
            0x6 => self.set_literal(x, nn),
            0x7 => self.add_literal(x, nn),
            0x8 => match n {
                0x0 => self.set(x, y),
                0x1 => self.binary_or(x, y),
                0x2 => self.binary_and(x, y),
                0x3 => self.logical_xor(x, y),
                0x4 => self.add(x, y),
                0x5 => self.subtract1(x, y),
                0x6 => self.shift_right(x, y),
                0x7 => self.subtract2(x, y),
                0xE => self.shift_left(x, y),
                _ => unimplemented!(),
            },
            0x9 => self.skip_if_not_equal(x, y),
            0xA => self.set_index_register(nnn),
            0xB => self.jump_with_offset(x, nnn),
            0xC => self.random(x, nn),
            0xD => {
                self.display(x, y, n);
                redraw_required = true;
            }
            0xE => match nn {
                0x9E => self.skip_if_key_pressed(x),
                0xA1 => self.skip_if_key_not_pressed(x),
                _ => unreachable!(),
            },
            0xF => match nn {
                0x07 => self.get_delay_timer(x),
                0x15 => self.set_delay_timer(x),
                0x18 => self.set_sound_timer(x),
                0x1E => self.add_to_index(x),
                0x0A => self.get_key(x),
                0x29 => self.get_font_character(x),
                0x33 => self.convert_bcd(x),
                0x55 => self.store_memory(x),
                0x65 => self.load_memory(x),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
        redraw_required
    }

    fn push_stack(&mut self, value: u16) {
        self.stack[self.stack_pointer] = value;
        self.stack_pointer += 1;

        if self.stack_pointer >= self.stack.len() {
            panic!("Stack overflow!");
        }
    }

    fn pop_stack(&mut self) -> u16 {
        let value = self.stack[self.stack_pointer - 1];
        self.stack_pointer -= 1;
        value
    }

    fn clear_screen(&mut self) {
        for pixel in self.screen.iter_mut() {
            *pixel = false;
        }
    }

    fn return_from_sub(&mut self) {
        self.pc = self.pop_stack();
    }

    fn jump(&mut self, location: u16) {
        self.pc = location;
    }

    fn call_sub(&mut self, location: u16) {
        self.push_stack(self.pc);
        self.pc = location;
    }

    fn skip_if_equal_literal(&mut self, register: u16, value: u16) {
        if self.registers[register as usize] as u16 == value {
            self.pc += 2;
        }
    }

    fn skip_if_not_equal_literal(&mut self, register: u16, value: u16) {
        if self.registers[register as usize] as u16 != value {
            self.pc += 2;
        }
    }

    fn skip_if_equal(&mut self, register1: u16, register2: u16) {
        if self.registers[register1 as usize] == self.registers[register2 as usize] {
            self.pc += 2;
        }
    }

    fn set_literal(&mut self, register: u16, value: u16) {
        self.registers[register as usize] = value.try_into().expect("whoops!");
    }

    fn add_literal(&mut self, register: u16, value: u16) {
        self.registers[register as usize] =
            self.registers[register as usize].wrapping_add(value.try_into().expect("whoops!"));
    }

    fn set(&mut self, register1: u16, register2: u16) {
        self.registers[register1 as usize] = self.registers[register2 as usize];
    }

    fn binary_or(&mut self, register1: u16, register2: u16) {
        self.registers[register1 as usize] =
            self.registers[register1 as usize] | self.registers[register2 as usize];
    }

    fn binary_and(&mut self, register1: u16, register2: u16) {
        self.registers[register1 as usize] =
            self.registers[register1 as usize] & self.registers[register2 as usize];
    }

    fn logical_xor(&mut self, register1: u16, register2: u16) {
        self.registers[register1 as usize] =
            self.registers[register1 as usize] ^ self.registers[register2 as usize];
    }

    fn add(&mut self, register1: u16, register2: u16) {
        let overflowed;
        (self.registers[register1 as usize], overflowed) =
            self.registers[register1 as usize].overflowing_add(self.registers[register2 as usize]);
        if overflowed {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }

    fn subtract1(&mut self, register1: u16, register2: u16) {
        let overflowed;
        (self.registers[register1 as usize], overflowed) =
            self.registers[register1 as usize].overflowing_sub(self.registers[register2 as usize]);
        if overflowed {
            self.registers[0xF] = 0;
        } else {
            self.registers[0xF] = 1;
        }
    }

    fn subtract2(&mut self, register1: u16, register2: u16) {
        let overflowed;
        (self.registers[register1 as usize], overflowed) =
            self.registers[register2 as usize].overflowing_sub(self.registers[register1 as usize]);
        if overflowed {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }

    fn shift_right(&mut self, register1: u16, register2: u16) {
        if !MODERN_SHIFT {
            self.registers[register1 as usize] = self.registers[register2 as usize];
        }
        let overflowed;
        (self.registers[register1 as usize], overflowed) =
            self.registers[register1 as usize].overflowing_shr(1);
        if overflowed {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }

    fn shift_left(&mut self, register1: u16, register2: u16) {
        if !MODERN_SHIFT {
            self.registers[register1 as usize] = self.registers[register2 as usize];
        }
        let overflowed;
        (self.registers[register1 as usize], overflowed) =
            self.registers[register1 as usize].overflowing_shl(1);
        if overflowed {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }

    fn skip_if_not_equal(&mut self, register1: u16, register2: u16) {
        if self.registers[register1 as usize] != self.registers[register2 as usize] {
            self.pc += 2;
        }
    }

    fn set_index_register(&mut self, value: u16) {
        self.i = value;
    }

    fn jump_with_offset(&mut self, register: u16, location: u16) {
        if MODERN_JUMP_WITH_OFFSET {
            self.pc = location + self.registers[register as usize] as u16;
        } else {
            self.pc = location + self.registers[0] as u16;
        }
    }

    fn random(&mut self, register: u16, value: u16) {
        let mut rng = rand::thread_rng();
        let random_value = rng.gen_range(0..u8::MAX);
        self.registers[register as usize] = (random_value as u16 & value) as u8;
    }

    fn display(&mut self, x_register: u16, y_register: u16, sprite_height: u16) {
        let mut x_coord = (self.registers[x_register as usize] % (SCREEN_SIZE_X as u8)) as u32;
        let mut y_coord = (self.registers[y_register as usize] % (SCREEN_SIZE_Y as u8)) as u32;

        self.registers[0xF] = 0;

        for row in 0..sprite_height {
            if y_coord >= SCREEN_SIZE_Y {
                break;
            }

            let sprite_row = self.memory[(self.i + row) as usize];
            for n in 0..8 {
                if x_coord >= SCREEN_SIZE_X {
                    break;
                }

                let mask = sprite_row & (1 << 7 >> n);
                if mask != 0 && self.screen[(y_coord * SCREEN_SIZE_X + x_coord) as usize] {
                    self.screen[(y_coord * SCREEN_SIZE_X + x_coord) as usize] = false;
                    self.registers[0xF] = 1;
                } else if mask != 0 && !self.screen[(y_coord * SCREEN_SIZE_X + x_coord) as usize] {
                    self.screen[(y_coord * SCREEN_SIZE_X + x_coord) as usize] = true;
                }

                x_coord += 1;
            }

            y_coord += 1;
            x_coord = (self.registers[x_register as usize] % (SCREEN_SIZE_X as u8)) as u32;
        }
    }

    fn skip_if_key_pressed(&mut self, register: u16) {
        println!("Key presses NYI");
    }

    fn skip_if_key_not_pressed(&mut self, register: u16) {
        println!("Key presses NYI");
    }

    fn get_delay_timer(&mut self, register: u16) {
        self.registers[register as usize] = self.delay_timer;
    }

    fn set_delay_timer(&mut self, register: u16) {
        self.delay_timer = self.registers[register as usize];
    }

    fn set_sound_timer(&mut self, register: u16) {
        self.sound_timer = self.registers[register as usize];
    }

    fn add_to_index(&mut self, register: u16) {
        self.i += self.registers[register as usize] as u16;

        if self.i > 0x0FFF {
            self.registers[0xF] = 1;
        }
    }

    fn get_key(&mut self, register: u16) {
        todo!();
    }

    fn get_font_character(&mut self, register: u16) {
        self.i = 0x0050 + self.registers[register as usize] as u16 * 5;
    }

    fn convert_bcd(&mut self, register: u16) {
        self.memory[self.i as usize] = self.registers[register as usize] / 100;
        self.memory[self.i as usize + 1] = (self.registers[register as usize] / 10) % 10;
        self.memory[self.i as usize + 2] = (self.registers[register as usize] % 100) % 10;
    }

    fn store_memory(&mut self, max_register: u16) {
        for register in 0..=max_register {
            self.memory[(self.i + register) as usize] = self.registers[register as usize];
        }

        if !MODERN_LOAD_STORE {
            self.i += max_register;
        }
    }

    fn load_memory(&mut self, max_register: u16) {
        for register in 0..=max_register {
            self.registers[register as usize] = self.memory[(self.i + register) as usize];
        }

        if !MODERN_LOAD_STORE {
            self.i += max_register;
        }
    }
}

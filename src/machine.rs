const MEMORY_SIZE: usize = 4096;
const STACK_SIZE: usize = 16;
const NUM_REGISTERS: usize = 16;
pub const CLOCK_SPEED: u64 = 700; // Hz (instructions/second)
pub const SCREEN_SIZE_X: u32 = 64;
pub const SCREEN_SIZE_Y: u32 = 32;

pub struct CHIP8 {
    memory: [u8; MEMORY_SIZE],
    pub screen: [bool; (SCREEN_SIZE_X * SCREEN_SIZE_Y) as usize],
    pub pc: u16,
    i: u16,
    stack: [u16; STACK_SIZE],
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
            0x0 => {
                if instruction == 0x00E0 {
                    self.clear_screen();
                } else {
                    unimplemented!();
                }
            }
            0x1 => self.jump(nnn),
            0x2 => todo!(),
            0x3 => todo!(),
            0x4 => todo!(),
            0x5 => todo!(),
            0x6 => self.set_register(x, nn),
            0x7 => self.add_to_register(x, nn),
            0x8 => todo!(),
            0x9 => todo!(),
            0xA => self.set_index_register(nnn),
            0xB => todo!(),
            0xC => todo!(),
            0xD => {
                self.display(x, y, n);
                redraw_required = true;
            }
            0xE => todo!(),
            0xF => todo!(),
            _ => unreachable!(),
        }
        redraw_required
    }

    fn clear_screen(&mut self) {
        for pixel in self.screen.iter_mut() {
            *pixel = false;
        }
    }

    fn jump(&mut self, location: u16) {
        self.pc = location;
    }

    fn set_register(&mut self, register: u16, value: u16) {
        self.registers[register as usize] = value.try_into().expect("whoops!");
    }

    fn add_to_register(&mut self, register: u16, value: u16) {
        self.registers[register as usize] =
            self.registers[register as usize].wrapping_add(value.try_into().expect("whoops!"));
    }

    fn set_index_register(&mut self, value: u16) {
        self.i = value;
    }

    fn display(&mut self, x_register: u16, y_register: u16, sprite_height: u16) {
        let mut x_coord = (self.registers[x_register as usize] % (SCREEN_SIZE_X as u8)) as u32;
        let mut y_coord = (self.registers[y_register as usize] % (SCREEN_SIZE_Y as u8)) as u32;

        println!(
            "Display call: {} px high sprite @ x = {}, y = {}",
            sprite_height, x_coord, y_coord
        );

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
}

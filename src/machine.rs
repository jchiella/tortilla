const MEMORY_SIZE: usize = 4096;
const STACK_SIZE: usize = 16;
const NUM_REGISTERS: usize = 16;
pub const CLOCK_SPEED: u64 = 700; // Hz (instructions/second)
pub const SCREEN_SIZE_ROWS: u32 = 64;
pub const SCREEN_SIZE_COLS: u32 = 32;

pub struct CHIP8 {
    memory: [u16; MEMORY_SIZE],
    screen: [bool; (SCREEN_SIZE_ROWS * SCREEN_SIZE_COLS) as usize],
    pc: u16,
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
            screen: [false; (SCREEN_SIZE_ROWS * SCREEN_SIZE_COLS) as usize],
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

    pub fn fetch(&mut self) -> u16 {
        let instruction = self.memory[self.pc as usize] | (self.memory[self.pc as usize + 1] << 8);
        self.pc += 2;
        instruction
    }

    pub fn decode_and_execute(&mut self, instruction: u16) {
        let opcode = instruction & 0xf000 >> 12;
        let x = instruction & 0x0f00 >> 8;
        let y = instruction & 0x00f0 >> 4;
        let n = instruction & 0x000f;
        let nn = instruction & 0x0ff0 >> 4;
        let nnn = instruction & 0x0fff;

        match opcode {
            0x0 => todo!(),
            0x1 => todo!(),
            0x2 => todo!(),
            0x3 => todo!(),
            0x4 => todo!(),
            0x5 => todo!(),
            0x6 => todo!(),
            0x7 => todo!(),
            0x8 => todo!(),
            0x9 => todo!(),
            0xA => todo!(),
            0xB => todo!(),
            0xC => todo!(),
            0xD => todo!(),
            0xE => todo!(),
            0xF => todo!(),
            _ => unreachable!(),
        }
    }
}

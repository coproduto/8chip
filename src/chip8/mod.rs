mod instruction;
mod register;
mod types;
mod timer;

use self::instruction::Instruction;

pub struct Chip8 {
    memory: types::Memory,
    gfx: types::Display,
    stack: types::Stack,
    key: types::Keypad,
    registers: register::Registers,
    timers: timer::Timers,
}

impl Chip8 {
    pub fn new() -> Chip8 {        
        Chip8 {
            memory: [0; 4096],
            gfx: [false; 64 * 32],
            stack: [0; 16],
            key: [false; 16],
            registers: register::Registers::new(),
            timers: timer::Timers::new(),
        }
    }
    
    pub fn cycle(&mut self) {
        let opcode = self.fetch_opcode();
        let instruction = Instruction::decode(opcode);
        
        instruction.execute(self);
        self.timers.cycle();
    }

    fn fetch_opcode(&self) -> u16 {
        let ix = self.registers.program_counter as usize;
        read_from_slice(&self.memory[ix..ix+2])
    }
}

pub fn read_from_slice(slice: &[u8]) -> u16 {
    (slice[0] as u16) << 8 | (slice[1] as u16)
}

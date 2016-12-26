mod instruction;
mod register;
mod types;
mod timer;

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
    
    fn cycle(&mut self) {
        let opcode = self.fetch_instruction();
        let instruction = self.decode_instruction(opcode);
        
        self.execute_instruction(instruction);
        self.increment_counter(instruction);

        self.timers.timer_cycle();
    }
}

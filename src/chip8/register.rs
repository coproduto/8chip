pub struct Registers {
    pub program_counter: u16, //instruction pointer
    pub index: u16,           //index register
    pub stack_pointer: u16,   //stack pointer
    pub general: [u8; 16],    //general-purpose registers
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            program_counter: 0x200,
            index: 0,
            stack_pointer: 0,
            general: [0; 16],
        }
    }
}

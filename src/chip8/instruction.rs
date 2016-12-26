//TODO: Change this to represent actual instructions
pub struct Instruction {
    opcode: u16,
}

impl Instruction {    
    pub fn decode(opcode: u16) -> Instruction {
        //dummy implementation
        Instruction {
            opcode: opcode,
        }
    }

    pub fn execute(&self, chip: &mut super::Chip8) {
        //dummy implementation
        println!("executing opcode: {}", self.opcode);
        self.increment_counter(chip);
    }

    fn increment_counter(&self, chip: &mut super::Chip8) {
        //memory is byte-addressed, instructions are 2 bytes long
        //for some operations we might want to not increment the counter
        chip.registers.program_counter += 2;
    }
}

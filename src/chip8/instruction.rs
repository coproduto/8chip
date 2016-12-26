fn read_from_slice(slice: &[u8]) -> u16 {
    (slice[0] as u16) << 8 | (slice[1] as u16)
}

//TODO: Change this to represent actual instructions
#[derive(Clone, Copy)]
pub struct Instruction {
    opcode: u16,
}

impl super::Chip8 {
    pub fn fetch_instruction(&self) -> u16 {
      let ix = self.registers.program_counter as usize;
      read_from_slice(&self.memory[ix..ix+2])
    }

    pub fn decode_instruction(&self, opcode: u16) -> Instruction {
        //dummy implementation
        Instruction {
            opcode: opcode,
        }
    }

    pub fn execute_instruction(&mut self, instruction: Instruction) {
        //dummy implementation
        println!("executing opcode: {}", instruction.opcode);
    }

    pub fn increment_counter(&mut self, instruction: Instruction) {
        //memory is byte-addressed, instructions are 2 bytes long
        //for some operations we might want to not increment the counter
        self.registers.program_counter += 2;
    }
}

    

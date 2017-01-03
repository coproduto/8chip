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

#[cfg(test)]
mod tests {
    use super::*;
    
    quickcheck! {
        fn test_instruction_decode(opcode: u16) -> bool {
            let i = Instruction::decode(opcode);
            i.opcode == opcode
        }

        fn test_increment_counter(opcode: u16) -> bool {
            let i = Instruction::decode(opcode);
            let mut chip = super::super::Chip8::new();
            let pc = chip.registers.program_counter;

            i.increment_counter(&mut chip);

            chip.registers.program_counter > pc
        }

    }
}

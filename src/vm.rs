use crate::instruction::Opcode;
#[derive(Debug)]
pub struct VM {
    registers: [i32; 32],
    // Contains a small amount of fast storage, usually
    // indicated by the number of bits they can hold.
    pc: usize,
    // program counter: will track which byte is currently executing
    program: Vec<u8>,
    // A series of bytes representing opcodes to be executed as instructions.
    remainder: u32, 
    // Stores the potential remainder of DIV opcode executions.
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            program: vec![],
            pc: 0,
            remainder: 0,
        }
    }

    pub fn run(&mut self) {
        let mut is_done = false;

        while !is_done {
            is_done = self.execute_instruction();
        }
    }

    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    fn execute_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            // When this conditional is true, we will have executed all of the
            // instructions given in our program.
            return false;
        }
        match self.decode_opcode() {
            Opcode::LOAD => {
                // *Assuming LOAD is the first instruction in our program* 
                // Initially, the program counter is set to 0, targeting the first
                // byte in the program field on our VM struct.
                // We call `next_8_bits()`, and increment the program counter,
                // which we use to index into our program vector, and return the
                // byte (*the next 8 bits*) containing the address of of the register.
                let register = self.next_8_bits() as usize;
                let number = self.next_16_bits() as u32;
                self.registers[register] = number as i32;
            }
            Opcode::HLT => {
                println!("HLT Encountered");
                return false;
            }

            Opcode::ADD => {
                let register_1 = self.registers[self.next_8_bits() as usize];
                let register_2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register_1 + register_2;
            }

            Opcode::DIV => {
                let register_1 = self.registers[self.next_8_bits() as usize];
                let register_2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register_1 / register_2;
                self.remainder = (register_1 % register_2) as u32;
            },

            Opcode::JMP => {
                let target = self.registers[self.next_8_bits() as usize];
            }
            _ => return true,
        }

        true
    }

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        return opcode;
    }

    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        result
    }
    fn next_16_bits(&mut self) -> u16 {
        // Original state of two bytes: a = [x,x,x,x,x,x,x,x], b = [y,y,y,y,y,y,y,y].

        // x as u16 -> [0,0,0,0,0,0,0,0, x,x,x,x,x,x,x,x]
        // y as u16 -> [0,0,0,0,0,0,0,0, y,y,y,y,y,y,y,y]
        // x << 8   -> [x,x,x,x,x,x,x,x, 0,0,0,0,0,0,0,0]
        // v = x | y -> [x,x,x,x,x,x,x,x,y,y,y,y,y,y,y,y]
        // The pipe (|) is a bitwise or operator, that returns a 1 in each bit 
        // position for which the associated bits of each operand are 1, else 0.
        let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
        self.pc += 2;
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0)
    }

    fn test_opcode_hlt() {
        let mut test_vm = VM::new();
        let test_bytes = vec![200, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }
}

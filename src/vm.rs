use crate::instruction::Opcode;
#[derive(Debug)]
pub struct VM {
    registers: [i32; 32],
    // Contains a small amount of fast storage, usually
    // indicated by the number of bits they can hold.
    pc: usize,
    // program counter: will track which byte is currently executing
   pub program: Vec<u8>,
    // A series of bytes representing opcodes to be executed as instructions.
    remainder: u32, 
    // Stores the potential remainder of DIV opcode executions.
    equal_flag: bool,
    // Stores the result of the most recent comparison operation.
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            program: vec![],
            pc: 0,
            remainder: 0,
            equal_flag: false
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
 
                let register = self.next_8_bits() as usize;

                // *Assuming LOAD is the first instruction in our program* 
                // Initially, the program counter is set to 0, targeting the first
                // byte in the program field on our VM struct.
                // We call `next_8_bits()`, where we increment the program counter,
                // which we use to index into our program vector, and return the
                // byte (*the next 8 bits*) containing the address of the register.
                let number = self.next_16_bits() as u32;

                // Similar to register, except when this is called, we increment
                // the program counter twice, to indicate we've moved through
                // two bytes, or sixteen bits.

                self.registers[register] = number as i32;

            }
            Opcode::HLT => {
                println!("HLT Encountered");
                return false
            }

            Opcode::ADD => {
                let register_1 = self.registers[self.next_8_bits() as usize];
                let register_2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register_1 + register_2;
            }
            Opcode::SUB => {
                let register_1 = self.registers[self.next_8_bits() as usize];
                let register_2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register_1 - register_2;
            }
            Opcode::MUL => {
                let register_1 = self.registers[self.next_8_bits() as usize];
                let register_2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register_1 * register_2;
            }

            Opcode::DIV => {
                let register_1 = self.registers[self.next_8_bits() as usize];
                let register_2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register_1 / register_2;
                self.remainder = (register_1 % register_2) as u32;
            },

            Opcode::JMP => {
                let target = self.registers[self.next_8_bits() as usize];

                self.pc = target as usize;
            },

            Opcode::JMPB => {
                let value = self.registers[self.next_8_bits() as usize];
                self.pc -= value as usize;
            },

            Opcode::JMPF => { 
                let value = self.registers[self.next_8_bits() as usize];
                self.pc += value as usize;
            },

            Opcode::EQ => {
                let register_1 = self.registers[self.next_8_bits() as usize];
                let register_2 = self.registers[self.next_8_bits() as usize];

                if register_1 == register_2 {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }

                self.next_8_bits();
            }
            Opcode::NEQ => {
                let register_1 = self.registers[self.next_8_bits() as usize];
                let register_2 = self.registers[self.next_8_bits() as usize];

                if register_1 != register_2 {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }

                self.next_8_bits();
            },

            Opcode::GT => {
                let register_1 = self.registers[self.next_8_bits() as usize];
                let register_2 = self.registers[self.next_8_bits() as usize];

                if register_1 > register_2 {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }

                self.next_8_bits();

            },
            Opcode::LT => {
                let register_1 = self.registers[self.next_8_bits() as usize];
                let register_2 = self.registers[self.next_8_bits() as usize];

                if register_1 < register_2 {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }

                self.next_8_bits();

            }
            Opcode::GTQ => {
                let register_1 = self.registers[self.next_8_bits() as usize];
                let register_2 = self.registers[self.next_8_bits() as usize];

                if register_1 >= register_2 {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }

                self.next_8_bits();

            },
            Opcode::LTQ => {
                let register_1 = self.registers[self.next_8_bits() as usize];
                let register_2 = self.registers[self.next_8_bits() as usize];

                if register_1 <= register_2 {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }

                self.next_8_bits();

            },

            Opcode::JEQ => {
                let register = self.next_8_bits() as usize;
                let target = self.registers[register];
                if self.equal_flag {
                    self.pc = target as usize;
                }
            },
            Opcode::JNEQ => {
                let register = self.next_8_bits() as usize;
                let target = self.registers[register];
                if !self.equal_flag {
                    self.pc = target as usize;
                }
            },

            _ => return true,
        }

        true
    }

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        println!("Our opcode is : {:?}", opcode);
        println!("Our program counter is : {:?}", self.pc);
        self.pc += 1;
        println!("Our opcode is : {:?}", opcode);
        println!("Our program counter is : {:?}", self.pc);

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
        assert_eq!(test_vm.registers[0], 0);
    }
    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = VM::new();
        let test_bytes = vec![5, 0, 0, 0, 1];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }
    #[test]
    fn test_load_opcode() {
        let mut test_vm = VM::new();
        test_vm.program = vec![0,0,1,244];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn test_add_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 5;
        test_vm.registers[1] = 4;
        test_vm.program = vec![1,0,1,2];
        test_vm.run();
        assert_eq!(test_vm.registers[2], 9);
    }
    #[test]
    fn test_sub_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 5;
        test_vm.registers[1] = 4;
        test_vm.program = vec![2,0,1,2];
        test_vm.run();
        assert_eq!(test_vm.registers[2], 1);
    }
    #[test]
    fn test_mul_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 5;
        test_vm.registers[1] = 4;
        test_vm.program = vec![3,0,1,2];
        test_vm.run();
        assert_eq!(test_vm.registers[2], 20);
    }
    #[test]
    fn test_div_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 5;
        test_vm.registers[1] = 4;
        test_vm.program = vec![4,0,1,2];
        test_vm.run();
        assert_eq!(test_vm.registers[2], 1);
        assert_eq!(test_vm.remainder, 1);
    }
    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        let test_bytes = vec![200,0,0,0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }
    #[test]

    fn test_jmp_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 1;
        test_vm.program = vec![6, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]

    fn test_jmpf_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 2;
        test_vm.program = vec![7, 0, 0, 0, 6, 0, 0, 0,];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 4);
    }
}

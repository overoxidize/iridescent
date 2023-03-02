use std;
use std::io;
use std::io::Write;
use nom::types::CompleteStr;
use std::num::ParseIntError;
use crate::assembler::program_parsers::{Program, program};
use crate::instruction;
use crate::vm::VM;

pub struct REPL {
    /// Provides a repl
    pub command_buffer: Vec<String>,
    pub vm: VM,
}


impl REPL {
    /// Provides a new instance of a REPL to feed programs into the VM.
    pub fn new() -> REPL {
        REPL {
            vm: VM::new(),
            command_buffer: vec![]
        }
    }

    /// Expects a hexadecimal string, not including a leading `0x`, and returns
    /// a vector of u8's.
    /// An example LOAD command would be `00 01 03 E8`.
    fn parse_hex(&mut self, instruction: &str) -> Result<Vec<u8>, ParseIntError> {
        let split = instruction.split(" ").collect::<Vec<&str>>();

        let mut results: Vec<u8> = vec![];

        for hex_string in split {
            let byte = u8::from_str_radix(&hex_string, 16);
            
            match byte {
                Ok(result) => {
                    results.push(result);
                },
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(results)
    }
    fn program(&mut self, _args: &[&str]) {
        // self.send_message("Listing instructions currently in VM's program vector: ".to_string());
        let mut results = vec![];
        for instruction in &self.vm.program {
            results.push(instruction.clone())
        }
        // self.send_message(format!("{:#?}", results));
        // self.send_message("End of Program Listing".to_string());
    }

    pub fn run(&mut self) {
        /// Runs the repl in the terminal, allows for viewing
        /// the history of instructions fed to the repl.
        println!("Welcome to Iridescent! May your code compile.");

        loop {
            let mut buffer = String::new();

            let stdin = io::stdin();
            print!(">>> ");
            io::stdout().flush().expect("Unable to flush stdout");

            stdin.read_line(&mut buffer).expect("Unable to read line from user");

            let buffer = buffer.trim();

            self.command_buffer.push(buffer.to_string());

            match buffer {
                ".quit" => {
                    println!("Farewell!");
                    std::process::exit(0);
                },

                ".history" => {
                    for command in &self.command_buffer {
                        println!("{}", command);
                    }
                }
                ".program" => {
                    println!("Listing instructions current in VM.s program vector:");
                    for instruction in &self.vm.program {
                        println!("{}", instruction);
                    }
                    println!("End of program instructions");
                }
                ".registers" => {
                    println!("Listing registers and all contents:");
                    println!("{:#?}", self.vm.registers);
                    println!("End of Register Listing")
                }
                _ => {
                    println!("Error");
                }
                // _ => {
                //     let parsed_program = program.push((CompleteStr(buffer));
                //     if !parsed_program.is_ok() {
                //         println!("Unable to parse input");
                //         continue;
                //     }
                //     let (_, result) = parsed_program.unwrap();
                //     let bytecode = result.to_bytes();
                //     // TODO: Make a function to let us add bytes to the VM
                //     for byte in bytecode {
                //         self.vm.add_byte(byte);
                //     }
                //     self.vm.run_once();
                // }
            }
        }
    }
}
use std;
use std::io;
use std::io::Write;
use nom::types::CompleteStr;

use crate::vm::VM;

pub struct REPL {
    pub command_buffer: Vec<String>,
    pub vm: VM,
}


impl REPL {
    pub fn new() -> REPL {
        REPL {
            vm: VM::new(),
            command_buffer: vec![]
        }
    }

    pub fn run(&mut self) {
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
                _ => {
                    let parsed_program = self.program(CompleteStr(buffer));

                    if !parsed_program.is_ok() {
                        println!("Unable to parse input");
                        continue;
                    }

                    let (_, result) = parsed_program.unwrap();

                    let bytecode = result.to_bytes();

                    // for byte in bytecode {
                    //     self.vm.add_byte(byte)
                    // }
                    self.vm.run_once();
                }
            }
        }
    }
}
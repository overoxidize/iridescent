#![warn(missing_docs)]
#[macro_use]
pub mod vm;
pub mod instruction;
pub mod repl;
pub mod assembler;

fn main() {
    let mut repl = repl::REPL::new();
    repl.run();
}

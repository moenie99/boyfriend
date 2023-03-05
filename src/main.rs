#![deny(clippy::all)]

use std::{
    env::args,
    error::Error,
    fs,
    io::{stdin, stdout, Read, StdinLock, StdoutLock, Write},
};

use codegen::{codegen, Opcode};
use parser::parse;

mod codegen;
mod parser;

fn get(stdin: &mut StdinLock) -> u8 {
    stdin.by_ref().bytes().next().unwrap().unwrap()
}

fn put(stdout: &mut StdoutLock, byte: u8) {
    stdout.write_all(&[byte]).unwrap();
}

/// This function is responsible for interpreting our generated opcodes.
/// (Look at how cute and smol it is :O)
fn execute(program: &[Opcode], tape: &mut [u8]) {
    let mut program_counter = 0;
    let mut data_pointer = 0;

    let mut stdin = stdin().lock();
    let mut stdout = stdout().lock();

    while let Some(opcode) = program.get(program_counter) {
        match opcode {
            Opcode::MoveLeft(n) => data_pointer -= n,
            Opcode::MoveRight(n) => data_pointer += n,
            Opcode::Add(n) => tape[data_pointer] = tape[data_pointer].wrapping_add(*n),
            Opcode::Sub(n) => tape[data_pointer] = tape[data_pointer].wrapping_sub(*n),
            Opcode::Write => put(&mut stdout, tape[data_pointer]),
            Opcode::Read => tape[data_pointer] = get(&mut stdin),
            Opcode::JumpIfZero(addr) => {
                if tape[data_pointer] == 0 {
                    program_counter = *addr;
                }
            }
            Opcode::JumpUnlessZero(addr) => {
                if tape[data_pointer] != 0 {
                    program_counter = *addr;
                }
            }
        }
        program_counter += 1;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = args().skip(1);

    match args.next() {
        Some(path) => {
            if let Some(unused) = args.next() {
                return Err(format!("Incorrect usage: {unused} and all subsequent arguments are ignored. Usage: boyfriend [path]").into());
            }

            let source = fs::read_to_string(&path)?;
            let ast = parse(&source).ok_or_else(|| format!("The file at `{path}` couldn't be parsed. This is likely due to misbalanced brackets. Good luck finding them!"))?;
            let program = codegen(ast);
            let mut tape = vec![0; 30_000];
            execute(&program, &mut tape);
        }

        None => return Err("Usage: boyfriend [path]".into()),
    }

    Ok(())
}

use crate::parser::{Ast, Instruction};

#[derive(Debug, PartialEq)]
pub enum Opcode {
    MoveLeft(usize),
    MoveRight(usize),
    Add(u8),
    Sub(u8),
    Write,
    Read,
    JumpIfZero(usize),
    JumpUnlessZero(usize),
}

pub type Program = Vec<Opcode>;

pub fn codegen(ast: Ast) -> Program {
    codegen_at_depth(ast, 0)
}

pub fn codegen_at_depth(ast: Ast, depth: usize) -> Program {
    let mut program = vec![];

    for instruction in ast {
        match instruction {
            Instruction::MoveRight => match program.last_mut() {
                Some(Opcode::MoveRight(n)) => *n = n.wrapping_add(1),
                _ => program.push(Opcode::MoveRight(1)),
            },
            Instruction::MoveLeft => match program.last_mut() {
                Some(Opcode::MoveLeft(n)) => *n = n.wrapping_add(1),
                _ => program.push(Opcode::MoveLeft(1)),
            },
            Instruction::Increment => match program.last_mut() {
                Some(Opcode::Add(n)) => *n = n.wrapping_add(1),
                _ => program.push(Opcode::Add(1)),
            },
            Instruction::Decrement => match program.last_mut() {
                Some(Opcode::Sub(n)) => *n = n.wrapping_add(1),
                _ => program.push(Opcode::Sub(1)),
            },
            Instruction::Write => program.push(Opcode::Write),
            Instruction::Read => program.push(Opcode::Read),
            Instruction::Loop(inner) => {
                let mut inner_program = codegen_at_depth(inner, depth + 1);
                let loop_start_location = program.len() + depth;
                let delta = inner_program.len() + depth;
                program.push(Opcode::JumpIfZero(program.len() + delta + 1));
                program.append(&mut inner_program);
                program.push(Opcode::JumpUnlessZero(loop_start_location));
            }
        };
    }
    program
}

#[cfg(test)]
mod tests {
    use crate::parser::parse;

    use super::*;
    use Opcode::*;

    #[track_caller]
    fn assert_codegen(source: &str, expected: Program) {
        let ast = parse(source).unwrap();
        let program = codegen(ast);
        assert_eq!(expected, program);
    }

    #[test]
    fn empty_ast() {
        assert_codegen("", vec![])
    }

    #[test]
    fn folding() {
        assert_codegen("++", vec![Add(2)]);
    }

    #[test]
    fn calculate_loop_offsets() {
        assert_codegen("[]", vec![JumpIfZero(1), JumpUnlessZero(0)]);

        assert_codegen("[+]", vec![JumpIfZero(2), Add(1), JumpUnlessZero(0)]);

        assert_codegen("[+++]", vec![JumpIfZero(2), Add(3), JumpUnlessZero(0)]);

        assert_codegen(
            "[[[]]]",
            vec![
                JumpIfZero(5),
                JumpIfZero(4),
                JumpIfZero(3),
                JumpUnlessZero(2),
                JumpUnlessZero(1),
                JumpUnlessZero(0),
            ],
        );

        assert_codegen(
            "[[]]",
            vec![
                JumpIfZero(3),
                JumpIfZero(2),
                JumpUnlessZero(1),
                JumpUnlessZero(0),
            ],
        );

        assert_codegen(
            "[+[+]+]",
            vec![
                JumpIfZero(6),
                Add(1),
                JumpIfZero(4),
                Add(1),
                JumpUnlessZero(2),
                Add(1),
                JumpUnlessZero(0),
            ],
        );
    }
}

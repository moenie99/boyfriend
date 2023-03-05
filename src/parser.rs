use std::str::Chars;

pub type Ast = Vec<Instruction>;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    MoveRight,
    MoveLeft,
    Increment,
    Decrement,
    Write,
    Read,
    Loop(Ast),
}

use Instruction::*;

pub fn parse(source: &str) -> Option<Ast> {
    parse_at_depth(&mut source.chars(), &mut 0)
}

fn parse_at_depth(chars: &mut Chars<'_>, loop_depth: &mut usize) -> Option<Ast> {
    let mut ast = Vec::new();

    while let Some(c) = chars.next() {
        let instruction = match c {
            '>' => MoveRight,
            '<' => MoveLeft,
            '+' => Increment,
            '-' => Decrement,
            '.' => Write,
            ',' => Read,
            '[' => {
                let old_loop_depth = *loop_depth;
                *loop_depth += 1;
                let inner = parse_at_depth(chars, loop_depth)?;
                if *loop_depth == old_loop_depth {
                    Loop(inner)
                } else {
                    return None;
                }
            }
            ']' => {
                if *loop_depth > 0 {
                    *loop_depth -= 1;
                    break;
                } else {
                    return None;
                }
            }
            _ => continue,
        };

        ast.push(instruction);
    }

    Some(ast)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[track_caller]
    fn assert_parse(source: &str, expected: Ast) {
        let mut chars = source.chars();
        let program = parse(source).unwrap();
        assert_eq!(expected, program);
        assert!(chars.as_str().is_empty());
    }

    #[track_caller]
    #[should_panic]
    fn fail(source: &str) {
        assert_parse(source, vec![]);
    }

    #[test]
    fn empty_source() {
        assert_parse("this program is effectively empty", vec![]);
        assert_parse("", vec![]);
    }

    #[test]
    fn singleton_instructions() {
        assert_parse(
            "><+-.,",
            vec![MoveRight, MoveLeft, Increment, Decrement, Write, Read],
        )
    }

    #[test]
    fn empty_loop() {
        assert_parse("[]", vec![Loop(vec![])])
    }

    #[test]
    fn simple_loop() {
        assert_parse("+[-]+", vec![Increment, Loop(vec![Decrement]), Increment]);
    }

    #[test]
    fn nested_loop() {
        assert_parse(
            "+[-[+]-]+",
            vec![
                Increment,
                Loop(vec![Decrement, Loop(vec![Increment]), Decrement]),
                Increment,
            ],
        );
    }

    #[test]
    #[should_panic]
    fn missing_close_loop() {
        fail("+[-+");
    }

    #[test]
    #[should_panic]
    fn missing_open_loop() {
        fail("+[-]+]");
    }
}

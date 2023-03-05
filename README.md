# boyfriend
An optimizing (folding instructions and flattening loops to jumps) brainfuck compiler.

The 8 brainfuck instructions we all know and love get converted to a set of more primitive opcodes:

```rust
pub enum Opcode {
    /// Move the data pointer to the left by the specified amount.
    MoveLeft(usize),
    /// Move the data pointer to the right by the specified amount.
    MoveRight(usize),
    /// Add the specified amount to the current cell value.
    Add(u8),
    /// Substract the specified amount to the current cell value.
    Sub(u8),
    /// Write the current cell value to stdout.
    Write,
    /// Read one byte from stdin and write it to the current cell.
    Read,
    /// Jump to the specified instruction if the current cell value is 0.
    JumpIfZero(usize),
    /// Jump to the specified instruction if the current cell value is not equal to 0.
    JumpUnlessZero(usize),
}
```

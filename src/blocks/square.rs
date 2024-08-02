// This is free and unencumbered software released into the public domain.

use asimov_sdk::flow::derive::Block;
use asimov_sdk::flow::{Block, BlockResult, BlockRuntime, InputPort, OutputPort, Port};

/// A block that computes the square of integer inputs.
#[derive(Block, Clone)]
pub struct Square {
    /// The input message stream.
    #[input]
    pub input: InputPort<i64>,

    /// The output message stream.
    #[output]
    pub output: OutputPort<i64>,
}

impl Square {
    pub fn new(input: InputPort<i64>, output: OutputPort<i64>) -> Self {
        Self { input, output }
    }
}

impl Block for Square {
    fn execute(&mut self, _runtime: &dyn BlockRuntime) -> BlockResult {
        while let Some(input) = self.input.recv()? {
            if !self.output.is_connected() {
                continue;
            }

            let output = input * input;
            self.output.send(&output)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Square;
    use asimov_sdk::flow::{transports::MockTransport, System};

    #[test]
    fn instantiate_square_block() {
        // Check that the block is constructible:
        let _ = System::<MockTransport>::build(|s| {
            let _ = s.block(Square::new(s.input(), s.output()));
        });
    }
}

// This is free and unencumbered software released into the public domain.

use asimov_sdk::flow::derive::Block;
use asimov_sdk::flow::{Block, BlockError, BlockRuntime, InputPort, OutputPort, Port};

/// A block that computes the square of integer inputs.
#[derive(Block)]
pub struct Square {
    /// The input message stream.
    #[input]
    input: InputPort<u64>,

    /// The output message stream.
    #[output]
    output: OutputPort<u64>,
}

impl Block for Square {
    fn execute(&mut self, _runtime: &dyn BlockRuntime) -> Result<(), BlockError> {
        while let Some(input) = self.input.receive()? {
            if !self.output.is_connected() {
                continue;
            }

            let output = input * input;
            self.output.send(&output)?;
        }
        Ok(())
    }
}

// This is free and unencumbered software released into the public domain.

use crate::prelude::{vec, Vec};
use asimov_sdk::flow::{Block, InputPort, OutputPort, Port, PortDescriptor, Scheduler};

/// A block that computes the square of integer inputs.
pub struct Square {
    /// The input message stream.
    input: InputPort<u64>,
    /// The output message stream.
    output: OutputPort<u64>,
}

impl Block for Square {
    fn inputs(&self) -> Vec<PortDescriptor> {
        vec![PortDescriptor::from(&self.input)]
    }

    fn outputs(&self) -> Vec<PortDescriptor> {
        vec![PortDescriptor::from(&self.output)]
    }

    fn execute(&mut self, _scheduler: &dyn Scheduler) -> Result<(), ()> {
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

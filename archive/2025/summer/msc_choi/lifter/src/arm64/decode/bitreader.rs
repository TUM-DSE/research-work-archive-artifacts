use crate::arm64::common::types::{BITS_IN_BYTE, INSTRUCTION_BYTE_SIZE};
use crate::arm64::AArch64LifterError;
use crate::arm64::AArch64LifterError::{InvalidFieldRange, NoLoadedInstruction, UnexpectedEndOfInput};

pub struct BitReader<'a> {
    data: &'a [u8],
    position: usize,
    instruction: Option<String>,
}

impl<'a> BitReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        BitReader {
            data,
            position: 0,
            instruction: None,
        }
    }

    pub fn load_instruction(&mut self) -> Result<bool, AArch64LifterError> {
        if self.position == self.data.len() {
            return Ok(false);
        } else if self.position + INSTRUCTION_BYTE_SIZE > self.data.len() {
            return Err(UnexpectedEndOfInput);
        }

        let instruction_bytes = &self.data[self.position..self.position + INSTRUCTION_BYTE_SIZE];

        self.instruction = Some(
            instruction_bytes
                .iter()
                .rev()
                .map(|&byte| format!("{:08b}", byte))
                .collect::<Vec<_>>()
                .concat(),
        );

        self.position += INSTRUCTION_BYTE_SIZE;
        // println!("Instruction: {}", self.instruction.as_ref().unwrap());

        Ok(true)
    }

    pub fn extract_slice(&mut self, offset: usize, length: usize) -> Result<String, AArch64LifterError> {
        if self.instruction.is_none() {
            return Err(NoLoadedInstruction);
        }

        if offset + length > INSTRUCTION_BYTE_SIZE * BITS_IN_BYTE {
            return Err(InvalidFieldRange(offset, length));
        }

        let instruction = self.instruction.as_ref().unwrap();
        let logical_offset = INSTRUCTION_BYTE_SIZE * BITS_IN_BYTE - offset - length;

        Ok(instruction[logical_offset..logical_offset + length].to_string())
    }

    pub fn match_bits(bits_literal: &str, bits_pattern: &str) -> bool {
        bits_literal.len() == bits_pattern.len() && bits_literal.chars().zip(bits_pattern.chars()).all(|(a, b)| b == 'x' || a == b)
    }
}

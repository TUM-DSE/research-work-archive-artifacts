use crate::arm64::common::types::Instruction;
use crate::arm64::decode::bitreader::BitReader;
use crate::arm64::decode::generated::decode_logic::generated_decode_logic;
use crate::arm64::AArch64LifterError;

pub struct Decoder;

impl Decoder {
    pub fn decode(&self, reader: &mut BitReader) -> Result<Option<Instruction>, AArch64LifterError> {
        let end_of_input = !reader.load_instruction()?;
        if end_of_input {
            return Ok(None);
        }

        Ok(Some(generated_decode_logic(reader)?))
    }
}

use crate::arm64::common::types::INSTRUCTION_BYTE_SIZE;
use crate::arm64::decode::BitReader;
use crate::arm64::decode::Decoder;
use crate::arm64::lift::generated::lift_logic::generated_lift_logic;
use crate::arm64::lift::types::{BlockSequencer, BlockType};
use crate::Lifter;
use target_lexicon::{Aarch64Architecture, Architecture};
use thiserror::Error;
use tnj::air::instructions::CodeRegion;
use tnj::arch::get_arch;

/// A lifter for AArch64
pub struct AArch64Lifter;

impl Lifter for AArch64Lifter {
    type E = AArch64LifterError;

    fn lift(&self, code: &[u8], _proofs: &[u8], decode_only: bool) -> Result<CodeRegion, Self::E> {
        let decoder = Decoder;
        let mut reader = BitReader::new(code);

        let arch = get_arch(Architecture::Aarch64(Aarch64Architecture::Aarch64)).unwrap();
        let mut blob = CodeRegion::with_entry_block(arch);
        let mut builder = blob.insert();
        let mut sequencer = BlockSequencer::new(code, &decoder, decode_only)?;

        if !decode_only {
            sequencer.create_blocks(&mut builder)?;
        }

        // let total = code.len() / INSTRUCTION_BYTE_SIZE;
        // let pb = ProgressBar::new(total as u64);
        // pb.set_message("Pass 2: lifting instructions");
        // pb.set_style(
        //     ProgressStyle::with_template("\t\t[{bar:20}] {pos}/{len} {msg}")
        //         .unwrap()
        //         .progress_chars("=> "),
        // );

        let mut pc = 0u64;

        loop {
            if let Some(instruction) = decoder.decode(&mut reader)? {
                if !decode_only {
                    match sequencer.get_block(pc, BlockType::InterBlock, &mut builder, &[]) {
                        Ok(block) => {
                            builder.jump(block, vec![]);
                            builder.set_insert_block(block);
                        }
                        Err(AArch64LifterError::MissingBlockInSequencer) => {}
                        Err(e) => return Err(e),
                    }

                    generated_lift_logic(instruction, &mut builder, &mut sequencer, pc)?;
                }

                // pb.inc(1);
            } else {
                // pb.finish_and_clear();
                break;
            }

            pc += INSTRUCTION_BYTE_SIZE as u64;
        }

        Ok(blob)
    }
}

/// Error type for lifting from machine code to AIR.
#[derive(Debug, Clone, Error)]
pub enum AArch64LifterError {
    /// The total length of decoder slices is invalid. It must equal 32.
    #[error("Total length of decoder slices {0} must be 32")]
    InvalidDecoderSlices(usize),

    /// The field length is invalid. It must be a nonzero value less than 32.
    #[error("Field length {0} must be a nonzero number less than 32")]
    InvalidFieldLength(usize),

    /// The field length for a boolean must be exactly 1.
    #[error("Bool field length {0} must be 1")]
    InvalidBoolFieldLength(usize),

    /// The field offset and length must fall within the instruction slice (64 bits).
    #[error("Field offset {0} and length {1} should be within the instruction slice of length 64")]
    InvalidFieldRange(usize, usize),

    /// The next instruction has yet to be loaded.
    #[error("load_instruction() must be called before slice_instruction() or extract_field()")]
    NoLoadedInstruction,

    /// End of input occurred unexpectedly while reading bits.
    #[error("Unexpected end of input while reading bits")]
    UnexpectedEndOfInput,

    /// The instruction is unpredictable based on the bit sequence.
    #[error("Instruction unpredictable from bit sequence")]
    UnpredictableInstruction,

    /// The instruction is unallocated based on the bit sequence.
    #[error("Instruction unallocated from bit sequence")]
    UnallocatedInstruction,

    /// The instruction behavior is undefined in current condition.
    #[error("Instruction behavior undefined in current condition")]
    UndefinedInstruction,

    /// The instruction is not covered by the ASL decode specification.
    #[error("Instruction not covered by specification")]
    UnspecifiedInstruction,

    /// The instruction's translation is not implemented in the generator.
    #[error("Instruction not implemented in generator file {0} line {1}")]
    UnimplementedInstruction(String, usize),

    /// The builtin function needs to be implemented manually.
    #[error("Builtin function {0} not implemented in lifter/src/arm64/helpers.rs")]
    UnimplementedBuiltinFunction(String),

    /// A bits literal contains invalid characters (only '0' and '1' are allowed).
    #[error("Bits literal {0} must only contain 0s and 1s")]
    InvalidBitsLiteral(String),

    /// No path leads to a return statement in the generated function.
    #[error("No path with a return statement in function {0}")]
    NothingToReturn(String),

    /// Operand type not appropriate for operation.
    #[error("Operand type not appropriate for operation")]
    InvalidType,

    /// Slice indices are out of range.
    #[error("Invalid slice range")]
    InvalidSliceRange,

    /// Concatenation of bits cannot have a length bigger than 64.
    #[error("Invalid concat length")]
    InvalidConcatLength,

    /// For certain operations BigInt has restricted size (shl, shr, pow2).
    #[error("Integer value too big for operation")]
    IntegerTooBig,

    /// AIR int size should be 1, 8, 16, 32, 64, or 128.
    #[error("Invalid size for AIR int creation")]
    InvalidBitsLength,

    /// Invalid AIR type
    #[error("Invalid AIR type")]
    InvalidAirType,

    /// Variable is not of type integer
    #[error("Variable is not of type integer")]
    VariableNotInteger,

    /// Variable is not of type bits
    #[error("Variable is not of type bits")]
    VariableNotBits,

    /// Variable is not of type integer or bits
    #[error("Variable is not of type integer or bits")]
    VariableNotIntegerOrBits,

    /// Variable is not of type expected enum
    #[error("Variable is not of type expected enum")]
    VariableNotExpectedEnum,

    /// Variable is not of type expected record
    #[error("Variable is not of type expected record")]
    VariableNotExpectedRecord,

    /// Variable is not of type air
    #[error("Variable is not of type air")]
    VariableNotAir,

    /// Variable is already of type air
    #[error("Variable is already of type air")]
    VariableAlreadyAir,

    /// Variable value is not propagated
    #[error("Variable value is not propagated")]
    VariableNotPropagated,

    /// Return statement is expected but not found
    #[error("Return statement is expected but not found")]
    ExpectedReturnNotFound,

    /// Check the decode stage to see if the operands are correctly extracted.
    #[error("Operand {0} not extracted during decode stage")]
    OperandNotFound(String),

    /// You can't extract the value of AIR variable that depends on register value.
    #[error("AIR variable must not depend on register value")]
    ConstNotAvailableFromAir,

    /// Register type not supported
    #[error("Register type not supported")]
    UnsupportedRegister,

    /// AIR block was not added to sequencer during the checkpointing pass.
    #[error("AIR block was not added to sequencer during the checkpointing pass")]
    MissingBlockInSequencer,

    /// Mismatched packable type
    #[error("Mismatched packable type: expected one of {0}, but found {1}")]
    MismatchedAirPackableType(String, String),

    /// Not implemented
    #[error("Not implemented at {0}:{1}")]
    NotImplemented(&'static str, u32),

    /// A custom error with a specific message.
    #[error("{0}")]
    CustomError(String),
}

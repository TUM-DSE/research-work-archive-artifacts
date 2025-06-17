#![allow(nonstandard_style, unused)]
use crate::arm64::AArch64LifterError;
use crate::arm64::{decode, common};
use crate::arm64::common::types::BigIntExt;
use crate::integer_to_usize;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive};
use std::collections::BTreeMap;
pub fn decode_aarch64_branch_unconditional_eret(
    reader: &mut decode::BitReader,
) -> Result<common::types::Instruction, AArch64LifterError> {
    let A = common::types::bits::from_bits_literal(
        &reader.extract_slice(11usize, 1usize)?,
    )?;
    let M = common::types::bits::from_bits_literal(
        &reader.extract_slice(10usize, 1usize)?,
    )?;
    let Rn = common::types::bits::from_bits_literal(
        &reader.extract_slice(5usize, 5usize)?,
    )?;
    let op4 = common::types::bits::from_bits_literal(
        &reader.extract_slice(0usize, 5usize)?,
    )?;
    Ok(
        common::types::Instruction::aarch64_branch_unconditional_eret(
            Box::new(common::types::aarch64_branch_unconditional_eret_operands {
            }),
        ),
    )
}

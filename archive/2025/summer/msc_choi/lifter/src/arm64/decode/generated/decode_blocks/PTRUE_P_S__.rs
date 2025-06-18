#![allow(nonstandard_style, unused)]
use crate::arm64::AArch64LifterError;
use crate::arm64::{decode, common};
use crate::arm64::common::types::BigIntExt;
use crate::integer_to_usize;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive};
use std::collections::BTreeMap;
pub fn decode_PTRUE_P_S__(
    reader: &mut decode::BitReader,
) -> Result<common::types::Instruction, AArch64LifterError> {
    let size = common::types::bits::from_bits_literal(
        &reader.extract_slice(22usize, 2usize)?,
    )?;
    let pattern = common::types::bits::from_bits_literal(
        &reader.extract_slice(5usize, 5usize)?,
    )?;
    let Pd = common::types::bits::from_bits_literal(
        &reader.extract_slice(0usize, 4usize)?,
    )?;
    let mut d: common::types::integer = decode::helpers::UInt_1(
        Pd,
        common::types::integer::from(4),
    )?;
    Ok(
        common::types::Instruction::PTRUE_P_S__(
            Box::new(common::types::PTRUE_P_S___operands {
                d,
            }),
        ),
    )
}

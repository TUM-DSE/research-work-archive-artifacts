#![allow(nonstandard_style, unused)]
use crate::arm64::AArch64LifterError;
use crate::arm64::{decode, common};
use crate::arm64::common::types::BigIntExt;
use crate::integer_to_usize;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive};
use std::collections::BTreeMap;
pub fn decode_aarch64_vector_shift_right_narrow_logical(
    reader: &mut decode::BitReader,
) -> Result<common::types::Instruction, AArch64LifterError> {
    let Q = common::types::bits::from_bits_literal(
        &reader.extract_slice(30usize, 1usize)?,
    )?;
    let immh = common::types::bits::from_bits_literal(
        &reader.extract_slice(19usize, 4usize)?,
    )?;
    let immb = common::types::bits::from_bits_literal(
        &reader.extract_slice(16usize, 3usize)?,
    )?;
    let op = common::types::bits::from_bits_literal(
        &reader.extract_slice(11usize, 1usize)?,
    )?;
    let Rn = common::types::bits::from_bits_literal(
        &reader.extract_slice(5usize, 5usize)?,
    )?;
    let Rd = common::types::bits::from_bits_literal(
        &reader.extract_slice(0usize, 5usize)?,
    )?;
    let mut d: common::types::integer = decode::helpers::UInt_1(
        Rd,
        common::types::integer::from(5),
    )?;
    let mut n: common::types::integer = decode::helpers::UInt_1(
        Rn,
        common::types::integer::from(5),
    )?;
    Ok(
        common::types::Instruction::aarch64_vector_shift_right_narrow_logical(
            Box::new(common::types::aarch64_vector_shift_right_narrow_logical_operands {
                d,
                n,
            }),
        ),
    )
}

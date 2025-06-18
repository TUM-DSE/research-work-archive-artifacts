#![allow(nonstandard_style, unused)]
use crate::arm64::AArch64LifterError;
use crate::arm64::{decode, common};
use crate::arm64::common::types::BigIntExt;
use crate::integer_to_usize;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive};
use std::collections::BTreeMap;
pub fn decode_aarch64_float_compare_cond(
    reader: &mut decode::BitReader,
) -> Result<common::types::Instruction, AArch64LifterError> {
    let ftype = common::types::bits::from_bits_literal(
        &reader.extract_slice(22usize, 2usize)?,
    )?;
    let Rm = common::types::bits::from_bits_literal(
        &reader.extract_slice(16usize, 5usize)?,
    )?;
    let cond = common::types::bits::from_bits_literal(
        &reader.extract_slice(12usize, 4usize)?,
    )?;
    let Rn = common::types::bits::from_bits_literal(
        &reader.extract_slice(5usize, 5usize)?,
    )?;
    let op = common::types::bits::from_bits_literal(
        &reader.extract_slice(4usize, 1usize)?,
    )?;
    let nzcv = common::types::bits::from_bits_literal(
        &reader.extract_slice(0usize, 4usize)?,
    )?;
    let mut n: common::types::integer = decode::helpers::UInt_1(
        Rn,
        common::types::integer::from(5),
    )?;
    Ok(
        common::types::Instruction::aarch64_float_compare_cond(
            Box::new(common::types::aarch64_float_compare_cond_operands {
                n,
            }),
        ),
    )
}

#![allow(nonstandard_style, unused)]
use crate::arm64::AArch64LifterError;
use crate::arm64::{decode, common};
use crate::arm64::common::types::BigIntExt;
use crate::integer_to_usize;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive};
use std::collections::BTreeMap;
pub fn decode_aarch64_vector_fp16_movi(
    reader: &mut decode::BitReader,
) -> Result<common::types::Instruction, AArch64LifterError> {
    let Q = common::types::bits::from_bits_literal(
        &reader.extract_slice(30usize, 1usize)?,
    )?;
    let a = common::types::bits::from_bits_literal(
        &reader.extract_slice(18usize, 1usize)?,
    )?;
    let b = common::types::bits::from_bits_literal(
        &reader.extract_slice(17usize, 1usize)?,
    )?;
    let c = common::types::bits::from_bits_literal(
        &reader.extract_slice(16usize, 1usize)?,
    )?;
    let d = common::types::bits::from_bits_literal(
        &reader.extract_slice(9usize, 1usize)?,
    )?;
    let e = common::types::bits::from_bits_literal(
        &reader.extract_slice(8usize, 1usize)?,
    )?;
    let f = common::types::bits::from_bits_literal(
        &reader.extract_slice(7usize, 1usize)?,
    )?;
    let g = common::types::bits::from_bits_literal(
        &reader.extract_slice(6usize, 1usize)?,
    )?;
    let h = common::types::bits::from_bits_literal(
        &reader.extract_slice(5usize, 1usize)?,
    )?;
    let Rd = common::types::bits::from_bits_literal(
        &reader.extract_slice(0usize, 5usize)?,
    )?;
    Ok(
        common::types::Instruction::aarch64_vector_fp16_movi(
            Box::new(common::types::aarch64_vector_fp16_movi_operands {
            }),
        ),
    )
}

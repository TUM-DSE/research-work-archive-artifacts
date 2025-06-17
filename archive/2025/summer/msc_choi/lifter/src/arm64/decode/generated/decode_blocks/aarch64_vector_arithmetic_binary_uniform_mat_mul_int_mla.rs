#![allow(nonstandard_style, unused)]
use crate::arm64::AArch64LifterError;
use crate::arm64::{decode, common};
use crate::arm64::common::types::BigIntExt;
use crate::integer_to_usize;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive};
use std::collections::BTreeMap;
pub fn decode_aarch64_vector_arithmetic_binary_uniform_mat_mul_int_mla(
    reader: &mut decode::BitReader,
) -> Result<common::types::Instruction, AArch64LifterError> {
    let U = common::types::bits::from_bits_literal(
        &reader.extract_slice(29usize, 1usize)?,
    )?;
    let Rm = common::types::bits::from_bits_literal(
        &reader.extract_slice(16usize, 5usize)?,
    )?;
    let B = common::types::bits::from_bits_literal(
        &reader.extract_slice(11usize, 1usize)?,
    )?;
    let Rn = common::types::bits::from_bits_literal(
        &reader.extract_slice(5usize, 5usize)?,
    )?;
    let Rd = common::types::bits::from_bits_literal(
        &reader.extract_slice(0usize, 5usize)?,
    )?;
    let mut n: common::types::integer = decode::helpers::UInt_1(
        Rn,
        common::types::integer::from(5),
    )?;
    let mut d: common::types::integer = decode::helpers::UInt_1(
        Rd,
        common::types::integer::from(5),
    )?;
    Ok(
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_mat_mul_int_mla(
            Box::new(common::types::aarch64_vector_arithmetic_binary_uniform_mat_mul_int_mla_operands {
                d,
                n,
            }),
        ),
    )
}

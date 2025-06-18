#![allow(nonstandard_style, unused)]
use crate::arm64::AArch64LifterError;
use crate::arm64::{decode, common};
use crate::arm64::common::types::BigIntExt;
use crate::integer_to_usize;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive};
use std::collections::BTreeMap;
pub fn decode_aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd(
    reader: &mut decode::BitReader,
) -> Result<common::types::Instruction, AArch64LifterError> {
    let sz = common::types::bits::from_bits_literal(
        &reader.extract_slice(22usize, 1usize)?,
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
        common::types::Instruction::aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd(
            Box::new(common::types::aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd_operands {
                d,
                n,
            }),
        ),
    )
}

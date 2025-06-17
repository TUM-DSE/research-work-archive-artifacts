#![allow(nonstandard_style, unused)]
use crate::arm64::AArch64LifterError;
use crate::arm64::{decode, common};
use crate::arm64::common::types::BigIntExt;
use crate::integer_to_usize;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive};
use std::collections::BTreeMap;
pub fn decode_aarch64_integer_arithmetic_mul_widening_32_64(
    reader: &mut decode::BitReader,
) -> Result<common::types::Instruction, AArch64LifterError> {
    let U = common::types::bits::from_bits_literal(
        &reader.extract_slice(23usize, 1usize)?,
    )?;
    let Rm = common::types::bits::from_bits_literal(
        &reader.extract_slice(16usize, 5usize)?,
    )?;
    let o0 = common::types::bits::from_bits_literal(
        &reader.extract_slice(15usize, 1usize)?,
    )?;
    let Ra = common::types::bits::from_bits_literal(
        &reader.extract_slice(10usize, 5usize)?,
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
    let mut m: common::types::integer = decode::helpers::UInt_1(
        Rm,
        common::types::integer::from(5),
    )?;
    let mut a: common::types::integer = decode::helpers::UInt_1(
        Ra,
        common::types::integer::from(5),
    )?;
    let mut destsize: common::types::integer = common::types::integer::from(64);
    let mut datasize: common::types::integer = common::types::integer::from(32);
    let mut sub_op: common::types::boolean = (decode::helpers::eq_bits_0(
        o0,
        common::types::bits::from_bits_literal("1")?,
        common::types::integer::from(1),
    )?);
    let mut unsigned: common::types::boolean = (decode::helpers::eq_bits_0(
        U,
        common::types::bits::from_bits_literal("1")?,
        common::types::integer::from(1),
    )?);
    Ok(
        common::types::Instruction::aarch64_integer_arithmetic_mul_widening_32_64(
            Box::new(common::types::aarch64_integer_arithmetic_mul_widening_32_64_operands {
                a,
                d,
                datasize,
                destsize,
                m,
                n,
                sub_op,
                unsigned,
            }),
        ),
    )
}

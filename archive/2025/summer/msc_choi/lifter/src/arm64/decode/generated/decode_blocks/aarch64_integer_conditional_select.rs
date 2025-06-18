#![allow(nonstandard_style, unused)]
use crate::arm64::AArch64LifterError;
use crate::arm64::{decode, common};
use crate::arm64::common::types::BigIntExt;
use crate::integer_to_usize;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive};
use std::collections::BTreeMap;
pub fn decode_aarch64_integer_conditional_select(
    reader: &mut decode::BitReader,
) -> Result<common::types::Instruction, AArch64LifterError> {
    let sf = common::types::bits::from_bits_literal(
        &reader.extract_slice(31usize, 1usize)?,
    )?;
    let op = common::types::bits::from_bits_literal(
        &reader.extract_slice(30usize, 1usize)?,
    )?;
    let Rm = common::types::bits::from_bits_literal(
        &reader.extract_slice(16usize, 5usize)?,
    )?;
    let cond = common::types::bits::from_bits_literal(
        &reader.extract_slice(12usize, 4usize)?,
    )?;
    let o2 = common::types::bits::from_bits_literal(
        &reader.extract_slice(10usize, 1usize)?,
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
    let mut datasize: common::types::integer = if (decode::helpers::eq_bits_0(
        sf,
        common::types::bits::from_bits_literal("1")?,
        common::types::integer::from(1),
    )?) == common::types::boolean::TRUE
    {
        common::types::integer::from(64)
    } else {
        common::types::integer::from(32)
    };
    let mut condition: common::types::bits = cond;
    let mut else_inv: common::types::boolean = (decode::helpers::eq_bits_0(
        op,
        common::types::bits::from_bits_literal("1")?,
        common::types::integer::from(1),
    )?);
    let mut else_inc: common::types::boolean = (decode::helpers::eq_bits_0(
        o2,
        common::types::bits::from_bits_literal("1")?,
        common::types::integer::from(1),
    )?);
    Ok(
        common::types::Instruction::aarch64_integer_conditional_select(
            Box::new(common::types::aarch64_integer_conditional_select_operands {
                condition,
                d,
                datasize,
                else_inc,
                else_inv,
                m,
                n,
            }),
        ),
    )
}

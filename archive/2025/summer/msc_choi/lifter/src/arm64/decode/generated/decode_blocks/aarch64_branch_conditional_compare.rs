#![allow(nonstandard_style, unused)]
use crate::arm64::AArch64LifterError;
use crate::arm64::{decode, common};
use crate::arm64::common::types::BigIntExt;
use crate::integer_to_usize;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive};
use std::collections::BTreeMap;
pub fn decode_aarch64_branch_conditional_compare(
    reader: &mut decode::BitReader,
) -> Result<common::types::Instruction, AArch64LifterError> {
    let sf = common::types::bits::from_bits_literal(
        &reader.extract_slice(31usize, 1usize)?,
    )?;
    let op = common::types::bits::from_bits_literal(
        &reader.extract_slice(24usize, 1usize)?,
    )?;
    let imm19 = common::types::bits::from_bits_literal(
        &reader.extract_slice(5usize, 19usize)?,
    )?;
    let Rt = common::types::bits::from_bits_literal(
        &reader.extract_slice(0usize, 5usize)?,
    )?;
    let mut t: common::types::integer = decode::helpers::UInt_1(
        Rt,
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
    let mut iszero: common::types::boolean = (decode::helpers::eq_bits_0(
        op,
        common::types::bits::from_bits_literal("0")?,
        common::types::integer::from(1),
    )?);
    let mut offset: common::types::bits = decode::helpers::SignExtend_0(
        decode::helpers::append_bits_0(
            imm19,
            common::types::bits::from_bits_literal("00")?,
            common::types::integer::from(19),
            common::types::integer::from(2),
        )?,
        common::types::integer::from(64),
        decode::helpers::add_int_0(
            common::types::integer::from(19),
            common::types::integer::from(2),
        )?,
        common::types::integer::from(64),
    )?;
    Ok(
        common::types::Instruction::aarch64_branch_conditional_compare(
            Box::new(common::types::aarch64_branch_conditional_compare_operands {
                datasize,
                iszero,
                offset,
                t,
            }),
        ),
    )
}

#![allow(nonstandard_style, unused)]
use crate::arm64::AArch64LifterError;
use crate::arm64::{decode, common};
use crate::arm64::common::types::BigIntExt;
use crate::integer_to_usize;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive};
use std::collections::BTreeMap;
pub fn decode_aarch64_system_hints(
    reader: &mut decode::BitReader,
) -> Result<common::types::Instruction, AArch64LifterError> {
    let CRm = common::types::bits::from_bits_literal(
        &reader.extract_slice(8usize, 4usize)?,
    )?;
    let op2 = common::types::bits::from_bits_literal(
        &reader.extract_slice(5usize, 3usize)?,
    )?;
    if (decode::helpers::append_bits_0(
            CRm,
            op2,
            common::types::integer::from(4),
            common::types::integer::from(3),
        )?
        .match_with_pattern("0000 000")) && true
    {} else if (decode::helpers::append_bits_0(
            CRm,
            op2,
            common::types::integer::from(4),
            common::types::integer::from(3),
        )?
        .match_with_pattern("0000 001")) && true
    {} else if (decode::helpers::append_bits_0(
            CRm,
            op2,
            common::types::integer::from(4),
            common::types::integer::from(3),
        )?
        .match_with_pattern("0000 010")) && true
    {} else if (decode::helpers::append_bits_0(
            CRm,
            op2,
            common::types::integer::from(4),
            common::types::integer::from(3),
        )?
        .match_with_pattern("0000 011")) && true
    {} else if (decode::helpers::append_bits_0(
            CRm,
            op2,
            common::types::integer::from(4),
            common::types::integer::from(3),
        )?
        .match_with_pattern("0000 100")) && true
    {} else if (decode::helpers::append_bits_0(
            CRm,
            op2,
            common::types::integer::from(4),
            common::types::integer::from(3),
        )?
        .match_with_pattern("0000 101")) && true
    {} else if (decode::helpers::append_bits_0(
            CRm,
            op2,
            common::types::integer::from(4),
            common::types::integer::from(3),
        )?
        .match_with_pattern("0000 110")) && true
    {} else if (decode::helpers::append_bits_0(
            CRm,
            op2,
            common::types::integer::from(4),
            common::types::integer::from(3),
        )?
        .match_with_pattern("0000 111")) && true
    {
        return Ok(
            decode::generated::decode_blocks::decode_aarch64_integer_pac_strip_dp_1src(
                reader,
            )?,
        );
    } else if (decode::helpers::append_bits_0(
            CRm,
            op2,
            common::types::integer::from(4),
            common::types::integer::from(3),
        )?
        .match_with_pattern("0001 xxx")) && true
    {} else if (decode::helpers::append_bits_0(
            CRm,
            op2,
            common::types::integer::from(4),
            common::types::integer::from(3),
        )?
        .match_with_pattern("0010 000")) && true
    {} else if (decode::helpers::append_bits_0(
            CRm,
            op2,
            common::types::integer::from(4),
            common::types::integer::from(3),
        )?
        .match_with_pattern("0010 001")) && true
    {} else if (decode::helpers::append_bits_0(
            CRm,
            op2,
            common::types::integer::from(4),
            common::types::integer::from(3),
        )?
        .match_with_pattern("0010 010")) && true
    {} else if (decode::helpers::append_bits_0(
            CRm,
            op2,
            common::types::integer::from(4),
            common::types::integer::from(3),
        )?
        .match_with_pattern("0010 100")) && true
    {} else if (decode::helpers::append_bits_0(
            CRm,
            op2,
            common::types::integer::from(4),
            common::types::integer::from(3),
        )?
        .match_with_pattern("0011 xxx")) && true
    {} else if (decode::helpers::append_bits_0(
            CRm,
            op2,
            common::types::integer::from(4),
            common::types::integer::from(3),
        )?
        .match_with_pattern("0100 xx0")) && true
    {} else {}
    Ok(
        common::types::Instruction::aarch64_system_hints(
            Box::new(common::types::aarch64_system_hints_operands {
            }),
        ),
    )
}

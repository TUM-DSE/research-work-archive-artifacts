#![allow(nonstandard_style, unused)]
use crate::arm64::AArch64LifterError;
use crate::arm64::{decode, common};
use crate::arm64::common::types::BigIntExt;
use crate::integer_to_usize;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive};
use std::collections::BTreeMap;
pub fn decode_aarch64_integer_tags_mcsubtag(
    reader: &mut decode::BitReader,
) -> Result<common::types::Instruction, AArch64LifterError> {
    let uimm6 = common::types::bits::from_bits_literal(
        &reader.extract_slice(16usize, 6usize)?,
    )?;
    let op3 = common::types::bits::from_bits_literal(
        &reader.extract_slice(14usize, 2usize)?,
    )?;
    let uimm4 = common::types::bits::from_bits_literal(
        &reader.extract_slice(10usize, 4usize)?,
    )?;
    let Xn = common::types::bits::from_bits_literal(
        &reader.extract_slice(5usize, 5usize)?,
    )?;
    let Xd = common::types::bits::from_bits_literal(
        &reader.extract_slice(0usize, 5usize)?,
    )?;
    let mut d: common::types::integer = decode::helpers::UInt_1(
        Xd,
        common::types::integer::from(5),
    )?;
    let mut n: common::types::integer = decode::helpers::UInt_1(
        Xn,
        common::types::integer::from(5),
    )?;
    Ok(
        common::types::Instruction::aarch64_integer_tags_mcsubtag(
            Box::new(common::types::aarch64_integer_tags_mcsubtag_operands {
                d,
                n,
            }),
        ),
    )
}

#![allow(nonstandard_style, unused)]
use crate::arm64::AArch64LifterError;
use crate::arm64::{decode, common};
use crate::arm64::common::types::BigIntExt;
use crate::integer_to_usize;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive};
use std::collections::BTreeMap;
pub fn decode_AND_P_P_PP_Z(
    reader: &mut decode::BitReader,
) -> Result<common::types::Instruction, AArch64LifterError> {
    let S = common::types::bits::from_bits_literal(
        &reader.extract_slice(22usize, 1usize)?,
    )?;
    let Pm = common::types::bits::from_bits_literal(
        &reader.extract_slice(16usize, 4usize)?,
    )?;
    let Pg = common::types::bits::from_bits_literal(
        &reader.extract_slice(10usize, 4usize)?,
    )?;
    let Pn = common::types::bits::from_bits_literal(
        &reader.extract_slice(5usize, 4usize)?,
    )?;
    let Pd = common::types::bits::from_bits_literal(
        &reader.extract_slice(0usize, 4usize)?,
    )?;
    let mut n: common::types::integer = decode::helpers::UInt_1(
        Pn,
        common::types::integer::from(4),
    )?;
    let mut d: common::types::integer = decode::helpers::UInt_1(
        Pd,
        common::types::integer::from(4),
    )?;
    Ok(
        common::types::Instruction::AND_P_P_PP_Z(
            Box::new(common::types::AND_P_P_PP_Z_operands {
                d,
                n,
            }),
        ),
    )
}

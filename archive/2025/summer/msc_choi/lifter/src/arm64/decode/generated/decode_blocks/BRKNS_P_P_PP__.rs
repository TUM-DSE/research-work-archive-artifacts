#![allow(nonstandard_style, unused)]
use crate::arm64::AArch64LifterError;
use crate::arm64::{decode, common};
use crate::arm64::common::types::BigIntExt;
use crate::integer_to_usize;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive};
use std::collections::BTreeMap;
pub fn decode_BRKNS_P_P_PP__(
    reader: &mut decode::BitReader,
) -> Result<common::types::Instruction, AArch64LifterError> {
    let Pg = common::types::bits::from_bits_literal(
        &reader.extract_slice(10usize, 4usize)?,
    )?;
    let Pn = common::types::bits::from_bits_literal(
        &reader.extract_slice(5usize, 4usize)?,
    )?;
    let Pdm = common::types::bits::from_bits_literal(
        &reader.extract_slice(0usize, 4usize)?,
    )?;
    let mut n: common::types::integer = decode::helpers::UInt_1(
        Pn,
        common::types::integer::from(4),
    )?;
    Ok(
        common::types::Instruction::BRKN_P_P_PP__(
            Box::new(common::types::BRKN_P_P_PP___operands {
                n,
            }),
        ),
    )
}

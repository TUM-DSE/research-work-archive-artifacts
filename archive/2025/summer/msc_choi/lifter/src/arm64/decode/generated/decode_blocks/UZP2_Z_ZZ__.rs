#![allow(nonstandard_style, unused)]
use crate::arm64::AArch64LifterError;
use crate::arm64::{decode, common};
use crate::arm64::common::types::BigIntExt;
use crate::integer_to_usize;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive};
use std::collections::BTreeMap;
pub fn decode_UZP2_Z_ZZ__(
    reader: &mut decode::BitReader,
) -> Result<common::types::Instruction, AArch64LifterError> {
    let size = common::types::bits::from_bits_literal(
        &reader.extract_slice(22usize, 2usize)?,
    )?;
    let Zm = common::types::bits::from_bits_literal(
        &reader.extract_slice(16usize, 5usize)?,
    )?;
    let Zn = common::types::bits::from_bits_literal(
        &reader.extract_slice(5usize, 5usize)?,
    )?;
    let Zd = common::types::bits::from_bits_literal(
        &reader.extract_slice(0usize, 5usize)?,
    )?;
    let mut n: common::types::integer = decode::helpers::UInt_1(
        Zn,
        common::types::integer::from(5),
    )?;
    let mut d: common::types::integer = decode::helpers::UInt_1(
        Zd,
        common::types::integer::from(5),
    )?;
    Ok(
        common::types::Instruction::UZP1_Z_ZZ__(
            Box::new(common::types::UZP1_Z_ZZ___operands {
                d,
                n,
            }),
        ),
    )
}

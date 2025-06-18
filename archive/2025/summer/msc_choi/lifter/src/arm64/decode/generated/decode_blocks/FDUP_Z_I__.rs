#![allow(nonstandard_style, unused)]
use crate::arm64::AArch64LifterError;
use crate::arm64::{decode, common};
use crate::arm64::common::types::BigIntExt;
use crate::integer_to_usize;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive};
use std::collections::BTreeMap;
pub fn decode_FDUP_Z_I__(
    reader: &mut decode::BitReader,
) -> Result<common::types::Instruction, AArch64LifterError> {
    let size = common::types::bits::from_bits_literal(
        &reader.extract_slice(22usize, 2usize)?,
    )?;
    let imm8 = common::types::bits::from_bits_literal(
        &reader.extract_slice(5usize, 8usize)?,
    )?;
    let Zd = common::types::bits::from_bits_literal(
        &reader.extract_slice(0usize, 5usize)?,
    )?;
    let mut d: common::types::integer = decode::helpers::UInt_1(
        Zd,
        common::types::integer::from(5),
    )?;
    Ok(
        common::types::Instruction::FDUP_Z_I__(
            Box::new(common::types::FDUP_Z_I___operands {
                d,
            }),
        ),
    )
}

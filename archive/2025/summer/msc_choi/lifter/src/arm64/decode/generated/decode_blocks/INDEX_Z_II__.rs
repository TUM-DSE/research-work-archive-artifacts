#![allow(nonstandard_style, unused)]
use crate::arm64::AArch64LifterError;
use crate::arm64::{decode, common};
use crate::arm64::common::types::BigIntExt;
use crate::integer_to_usize;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive};
use std::collections::BTreeMap;
pub fn decode_INDEX_Z_II__(
    reader: &mut decode::BitReader,
) -> Result<common::types::Instruction, AArch64LifterError> {
    let size = common::types::bits::from_bits_literal(
        &reader.extract_slice(22usize, 2usize)?,
    )?;
    let imm5b = common::types::bits::from_bits_literal(
        &reader.extract_slice(16usize, 5usize)?,
    )?;
    let imm5 = common::types::bits::from_bits_literal(
        &reader.extract_slice(5usize, 5usize)?,
    )?;
    let Zd = common::types::bits::from_bits_literal(
        &reader.extract_slice(0usize, 5usize)?,
    )?;
    let mut d: common::types::integer = decode::helpers::UInt_1(
        Zd,
        common::types::integer::from(5),
    )?;
    Ok(
        common::types::Instruction::INDEX_Z_II__(
            Box::new(common::types::INDEX_Z_II___operands {
                d,
            }),
        ),
    )
}

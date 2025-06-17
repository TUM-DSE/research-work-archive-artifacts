#![allow(nonstandard_style, unused)]
use crate::arm64::AArch64LifterError;
use crate::arm64::{decode, common};
use crate::arm64::common::types::BigIntExt;
use crate::integer_to_usize;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive};
use std::collections::BTreeMap;
pub fn decode_aarch64_system_register_system(
    reader: &mut decode::BitReader,
) -> Result<common::types::Instruction, AArch64LifterError> {
    let L = common::types::bits::from_bits_literal(
        &reader.extract_slice(21usize, 1usize)?,
    )?;
    let o0 = common::types::bits::from_bits_literal(
        &reader.extract_slice(19usize, 1usize)?,
    )?;
    let op1 = common::types::bits::from_bits_literal(
        &reader.extract_slice(16usize, 3usize)?,
    )?;
    let CRn = common::types::bits::from_bits_literal(
        &reader.extract_slice(12usize, 4usize)?,
    )?;
    let CRm = common::types::bits::from_bits_literal(
        &reader.extract_slice(8usize, 4usize)?,
    )?;
    let op2 = common::types::bits::from_bits_literal(
        &reader.extract_slice(5usize, 3usize)?,
    )?;
    let Rt = common::types::bits::from_bits_literal(
        &reader.extract_slice(0usize, 5usize)?,
    )?;
    let mut t: common::types::integer = decode::helpers::UInt_1(
        Rt,
        common::types::integer::from(5),
    )?;
    Ok(
        common::types::Instruction::aarch64_system_register_system(
            Box::new(common::types::aarch64_system_register_system_operands {
                t,
            }),
        ),
    )
}

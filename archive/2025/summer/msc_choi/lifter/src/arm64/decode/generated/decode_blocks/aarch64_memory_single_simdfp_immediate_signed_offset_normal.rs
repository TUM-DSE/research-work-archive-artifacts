#![allow(nonstandard_style, unused)]
use crate::arm64::AArch64LifterError;
use crate::arm64::{decode, common};
use crate::arm64::common::types::BigIntExt;
use crate::integer_to_usize;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive};
use std::collections::BTreeMap;
pub fn decode_aarch64_memory_single_simdfp_immediate_signed_offset_normal(
    reader: &mut decode::BitReader,
) -> Result<common::types::Instruction, AArch64LifterError> {
    let size = common::types::bits::from_bits_literal(
        &reader.extract_slice(30usize, 2usize)?,
    )?;
    let opc = common::types::bits::from_bits_literal(
        &reader.extract_slice(22usize, 2usize)?,
    )?;
    let imm9 = common::types::bits::from_bits_literal(
        &reader.extract_slice(12usize, 9usize)?,
    )?;
    let Rn = common::types::bits::from_bits_literal(
        &reader.extract_slice(5usize, 5usize)?,
    )?;
    let Rt = common::types::bits::from_bits_literal(
        &reader.extract_slice(0usize, 5usize)?,
    )?;
    let mut n: common::types::integer = decode::helpers::UInt_1(
        Rn,
        common::types::integer::from(5),
    )?;
    let mut t: common::types::integer = decode::helpers::UInt_1(
        Rt,
        common::types::integer::from(5),
    )?;
    Ok(
        common::types::Instruction::aarch64_memory_single_simdfp_immediate_signed_offset_normal(
            Box::new(common::types::aarch64_memory_single_simdfp_immediate_signed_offset_normal_operands {
                n,
                t,
            }),
        ),
    )
}

#![allow(nonstandard_style, unused)]
use crate::arm64::AArch64LifterError;
use crate::arm64::{decode, common};
use crate::arm64::common::types::BigIntExt;
use crate::integer_to_usize;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive};
use std::collections::BTreeMap;
pub fn decode_aarch64_system_monitors(
    reader: &mut decode::BitReader,
) -> Result<common::types::Instruction, AArch64LifterError> {
    let CRm = common::types::bits::from_bits_literal(
        &reader.extract_slice(8usize, 4usize)?,
    )?;
    Ok(
        common::types::Instruction::aarch64_system_monitors(
            Box::new(common::types::aarch64_system_monitors_operands {
            }),
        ),
    )
}

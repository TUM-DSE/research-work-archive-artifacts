#![allow(nonstandard_style, unused)]
use crate::arm64::AArch64LifterError;
use crate::arm64::{decode, common};
use crate::arm64::common::types::BigIntExt;
use crate::integer_to_usize;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive};
use std::collections::BTreeMap;
pub fn decode_aarch64_system_register_cpsr(
    reader: &mut decode::BitReader,
) -> Result<common::types::Instruction, AArch64LifterError> {
    let op1 = common::types::bits::from_bits_literal(
        &reader.extract_slice(16usize, 3usize)?,
    )?;
    let CRm = common::types::bits::from_bits_literal(
        &reader.extract_slice(8usize, 4usize)?,
    )?;
    let op2 = common::types::bits::from_bits_literal(
        &reader.extract_slice(5usize, 3usize)?,
    )?;
    Ok(
        common::types::Instruction::aarch64_system_register_cpsr(
            Box::new(common::types::aarch64_system_register_cpsr_operands {
            }),
        ),
    )
}

#![allow(nonstandard_style, unused)]
use crate::arm64::AArch64LifterError;
use crate::arm64::{decode, common};
use crate::arm64::common::types::BigIntExt;
use crate::integer_to_usize;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive};
use std::collections::BTreeMap;
pub fn decode_aarch64_udf(
    reader: &mut decode::BitReader,
) -> Result<common::types::Instruction, AArch64LifterError> {
    let imm16 = common::types::bits::from_bits_literal(
        &reader.extract_slice(0usize, 16usize)?,
    )?;
    Ok(
        common::types::Instruction::aarch64_udf(
            Box::new(common::types::aarch64_udf_operands {
            }),
        ),
    )
}

#![allow(nonstandard_style, unused)]
use crate::arm64::AArch64LifterError;
use crate::arm64::{decode, common};
use crate::arm64::common::types::BigIntExt;
use crate::integer_to_usize;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive};
use std::collections::BTreeMap;
pub fn decode_aarch64_branch_conditional_cond(
    reader: &mut decode::BitReader,
) -> Result<common::types::Instruction, AArch64LifterError> {
    let imm19 = common::types::bits::from_bits_literal(
        &reader.extract_slice(5usize, 19usize)?,
    )?;
    let cond = common::types::bits::from_bits_literal(
        &reader.extract_slice(0usize, 4usize)?,
    )?;
    let mut offset: common::types::bits = decode::helpers::SignExtend_0(
        decode::helpers::append_bits_0(
            imm19,
            common::types::bits::from_bits_literal("00")?,
            common::types::integer::from(19),
            common::types::integer::from(2),
        )?,
        common::types::integer::from(64),
        decode::helpers::add_int_0(
            common::types::integer::from(19),
            common::types::integer::from(2),
        )?,
        common::types::integer::from(64),
    )?;
    let mut condition: common::types::bits = cond;
    Ok(
        common::types::Instruction::aarch64_branch_conditional_cond(
            Box::new(common::types::aarch64_branch_conditional_cond_operands {
                condition,
                offset,
            }),
        ),
    )
}

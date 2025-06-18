#![allow(nonstandard_style, unused)]
use crate::arm64::AArch64LifterError;
use crate::arm64::{decode, common};
use crate::arm64::common::types::BigIntExt;
use crate::integer_to_usize;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive};
use std::collections::BTreeMap;
pub fn decode_aarch64_branch_unconditional_immediate(
    reader: &mut decode::BitReader,
) -> Result<common::types::Instruction, AArch64LifterError> {
    let op = common::types::bits::from_bits_literal(
        &reader.extract_slice(31usize, 1usize)?,
    )?;
    let imm26 = common::types::bits::from_bits_literal(
        &reader.extract_slice(0usize, 26usize)?,
    )?;
    let mut branch_type: common::types::BranchType = if (decode::helpers::eq_bits_0(
        op,
        common::types::bits::from_bits_literal("1")?,
        common::types::integer::from(1),
    )?) == common::types::boolean::TRUE
    {
        common::types::BranchType::BranchType_DIRCALL
    } else {
        common::types::BranchType::BranchType_DIR
    };
    let mut offset: common::types::bits = decode::helpers::SignExtend_0(
        decode::helpers::append_bits_0(
            imm26,
            common::types::bits::from_bits_literal("00")?,
            common::types::integer::from(26),
            common::types::integer::from(2),
        )?,
        common::types::integer::from(64),
        decode::helpers::add_int_0(
            common::types::integer::from(26),
            common::types::integer::from(2),
        )?,
        common::types::integer::from(64),
    )?;
    Ok(
        common::types::Instruction::aarch64_branch_unconditional_immediate(
            Box::new(common::types::aarch64_branch_unconditional_immediate_operands {
                branch_type,
                offset,
            }),
        ),
    )
}

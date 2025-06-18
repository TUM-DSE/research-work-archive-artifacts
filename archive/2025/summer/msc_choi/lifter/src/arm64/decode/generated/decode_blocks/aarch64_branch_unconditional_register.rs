#![allow(nonstandard_style, unused)]
use crate::arm64::AArch64LifterError;
use crate::arm64::{decode, common};
use crate::arm64::common::types::BigIntExt;
use crate::integer_to_usize;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive};
use std::collections::BTreeMap;
pub fn decode_aarch64_branch_unconditional_register(
    reader: &mut decode::BitReader,
) -> Result<common::types::Instruction, AArch64LifterError> {
    let Z = common::types::bits::from_bits_literal(
        &reader.extract_slice(24usize, 1usize)?,
    )?;
    let op = common::types::bits::from_bits_literal(
        &reader.extract_slice(21usize, 2usize)?,
    )?;
    let A = common::types::bits::from_bits_literal(
        &reader.extract_slice(11usize, 1usize)?,
    )?;
    let M = common::types::bits::from_bits_literal(
        &reader.extract_slice(10usize, 1usize)?,
    )?;
    let Rn = common::types::bits::from_bits_literal(
        &reader.extract_slice(5usize, 5usize)?,
    )?;
    let Rm = common::types::bits::from_bits_literal(
        &reader.extract_slice(0usize, 5usize)?,
    )?;
    let mut n: common::types::integer = decode::helpers::UInt_1(
        Rn,
        common::types::integer::from(5),
    )?;
    let mut branch_type: common::types::BranchType = common::types::BranchType::default();
    let mut m: common::types::integer = decode::helpers::UInt_1(
        Rm,
        common::types::integer::from(5),
    )?;
    let mut pac: common::types::boolean = (decode::helpers::eq_bits_0(
        A,
        common::types::bits::from_bits_literal("1")?,
        common::types::integer::from(1),
    )?);
    let mut use_key_a: common::types::boolean = (decode::helpers::eq_bits_0(
        M,
        common::types::bits::from_bits_literal("0")?,
        common::types::integer::from(1),
    )?);
    let mut source_is_sp: common::types::boolean = (decode::helpers::and_bool_0(
        (decode::helpers::eq_bits_0(
            Z,
            common::types::bits::from_bits_literal("1")?,
            common::types::integer::from(1),
        )?),
        (decode::helpers::eq_int_0(m.clone(), common::types::integer::from(31))?),
    )?);
    if (decode::helpers::and_bool_0(
        decode::helpers::not_bool_0(pac)?,
        decode::helpers::ne_int_0(m.clone(), common::types::integer::from(0))?,
    )?) == common::types::boolean::TRUE
    {
        return Err(AArch64LifterError::UndefinedInstruction);
    } else if (decode::helpers::and_bool_0(
        pac,
        decode::helpers::not_bool_0(decode::helpers::HavePACExt_0()?)?,
    )?) == common::types::boolean::TRUE
    {
        return Err(AArch64LifterError::UndefinedInstruction);
    } else {}
    if (op.match_with_pattern("00")) && true {
        branch_type = common::types::BranchType::BranchType_INDIR;
    } else if (op.match_with_pattern("01")) && true {
        branch_type = common::types::BranchType::BranchType_INDCALL;
    } else if (op.match_with_pattern("10")) && true {
        branch_type = common::types::BranchType::BranchType_RET;
    } else {
        return Err(AArch64LifterError::UndefinedInstruction);
    }
    if (pac) == common::types::boolean::TRUE {
        if (decode::helpers::and_bool_0(
            decode::helpers::eq_bits_0(
                Z,
                common::types::bits::from_bits_literal("0")?,
                common::types::integer::from(1),
            )?,
            decode::helpers::ne_int_0(m.clone(), common::types::integer::from(31))?,
        )?) == common::types::boolean::TRUE
        {
            return Err(AArch64LifterError::UndefinedInstruction);
        } else {}
        if (decode::helpers::eq_enum_16(
            branch_type,
            common::types::BranchType::BranchType_RET,
        )?) == common::types::boolean::TRUE
        {
            if (decode::helpers::ne_int_0(n.clone(), common::types::integer::from(31))?)
                == common::types::boolean::TRUE
            {
                return Err(AArch64LifterError::UndefinedInstruction);
            } else {}
            n = common::types::integer::from(30);
            source_is_sp = common::types::boolean::TRUE;
        } else {}
    } else {}
    Ok(
        common::types::Instruction::aarch64_branch_unconditional_register(
            Box::new(common::types::aarch64_branch_unconditional_register_operands {
                branch_type,
                m,
                n,
                pac,
                source_is_sp,
                use_key_a,
            }),
        ),
    )
}

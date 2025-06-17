#![allow(nonstandard_style, unused)]
use crate::arm64::AArch64LifterError;
use crate::arm64::{decode, common};
use crate::arm64::common::types::BigIntExt;
use crate::integer_to_usize;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive};
use std::collections::BTreeMap;
pub fn decode_aarch64_integer_arithmetic_add_sub_shiftedreg(
    reader: &mut decode::BitReader,
) -> Result<common::types::Instruction, AArch64LifterError> {
    let sf = common::types::bits::from_bits_literal(
        &reader.extract_slice(31usize, 1usize)?,
    )?;
    let op = common::types::bits::from_bits_literal(
        &reader.extract_slice(30usize, 1usize)?,
    )?;
    let S = common::types::bits::from_bits_literal(
        &reader.extract_slice(29usize, 1usize)?,
    )?;
    let shift = common::types::bits::from_bits_literal(
        &reader.extract_slice(22usize, 2usize)?,
    )?;
    let Rm = common::types::bits::from_bits_literal(
        &reader.extract_slice(16usize, 5usize)?,
    )?;
    let imm6 = common::types::bits::from_bits_literal(
        &reader.extract_slice(10usize, 6usize)?,
    )?;
    let Rn = common::types::bits::from_bits_literal(
        &reader.extract_slice(5usize, 5usize)?,
    )?;
    let Rd = common::types::bits::from_bits_literal(
        &reader.extract_slice(0usize, 5usize)?,
    )?;
    let mut d: common::types::integer = decode::helpers::UInt_1(
        Rd,
        common::types::integer::from(5),
    )?;
    let mut n: common::types::integer = decode::helpers::UInt_1(
        Rn,
        common::types::integer::from(5),
    )?;
    let mut m: common::types::integer = decode::helpers::UInt_1(
        Rm,
        common::types::integer::from(5),
    )?;
    let mut datasize: common::types::integer = if (decode::helpers::eq_bits_0(
        sf,
        common::types::bits::from_bits_literal("1")?,
        common::types::integer::from(1),
    )?) == common::types::boolean::TRUE
    {
        common::types::integer::from(64)
    } else {
        common::types::integer::from(32)
    };
    let mut sub_op: common::types::boolean = (decode::helpers::eq_bits_0(
        op,
        common::types::bits::from_bits_literal("1")?,
        common::types::integer::from(1),
    )?);
    let mut setflags: common::types::boolean = (decode::helpers::eq_bits_0(
        S,
        common::types::bits::from_bits_literal("1")?,
        common::types::integer::from(1),
    )?);
    if (decode::helpers::eq_bits_0(
        shift,
        common::types::bits::from_bits_literal("11")?,
        common::types::integer::from(2),
    )?) == common::types::boolean::TRUE
    {
        return Err(AArch64LifterError::UndefinedInstruction);
    } else {}
    if (decode::helpers::and_bool_0(
        decode::helpers::eq_bits_0(
            sf,
            common::types::bits::from_bits_literal("0")?,
            common::types::integer::from(1),
        )?,
        decode::helpers::eq_bits_0(
            imm6.extract_slice(integer_to_usize!(common::types::integer::from(5)), 1)?,
            common::types::bits::from_bits_literal("1")?,
            decode::helpers::add_int_0(
                common::types::integer::from(0),
                common::types::integer::from(1),
            )?,
        )?,
    )?) == common::types::boolean::TRUE
    {
        return Err(AArch64LifterError::UndefinedInstruction);
    } else {}
    let mut shift_type: common::types::ShiftType = decode::helpers::DecodeShift_0(
        shift,
    )?;
    let mut shift_amount: common::types::integer = decode::helpers::UInt_1(
        imm6,
        common::types::integer::from(6),
    )?;
    Ok(
        common::types::Instruction::aarch64_integer_arithmetic_add_sub_shiftedreg(
            Box::new(common::types::aarch64_integer_arithmetic_add_sub_shiftedreg_operands {
                d,
                datasize,
                m,
                n,
                setflags,
                shift_amount,
                shift_type,
                sub_op,
            }),
        ),
    )
}

#![allow(nonstandard_style, unused)]
use crate::arm64::AArch64LifterError;
use crate::arm64::{decode, common};
use crate::arm64::common::types::BigIntExt;
use crate::integer_to_usize;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive};
use std::collections::BTreeMap;
pub fn decode_aarch64_integer_ins_ext_insert_movewide(
    reader: &mut decode::BitReader,
) -> Result<common::types::Instruction, AArch64LifterError> {
    let sf = common::types::bits::from_bits_literal(
        &reader.extract_slice(31usize, 1usize)?,
    )?;
    let opc = common::types::bits::from_bits_literal(
        &reader.extract_slice(29usize, 2usize)?,
    )?;
    let hw = common::types::bits::from_bits_literal(
        &reader.extract_slice(21usize, 2usize)?,
    )?;
    let imm16 = common::types::bits::from_bits_literal(
        &reader.extract_slice(5usize, 16usize)?,
    )?;
    let Rd = common::types::bits::from_bits_literal(
        &reader.extract_slice(0usize, 5usize)?,
    )?;
    let mut d: common::types::integer = decode::helpers::UInt_1(
        Rd,
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
    let mut imm: common::types::bits = imm16;
    let mut pos: common::types::integer = common::types::integer::default();
    let mut opcode: common::types::MoveWideOp = common::types::MoveWideOp::default();
    if (opc.match_with_pattern("00")) && true {
        opcode = common::types::MoveWideOp::MoveWideOp_N;
    } else if (opc.match_with_pattern("10")) && true {
        opcode = common::types::MoveWideOp::MoveWideOp_Z;
    } else if (opc.match_with_pattern("11")) && true {
        opcode = common::types::MoveWideOp::MoveWideOp_K;
    } else {
        return Err(AArch64LifterError::UndefinedInstruction);
    }
    if (decode::helpers::and_bool_0(
        decode::helpers::eq_bits_0(
            sf,
            common::types::bits::from_bits_literal("0")?,
            common::types::integer::from(1),
        )?,
        decode::helpers::eq_bits_0(
            hw.extract_slice(integer_to_usize!(common::types::integer::from(1)), 1)?,
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
    pos = decode::helpers::UInt_1(
        decode::helpers::append_bits_0(
            hw,
            common::types::bits::from_bits_literal("0000")?,
            common::types::integer::from(2),
            common::types::integer::from(4),
        )?,
        decode::helpers::add_int_0(
            common::types::integer::from(2),
            common::types::integer::from(4),
        )?,
    )?;
    Ok(
        common::types::Instruction::aarch64_integer_ins_ext_insert_movewide(
            Box::new(common::types::aarch64_integer_ins_ext_insert_movewide_operands {
                d,
                datasize,
                imm,
                opcode,
                pos,
            }),
        ),
    )
}

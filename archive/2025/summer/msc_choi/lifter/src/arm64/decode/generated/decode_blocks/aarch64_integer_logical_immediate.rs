#![allow(nonstandard_style, unused)]
use crate::arm64::AArch64LifterError;
use crate::arm64::{decode, common};
use crate::arm64::common::types::BigIntExt;
use crate::integer_to_usize;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive};
use std::collections::BTreeMap;
pub fn decode_aarch64_integer_logical_immediate(
    reader: &mut decode::BitReader,
) -> Result<common::types::Instruction, AArch64LifterError> {
    let sf = common::types::bits::from_bits_literal(
        &reader.extract_slice(31usize, 1usize)?,
    )?;
    let opc = common::types::bits::from_bits_literal(
        &reader.extract_slice(29usize, 2usize)?,
    )?;
    let N = common::types::bits::from_bits_literal(
        &reader.extract_slice(22usize, 1usize)?,
    )?;
    let immr = common::types::bits::from_bits_literal(
        &reader.extract_slice(16usize, 6usize)?,
    )?;
    let imms = common::types::bits::from_bits_literal(
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
    let mut setflags: common::types::boolean = common::types::boolean::default();
    let mut op: common::types::LogicalOp = common::types::LogicalOp::default();
    if (opc.match_with_pattern("00")) && true {
        op = common::types::LogicalOp::LogicalOp_AND;
        setflags = common::types::boolean::FALSE;
    } else if (opc.match_with_pattern("01")) && true {
        op = common::types::LogicalOp::LogicalOp_ORR;
        setflags = common::types::boolean::FALSE;
    } else if (opc.match_with_pattern("10")) && true {
        op = common::types::LogicalOp::LogicalOp_EOR;
        setflags = common::types::boolean::FALSE;
    } else if (opc.match_with_pattern("11")) && true {
        op = common::types::LogicalOp::LogicalOp_AND;
        setflags = common::types::boolean::TRUE;
    }
    let mut imm: common::types::bits = common::types::bits::new(
        0,
        integer_to_usize!(datasize),
    );
    if (decode::helpers::and_bool_0(
        decode::helpers::eq_bits_0(
            sf,
            common::types::bits::from_bits_literal("0")?,
            common::types::integer::from(1),
        )?,
        decode::helpers::ne_bits_0(
            N,
            common::types::bits::from_bits_literal("0")?,
            common::types::integer::from(1),
        )?,
    )?) == common::types::boolean::TRUE
    {
        return Err(AArch64LifterError::UndefinedInstruction);
    } else {}
    (imm, _) = decode::helpers::DecodeBitMasks_0(
        N,
        imms,
        immr,
        common::types::boolean::TRUE,
        datasize.clone(),
    )?;
    Ok(
        common::types::Instruction::aarch64_integer_logical_immediate(
            Box::new(common::types::aarch64_integer_logical_immediate_operands {
                d,
                datasize,
                imm,
                n,
                op,
                setflags,
            }),
        ),
    )
}

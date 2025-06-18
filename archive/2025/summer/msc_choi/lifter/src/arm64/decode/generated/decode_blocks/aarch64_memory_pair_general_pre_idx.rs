#![allow(nonstandard_style, unused)]
use crate::arm64::AArch64LifterError;
use crate::arm64::{decode, common};
use crate::arm64::common::types::BigIntExt;
use crate::integer_to_usize;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive};
use std::collections::BTreeMap;
pub fn decode_aarch64_memory_pair_general_pre_idx(
    reader: &mut decode::BitReader,
) -> Result<common::types::Instruction, AArch64LifterError> {
    let opc = common::types::bits::from_bits_literal(
        &reader.extract_slice(30usize, 2usize)?,
    )?;
    let L = common::types::bits::from_bits_literal(
        &reader.extract_slice(22usize, 1usize)?,
    )?;
    let imm7 = common::types::bits::from_bits_literal(
        &reader.extract_slice(15usize, 7usize)?,
    )?;
    let Rt2 = common::types::bits::from_bits_literal(
        &reader.extract_slice(10usize, 5usize)?,
    )?;
    let Rn = common::types::bits::from_bits_literal(
        &reader.extract_slice(5usize, 5usize)?,
    )?;
    let Rt = common::types::bits::from_bits_literal(
        &reader.extract_slice(0usize, 5usize)?,
    )?;
    let mut wback: common::types::boolean = common::types::boolean::TRUE;
    let mut postindex: common::types::boolean = common::types::boolean::FALSE;
    let mut n: common::types::integer = decode::helpers::UInt_1(
        Rn,
        common::types::integer::from(5),
    )?;
    let mut t: common::types::integer = decode::helpers::UInt_1(
        Rt,
        common::types::integer::from(5),
    )?;
    let mut t2: common::types::integer = decode::helpers::UInt_1(
        Rt2,
        common::types::integer::from(5),
    )?;
    let mut acctype: common::types::AccType = common::types::AccType::AccType_NORMAL;
    let mut memop: common::types::MemOp = if (decode::helpers::eq_bits_0(
        L,
        common::types::bits::from_bits_literal("1")?,
        common::types::integer::from(1),
    )?) == common::types::boolean::TRUE
    {
        common::types::MemOp::MemOp_LOAD
    } else {
        common::types::MemOp::MemOp_STORE
    };
    if (decode::helpers::or_bool_0(
        decode::helpers::eq_bits_0(
            decode::helpers::append_bits_0(
                L,
                opc
                    .extract_slice(
                        integer_to_usize!(common::types::integer::from(0)),
                        1,
                    )?,
                common::types::integer::from(1),
                decode::helpers::add_int_0(
                    common::types::integer::from(0),
                    common::types::integer::from(1),
                )?,
            )?,
            common::types::bits::from_bits_literal("01")?,
            decode::helpers::add_int_0(
                common::types::integer::from(1),
                decode::helpers::add_int_0(
                    common::types::integer::from(0),
                    common::types::integer::from(1),
                )?,
            )?,
        )?,
        decode::helpers::eq_bits_0(
            opc,
            common::types::bits::from_bits_literal("11")?,
            common::types::integer::from(2),
        )?,
    )?) == common::types::boolean::TRUE
    {
        return Err(AArch64LifterError::UndefinedInstruction);
    } else {}
    let mut signed: common::types::boolean = (decode::helpers::ne_bits_0(
        opc.extract_slice(integer_to_usize!(common::types::integer::from(0)), 1)?,
        common::types::bits::from_bits_literal("0")?,
        decode::helpers::add_int_0(
            common::types::integer::from(0),
            common::types::integer::from(1),
        )?,
    )?);
    let mut scale: common::types::integer = decode::helpers::add_int_0(
        common::types::integer::from(2),
        decode::helpers::UInt_1(
            opc.extract_slice(integer_to_usize!(common::types::integer::from(1)), 1)?,
            decode::helpers::add_int_0(
                common::types::integer::from(0),
                common::types::integer::from(1),
            )?,
        )?,
    )?;
    let mut datasize: common::types::integer = decode::helpers::shift_left_int_0(
        common::types::integer::from(8),
        scale.clone(),
    )?;
    let mut offset: common::types::bits = decode::helpers::LSL_0(
        decode::helpers::SignExtend_0(
            imm7,
            common::types::integer::from(64),
            common::types::integer::from(7),
            common::types::integer::from(64),
        )?,
        scale.clone(),
        common::types::integer::from(64),
    )?;
    let mut tag_checked: common::types::boolean = decode::helpers::or_bool_0(
        wback,
        decode::helpers::ne_int_0(n.clone(), common::types::integer::from(31))?,
    )?;
    Ok(
        common::types::Instruction::aarch64_memory_pair_general_post_idx(
            Box::new(common::types::aarch64_memory_pair_general_post_idx_operands {
                acctype,
                datasize,
                memop,
                n,
                offset,
                postindex,
                scale,
                signed,
                t,
                t2,
                tag_checked,
                wback,
            }),
        ),
    )
}

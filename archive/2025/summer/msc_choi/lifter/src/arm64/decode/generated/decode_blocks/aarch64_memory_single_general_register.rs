#![allow(nonstandard_style, unused)]
use crate::arm64::AArch64LifterError;
use crate::arm64::{decode, common};
use crate::arm64::common::types::BigIntExt;
use crate::integer_to_usize;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive};
use std::collections::BTreeMap;
pub fn decode_aarch64_memory_single_general_register(
    reader: &mut decode::BitReader,
) -> Result<common::types::Instruction, AArch64LifterError> {
    let size = common::types::bits::from_bits_literal(
        &reader.extract_slice(30usize, 2usize)?,
    )?;
    let opc = common::types::bits::from_bits_literal(
        &reader.extract_slice(22usize, 2usize)?,
    )?;
    let Rm = common::types::bits::from_bits_literal(
        &reader.extract_slice(16usize, 5usize)?,
    )?;
    let option = common::types::bits::from_bits_literal(
        &reader.extract_slice(13usize, 3usize)?,
    )?;
    let S = common::types::bits::from_bits_literal(
        &reader.extract_slice(12usize, 1usize)?,
    )?;
    let Rn = common::types::bits::from_bits_literal(
        &reader.extract_slice(5usize, 5usize)?,
    )?;
    let Rt = common::types::bits::from_bits_literal(
        &reader.extract_slice(0usize, 5usize)?,
    )?;
    let mut wback: common::types::boolean = common::types::boolean::FALSE;
    let mut postindex: common::types::boolean = common::types::boolean::FALSE;
    let mut scale: common::types::integer = decode::helpers::UInt_1(
        size,
        common::types::integer::from(2),
    )?;
    if (decode::helpers::eq_bits_0(
        option.extract_slice(integer_to_usize!(common::types::integer::from(1)), 1)?,
        common::types::bits::from_bits_literal("0")?,
        decode::helpers::add_int_0(
            common::types::integer::from(0),
            common::types::integer::from(1),
        )?,
    )?) == common::types::boolean::TRUE
    {
        return Err(AArch64LifterError::UndefinedInstruction);
    } else {}
    let mut extend_type: common::types::ExtendType = decode::helpers::DecodeRegExtend_0(
        option,
    )?;
    let mut shift: common::types::integer = if (decode::helpers::eq_bits_0(
        S,
        common::types::bits::from_bits_literal("1")?,
        common::types::integer::from(1),
    )?) == common::types::boolean::TRUE
    {
        scale.clone()
    } else {
        common::types::integer::from(0)
    };
    let mut n: common::types::integer = decode::helpers::UInt_1(
        Rn,
        common::types::integer::from(5),
    )?;
    let mut t: common::types::integer = decode::helpers::UInt_1(
        Rt,
        common::types::integer::from(5),
    )?;
    let mut m: common::types::integer = decode::helpers::UInt_1(
        Rm,
        common::types::integer::from(5),
    )?;
    let mut acctype: common::types::AccType = common::types::AccType::AccType_NORMAL;
    let mut memop: common::types::MemOp = common::types::MemOp::default();
    let mut signed: common::types::boolean = common::types::boolean::default();
    let mut regsize: common::types::integer = common::types::integer::default();
    if (decode::helpers::eq_bits_0(
        opc.extract_slice(integer_to_usize!(common::types::integer::from(1)), 1)?,
        common::types::bits::from_bits_literal("0")?,
        decode::helpers::add_int_0(
            common::types::integer::from(0),
            common::types::integer::from(1),
        )?,
    )?) == common::types::boolean::TRUE
    {
        memop = if (decode::helpers::eq_bits_0(
            opc.extract_slice(integer_to_usize!(common::types::integer::from(0)), 1)?,
            common::types::bits::from_bits_literal("1")?,
            decode::helpers::add_int_0(
                common::types::integer::from(0),
                common::types::integer::from(1),
            )?,
        )?) == common::types::boolean::TRUE
        {
            common::types::MemOp::MemOp_LOAD
        } else {
            common::types::MemOp::MemOp_STORE
        };
        regsize = if (decode::helpers::eq_bits_0(
            size,
            common::types::bits::from_bits_literal("11")?,
            common::types::integer::from(2),
        )?) == common::types::boolean::TRUE
        {
            common::types::integer::from(64)
        } else {
            common::types::integer::from(32)
        };
        signed = common::types::boolean::FALSE;
    } else {
        if (decode::helpers::eq_bits_0(
            size,
            common::types::bits::from_bits_literal("11")?,
            common::types::integer::from(2),
        )?) == common::types::boolean::TRUE
        {
            memop = common::types::MemOp::MemOp_PREFETCH;
            if (decode::helpers::eq_bits_0(
                opc
                    .extract_slice(
                        integer_to_usize!(common::types::integer::from(0)),
                        1,
                    )?,
                common::types::bits::from_bits_literal("1")?,
                decode::helpers::add_int_0(
                    common::types::integer::from(0),
                    common::types::integer::from(1),
                )?,
            )?) == common::types::boolean::TRUE
            {
                return Err(AArch64LifterError::UndefinedInstruction);
            } else {}
        } else {
            memop = common::types::MemOp::MemOp_LOAD;
            if (decode::helpers::and_bool_0(
                decode::helpers::eq_bits_0(
                    size,
                    common::types::bits::from_bits_literal("10")?,
                    common::types::integer::from(2),
                )?,
                decode::helpers::eq_bits_0(
                    opc
                        .extract_slice(
                            integer_to_usize!(common::types::integer::from(0)),
                            1,
                        )?,
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
            regsize = if (decode::helpers::eq_bits_0(
                opc
                    .extract_slice(
                        integer_to_usize!(common::types::integer::from(0)),
                        1,
                    )?,
                common::types::bits::from_bits_literal("1")?,
                decode::helpers::add_int_0(
                    common::types::integer::from(0),
                    common::types::integer::from(1),
                )?,
            )?) == common::types::boolean::TRUE
            {
                common::types::integer::from(32)
            } else {
                common::types::integer::from(64)
            };
            signed = common::types::boolean::TRUE;
        }
    }
    let mut datasize: common::types::integer = decode::helpers::shift_left_int_0(
        common::types::integer::from(8),
        scale.clone(),
    )?;
    let mut tag_checked: common::types::boolean = decode::helpers::ne_enum_37(
        memop,
        common::types::MemOp::MemOp_PREFETCH,
    )?;
    Ok(
        common::types::Instruction::aarch64_memory_single_general_register(
            Box::new(common::types::aarch64_memory_single_general_register_operands {
                acctype,
                datasize,
                extend_type,
                m,
                memop,
                n,
                postindex,
                regsize,
                scale,
                shift,
                signed,
                t,
                tag_checked,
                wback,
            }),
        ),
    )
}

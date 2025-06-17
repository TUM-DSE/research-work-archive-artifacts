#![allow(nonstandard_style, unused)]
use crate::arm64::AArch64LifterError;
use crate::arm64::{decode, common};
use crate::arm64::common::types::BigIntExt;
use crate::integer_to_usize;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive};
use std::collections::BTreeMap;
pub fn decode_aarch64_memory_literal_general(
    reader: &mut decode::BitReader,
) -> Result<common::types::Instruction, AArch64LifterError> {
    let opc = common::types::bits::from_bits_literal(
        &reader.extract_slice(30usize, 2usize)?,
    )?;
    let imm19 = common::types::bits::from_bits_literal(
        &reader.extract_slice(5usize, 19usize)?,
    )?;
    let Rt = common::types::bits::from_bits_literal(
        &reader.extract_slice(0usize, 5usize)?,
    )?;
    let mut t: common::types::integer = decode::helpers::UInt_1(
        Rt,
        common::types::integer::from(5),
    )?;
    let mut memop: common::types::MemOp = common::types::MemOp::MemOp_LOAD;
    let mut signed: common::types::boolean = common::types::boolean::FALSE;
    let mut size: common::types::integer = common::types::integer::default();
    let mut offset: common::types::bits = common::types::bits::new(
        0,
        integer_to_usize!(common::types::integer::from(64)),
    );
    if (opc.match_with_pattern("00")) && true {
        size = common::types::integer::from(4);
    } else if (opc.match_with_pattern("01")) && true {
        size = common::types::integer::from(8);
    } else if (opc.match_with_pattern("10")) && true {
        size = common::types::integer::from(4);
        signed = common::types::boolean::TRUE;
    } else if (opc.match_with_pattern("11")) && true {
        memop = common::types::MemOp::MemOp_PREFETCH;
    }
    offset = decode::helpers::SignExtend_0(
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
    let mut tag_checked: common::types::boolean = common::types::boolean::FALSE;
    Ok(
        common::types::Instruction::aarch64_memory_literal_general(
            Box::new(common::types::aarch64_memory_literal_general_operands {
                memop,
                offset,
                signed,
                size,
                t,
                tag_checked,
            }),
        ),
    )
}

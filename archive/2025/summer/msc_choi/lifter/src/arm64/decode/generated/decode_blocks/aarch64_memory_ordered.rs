#![allow(nonstandard_style, unused)]
use crate::arm64::AArch64LifterError;
use crate::arm64::{decode, common};
use crate::arm64::common::types::BigIntExt;
use crate::integer_to_usize;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive};
use std::collections::BTreeMap;
pub fn decode_aarch64_memory_ordered(
    reader: &mut decode::BitReader,
) -> Result<common::types::Instruction, AArch64LifterError> {
    let size = common::types::bits::from_bits_literal(
        &reader.extract_slice(30usize, 2usize)?,
    )?;
    let L = common::types::bits::from_bits_literal(
        &reader.extract_slice(22usize, 1usize)?,
    )?;
    let Rs = common::types::bits::from_bits_literal(
        &reader.extract_slice(16usize, 5usize)?,
    )?;
    let o0 = common::types::bits::from_bits_literal(
        &reader.extract_slice(15usize, 1usize)?,
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
    let mut s: common::types::integer = decode::helpers::UInt_1(
        Rs,
        common::types::integer::from(5),
    )?;
    let mut acctype: common::types::AccType = if (decode::helpers::eq_bits_0(
        o0,
        common::types::bits::from_bits_literal("0")?,
        common::types::integer::from(1),
    )?) == common::types::boolean::TRUE
    {
        common::types::AccType::AccType_LIMITEDORDERED
    } else {
        common::types::AccType::AccType_ORDERED
    };
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
    let mut elsize: common::types::integer = decode::helpers::shift_left_int_0(
        common::types::integer::from(8),
        decode::helpers::UInt_1(size, common::types::integer::from(2))?,
    )?;
    let mut regsize: common::types::integer = if (decode::helpers::eq_int_0(
        elsize.clone(),
        common::types::integer::from(64),
    )?) == common::types::boolean::TRUE
    {
        common::types::integer::from(64)
    } else {
        common::types::integer::from(32)
    };
    let mut datasize: common::types::integer = elsize.clone();
    let mut tag_checked: common::types::boolean = decode::helpers::ne_int_0(
        n.clone(),
        common::types::integer::from(31),
    )?;
    Ok(
        common::types::Instruction::aarch64_memory_ordered(
            Box::new(common::types::aarch64_memory_ordered_operands {
                acctype,
                datasize,
                elsize,
                memop,
                n,
                regsize,
                s,
                t,
                t2,
                tag_checked,
            }),
        ),
    )
}

#![allow(nonstandard_style, unused)]
use crate::arm64::AArch64LifterError;
use crate::arm64::{decode, common};
use crate::arm64::common::types::BigIntExt;
use crate::integer_to_usize;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive};
use std::collections::BTreeMap;
pub fn decode_aarch64_integer_arithmetic_address_pc_rel(
    reader: &mut decode::BitReader,
) -> Result<common::types::Instruction, AArch64LifterError> {
    let op = common::types::bits::from_bits_literal(
        &reader.extract_slice(31usize, 1usize)?,
    )?;
    let immlo = common::types::bits::from_bits_literal(
        &reader.extract_slice(29usize, 2usize)?,
    )?;
    let immhi = common::types::bits::from_bits_literal(
        &reader.extract_slice(5usize, 19usize)?,
    )?;
    let Rd = common::types::bits::from_bits_literal(
        &reader.extract_slice(0usize, 5usize)?,
    )?;
    let mut d: common::types::integer = decode::helpers::UInt_1(
        Rd,
        common::types::integer::from(5),
    )?;
    let mut page: common::types::boolean = (decode::helpers::eq_bits_0(
        op,
        common::types::bits::from_bits_literal("1")?,
        common::types::integer::from(1),
    )?);
    let mut imm: common::types::bits = common::types::bits::new(
        0,
        integer_to_usize!(common::types::integer::from(64)),
    );
    if (page) == common::types::boolean::TRUE {
        imm = decode::helpers::SignExtend_0(
            decode::helpers::append_bits_0(
                decode::helpers::append_bits_0(
                    immhi,
                    immlo,
                    common::types::integer::from(19),
                    common::types::integer::from(2),
                )?,
                decode::helpers::Zeros_0(
                    common::types::integer::from(12),
                    common::types::integer::from(12),
                )?,
                decode::helpers::add_int_0(
                    common::types::integer::from(19),
                    common::types::integer::from(2),
                )?,
                common::types::integer::from(12),
            )?,
            common::types::integer::from(64),
            decode::helpers::add_int_0(
                decode::helpers::add_int_0(
                    common::types::integer::from(19),
                    common::types::integer::from(2),
                )?,
                common::types::integer::from(12),
            )?,
            common::types::integer::from(64),
        )?;
    } else {
        imm = decode::helpers::SignExtend_0(
            decode::helpers::append_bits_0(
                immhi,
                immlo,
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
    }
    Ok(
        common::types::Instruction::aarch64_integer_arithmetic_address_pc_rel(
            Box::new(common::types::aarch64_integer_arithmetic_address_pc_rel_operands {
                d,
                imm,
                page,
            }),
        ),
    )
}

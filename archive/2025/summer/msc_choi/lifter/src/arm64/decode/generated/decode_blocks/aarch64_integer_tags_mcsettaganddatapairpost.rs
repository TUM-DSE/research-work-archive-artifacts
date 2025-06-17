#![allow(nonstandard_style, unused)]
use crate::arm64::AArch64LifterError;
use crate::arm64::{decode, common};
use crate::arm64::common::types::BigIntExt;
use crate::integer_to_usize;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive};
use std::collections::BTreeMap;
pub fn decode_aarch64_integer_tags_mcsettaganddatapairpost(
    reader: &mut decode::BitReader,
) -> Result<common::types::Instruction, AArch64LifterError> {
    let simm7 = common::types::bits::from_bits_literal(
        &reader.extract_slice(15usize, 7usize)?,
    )?;
    let Xt2 = common::types::bits::from_bits_literal(
        &reader.extract_slice(10usize, 5usize)?,
    )?;
    let Xn = common::types::bits::from_bits_literal(
        &reader.extract_slice(5usize, 5usize)?,
    )?;
    let Xt = common::types::bits::from_bits_literal(
        &reader.extract_slice(0usize, 5usize)?,
    )?;
    let mut n: common::types::integer = decode::helpers::UInt_1(
        Xn,
        common::types::integer::from(5),
    )?;
    let mut t: common::types::integer = decode::helpers::UInt_1(
        Xt,
        common::types::integer::from(5),
    )?;
    let mut t2: common::types::integer = decode::helpers::UInt_1(
        Xt2,
        common::types::integer::from(5),
    )?;
    Ok(
        common::types::Instruction::aarch64_integer_tags_mcsettaganddatapairpost(
            Box::new(common::types::aarch64_integer_tags_mcsettaganddatapairpost_operands {
                n,
                t,
                t2,
            }),
        ),
    )
}

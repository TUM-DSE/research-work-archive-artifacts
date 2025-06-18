#![allow(nonstandard_style, unused)]
use crate::arm64::AArch64LifterError;
use crate::arm64::{decode, common};
use crate::arm64::common::types::BigIntExt;
use crate::integer_to_usize;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive};
use std::collections::BTreeMap;
pub fn decode_aarch64_integer_tags_mcsettagpairandzerodatapost(
    reader: &mut decode::BitReader,
) -> Result<common::types::Instruction, AArch64LifterError> {
    let imm9 = common::types::bits::from_bits_literal(
        &reader.extract_slice(12usize, 9usize)?,
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
    Ok(
        common::types::Instruction::aarch64_integer_tags_mcsettagpairandzerodatapost(
            Box::new(common::types::aarch64_integer_tags_mcsettagpairandzerodatapost_operands {
                n,
                t,
            }),
        ),
    )
}

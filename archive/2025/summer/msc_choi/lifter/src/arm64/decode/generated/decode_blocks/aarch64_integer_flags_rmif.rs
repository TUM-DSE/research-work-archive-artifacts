#![allow(nonstandard_style, unused)]
use crate::arm64::AArch64LifterError;
use crate::arm64::{decode, common};
use crate::arm64::common::types::BigIntExt;
use crate::integer_to_usize;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive};
use std::collections::BTreeMap;
pub fn decode_aarch64_integer_flags_rmif(
    reader: &mut decode::BitReader,
) -> Result<common::types::Instruction, AArch64LifterError> {
    let sf = common::types::bits::from_bits_literal(
        &reader.extract_slice(31usize, 1usize)?,
    )?;
    let imm6 = common::types::bits::from_bits_literal(
        &reader.extract_slice(15usize, 6usize)?,
    )?;
    let Rn = common::types::bits::from_bits_literal(
        &reader.extract_slice(5usize, 5usize)?,
    )?;
    let mask = common::types::bits::from_bits_literal(
        &reader.extract_slice(0usize, 4usize)?,
    )?;
    let mut n: common::types::integer = decode::helpers::UInt_1(
        Rn,
        common::types::integer::from(5),
    )?;
    Ok(
        common::types::Instruction::aarch64_integer_flags_rmif(
            Box::new(common::types::aarch64_integer_flags_rmif_operands {
                n,
            }),
        ),
    )
}

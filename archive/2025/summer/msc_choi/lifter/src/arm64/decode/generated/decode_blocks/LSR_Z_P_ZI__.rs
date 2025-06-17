#![allow(nonstandard_style, unused)]
use crate::arm64::AArch64LifterError;
use crate::arm64::{decode, common};
use crate::arm64::common::types::BigIntExt;
use crate::integer_to_usize;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive};
use std::collections::BTreeMap;
pub fn decode_LSR_Z_P_ZI__(
    reader: &mut decode::BitReader,
) -> Result<common::types::Instruction, AArch64LifterError> {
    let tszh = common::types::bits::from_bits_literal(
        &reader.extract_slice(22usize, 2usize)?,
    )?;
    let Pg = common::types::bits::from_bits_literal(
        &reader.extract_slice(10usize, 3usize)?,
    )?;
    let tszl = common::types::bits::from_bits_literal(
        &reader.extract_slice(8usize, 2usize)?,
    )?;
    let imm3 = common::types::bits::from_bits_literal(
        &reader.extract_slice(5usize, 3usize)?,
    )?;
    let Zdn = common::types::bits::from_bits_literal(
        &reader.extract_slice(0usize, 5usize)?,
    )?;
    let mut dn: common::types::integer = decode::helpers::UInt_1(
        Zdn,
        common::types::integer::from(5),
    )?;
    Ok(
        common::types::Instruction::LSR_Z_P_ZI__(
            Box::new(common::types::LSR_Z_P_ZI___operands {
                dn,
            }),
        ),
    )
}

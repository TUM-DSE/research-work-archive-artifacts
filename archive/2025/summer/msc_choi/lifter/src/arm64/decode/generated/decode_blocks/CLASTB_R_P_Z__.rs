#![allow(nonstandard_style, unused)]
use crate::arm64::AArch64LifterError;
use crate::arm64::{decode, common};
use crate::arm64::common::types::BigIntExt;
use crate::integer_to_usize;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive};
use std::collections::BTreeMap;
pub fn decode_CLASTB_R_P_Z__(
    reader: &mut decode::BitReader,
) -> Result<common::types::Instruction, AArch64LifterError> {
    let size = common::types::bits::from_bits_literal(
        &reader.extract_slice(22usize, 2usize)?,
    )?;
    let Pg = common::types::bits::from_bits_literal(
        &reader.extract_slice(10usize, 3usize)?,
    )?;
    let Zm = common::types::bits::from_bits_literal(
        &reader.extract_slice(5usize, 5usize)?,
    )?;
    let Rdn = common::types::bits::from_bits_literal(
        &reader.extract_slice(0usize, 5usize)?,
    )?;
    let mut dn: common::types::integer = decode::helpers::UInt_1(
        Rdn,
        common::types::integer::from(5),
    )?;
    Ok(
        common::types::Instruction::CLASTB_R_P_Z__(
            Box::new(common::types::CLASTB_R_P_Z___operands {
                dn,
            }),
        ),
    )
}

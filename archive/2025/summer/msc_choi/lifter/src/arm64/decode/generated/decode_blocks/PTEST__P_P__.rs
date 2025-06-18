#![allow(nonstandard_style, unused)]
use crate::arm64::AArch64LifterError;
use crate::arm64::{decode, common};
use crate::arm64::common::types::BigIntExt;
use crate::integer_to_usize;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive};
use std::collections::BTreeMap;
pub fn decode_PTEST__P_P__(
    reader: &mut decode::BitReader,
) -> Result<common::types::Instruction, AArch64LifterError> {
    let Pg = common::types::bits::from_bits_literal(
        &reader.extract_slice(10usize, 4usize)?,
    )?;
    let Pn = common::types::bits::from_bits_literal(
        &reader.extract_slice(5usize, 4usize)?,
    )?;
    let mut n: common::types::integer = decode::helpers::UInt_1(
        Pn,
        common::types::integer::from(4),
    )?;
    Ok(
        common::types::Instruction::PTEST__P_P__(
            Box::new(common::types::PTEST__P_P___operands {
                n,
            }),
        ),
    )
}

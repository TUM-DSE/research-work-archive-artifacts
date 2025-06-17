#![allow(nonstandard_style, unused)]
use crate::arm64::AArch64LifterError;
use crate::arm64::{decode, common};
use crate::arm64::common::types::BigIntExt;
use crate::integer_to_usize;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive};
use std::collections::BTreeMap;
pub fn decode_aarch64_integer_pac_strip_hint(
    reader: &mut decode::BitReader,
) -> Result<common::types::Instruction, AArch64LifterError> {
    let mut d: common::types::integer = common::types::integer::from(30);
    Ok(
        common::types::Instruction::aarch64_integer_pac_strip_dp_1src(
            Box::new(common::types::aarch64_integer_pac_strip_dp_1src_operands {
                d,
            }),
        ),
    )
}

#![allow(nonstandard_style, unused)]
use crate::arm64::AArch64LifterError;
use crate::arm64::{decode, common};
use crate::arm64::common::types::BigIntExt;
use crate::integer_to_usize;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive};
use std::collections::BTreeMap;
pub fn decode_aarch64_integer_pac_autib_hint(
    reader: &mut decode::BitReader,
) -> Result<common::types::Instruction, AArch64LifterError> {
    let CRm = common::types::bits::from_bits_literal(
        &reader.extract_slice(8usize, 4usize)?,
    )?;
    let op2 = common::types::bits::from_bits_literal(
        &reader.extract_slice(5usize, 3usize)?,
    )?;
    let mut d: common::types::integer = common::types::integer::default();
    let mut n: common::types::integer = common::types::integer::default();
    if (decode::helpers::append_bits_0(
            CRm,
            op2,
            common::types::integer::from(4),
            common::types::integer::from(3),
        )?
        .match_with_pattern("0011 110")) && true
    {
        d = common::types::integer::from(30);
        n = common::types::integer::from(31);
    } else if (decode::helpers::append_bits_0(
            CRm,
            op2,
            common::types::integer::from(4),
            common::types::integer::from(3),
        )?
        .match_with_pattern("0011 111")) && true
    {
        d = common::types::integer::from(30);
    } else if (decode::helpers::append_bits_0(
            CRm,
            op2,
            common::types::integer::from(4),
            common::types::integer::from(3),
        )?
        .match_with_pattern("0001 110")) && true
    {
        d = common::types::integer::from(17);
        n = common::types::integer::from(16);
    } else if (decode::helpers::append_bits_0(
            CRm,
            op2,
            common::types::integer::from(4),
            common::types::integer::from(3),
        )?
        .match_with_pattern("0001 000")) && true
    {
        return Ok(
            decode::generated::decode_blocks::decode_aarch64_integer_pac_pacia_dp_1src(
                reader,
            )?,
        );
    } else if (decode::helpers::append_bits_0(
            CRm,
            op2,
            common::types::integer::from(4),
            common::types::integer::from(3),
        )?
        .match_with_pattern("0001 010")) && true
    {
        return Ok(
            decode::generated::decode_blocks::decode_aarch64_integer_pac_pacib_dp_1src(
                reader,
            )?,
        );
    } else if (decode::helpers::append_bits_0(
            CRm,
            op2,
            common::types::integer::from(4),
            common::types::integer::from(3),
        )?
        .match_with_pattern("0001 100")) && true
    {
        return Ok(
            decode::generated::decode_blocks::decode_aarch64_integer_pac_autia_dp_1src(
                reader,
            )?,
        );
    } else if (decode::helpers::append_bits_0(
            CRm,
            op2,
            common::types::integer::from(4),
            common::types::integer::from(3),
        )?
        .match_with_pattern("0011 00x")) && true
    {
        return Ok(
            decode::generated::decode_blocks::decode_aarch64_integer_pac_pacia_dp_1src(
                reader,
            )?,
        );
    } else if (decode::helpers::append_bits_0(
            CRm,
            op2,
            common::types::integer::from(4),
            common::types::integer::from(3),
        )?
        .match_with_pattern("0011 01x")) && true
    {
        return Ok(
            decode::generated::decode_blocks::decode_aarch64_integer_pac_pacib_dp_1src(
                reader,
            )?,
        );
    } else if (decode::helpers::append_bits_0(
            CRm,
            op2,
            common::types::integer::from(4),
            common::types::integer::from(3),
        )?
        .match_with_pattern("0011 10x")) && true
    {
        return Ok(
            decode::generated::decode_blocks::decode_aarch64_integer_pac_autia_dp_1src(
                reader,
            )?,
        );
    } else if (decode::helpers::append_bits_0(
            CRm,
            op2,
            common::types::integer::from(4),
            common::types::integer::from(3),
        )?
        .match_with_pattern("0000 111")) && true
    {
        return Ok(
            decode::generated::decode_blocks::decode_aarch64_integer_pac_strip_dp_1src(
                reader,
            )?,
        );
    } else {
        return Ok(
            decode::generated::decode_blocks::decode_aarch64_system_hints(reader)?,
        );
    }
    Ok(
        common::types::Instruction::aarch64_integer_pac_autib_dp_1src(
            Box::new(common::types::aarch64_integer_pac_autib_dp_1src_operands {
                d,
                n,
            }),
        ),
    )
}

#![allow(nonstandard_style, unused)]
use crate::arm64::AArch64LifterError;
use crate::arm64::{decode, common};
use crate::arm64::common::types::BigIntExt;
use crate::integer_to_usize;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive};
use std::collections::BTreeMap;
pub fn decode_aarch64_integer_bitfield(
    reader: &mut decode::BitReader,
) -> Result<common::types::Instruction, AArch64LifterError> {
    let sf = common::types::bits::from_bits_literal(
        &reader.extract_slice(31usize, 1usize)?,
    )?;
    let opc = common::types::bits::from_bits_literal(
        &reader.extract_slice(29usize, 2usize)?,
    )?;
    let N = common::types::bits::from_bits_literal(
        &reader.extract_slice(22usize, 1usize)?,
    )?;
    let immr = common::types::bits::from_bits_literal(
        &reader.extract_slice(16usize, 6usize)?,
    )?;
    let imms = common::types::bits::from_bits_literal(
        &reader.extract_slice(10usize, 6usize)?,
    )?;
    let Rn = common::types::bits::from_bits_literal(
        &reader.extract_slice(5usize, 5usize)?,
    )?;
    let Rd = common::types::bits::from_bits_literal(
        &reader.extract_slice(0usize, 5usize)?,
    )?;
    let mut d: common::types::integer = decode::helpers::UInt_1(
        Rd,
        common::types::integer::from(5),
    )?;
    let mut n: common::types::integer = decode::helpers::UInt_1(
        Rn,
        common::types::integer::from(5),
    )?;
    let mut datasize: common::types::integer = if (decode::helpers::eq_bits_0(
        sf,
        common::types::bits::from_bits_literal("1")?,
        common::types::integer::from(1),
    )?) == common::types::boolean::TRUE
    {
        common::types::integer::from(64)
    } else {
        common::types::integer::from(32)
    };
    let mut inzero: common::types::boolean = common::types::boolean::default();
    let mut extend: common::types::boolean = common::types::boolean::default();
    let mut R: common::types::integer = common::types::integer::default();
    let mut S: common::types::integer = common::types::integer::default();
    let mut wmask: common::types::bits = common::types::bits::new(
        0,
        integer_to_usize!(datasize),
    );
    let mut tmask: common::types::bits = common::types::bits::new(
        0,
        integer_to_usize!(datasize),
    );
    if (opc.match_with_pattern("00")) && true {
        inzero = common::types::boolean::TRUE;
        extend = common::types::boolean::TRUE;
    } else if (opc.match_with_pattern("01")) && true {
        inzero = common::types::boolean::FALSE;
        extend = common::types::boolean::FALSE;
    } else if (opc.match_with_pattern("10")) && true {
        inzero = common::types::boolean::TRUE;
        extend = common::types::boolean::FALSE;
    } else if (opc.match_with_pattern("11")) && true {
        return Err(AArch64LifterError::UndefinedInstruction);
    }
    if (decode::helpers::and_bool_0(
        decode::helpers::eq_bits_0(
            sf,
            common::types::bits::from_bits_literal("1")?,
            common::types::integer::from(1),
        )?,
        decode::helpers::ne_bits_0(
            N,
            common::types::bits::from_bits_literal("1")?,
            common::types::integer::from(1),
        )?,
    )?) == common::types::boolean::TRUE
    {
        return Err(AArch64LifterError::UndefinedInstruction);
    } else {}
    if (decode::helpers::and_bool_0(
        decode::helpers::eq_bits_0(
            sf,
            common::types::bits::from_bits_literal("0")?,
            common::types::integer::from(1),
        )?,
        (decode::helpers::or_bool_0(
            decode::helpers::or_bool_0(
                decode::helpers::ne_bits_0(
                    N,
                    common::types::bits::from_bits_literal("0")?,
                    common::types::integer::from(1),
                )?,
                decode::helpers::ne_bits_0(
                    immr
                        .extract_slice(
                            integer_to_usize!(common::types::integer::from(5)),
                            1,
                        )?,
                    common::types::bits::from_bits_literal("0")?,
                    decode::helpers::add_int_0(
                        common::types::integer::from(0),
                        common::types::integer::from(1),
                    )?,
                )?,
            )?,
            decode::helpers::ne_bits_0(
                imms
                    .extract_slice(
                        integer_to_usize!(common::types::integer::from(5)),
                        1,
                    )?,
                common::types::bits::from_bits_literal("0")?,
                decode::helpers::add_int_0(
                    common::types::integer::from(0),
                    common::types::integer::from(1),
                )?,
            )?,
        )?),
    )?) == common::types::boolean::TRUE
    {
        return Err(AArch64LifterError::UndefinedInstruction);
    } else {}
    R = decode::helpers::UInt_1(immr, common::types::integer::from(6))?;
    S = decode::helpers::UInt_1(imms, common::types::integer::from(6))?;
    (wmask, tmask) = decode::helpers::DecodeBitMasks_0(
        N,
        imms,
        immr,
        common::types::boolean::FALSE,
        datasize.clone(),
    )?;
    Ok(
        common::types::Instruction::aarch64_integer_bitfield(
            Box::new(common::types::aarch64_integer_bitfield_operands {
                R,
                S,
                d,
                datasize,
                extend,
                inzero,
                n,
                tmask,
                wmask,
            }),
        ),
    )
}

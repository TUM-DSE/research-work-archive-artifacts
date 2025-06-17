#![allow(nonstandard_style, unused)]
use crate::arm64::AArch64LifterError;
use crate::arm64::{decode, common};
use crate::arm64::common::types::BigIntExt;
use crate::integer_to_usize;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive};
pub fn DecodeBitMasks_0(
    immN: common::types::bits,
    imms: common::types::bits,
    immr: common::types::bits,
    immediate: common::types::boolean,
    M: common::types::integer,
) -> Result<(common::types::bits, common::types::bits), AArch64LifterError> {
    let mut R: common::types::integer = common::types::integer::default();
    let mut S: common::types::integer = common::types::integer::default();
    let mut diff: common::types::integer = common::types::integer::default();
    let mut len: common::types::integer = common::types::integer::default();
    let (mut tmask, mut wmask): (common::types::bits, common::types::bits) = (
        common::types::bits::new(0, integer_to_usize!(common::types::integer::from(64))),
        common::types::bits::new(0, integer_to_usize!(common::types::integer::from(64))),
    );
    let (mut tmask_and, mut wmask_and): (common::types::bits, common::types::bits) = (
        common::types::bits::new(0, integer_to_usize!(common::types::integer::from(6))),
        common::types::bits::new(0, integer_to_usize!(common::types::integer::from(6))),
    );
    let (mut tmask_or, mut wmask_or): (common::types::bits, common::types::bits) = (
        common::types::bits::new(0, integer_to_usize!(common::types::integer::from(6))),
        common::types::bits::new(0, integer_to_usize!(common::types::integer::from(6))),
    );
    let mut levels: common::types::bits = common::types::bits::new(
        0,
        integer_to_usize!(common::types::integer::from(6)),
    );
    len = decode::helpers::HighestSetBit_0(
        decode::helpers::append_bits_0(
            immN,
            decode::helpers::not_bits_0((imms), common::types::integer::from(6))?,
            common::types::integer::from(1),
            common::types::integer::from(6),
        )?,
        decode::helpers::add_int_0(
            common::types::integer::from(1),
            common::types::integer::from(6),
        )?,
    )?;
    if (decode::helpers::lt_int_0(len.clone(), common::types::integer::from(1))?)
        == common::types::boolean::TRUE
    {
        return Err(AArch64LifterError::UndefinedInstruction);
    } else {}
    assert_eq!(
        decode::helpers::ge_int_0(M.clone(),
        (decode::helpers::shift_left_int_0(common::types::integer::from(1), len.clone())
        ?)) ?, common::types::boolean::TRUE
    );
    levels = decode::helpers::ZeroExtend_0(
        decode::helpers::Ones_0(len.clone(), len.clone())?,
        common::types::integer::from(6),
        len.clone(),
        common::types::integer::from(6),
    )?;
    if (decode::helpers::and_bool_0(
        immediate,
        decode::helpers::eq_bits_0(
            (decode::helpers::and_bits_0(
                imms,
                levels,
                common::types::integer::from(6),
            )?),
            levels,
            common::types::integer::from(6),
        )?,
    )?) == common::types::boolean::TRUE
    {
        return Err(AArch64LifterError::UndefinedInstruction);
    } else {}
    S = decode::helpers::UInt_1(
        decode::helpers::and_bits_0(imms, levels, common::types::integer::from(6))?,
        common::types::integer::from(6),
    )?;
    R = decode::helpers::UInt_1(
        decode::helpers::and_bits_0(immr, levels, common::types::integer::from(6))?,
        common::types::integer::from(6),
    )?;
    diff = decode::helpers::sub_int_0(S.clone(), R.clone())?;
    tmask_and = decode::helpers::or_bits_0(
        diff
            .extract_slice(
                integer_to_usize!(common::types::integer::from(0)),
                integer_to_usize!(common::types::integer::from(5)) + 1
                    - integer_to_usize!(common::types::integer::from(0)),
            )?,
        decode::helpers::not_bits_0((levels), common::types::integer::from(6))?,
        common::types::integer::from(6),
    )?;
    tmask_or = decode::helpers::and_bits_0(
        diff
            .extract_slice(
                integer_to_usize!(common::types::integer::from(0)),
                integer_to_usize!(common::types::integer::from(5)) + 1
                    - integer_to_usize!(common::types::integer::from(0)),
            )?,
        levels,
        decode::helpers::add_int_0(
            common::types::integer::from(0),
            decode::helpers::add_int_0(
                decode::helpers::sub_int_0(
                    common::types::integer::from(5),
                    common::types::integer::from(0),
                )?,
                common::types::integer::from(1),
            )?,
        )?,
    )?;
    tmask = decode::helpers::Ones_0(
        common::types::integer::from(64),
        common::types::integer::from(64),
    )?;
    tmask = (decode::helpers::or_bits_0(
        (decode::helpers::and_bits_0(
            tmask,
            decode::helpers::Replicate_0(
                decode::helpers::append_bits_0(
                    decode::helpers::Replicate_0(
                        tmask_and
                            .extract_slice(
                                integer_to_usize!(common::types::integer::from(0)),
                                1,
                            )?,
                        common::types::integer::from(1),
                        decode::helpers::add_int_0(
                            common::types::integer::from(0),
                            common::types::integer::from(1),
                        )?,
                        common::types::integer::from(1),
                    )?,
                    decode::helpers::Ones_0(
                        common::types::integer::from(1),
                        common::types::integer::from(1),
                    )?,
                    decode::helpers::mul_int_0(
                        decode::helpers::add_int_0(
                            common::types::integer::from(0),
                            common::types::integer::from(1),
                        )?,
                        common::types::integer::from(1),
                    )?,
                    common::types::integer::from(1),
                )?,
                common::types::integer::from(32),
                decode::helpers::add_int_0(
                    decode::helpers::mul_int_0(
                        decode::helpers::add_int_0(
                            common::types::integer::from(0),
                            common::types::integer::from(1),
                        )?,
                        common::types::integer::from(1),
                    )?,
                    common::types::integer::from(1),
                )?,
                common::types::integer::from(32),
            )?,
            decode::helpers::mul_int_0(
                decode::helpers::add_int_0(
                    common::types::integer::from(1),
                    decode::helpers::mul_int_0(
                        decode::helpers::add_int_0(
                            common::types::integer::from(0),
                            common::types::integer::from(1),
                        )?,
                        common::types::integer::from(1),
                    )?,
                )?,
                common::types::integer::from(32),
            )?,
        )?),
        decode::helpers::Replicate_0(
            decode::helpers::append_bits_0(
                decode::helpers::Zeros_0(
                    common::types::integer::from(1),
                    common::types::integer::from(1),
                )?,
                decode::helpers::Replicate_0(
                    tmask_or
                        .extract_slice(
                            integer_to_usize!(common::types::integer::from(0)),
                            1,
                        )?,
                    common::types::integer::from(1),
                    decode::helpers::add_int_0(
                        common::types::integer::from(0),
                        common::types::integer::from(1),
                    )?,
                    common::types::integer::from(1),
                )?,
                common::types::integer::from(1),
                decode::helpers::mul_int_0(
                    decode::helpers::add_int_0(
                        common::types::integer::from(0),
                        common::types::integer::from(1),
                    )?,
                    common::types::integer::from(1),
                )?,
            )?,
            common::types::integer::from(32),
            decode::helpers::add_int_0(
                common::types::integer::from(1),
                decode::helpers::mul_int_0(
                    decode::helpers::add_int_0(
                        common::types::integer::from(0),
                        common::types::integer::from(1),
                    )?,
                    common::types::integer::from(1),
                )?,
            )?,
            common::types::integer::from(32),
        )?,
        decode::helpers::mul_int_0(
            decode::helpers::add_int_0(
                common::types::integer::from(1),
                decode::helpers::mul_int_0(
                    decode::helpers::add_int_0(
                        common::types::integer::from(0),
                        common::types::integer::from(1),
                    )?,
                    common::types::integer::from(1),
                )?,
            )?,
            common::types::integer::from(32),
        )?,
    )?);
    tmask = (decode::helpers::or_bits_0(
        (decode::helpers::and_bits_0(
            tmask,
            decode::helpers::Replicate_0(
                decode::helpers::append_bits_0(
                    decode::helpers::Replicate_0(
                        tmask_and
                            .extract_slice(
                                integer_to_usize!(common::types::integer::from(1)),
                                1,
                            )?,
                        common::types::integer::from(2),
                        decode::helpers::add_int_0(
                            common::types::integer::from(0),
                            common::types::integer::from(1),
                        )?,
                        common::types::integer::from(2),
                    )?,
                    decode::helpers::Ones_0(
                        common::types::integer::from(2),
                        common::types::integer::from(2),
                    )?,
                    decode::helpers::mul_int_0(
                        decode::helpers::add_int_0(
                            common::types::integer::from(0),
                            common::types::integer::from(1),
                        )?,
                        common::types::integer::from(2),
                    )?,
                    common::types::integer::from(2),
                )?,
                common::types::integer::from(16),
                decode::helpers::add_int_0(
                    decode::helpers::mul_int_0(
                        decode::helpers::add_int_0(
                            common::types::integer::from(0),
                            common::types::integer::from(1),
                        )?,
                        common::types::integer::from(2),
                    )?,
                    common::types::integer::from(2),
                )?,
                common::types::integer::from(16),
            )?,
            decode::helpers::mul_int_0(
                decode::helpers::add_int_0(
                    common::types::integer::from(2),
                    decode::helpers::mul_int_0(
                        decode::helpers::add_int_0(
                            common::types::integer::from(0),
                            common::types::integer::from(1),
                        )?,
                        common::types::integer::from(2),
                    )?,
                )?,
                common::types::integer::from(16),
            )?,
        )?),
        decode::helpers::Replicate_0(
            decode::helpers::append_bits_0(
                decode::helpers::Zeros_0(
                    common::types::integer::from(2),
                    common::types::integer::from(2),
                )?,
                decode::helpers::Replicate_0(
                    tmask_or
                        .extract_slice(
                            integer_to_usize!(common::types::integer::from(1)),
                            1,
                        )?,
                    common::types::integer::from(2),
                    decode::helpers::add_int_0(
                        common::types::integer::from(0),
                        common::types::integer::from(1),
                    )?,
                    common::types::integer::from(2),
                )?,
                common::types::integer::from(2),
                decode::helpers::mul_int_0(
                    decode::helpers::add_int_0(
                        common::types::integer::from(0),
                        common::types::integer::from(1),
                    )?,
                    common::types::integer::from(2),
                )?,
            )?,
            common::types::integer::from(16),
            decode::helpers::add_int_0(
                common::types::integer::from(2),
                decode::helpers::mul_int_0(
                    decode::helpers::add_int_0(
                        common::types::integer::from(0),
                        common::types::integer::from(1),
                    )?,
                    common::types::integer::from(2),
                )?,
            )?,
            common::types::integer::from(16),
        )?,
        decode::helpers::mul_int_0(
            decode::helpers::add_int_0(
                common::types::integer::from(2),
                decode::helpers::mul_int_0(
                    decode::helpers::add_int_0(
                        common::types::integer::from(0),
                        common::types::integer::from(1),
                    )?,
                    common::types::integer::from(2),
                )?,
            )?,
            common::types::integer::from(16),
        )?,
    )?);
    tmask = (decode::helpers::or_bits_0(
        (decode::helpers::and_bits_0(
            tmask,
            decode::helpers::Replicate_0(
                decode::helpers::append_bits_0(
                    decode::helpers::Replicate_0(
                        tmask_and
                            .extract_slice(
                                integer_to_usize!(common::types::integer::from(2)),
                                1,
                            )?,
                        common::types::integer::from(4),
                        decode::helpers::add_int_0(
                            common::types::integer::from(0),
                            common::types::integer::from(1),
                        )?,
                        common::types::integer::from(4),
                    )?,
                    decode::helpers::Ones_0(
                        common::types::integer::from(4),
                        common::types::integer::from(4),
                    )?,
                    decode::helpers::mul_int_0(
                        decode::helpers::add_int_0(
                            common::types::integer::from(0),
                            common::types::integer::from(1),
                        )?,
                        common::types::integer::from(4),
                    )?,
                    common::types::integer::from(4),
                )?,
                common::types::integer::from(8),
                decode::helpers::add_int_0(
                    decode::helpers::mul_int_0(
                        decode::helpers::add_int_0(
                            common::types::integer::from(0),
                            common::types::integer::from(1),
                        )?,
                        common::types::integer::from(4),
                    )?,
                    common::types::integer::from(4),
                )?,
                common::types::integer::from(8),
            )?,
            decode::helpers::mul_int_0(
                decode::helpers::add_int_0(
                    common::types::integer::from(4),
                    decode::helpers::mul_int_0(
                        decode::helpers::add_int_0(
                            common::types::integer::from(0),
                            common::types::integer::from(1),
                        )?,
                        common::types::integer::from(4),
                    )?,
                )?,
                common::types::integer::from(8),
            )?,
        )?),
        decode::helpers::Replicate_0(
            decode::helpers::append_bits_0(
                decode::helpers::Zeros_0(
                    common::types::integer::from(4),
                    common::types::integer::from(4),
                )?,
                decode::helpers::Replicate_0(
                    tmask_or
                        .extract_slice(
                            integer_to_usize!(common::types::integer::from(2)),
                            1,
                        )?,
                    common::types::integer::from(4),
                    decode::helpers::add_int_0(
                        common::types::integer::from(0),
                        common::types::integer::from(1),
                    )?,
                    common::types::integer::from(4),
                )?,
                common::types::integer::from(4),
                decode::helpers::mul_int_0(
                    decode::helpers::add_int_0(
                        common::types::integer::from(0),
                        common::types::integer::from(1),
                    )?,
                    common::types::integer::from(4),
                )?,
            )?,
            common::types::integer::from(8),
            decode::helpers::add_int_0(
                common::types::integer::from(4),
                decode::helpers::mul_int_0(
                    decode::helpers::add_int_0(
                        common::types::integer::from(0),
                        common::types::integer::from(1),
                    )?,
                    common::types::integer::from(4),
                )?,
            )?,
            common::types::integer::from(8),
        )?,
        decode::helpers::mul_int_0(
            decode::helpers::add_int_0(
                common::types::integer::from(4),
                decode::helpers::mul_int_0(
                    decode::helpers::add_int_0(
                        common::types::integer::from(0),
                        common::types::integer::from(1),
                    )?,
                    common::types::integer::from(4),
                )?,
            )?,
            common::types::integer::from(8),
        )?,
    )?);
    tmask = (decode::helpers::or_bits_0(
        (decode::helpers::and_bits_0(
            tmask,
            decode::helpers::Replicate_0(
                decode::helpers::append_bits_0(
                    decode::helpers::Replicate_0(
                        tmask_and
                            .extract_slice(
                                integer_to_usize!(common::types::integer::from(3)),
                                1,
                            )?,
                        common::types::integer::from(8),
                        decode::helpers::add_int_0(
                            common::types::integer::from(0),
                            common::types::integer::from(1),
                        )?,
                        common::types::integer::from(8),
                    )?,
                    decode::helpers::Ones_0(
                        common::types::integer::from(8),
                        common::types::integer::from(8),
                    )?,
                    decode::helpers::mul_int_0(
                        decode::helpers::add_int_0(
                            common::types::integer::from(0),
                            common::types::integer::from(1),
                        )?,
                        common::types::integer::from(8),
                    )?,
                    common::types::integer::from(8),
                )?,
                common::types::integer::from(4),
                decode::helpers::add_int_0(
                    decode::helpers::mul_int_0(
                        decode::helpers::add_int_0(
                            common::types::integer::from(0),
                            common::types::integer::from(1),
                        )?,
                        common::types::integer::from(8),
                    )?,
                    common::types::integer::from(8),
                )?,
                common::types::integer::from(4),
            )?,
            decode::helpers::mul_int_0(
                decode::helpers::add_int_0(
                    common::types::integer::from(8),
                    decode::helpers::mul_int_0(
                        decode::helpers::add_int_0(
                            common::types::integer::from(0),
                            common::types::integer::from(1),
                        )?,
                        common::types::integer::from(8),
                    )?,
                )?,
                common::types::integer::from(4),
            )?,
        )?),
        decode::helpers::Replicate_0(
            decode::helpers::append_bits_0(
                decode::helpers::Zeros_0(
                    common::types::integer::from(8),
                    common::types::integer::from(8),
                )?,
                decode::helpers::Replicate_0(
                    tmask_or
                        .extract_slice(
                            integer_to_usize!(common::types::integer::from(3)),
                            1,
                        )?,
                    common::types::integer::from(8),
                    decode::helpers::add_int_0(
                        common::types::integer::from(0),
                        common::types::integer::from(1),
                    )?,
                    common::types::integer::from(8),
                )?,
                common::types::integer::from(8),
                decode::helpers::mul_int_0(
                    decode::helpers::add_int_0(
                        common::types::integer::from(0),
                        common::types::integer::from(1),
                    )?,
                    common::types::integer::from(8),
                )?,
            )?,
            common::types::integer::from(4),
            decode::helpers::add_int_0(
                common::types::integer::from(8),
                decode::helpers::mul_int_0(
                    decode::helpers::add_int_0(
                        common::types::integer::from(0),
                        common::types::integer::from(1),
                    )?,
                    common::types::integer::from(8),
                )?,
            )?,
            common::types::integer::from(4),
        )?,
        decode::helpers::mul_int_0(
            decode::helpers::add_int_0(
                common::types::integer::from(8),
                decode::helpers::mul_int_0(
                    decode::helpers::add_int_0(
                        common::types::integer::from(0),
                        common::types::integer::from(1),
                    )?,
                    common::types::integer::from(8),
                )?,
            )?,
            common::types::integer::from(4),
        )?,
    )?);
    tmask = (decode::helpers::or_bits_0(
        (decode::helpers::and_bits_0(
            tmask,
            decode::helpers::Replicate_0(
                decode::helpers::append_bits_0(
                    decode::helpers::Replicate_0(
                        tmask_and
                            .extract_slice(
                                integer_to_usize!(common::types::integer::from(4)),
                                1,
                            )?,
                        common::types::integer::from(16),
                        decode::helpers::add_int_0(
                            common::types::integer::from(0),
                            common::types::integer::from(1),
                        )?,
                        common::types::integer::from(16),
                    )?,
                    decode::helpers::Ones_0(
                        common::types::integer::from(16),
                        common::types::integer::from(16),
                    )?,
                    decode::helpers::mul_int_0(
                        decode::helpers::add_int_0(
                            common::types::integer::from(0),
                            common::types::integer::from(1),
                        )?,
                        common::types::integer::from(16),
                    )?,
                    common::types::integer::from(16),
                )?,
                common::types::integer::from(2),
                decode::helpers::add_int_0(
                    decode::helpers::mul_int_0(
                        decode::helpers::add_int_0(
                            common::types::integer::from(0),
                            common::types::integer::from(1),
                        )?,
                        common::types::integer::from(16),
                    )?,
                    common::types::integer::from(16),
                )?,
                common::types::integer::from(2),
            )?,
            decode::helpers::mul_int_0(
                decode::helpers::add_int_0(
                    common::types::integer::from(16),
                    decode::helpers::mul_int_0(
                        decode::helpers::add_int_0(
                            common::types::integer::from(0),
                            common::types::integer::from(1),
                        )?,
                        common::types::integer::from(16),
                    )?,
                )?,
                common::types::integer::from(2),
            )?,
        )?),
        decode::helpers::Replicate_0(
            decode::helpers::append_bits_0(
                decode::helpers::Zeros_0(
                    common::types::integer::from(16),
                    common::types::integer::from(16),
                )?,
                decode::helpers::Replicate_0(
                    tmask_or
                        .extract_slice(
                            integer_to_usize!(common::types::integer::from(4)),
                            1,
                        )?,
                    common::types::integer::from(16),
                    decode::helpers::add_int_0(
                        common::types::integer::from(0),
                        common::types::integer::from(1),
                    )?,
                    common::types::integer::from(16),
                )?,
                common::types::integer::from(16),
                decode::helpers::mul_int_0(
                    decode::helpers::add_int_0(
                        common::types::integer::from(0),
                        common::types::integer::from(1),
                    )?,
                    common::types::integer::from(16),
                )?,
            )?,
            common::types::integer::from(2),
            decode::helpers::add_int_0(
                common::types::integer::from(16),
                decode::helpers::mul_int_0(
                    decode::helpers::add_int_0(
                        common::types::integer::from(0),
                        common::types::integer::from(1),
                    )?,
                    common::types::integer::from(16),
                )?,
            )?,
            common::types::integer::from(2),
        )?,
        decode::helpers::mul_int_0(
            decode::helpers::add_int_0(
                common::types::integer::from(16),
                decode::helpers::mul_int_0(
                    decode::helpers::add_int_0(
                        common::types::integer::from(0),
                        common::types::integer::from(1),
                    )?,
                    common::types::integer::from(16),
                )?,
            )?,
            common::types::integer::from(2),
        )?,
    )?);
    tmask = (decode::helpers::or_bits_0(
        (decode::helpers::and_bits_0(
            tmask,
            decode::helpers::Replicate_0(
                decode::helpers::append_bits_0(
                    decode::helpers::Replicate_0(
                        tmask_and
                            .extract_slice(
                                integer_to_usize!(common::types::integer::from(5)),
                                1,
                            )?,
                        common::types::integer::from(32),
                        decode::helpers::add_int_0(
                            common::types::integer::from(0),
                            common::types::integer::from(1),
                        )?,
                        common::types::integer::from(32),
                    )?,
                    decode::helpers::Ones_0(
                        common::types::integer::from(32),
                        common::types::integer::from(32),
                    )?,
                    decode::helpers::mul_int_0(
                        decode::helpers::add_int_0(
                            common::types::integer::from(0),
                            common::types::integer::from(1),
                        )?,
                        common::types::integer::from(32),
                    )?,
                    common::types::integer::from(32),
                )?,
                common::types::integer::from(1),
                decode::helpers::add_int_0(
                    decode::helpers::mul_int_0(
                        decode::helpers::add_int_0(
                            common::types::integer::from(0),
                            common::types::integer::from(1),
                        )?,
                        common::types::integer::from(32),
                    )?,
                    common::types::integer::from(32),
                )?,
                common::types::integer::from(1),
            )?,
            decode::helpers::mul_int_0(
                decode::helpers::add_int_0(
                    common::types::integer::from(32),
                    decode::helpers::mul_int_0(
                        decode::helpers::add_int_0(
                            common::types::integer::from(0),
                            common::types::integer::from(1),
                        )?,
                        common::types::integer::from(32),
                    )?,
                )?,
                common::types::integer::from(1),
            )?,
        )?),
        decode::helpers::Replicate_0(
            decode::helpers::append_bits_0(
                decode::helpers::Zeros_0(
                    common::types::integer::from(32),
                    common::types::integer::from(32),
                )?,
                decode::helpers::Replicate_0(
                    tmask_or
                        .extract_slice(
                            integer_to_usize!(common::types::integer::from(5)),
                            1,
                        )?,
                    common::types::integer::from(32),
                    decode::helpers::add_int_0(
                        common::types::integer::from(0),
                        common::types::integer::from(1),
                    )?,
                    common::types::integer::from(32),
                )?,
                common::types::integer::from(32),
                decode::helpers::mul_int_0(
                    decode::helpers::add_int_0(
                        common::types::integer::from(0),
                        common::types::integer::from(1),
                    )?,
                    common::types::integer::from(32),
                )?,
            )?,
            common::types::integer::from(1),
            decode::helpers::add_int_0(
                common::types::integer::from(32),
                decode::helpers::mul_int_0(
                    decode::helpers::add_int_0(
                        common::types::integer::from(0),
                        common::types::integer::from(1),
                    )?,
                    common::types::integer::from(32),
                )?,
            )?,
            common::types::integer::from(1),
        )?,
        decode::helpers::mul_int_0(
            decode::helpers::add_int_0(
                common::types::integer::from(32),
                decode::helpers::mul_int_0(
                    decode::helpers::add_int_0(
                        common::types::integer::from(0),
                        common::types::integer::from(1),
                    )?,
                    common::types::integer::from(32),
                )?,
            )?,
            common::types::integer::from(1),
        )?,
    )?);
    wmask_and = decode::helpers::or_bits_0(
        immr,
        decode::helpers::not_bits_0((levels), common::types::integer::from(6))?,
        common::types::integer::from(6),
    )?;
    wmask_or = decode::helpers::and_bits_0(
        immr,
        levels,
        common::types::integer::from(6),
    )?;
    wmask = decode::helpers::Zeros_0(
        common::types::integer::from(64),
        common::types::integer::from(64),
    )?;
    wmask = (decode::helpers::or_bits_0(
        (decode::helpers::and_bits_0(
            wmask,
            decode::helpers::Replicate_0(
                decode::helpers::append_bits_0(
                    decode::helpers::Ones_0(
                        common::types::integer::from(1),
                        common::types::integer::from(1),
                    )?,
                    decode::helpers::Replicate_0(
                        wmask_and
                            .extract_slice(
                                integer_to_usize!(common::types::integer::from(0)),
                                1,
                            )?,
                        common::types::integer::from(1),
                        decode::helpers::add_int_0(
                            common::types::integer::from(0),
                            common::types::integer::from(1),
                        )?,
                        common::types::integer::from(1),
                    )?,
                    common::types::integer::from(1),
                    decode::helpers::mul_int_0(
                        decode::helpers::add_int_0(
                            common::types::integer::from(0),
                            common::types::integer::from(1),
                        )?,
                        common::types::integer::from(1),
                    )?,
                )?,
                common::types::integer::from(32),
                decode::helpers::add_int_0(
                    common::types::integer::from(1),
                    decode::helpers::mul_int_0(
                        decode::helpers::add_int_0(
                            common::types::integer::from(0),
                            common::types::integer::from(1),
                        )?,
                        common::types::integer::from(1),
                    )?,
                )?,
                common::types::integer::from(32),
            )?,
            decode::helpers::mul_int_0(
                decode::helpers::add_int_0(
                    decode::helpers::mul_int_0(
                        decode::helpers::add_int_0(
                            common::types::integer::from(0),
                            common::types::integer::from(1),
                        )?,
                        common::types::integer::from(1),
                    )?,
                    common::types::integer::from(1),
                )?,
                common::types::integer::from(32),
            )?,
        )?),
        decode::helpers::Replicate_0(
            decode::helpers::append_bits_0(
                decode::helpers::Replicate_0(
                    wmask_or
                        .extract_slice(
                            integer_to_usize!(common::types::integer::from(0)),
                            1,
                        )?,
                    common::types::integer::from(1),
                    decode::helpers::add_int_0(
                        common::types::integer::from(0),
                        common::types::integer::from(1),
                    )?,
                    common::types::integer::from(1),
                )?,
                decode::helpers::Zeros_0(
                    common::types::integer::from(1),
                    common::types::integer::from(1),
                )?,
                decode::helpers::mul_int_0(
                    decode::helpers::add_int_0(
                        common::types::integer::from(0),
                        common::types::integer::from(1),
                    )?,
                    common::types::integer::from(1),
                )?,
                common::types::integer::from(1),
            )?,
            common::types::integer::from(32),
            decode::helpers::add_int_0(
                decode::helpers::mul_int_0(
                    decode::helpers::add_int_0(
                        common::types::integer::from(0),
                        common::types::integer::from(1),
                    )?,
                    common::types::integer::from(1),
                )?,
                common::types::integer::from(1),
            )?,
            common::types::integer::from(32),
        )?,
        decode::helpers::mul_int_0(
            decode::helpers::add_int_0(
                decode::helpers::mul_int_0(
                    decode::helpers::add_int_0(
                        common::types::integer::from(0),
                        common::types::integer::from(1),
                    )?,
                    common::types::integer::from(1),
                )?,
                common::types::integer::from(1),
            )?,
            common::types::integer::from(32),
        )?,
    )?);
    wmask = (decode::helpers::or_bits_0(
        (decode::helpers::and_bits_0(
            wmask,
            decode::helpers::Replicate_0(
                decode::helpers::append_bits_0(
                    decode::helpers::Ones_0(
                        common::types::integer::from(2),
                        common::types::integer::from(2),
                    )?,
                    decode::helpers::Replicate_0(
                        wmask_and
                            .extract_slice(
                                integer_to_usize!(common::types::integer::from(1)),
                                1,
                            )?,
                        common::types::integer::from(2),
                        decode::helpers::add_int_0(
                            common::types::integer::from(0),
                            common::types::integer::from(1),
                        )?,
                        common::types::integer::from(2),
                    )?,
                    common::types::integer::from(2),
                    decode::helpers::mul_int_0(
                        decode::helpers::add_int_0(
                            common::types::integer::from(0),
                            common::types::integer::from(1),
                        )?,
                        common::types::integer::from(2),
                    )?,
                )?,
                common::types::integer::from(16),
                decode::helpers::add_int_0(
                    common::types::integer::from(2),
                    decode::helpers::mul_int_0(
                        decode::helpers::add_int_0(
                            common::types::integer::from(0),
                            common::types::integer::from(1),
                        )?,
                        common::types::integer::from(2),
                    )?,
                )?,
                common::types::integer::from(16),
            )?,
            decode::helpers::mul_int_0(
                decode::helpers::add_int_0(
                    decode::helpers::mul_int_0(
                        decode::helpers::add_int_0(
                            common::types::integer::from(0),
                            common::types::integer::from(1),
                        )?,
                        common::types::integer::from(2),
                    )?,
                    common::types::integer::from(2),
                )?,
                common::types::integer::from(16),
            )?,
        )?),
        decode::helpers::Replicate_0(
            decode::helpers::append_bits_0(
                decode::helpers::Replicate_0(
                    wmask_or
                        .extract_slice(
                            integer_to_usize!(common::types::integer::from(1)),
                            1,
                        )?,
                    common::types::integer::from(2),
                    decode::helpers::add_int_0(
                        common::types::integer::from(0),
                        common::types::integer::from(1),
                    )?,
                    common::types::integer::from(2),
                )?,
                decode::helpers::Zeros_0(
                    common::types::integer::from(2),
                    common::types::integer::from(2),
                )?,
                decode::helpers::mul_int_0(
                    decode::helpers::add_int_0(
                        common::types::integer::from(0),
                        common::types::integer::from(1),
                    )?,
                    common::types::integer::from(2),
                )?,
                common::types::integer::from(2),
            )?,
            common::types::integer::from(16),
            decode::helpers::add_int_0(
                decode::helpers::mul_int_0(
                    decode::helpers::add_int_0(
                        common::types::integer::from(0),
                        common::types::integer::from(1),
                    )?,
                    common::types::integer::from(2),
                )?,
                common::types::integer::from(2),
            )?,
            common::types::integer::from(16),
        )?,
        decode::helpers::mul_int_0(
            decode::helpers::add_int_0(
                decode::helpers::mul_int_0(
                    decode::helpers::add_int_0(
                        common::types::integer::from(0),
                        common::types::integer::from(1),
                    )?,
                    common::types::integer::from(2),
                )?,
                common::types::integer::from(2),
            )?,
            common::types::integer::from(16),
        )?,
    )?);
    wmask = (decode::helpers::or_bits_0(
        (decode::helpers::and_bits_0(
            wmask,
            decode::helpers::Replicate_0(
                decode::helpers::append_bits_0(
                    decode::helpers::Ones_0(
                        common::types::integer::from(4),
                        common::types::integer::from(4),
                    )?,
                    decode::helpers::Replicate_0(
                        wmask_and
                            .extract_slice(
                                integer_to_usize!(common::types::integer::from(2)),
                                1,
                            )?,
                        common::types::integer::from(4),
                        decode::helpers::add_int_0(
                            common::types::integer::from(0),
                            common::types::integer::from(1),
                        )?,
                        common::types::integer::from(4),
                    )?,
                    common::types::integer::from(4),
                    decode::helpers::mul_int_0(
                        decode::helpers::add_int_0(
                            common::types::integer::from(0),
                            common::types::integer::from(1),
                        )?,
                        common::types::integer::from(4),
                    )?,
                )?,
                common::types::integer::from(8),
                decode::helpers::add_int_0(
                    common::types::integer::from(4),
                    decode::helpers::mul_int_0(
                        decode::helpers::add_int_0(
                            common::types::integer::from(0),
                            common::types::integer::from(1),
                        )?,
                        common::types::integer::from(4),
                    )?,
                )?,
                common::types::integer::from(8),
            )?,
            decode::helpers::mul_int_0(
                decode::helpers::add_int_0(
                    decode::helpers::mul_int_0(
                        decode::helpers::add_int_0(
                            common::types::integer::from(0),
                            common::types::integer::from(1),
                        )?,
                        common::types::integer::from(4),
                    )?,
                    common::types::integer::from(4),
                )?,
                common::types::integer::from(8),
            )?,
        )?),
        decode::helpers::Replicate_0(
            decode::helpers::append_bits_0(
                decode::helpers::Replicate_0(
                    wmask_or
                        .extract_slice(
                            integer_to_usize!(common::types::integer::from(2)),
                            1,
                        )?,
                    common::types::integer::from(4),
                    decode::helpers::add_int_0(
                        common::types::integer::from(0),
                        common::types::integer::from(1),
                    )?,
                    common::types::integer::from(4),
                )?,
                decode::helpers::Zeros_0(
                    common::types::integer::from(4),
                    common::types::integer::from(4),
                )?,
                decode::helpers::mul_int_0(
                    decode::helpers::add_int_0(
                        common::types::integer::from(0),
                        common::types::integer::from(1),
                    )?,
                    common::types::integer::from(4),
                )?,
                common::types::integer::from(4),
            )?,
            common::types::integer::from(8),
            decode::helpers::add_int_0(
                decode::helpers::mul_int_0(
                    decode::helpers::add_int_0(
                        common::types::integer::from(0),
                        common::types::integer::from(1),
                    )?,
                    common::types::integer::from(4),
                )?,
                common::types::integer::from(4),
            )?,
            common::types::integer::from(8),
        )?,
        decode::helpers::mul_int_0(
            decode::helpers::add_int_0(
                decode::helpers::mul_int_0(
                    decode::helpers::add_int_0(
                        common::types::integer::from(0),
                        common::types::integer::from(1),
                    )?,
                    common::types::integer::from(4),
                )?,
                common::types::integer::from(4),
            )?,
            common::types::integer::from(8),
        )?,
    )?);
    wmask = (decode::helpers::or_bits_0(
        (decode::helpers::and_bits_0(
            wmask,
            decode::helpers::Replicate_0(
                decode::helpers::append_bits_0(
                    decode::helpers::Ones_0(
                        common::types::integer::from(8),
                        common::types::integer::from(8),
                    )?,
                    decode::helpers::Replicate_0(
                        wmask_and
                            .extract_slice(
                                integer_to_usize!(common::types::integer::from(3)),
                                1,
                            )?,
                        common::types::integer::from(8),
                        decode::helpers::add_int_0(
                            common::types::integer::from(0),
                            common::types::integer::from(1),
                        )?,
                        common::types::integer::from(8),
                    )?,
                    common::types::integer::from(8),
                    decode::helpers::mul_int_0(
                        decode::helpers::add_int_0(
                            common::types::integer::from(0),
                            common::types::integer::from(1),
                        )?,
                        common::types::integer::from(8),
                    )?,
                )?,
                common::types::integer::from(4),
                decode::helpers::add_int_0(
                    common::types::integer::from(8),
                    decode::helpers::mul_int_0(
                        decode::helpers::add_int_0(
                            common::types::integer::from(0),
                            common::types::integer::from(1),
                        )?,
                        common::types::integer::from(8),
                    )?,
                )?,
                common::types::integer::from(4),
            )?,
            decode::helpers::mul_int_0(
                decode::helpers::add_int_0(
                    decode::helpers::mul_int_0(
                        decode::helpers::add_int_0(
                            common::types::integer::from(0),
                            common::types::integer::from(1),
                        )?,
                        common::types::integer::from(8),
                    )?,
                    common::types::integer::from(8),
                )?,
                common::types::integer::from(4),
            )?,
        )?),
        decode::helpers::Replicate_0(
            decode::helpers::append_bits_0(
                decode::helpers::Replicate_0(
                    wmask_or
                        .extract_slice(
                            integer_to_usize!(common::types::integer::from(3)),
                            1,
                        )?,
                    common::types::integer::from(8),
                    decode::helpers::add_int_0(
                        common::types::integer::from(0),
                        common::types::integer::from(1),
                    )?,
                    common::types::integer::from(8),
                )?,
                decode::helpers::Zeros_0(
                    common::types::integer::from(8),
                    common::types::integer::from(8),
                )?,
                decode::helpers::mul_int_0(
                    decode::helpers::add_int_0(
                        common::types::integer::from(0),
                        common::types::integer::from(1),
                    )?,
                    common::types::integer::from(8),
                )?,
                common::types::integer::from(8),
            )?,
            common::types::integer::from(4),
            decode::helpers::add_int_0(
                decode::helpers::mul_int_0(
                    decode::helpers::add_int_0(
                        common::types::integer::from(0),
                        common::types::integer::from(1),
                    )?,
                    common::types::integer::from(8),
                )?,
                common::types::integer::from(8),
            )?,
            common::types::integer::from(4),
        )?,
        decode::helpers::mul_int_0(
            decode::helpers::add_int_0(
                decode::helpers::mul_int_0(
                    decode::helpers::add_int_0(
                        common::types::integer::from(0),
                        common::types::integer::from(1),
                    )?,
                    common::types::integer::from(8),
                )?,
                common::types::integer::from(8),
            )?,
            common::types::integer::from(4),
        )?,
    )?);
    wmask = (decode::helpers::or_bits_0(
        (decode::helpers::and_bits_0(
            wmask,
            decode::helpers::Replicate_0(
                decode::helpers::append_bits_0(
                    decode::helpers::Ones_0(
                        common::types::integer::from(16),
                        common::types::integer::from(16),
                    )?,
                    decode::helpers::Replicate_0(
                        wmask_and
                            .extract_slice(
                                integer_to_usize!(common::types::integer::from(4)),
                                1,
                            )?,
                        common::types::integer::from(16),
                        decode::helpers::add_int_0(
                            common::types::integer::from(0),
                            common::types::integer::from(1),
                        )?,
                        common::types::integer::from(16),
                    )?,
                    common::types::integer::from(16),
                    decode::helpers::mul_int_0(
                        decode::helpers::add_int_0(
                            common::types::integer::from(0),
                            common::types::integer::from(1),
                        )?,
                        common::types::integer::from(16),
                    )?,
                )?,
                common::types::integer::from(2),
                decode::helpers::add_int_0(
                    common::types::integer::from(16),
                    decode::helpers::mul_int_0(
                        decode::helpers::add_int_0(
                            common::types::integer::from(0),
                            common::types::integer::from(1),
                        )?,
                        common::types::integer::from(16),
                    )?,
                )?,
                common::types::integer::from(2),
            )?,
            decode::helpers::mul_int_0(
                decode::helpers::add_int_0(
                    decode::helpers::mul_int_0(
                        decode::helpers::add_int_0(
                            common::types::integer::from(0),
                            common::types::integer::from(1),
                        )?,
                        common::types::integer::from(16),
                    )?,
                    common::types::integer::from(16),
                )?,
                common::types::integer::from(2),
            )?,
        )?),
        decode::helpers::Replicate_0(
            decode::helpers::append_bits_0(
                decode::helpers::Replicate_0(
                    wmask_or
                        .extract_slice(
                            integer_to_usize!(common::types::integer::from(4)),
                            1,
                        )?,
                    common::types::integer::from(16),
                    decode::helpers::add_int_0(
                        common::types::integer::from(0),
                        common::types::integer::from(1),
                    )?,
                    common::types::integer::from(16),
                )?,
                decode::helpers::Zeros_0(
                    common::types::integer::from(16),
                    common::types::integer::from(16),
                )?,
                decode::helpers::mul_int_0(
                    decode::helpers::add_int_0(
                        common::types::integer::from(0),
                        common::types::integer::from(1),
                    )?,
                    common::types::integer::from(16),
                )?,
                common::types::integer::from(16),
            )?,
            common::types::integer::from(2),
            decode::helpers::add_int_0(
                decode::helpers::mul_int_0(
                    decode::helpers::add_int_0(
                        common::types::integer::from(0),
                        common::types::integer::from(1),
                    )?,
                    common::types::integer::from(16),
                )?,
                common::types::integer::from(16),
            )?,
            common::types::integer::from(2),
        )?,
        decode::helpers::mul_int_0(
            decode::helpers::add_int_0(
                decode::helpers::mul_int_0(
                    decode::helpers::add_int_0(
                        common::types::integer::from(0),
                        common::types::integer::from(1),
                    )?,
                    common::types::integer::from(16),
                )?,
                common::types::integer::from(16),
            )?,
            common::types::integer::from(2),
        )?,
    )?);
    wmask = (decode::helpers::or_bits_0(
        (decode::helpers::and_bits_0(
            wmask,
            decode::helpers::Replicate_0(
                decode::helpers::append_bits_0(
                    decode::helpers::Ones_0(
                        common::types::integer::from(32),
                        common::types::integer::from(32),
                    )?,
                    decode::helpers::Replicate_0(
                        wmask_and
                            .extract_slice(
                                integer_to_usize!(common::types::integer::from(5)),
                                1,
                            )?,
                        common::types::integer::from(32),
                        decode::helpers::add_int_0(
                            common::types::integer::from(0),
                            common::types::integer::from(1),
                        )?,
                        common::types::integer::from(32),
                    )?,
                    common::types::integer::from(32),
                    decode::helpers::mul_int_0(
                        decode::helpers::add_int_0(
                            common::types::integer::from(0),
                            common::types::integer::from(1),
                        )?,
                        common::types::integer::from(32),
                    )?,
                )?,
                common::types::integer::from(1),
                decode::helpers::add_int_0(
                    common::types::integer::from(32),
                    decode::helpers::mul_int_0(
                        decode::helpers::add_int_0(
                            common::types::integer::from(0),
                            common::types::integer::from(1),
                        )?,
                        common::types::integer::from(32),
                    )?,
                )?,
                common::types::integer::from(1),
            )?,
            decode::helpers::mul_int_0(
                decode::helpers::add_int_0(
                    decode::helpers::mul_int_0(
                        decode::helpers::add_int_0(
                            common::types::integer::from(0),
                            common::types::integer::from(1),
                        )?,
                        common::types::integer::from(32),
                    )?,
                    common::types::integer::from(32),
                )?,
                common::types::integer::from(1),
            )?,
        )?),
        decode::helpers::Replicate_0(
            decode::helpers::append_bits_0(
                decode::helpers::Replicate_0(
                    wmask_or
                        .extract_slice(
                            integer_to_usize!(common::types::integer::from(5)),
                            1,
                        )?,
                    common::types::integer::from(32),
                    decode::helpers::add_int_0(
                        common::types::integer::from(0),
                        common::types::integer::from(1),
                    )?,
                    common::types::integer::from(32),
                )?,
                decode::helpers::Zeros_0(
                    common::types::integer::from(32),
                    common::types::integer::from(32),
                )?,
                decode::helpers::mul_int_0(
                    decode::helpers::add_int_0(
                        common::types::integer::from(0),
                        common::types::integer::from(1),
                    )?,
                    common::types::integer::from(32),
                )?,
                common::types::integer::from(32),
            )?,
            common::types::integer::from(1),
            decode::helpers::add_int_0(
                decode::helpers::mul_int_0(
                    decode::helpers::add_int_0(
                        common::types::integer::from(0),
                        common::types::integer::from(1),
                    )?,
                    common::types::integer::from(32),
                )?,
                common::types::integer::from(32),
            )?,
            common::types::integer::from(1),
        )?,
        decode::helpers::mul_int_0(
            decode::helpers::add_int_0(
                decode::helpers::mul_int_0(
                    decode::helpers::add_int_0(
                        common::types::integer::from(0),
                        common::types::integer::from(1),
                    )?,
                    common::types::integer::from(32),
                )?,
                common::types::integer::from(32),
            )?,
            common::types::integer::from(1),
        )?,
    )?);
    if (decode::helpers::ne_bits_0(
        diff.extract_slice(integer_to_usize!(common::types::integer::from(6)), 1)?,
        common::types::bits::from_bits_literal("0")?,
        decode::helpers::add_int_0(
            common::types::integer::from(0),
            common::types::integer::from(1),
        )?,
    )?) == common::types::boolean::TRUE
    {
        wmask = decode::helpers::and_bits_0(
            wmask,
            tmask,
            common::types::integer::from(64),
        )?;
    } else {
        wmask = decode::helpers::or_bits_0(
            wmask,
            tmask,
            common::types::integer::from(64),
        )?;
    }
    return Ok((
        wmask
            .extract_slice(
                integer_to_usize!(common::types::integer::from(0)),
                integer_to_usize!(
                    decode::helpers::sub_int_0(M.clone(),
                    common::types::integer::from(1)) ?
                ) + 1 - integer_to_usize!(common::types::integer::from(0)),
            )?,
        tmask
            .extract_slice(
                integer_to_usize!(common::types::integer::from(0)),
                integer_to_usize!(
                    decode::helpers::sub_int_0(M.clone(),
                    common::types::integer::from(1)) ?
                ) + 1 - integer_to_usize!(common::types::integer::from(0)),
            )?,
    ));
    return Err(AArch64LifterError::NothingToReturn("DecodeBitMasks_0".to_string()));
}
pub fn DecodeRegExtend_0(
    op: common::types::bits,
) -> Result<common::types::ExtendType, AArch64LifterError> {
    if (op.match_with_pattern("000")) && true {
        return Ok(common::types::ExtendType::ExtendType_UXTB);
    } else if (op.match_with_pattern("001")) && true {
        return Ok(common::types::ExtendType::ExtendType_UXTH);
    } else if (op.match_with_pattern("010")) && true {
        return Ok(common::types::ExtendType::ExtendType_UXTW);
    } else if (op.match_with_pattern("011")) && true {
        return Ok(common::types::ExtendType::ExtendType_UXTX);
    } else if (op.match_with_pattern("100")) && true {
        return Ok(common::types::ExtendType::ExtendType_SXTB);
    } else if (op.match_with_pattern("101")) && true {
        return Ok(common::types::ExtendType::ExtendType_SXTH);
    } else if (op.match_with_pattern("110")) && true {
        return Ok(common::types::ExtendType::ExtendType_SXTW);
    } else if (op.match_with_pattern("111")) && true {
        return Ok(common::types::ExtendType::ExtendType_SXTX);
    }
    return Err(AArch64LifterError::NothingToReturn("DecodeRegExtend_0".to_string()));
}
pub fn DecodeShift_0(
    op: common::types::bits,
) -> Result<common::types::ShiftType, AArch64LifterError> {
    if (op.match_with_pattern("00")) && true {
        return Ok(common::types::ShiftType::ShiftType_LSL);
    } else if (op.match_with_pattern("01")) && true {
        return Ok(common::types::ShiftType::ShiftType_LSR);
    } else if (op.match_with_pattern("10")) && true {
        return Ok(common::types::ShiftType::ShiftType_ASR);
    } else if (op.match_with_pattern("11")) && true {
        return Ok(common::types::ShiftType::ShiftType_ROR);
    }
    return Err(AArch64LifterError::NothingToReturn("DecodeShift_0".to_string()));
}
pub fn HasArchVersion_0(
    version: common::types::ArchVersion,
) -> Result<common::types::boolean, AArch64LifterError> {
    return Ok(
        decode::helpers::or_bool_0(
            decode::helpers::eq_enum_8(version, common::types::ArchVersion::ARMv8p0)?,
            common::types::IMPLEMENTATION_DEFINED_boolean,
        )?,
    );
    return Err(AArch64LifterError::NothingToReturn("HasArchVersion_0".to_string()));
}
pub fn HavePACExt_0() -> Result<common::types::boolean, AArch64LifterError> {
    return Ok(decode::helpers::HasArchVersion_0(common::types::ArchVersion::ARMv8p3)?);
    return Err(AArch64LifterError::NothingToReturn("HavePACExt_0".to_string()));
}
pub fn HighestSetBit_0(
    x: common::types::bits,
    N: common::types::integer,
) -> Result<common::types::integer, AArch64LifterError> {
    let mut i = decode::helpers::sub_int_0(N.clone(), common::types::integer::from(1))?;
    let start = decode::helpers::sub_int_0(N.clone(), common::types::integer::from(1))?;
    let end = common::types::integer::from(0);
    while i >= common::types::integer::from(0) {
        if (decode::helpers::eq_bits_0(
            x.extract_slice(integer_to_usize!(i), 1)?,
            common::types::bits::from_bits_literal("1")?,
            decode::helpers::add_int_0(
                common::types::integer::from(0),
                common::types::integer::from(1),
            )?,
        )?) == common::types::boolean::TRUE
        {
            return Ok(i);
        } else {}
        i = i - common::types::integer::one();
    }
    return Ok(decode::helpers::neg_int_0(common::types::integer::from(1))?);
    return Err(AArch64LifterError::NothingToReturn("HighestSetBit_0".to_string()));
}
pub fn LSL_0(
    x: common::types::bits,
    shift: common::types::integer,
    N: common::types::integer,
) -> Result<common::types::bits, AArch64LifterError> {
    let mut result: common::types::bits = common::types::bits::new(
        0,
        integer_to_usize!(N),
    );
    assert_eq!(
        decode::helpers::ge_int_0(shift.clone(), common::types::integer::from(0)) ?,
        common::types::boolean::TRUE
    );
    if (decode::helpers::eq_int_0(shift.clone(), common::types::integer::from(0))?)
        == common::types::boolean::TRUE
    {
        result = x;
    } else {
        (result, _) = decode::helpers::LSL_C_0(x, shift.clone(), N.clone())?;
    }
    return Ok(result);
    return Err(AArch64LifterError::NothingToReturn("LSL_0".to_string()));
}
pub fn LSL_C_0(
    x: common::types::bits,
    shift: common::types::integer,
    N: common::types::integer,
) -> Result<(common::types::bits, common::types::bits), AArch64LifterError> {
    let mut carry_out: common::types::bits = common::types::bits::new(
        0,
        integer_to_usize!(
            decode::helpers::add_int_0(common::types::integer::from(0),
            common::types::integer::from(1)) ?
        ),
    );
    let mut extended_x: common::types::bits = common::types::bits::new(
        0,
        integer_to_usize!(decode::helpers::add_int_0(N.clone(), shift.clone()) ?),
    );
    let mut result: common::types::bits = common::types::bits::new(
        0,
        integer_to_usize!(
            decode::helpers::add_int_0(common::types::integer::from(0),
            decode::helpers::add_int_0(decode::helpers::sub_int_0(decode::helpers::sub_int_0(N
            .clone(), common::types::integer::from(1)) ?,
            common::types::integer::from(0)) ?, common::types::integer::from(1)) ?) ?
        ),
    );
    assert_eq!(
        decode::helpers::gt_int_0(shift.clone(), common::types::integer::from(0)) ?,
        common::types::boolean::TRUE
    );
    extended_x = decode::helpers::append_bits_0(
        x,
        decode::helpers::Zeros_0(shift.clone(), shift.clone())?,
        N.clone(),
        shift.clone(),
    )?;
    result = extended_x
        .extract_slice(
            integer_to_usize!(common::types::integer::from(0)),
            integer_to_usize!(
                decode::helpers::sub_int_0(N.clone(), common::types::integer::from(1)) ?
            ) + 1 - integer_to_usize!(common::types::integer::from(0)),
        )?;
    carry_out = extended_x.extract_slice(integer_to_usize!(N), 1)?;
    return Ok((result, carry_out));
    return Err(AArch64LifterError::NothingToReturn("LSL_C_0".to_string()));
}
pub fn Ones_0(
    N: common::types::integer,
    _: common::types::integer,
) -> Result<common::types::bits, AArch64LifterError> {
    return Ok(decode::helpers::ones_bits_0(N.clone())?);
    return Err(AArch64LifterError::NothingToReturn("Ones_0".to_string()));
}
pub fn Replicate_0(
    x: common::types::bits,
    N: common::types::integer,
    M: common::types::integer,
    _: common::types::integer,
) -> Result<common::types::bits, AArch64LifterError> {
    return Ok(decode::helpers::replicate_bits_0(x, N.clone(), M.clone(), N.clone())?);
    return Err(AArch64LifterError::NothingToReturn("Replicate_0".to_string()));
}
pub fn SignExtend_0(
    x: common::types::bits,
    N: common::types::integer,
    M: common::types::integer,
    _: common::types::integer,
) -> Result<common::types::bits, AArch64LifterError> {
    assert_eq!(
        decode::helpers::ge_int_0(N.clone(), M.clone()) ?, common::types::boolean::TRUE
    );
    return Ok(
        decode::helpers::append_bits_0(
            decode::helpers::Replicate_0(
                x
                    .extract_slice(
                        integer_to_usize!(
                            decode::helpers::sub_int_0(M.clone(),
                            common::types::integer::from(1)) ?
                        ),
                        1,
                    )?,
                decode::helpers::sub_int_0(N.clone(), M.clone())?,
                decode::helpers::add_int_0(
                    common::types::integer::from(0),
                    common::types::integer::from(1),
                )?,
                decode::helpers::sub_int_0(N.clone(), M.clone())?,
            )?,
            x,
            decode::helpers::mul_int_0(
                decode::helpers::add_int_0(
                    common::types::integer::from(0),
                    common::types::integer::from(1),
                )?,
                decode::helpers::sub_int_0(N.clone(), M.clone())?,
            )?,
            M.clone(),
        )?,
    );
    return Err(AArch64LifterError::NothingToReturn("SignExtend_0".to_string()));
}
pub fn UInt_1(
    x: common::types::bits,
    N: common::types::integer,
) -> Result<common::types::integer, AArch64LifterError> {
    return Ok(decode::helpers::cvt_bits_uint_0(x, N.clone())?);
    return Err(AArch64LifterError::NothingToReturn("UInt_1".to_string()));
}
pub fn Unreachable_0() -> Result<(), AArch64LifterError> {
    assert_eq!(common::types::boolean::FALSE, common::types::boolean::TRUE);
    return Err(AArch64LifterError::NothingToReturn("Unreachable_0".to_string()));
}
pub fn ZeroExtend_0(
    x: common::types::bits,
    N: common::types::integer,
    M: common::types::integer,
    _: common::types::integer,
) -> Result<common::types::bits, AArch64LifterError> {
    assert_eq!(
        decode::helpers::ge_int_0(N.clone(), M.clone()) ?, common::types::boolean::TRUE
    );
    return Ok(
        decode::helpers::append_bits_0(
            decode::helpers::Zeros_0(
                decode::helpers::sub_int_0(N.clone(), M.clone())?,
                decode::helpers::sub_int_0(N.clone(), M.clone())?,
            )?,
            x,
            decode::helpers::sub_int_0(N.clone(), M.clone())?,
            M.clone(),
        )?,
    );
    return Err(AArch64LifterError::NothingToReturn("ZeroExtend_0".to_string()));
}
pub fn Zeros_0(
    N: common::types::integer,
    _: common::types::integer,
) -> Result<common::types::bits, AArch64LifterError> {
    return Ok(decode::helpers::zeros_bits_0(N.clone())?);
    return Err(AArch64LifterError::NothingToReturn("Zeros_0".to_string()));
}
pub fn pow_int_int_0(
    x: common::types::integer,
    y: common::types::integer,
) -> Result<common::types::integer, AArch64LifterError> {
    if (decode::helpers::eq_int_0(x.clone(), common::types::integer::from(2))?)
        == common::types::boolean::TRUE
    {
        return Ok(decode::helpers::pow2_int_0(y.clone())?);
    } else {
        assert_eq!(
            decode::helpers::ge_int_0(y.clone(), common::types::integer::from(0)) ?,
            common::types::boolean::TRUE
        );
        let mut result: common::types::integer = common::types::integer::from(1);
        let mut i = common::types::integer::from(1);
        let start = common::types::integer::from(1);
        let end = y.clone();
        while i <= y.clone() {
            result = decode::helpers::mul_int_0(result.clone(), x.clone())?;
            i = i + common::types::integer::one();
        }
        return Ok(result);
    }
    return Err(AArch64LifterError::NothingToReturn("pow_int_int_0".to_string()));
}
pub fn shift_left_int_0(
    x: common::types::integer,
    y: common::types::integer,
) -> Result<common::types::integer, AArch64LifterError> {
    return Ok(
        if (decode::helpers::ge_int_0(y.clone(), common::types::integer::from(0))?)
            == common::types::boolean::TRUE
        {
            decode::helpers::shl_int_0(x.clone(), y.clone())?
        } else {
            decode::helpers::shr_int_0(
                x.clone(),
                decode::helpers::neg_int_0(y.clone())?,
            )?
        },
    );
    return Err(AArch64LifterError::NothingToReturn("shift_left_int_0".to_string()));
}

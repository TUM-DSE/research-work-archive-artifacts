#![allow(nonstandard_style, unused)]

use crate::arm64::common::types::{bits, boolean, integer, ArchVersion, BranchType, MemOp};
use crate::arm64::{common, AArch64LifterError};
use crate::integer_to_usize;
use num_bigint::BigInt;
use num_traits::{Pow, ToPrimitive, Zero};

pub use crate::arm64::decode::generated::helpers::*;

pub fn eq_enum_8(x: ArchVersion, y: ArchVersion) -> Result<boolean, AArch64LifterError> {
    Ok(if x == y { boolean::TRUE } else { boolean::FALSE })
}

pub fn ne_enum_8(x: ArchVersion, y: ArchVersion) -> Result<boolean, AArch64LifterError> {
    Ok(if x != y { boolean::TRUE } else { boolean::FALSE })
}

pub fn eq_enum_16(x: BranchType, y: BranchType) -> Result<boolean, AArch64LifterError> {
    Ok(if x == y { boolean::TRUE } else { boolean::FALSE })
}

pub fn ne_enum_16(x: BranchType, y: BranchType) -> Result<boolean, AArch64LifterError> {
    Ok(if x != y { boolean::TRUE } else { boolean::FALSE })
}

pub fn eq_enum_37(x: MemOp, y: MemOp) -> Result<boolean, AArch64LifterError> {
    Ok(if x == y { boolean::TRUE } else { boolean::FALSE })
}

pub fn ne_enum_37(x: MemOp, y: MemOp) -> Result<boolean, AArch64LifterError> {
    Ok(if x != y { boolean::TRUE } else { boolean::FALSE })
}

pub fn eq_bool_0(x: boolean, y: boolean) -> Result<boolean, AArch64LifterError> {
    common::helpers::eq_bool_0(x, y)
}

pub fn ne_bool_0(x: boolean, y: boolean) -> Result<boolean, AArch64LifterError> {
    common::helpers::ne_bool_0(x, y)
}

pub fn not_bool_0(x: boolean) -> Result<boolean, AArch64LifterError> {
    common::helpers::not_bool_0(x)
}

pub fn and_bool_0(x: boolean, y: boolean) -> Result<boolean, AArch64LifterError> {
    common::helpers::and_bool_0(x, y)
}

pub fn or_bool_0(x: boolean, y: boolean) -> Result<boolean, AArch64LifterError> {
    common::helpers::or_bool_0(x, y)
}

pub fn equiv_bool_0(x: boolean, y: boolean) -> Result<boolean, AArch64LifterError> {
    common::helpers::equiv_bool_0(x, y)
}

pub fn implies_bool_0(x: boolean, y: boolean) -> Result<boolean, AArch64LifterError> {
    common::helpers::implies_bool_0(x, y)
}

pub fn eq_int_0(x: integer, y: integer) -> Result<boolean, AArch64LifterError> {
    common::helpers::eq_int_0(x, y)
}

pub fn ne_int_0(x: integer, y: integer) -> Result<boolean, AArch64LifterError> {
    common::helpers::ne_int_0(x, y)
}

pub fn gt_int_0(x: integer, y: integer) -> Result<boolean, AArch64LifterError> {
    common::helpers::gt_int_0(x, y)
}

pub fn ge_int_0(x: integer, y: integer) -> Result<boolean, AArch64LifterError> {
    common::helpers::ge_int_0(x, y)
}

pub fn le_int_0(x: integer, y: integer) -> Result<boolean, AArch64LifterError> {
    common::helpers::le_int_0(x, y)
}

pub fn lt_int_0(x: integer, y: integer) -> Result<boolean, AArch64LifterError> {
    common::helpers::lt_int_0(x, y)
}

pub fn add_int_0(x: integer, y: integer) -> Result<integer, AArch64LifterError> {
    common::helpers::add_int_0(x, y)
}

pub fn neg_int_0(x: integer) -> Result<integer, AArch64LifterError> {
    common::helpers::neg_int_0(x)
}

pub fn sub_int_0(x: integer, y: integer) -> Result<integer, AArch64LifterError> {
    common::helpers::sub_int_0(x, y)
}

pub fn shl_int_0(x: integer, y: integer) -> Result<integer, AArch64LifterError> {
    common::helpers::shl_int_0(x, y)
}

pub fn shr_int_0(x: integer, y: integer) -> Result<integer, AArch64LifterError> {
    common::helpers::shr_int_0(x, y)
}

pub fn mul_int_0(x: integer, y: integer) -> Result<integer, AArch64LifterError> {
    common::helpers::mul_int_0(x, y)
}

pub fn zdiv_int_0(x: integer, y: integer) -> Result<integer, AArch64LifterError> {
    common::helpers::zdiv_int_0(x, y)
}

pub fn zrem_int_0(x: integer, y: integer) -> Result<integer, AArch64LifterError> {
    common::helpers::zrem_int_0(x, y)
}

pub fn pow2_int_0(y: integer) -> Result<integer, AArch64LifterError> {
    common::helpers::pow2_int_0(y)
}

pub fn cvt_bits_sint_0(x: bits, N: integer) -> Result<integer, AArch64LifterError> {
    common::helpers::cvt_bits_sint_0(x)
}

pub fn cvt_bits_uint_0(x: bits, N: integer) -> Result<integer, AArch64LifterError> {
    common::helpers::cvt_bits_uint_0(x)
}

pub fn eq_bits_0(x: bits, y: bits, N: integer) -> Result<boolean, AArch64LifterError> {
    common::helpers::eq_bits_0(x, y, N)
}

pub fn ne_bits_0(x: bits, y: bits, N: integer) -> Result<boolean, AArch64LifterError> {
    common::helpers::ne_bits_0(x, y, N)
}

pub fn add_bits_0(x: bits, y: bits, N: integer) -> Result<bits, AArch64LifterError> {
    common::helpers::add_bits_0(x, y, N)
}

pub fn sub_bits_0(x: bits, y: bits, N: integer) -> Result<bits, AArch64LifterError> {
    common::helpers::sub_bits_0(x, y, N)
}

pub fn mul_bits_0(x: bits, y: bits, N: integer) -> Result<bits, AArch64LifterError> {
    common::helpers::mul_bits_0(x, y, N)
}

pub fn and_bits_0(x: bits, y: bits, N: integer) -> Result<bits, AArch64LifterError> {
    common::helpers::and_bits_0(x, y, N)
}

pub fn or_bits_0(x: bits, y: bits, N: integer) -> Result<bits, AArch64LifterError> {
    common::helpers::or_bits_0(x, y, N)
}

pub fn eor_bits_0(x: bits, y: bits, N: integer) -> Result<bits, AArch64LifterError> {
    common::helpers::eor_bits_0(x, y, N)
}

pub fn not_bits_0(x: bits, N: integer) -> Result<bits, AArch64LifterError> {
    common::helpers::not_bits_0(x)
}

pub fn zeros_bits_0(N: integer) -> Result<bits, AArch64LifterError> {
    common::helpers::zeros_bits_0(N)
}

pub fn ones_bits_0(N: integer) -> Result<bits, AArch64LifterError> {
    common::helpers::ones_bits_0(N)
}

pub fn replicate_bits_0(x: bits, N: integer, M: integer, _N: integer) -> Result<bits, AArch64LifterError> {
    common::helpers::replicate_bits_0(x, N)
}

pub fn append_bits_0(x: bits, y: bits, M: integer, N: integer) -> Result<bits, AArch64LifterError> {
    common::helpers::append_bits_0(x, y)
}

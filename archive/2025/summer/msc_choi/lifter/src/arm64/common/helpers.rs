#![allow(nonstandard_style)]

use crate::arm64::common::types::{bits, boolean, integer};
use crate::arm64::AArch64LifterError;
use crate::{integer_to_i128, integer_to_usize};
use num_traits::{One, Pow, ToPrimitive, Zero};

pub fn eq_bool_0(x: boolean, y: boolean) -> Result<boolean, AArch64LifterError> {
    if x == y {
        Ok(boolean::TRUE)
    } else {
        Ok(boolean::FALSE)
    }
}

pub fn ne_bool_0(x: boolean, y: boolean) -> Result<boolean, AArch64LifterError> {
    if x != y {
        Ok(boolean::TRUE)
    } else {
        Ok(boolean::FALSE)
    }
}

pub fn not_bool_0(x: boolean) -> Result<boolean, AArch64LifterError> {
    if x == boolean::TRUE {
        Ok(boolean::FALSE)
    } else {
        Ok(boolean::TRUE)
    }
}

pub fn and_bool_0(x: boolean, y: boolean) -> Result<boolean, AArch64LifterError> {
    let x_val = x == boolean::TRUE;
    let y_val = y == boolean::TRUE;

    if x_val && y_val {
        Ok(boolean::TRUE)
    } else {
        Ok(boolean::FALSE)
    }
}

pub fn or_bool_0(x: boolean, y: boolean) -> Result<boolean, AArch64LifterError> {
    let x_val = x == boolean::TRUE;
    let y_val = y == boolean::TRUE;

    if x_val || y_val {
        Ok(boolean::TRUE)
    } else {
        Ok(boolean::FALSE)
    }
}

pub fn equiv_bool_0(x: boolean, y: boolean) -> Result<boolean, AArch64LifterError> {
    Ok(eq_bool_0(x, y)?)
}

pub fn implies_bool_0(x: boolean, y: boolean) -> Result<boolean, AArch64LifterError> {
    let x_val = x == boolean::TRUE;
    let y_val = y == boolean::TRUE;

    if !x_val || y_val {
        Ok(boolean::TRUE)
    } else {
        Ok(boolean::FALSE)
    }
}

pub fn eq_int_0(x: integer, y: integer) -> Result<boolean, AArch64LifterError> {
    if x == y {
        Ok(boolean::TRUE)
    } else {
        Ok(boolean::FALSE)
    }
}

pub fn ne_int_0(x: integer, y: integer) -> Result<boolean, AArch64LifterError> {
    if x != y {
        Ok(boolean::TRUE)
    } else {
        Ok(boolean::FALSE)
    }
}

pub fn gt_int_0(x: integer, y: integer) -> Result<boolean, AArch64LifterError> {
    if x > y {
        Ok(boolean::TRUE)
    } else {
        Ok(boolean::FALSE)
    }
}

pub fn ge_int_0(x: integer, y: integer) -> Result<boolean, AArch64LifterError> {
    if x >= y {
        Ok(boolean::TRUE)
    } else {
        Ok(boolean::FALSE)
    }
}

pub fn le_int_0(x: integer, y: integer) -> Result<boolean, AArch64LifterError> {
    if x <= y {
        Ok(boolean::TRUE)
    } else {
        Ok(boolean::FALSE)
    }
}

pub fn lt_int_0(x: integer, y: integer) -> Result<boolean, AArch64LifterError> {
    if x < y {
        Ok(boolean::TRUE)
    } else {
        Ok(boolean::FALSE)
    }
}

pub fn add_int_0(x: integer, y: integer) -> Result<integer, AArch64LifterError> {
    Ok(x + y)
}

pub fn neg_int_0(x: integer) -> Result<integer, AArch64LifterError> {
    Ok(-x)
}

pub fn sub_int_0(x: integer, y: integer) -> Result<integer, AArch64LifterError> {
    Ok(x - y)
}

pub fn shl_int_0(x: integer, y: integer) -> Result<integer, AArch64LifterError> {
    let y_val = y.to_u128().ok_or(AArch64LifterError::IntegerTooBig)?;

    Ok(x << y_val)
}

pub fn shr_int_0(x: integer, y: integer) -> Result<integer, AArch64LifterError> {
    let y_val = y.to_u128().ok_or(AArch64LifterError::IntegerTooBig)?;

    Ok(x >> y_val)
}

pub fn mul_int_0(x: integer, y: integer) -> Result<integer, AArch64LifterError> {
    Ok(x * y)
}

pub fn zdiv_int_0(x: integer, y: integer) -> Result<integer, AArch64LifterError> {
    Ok(x / y)
}

pub fn zrem_int_0(x: integer, y: integer) -> Result<integer, AArch64LifterError> {
    Ok(x % y)
}

pub fn fdiv_int_0(x: integer, y: integer) -> Result<integer, AArch64LifterError> {
    let q = &x / &y;
    if &x % &y != integer::zero() && ((&x < &integer::zero()) != (&y < &integer::zero())) {
        Ok(q - integer::one())
    } else {
        Ok(q)
    }
}

pub fn frem_int_0(x: integer, y: integer) -> Result<integer, AArch64LifterError> {
    Ok(x.clone() - fdiv_int_0(x, y.clone())? * y)
}

pub fn align_int_0(x: integer, y: integer) -> Result<integer, AArch64LifterError> {
    Ok(x.clone() - (x % y))
}

pub fn pow2_int_0(y: integer) -> Result<integer, AArch64LifterError> {
    assert!(y >= integer::zero());

    let y_val = y.to_u128().ok_or(AArch64LifterError::IntegerTooBig)?;

    Ok(integer::from(2).pow(y_val))
}

pub fn cvt_int_bits_0(x: integer, N: integer) -> Result<bits, AArch64LifterError> {
    Ok(bits::new(integer_to_i128!(x) as u128, integer_to_usize!(N)).truncate())
}

pub fn cvt_bits_sint_0(x: bits) -> Result<integer, AArch64LifterError> {
    let shift = 64 - x.length;
    let value = (x.value << shift) as i64 >> shift;

    Ok(integer::from(value))
}

pub fn cvt_bits_uint_0(x: bits) -> Result<integer, AArch64LifterError> {
    Ok(integer::from(x.value))
}

pub fn eq_bits_0(x: bits, y: bits, N: integer) -> Result<boolean, AArch64LifterError> {
    assert_eq!(x.length, integer_to_usize!(N));
    assert_eq!(y.length, integer_to_usize!(N));

    if x.value == y.value {
        Ok(boolean::TRUE)
    } else {
        Ok(boolean::FALSE)
    }
}

pub fn ne_bits_0(x: bits, y: bits, N: integer) -> Result<boolean, AArch64LifterError> {
    assert_eq!(x.length, integer_to_usize!(N));
    assert_eq!(y.length, integer_to_usize!(N));

    if x.value != y.value {
        Ok(boolean::TRUE)
    } else {
        Ok(boolean::FALSE)
    }
}

pub fn add_bits_0(x: bits, y: bits, N: integer) -> Result<bits, AArch64LifterError> {
    assert_eq!(x.length, integer_to_usize!(N));
    assert_eq!(y.length, integer_to_usize!(N));

    Ok(bits::new(x.value + y.value, x.length).truncate())
}

pub fn sub_bits_0(x: bits, y: bits, N: integer) -> Result<bits, AArch64LifterError> {
    assert_eq!(x.length, integer_to_usize!(N));
    assert_eq!(y.length, integer_to_usize!(N));

    Ok(bits::new(x.value - y.value, x.length).truncate())
}

pub fn mul_bits_0(x: bits, y: bits, N: integer) -> Result<bits, AArch64LifterError> {
    assert_eq!(x.length, integer_to_usize!(N));
    assert_eq!(y.length, integer_to_usize!(N));

    Ok(bits::new(x.value * y.value, x.length).truncate())
}

pub fn and_bits_0(x: bits, y: bits, N: integer) -> Result<bits, AArch64LifterError> {
    assert_eq!(x.length, integer_to_usize!(N));
    assert_eq!(y.length, integer_to_usize!(N));

    Ok(bits::new(x.value & y.value, x.length))
}

pub fn or_bits_0(x: bits, y: bits, N: integer) -> Result<bits, AArch64LifterError> {
    assert_eq!(x.length, integer_to_usize!(N));
    assert_eq!(y.length, integer_to_usize!(N));

    Ok(bits::new(x.value | y.value, x.length))
}

pub fn eor_bits_0(x: bits, y: bits, N: integer) -> Result<bits, AArch64LifterError> {
    assert_eq!(x.length, integer_to_usize!(N));
    assert_eq!(y.length, integer_to_usize!(N));

    Ok(bits::new(x.value ^ y.value, x.length))
}

pub fn not_bits_0(x: bits) -> Result<bits, AArch64LifterError> {
    Ok(bits::new(!x.value, x.length).truncate())
}

pub fn zeros_bits_0(N: integer) -> Result<bits, AArch64LifterError> {
    Ok(bits::new(0, integer_to_usize!(N)))
}

pub fn ones_bits_0(N: integer) -> Result<bits, AArch64LifterError> {
    let n = integer_to_usize!(N);
    let ones = if n == 128 { u128::MAX } else { (1 << n) - 1 };
    Ok(bits::new(ones, n))
}

pub fn replicate_bits_0(x: bits, N: integer) -> Result<bits, AArch64LifterError> {
    let rep = N.to_usize().ok_or(AArch64LifterError::InvalidConcatLength)?;

    let new_length = x.length * rep;
    if x.length == 64 && rep == 1 {
        return Ok(x);
    }
    if x.length == 64 || new_length > 64 {
        return Err(AArch64LifterError::InvalidConcatLength);
    }

    let mut new_value = 0;
    for _ in 0..rep {
        new_value = (new_value << x.length) | x.value;
    }

    Ok(bits::new(new_value, new_length))
}

pub fn append_bits_0(x: bits, y: bits) -> Result<bits, AArch64LifterError> {
    if x.length == 0 {
        return Ok(y);
    }
    if y.length == 0 {
        return Ok(x);
    }

    let new_length = x.length + y.length;
    if new_length > 128 {
        return Err(AArch64LifterError::InvalidConcatLength);
    }

    let y_mask = if y.length == 128 { u128::MAX } else { (1 << y.length) - 1 };
    let shifted_x = if y.length == 128 { 0 } else { x.value << y.length };
    let new_value = shifted_x | (y.value & y_mask);

    Ok(bits::new(new_value, new_length))
}

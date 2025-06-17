#![allow(nonstandard_style, unused)]

use crate::arm64::AArch64LifterError;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive};

pub use crate::arm64::common::generated::types::*;

pub const BITS_IN_BYTE: usize = 8;
pub const INSTRUCTION_BYTE_SIZE: usize = 4;
pub const IMPLEMENTATION_DEFINED_boolean: boolean = boolean::TRUE;

#[derive(Debug, Clone, Copy, Default)]
pub struct Dummy;

/// Macro to convert BigInt to usize
#[macro_export]
macro_rules! integer_to_usize {
    ($bigint:expr) => {
        $bigint.to_usize().ok_or(AArch64LifterError::IntegerTooBig)?
    };
}

/// Macro to convert BigInt to u8
#[macro_export]
macro_rules! integer_to_u8 {
    ($bigint:expr) => {
        $bigint.to_u8().ok_or(AArch64LifterError::IntegerTooBig)?
    };
}

/// Macro to convert BigInt to u32
#[macro_export]
macro_rules! integer_to_u32 {
    ($bigint:expr) => {
        $bigint.to_u32().ok_or(AArch64LifterError::IntegerTooBig)?
    };
}

/// Macro to convert BigInt to u64
#[macro_export]
macro_rules! integer_to_u64 {
    ($bigint:expr) => {
        $bigint.to_u64().ok_or(AArch64LifterError::IntegerTooBig)?
    };
}

/// Macro to convert BigInt to u128
#[macro_export]
macro_rules! integer_to_u128 {
    ($bigint:expr) => {
        $bigint.to_u128().ok_or(AArch64LifterError::IntegerTooBig)?
    };
}

/// Macro to convert BigInt to i128
#[macro_export]
macro_rules! integer_to_i128 {
    ($bigint:expr) => {
        $bigint.to_i128().ok_or(AArch64LifterError::IntegerTooBig)?
    };
}

pub type integer = BigInt;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
pub struct bits {
    pub value: u128,
    pub length: usize,
}

impl bits {
    pub fn new(value: u128, length: usize) -> Self {
        bits { value, length }
    }

    pub fn from_bits_literal(bits_literal: &str) -> Result<Self, AArch64LifterError> {
        let length = bits_literal.len();

        if length == 0 || length > 128 {
            return Err(AArch64LifterError::InvalidFieldLength(length));
        }

        if !bits_literal.chars().all(|c| c == '0' || c == '1') {
            return Err(AArch64LifterError::InvalidBitsLiteral(bits_literal.to_string()));
        }

        let value = u128::from_str_radix(bits_literal, 2).map_err(|_| AArch64LifterError::InvalidBitsLiteral(bits_literal.to_string()))?;

        Ok(bits::new(value, length))
    }

    pub fn from_hex_literal(hex_literal: &str) -> Result<Self, AArch64LifterError> {
        let length = hex_literal.len();

        if length == 0 || length > 32 {
            return Err(AArch64LifterError::InvalidFieldLength(length));
        }

        if !hex_literal.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(AArch64LifterError::InvalidBitsLiteral(hex_literal.to_string()));
        }

        let value = u128::from_str_radix(hex_literal, 16).map_err(|_| AArch64LifterError::InvalidBitsLiteral(hex_literal.to_string()))?;

        let bit_length = length * 4;

        Ok(bits::new(value, bit_length))
    }

    fn to_bits_literal(&self) -> String {
        let mask = if self.length == 128 { u128::MAX } else { (1 << self.length) - 1 };

        let truncated = self.value & mask;

        format!("{:0width$b}", truncated, width = self.length)
    }

    pub fn match_with_pattern(&self, pattern: &str) -> bool {
        let literal = self.to_bits_literal();

        literal.len() == pattern.len() && literal.chars().zip(pattern.chars()).all(|(a, b)| b == 'x' || a == b)
    }

    pub fn extract_slice(&self, offset: usize, length: usize) -> Result<bits, AArch64LifterError> {
        if offset + length > self.length {
            panic!("Invalid slice range: offset {} length {} operand size {}", offset, length, self.length);
        }

        let mask = if length == 128 { u128::MAX } else { (1 << length) - 1 };
        let slice_value = (self.value >> offset) & mask;

        Ok(bits::new(slice_value, length))
    }

    pub fn assign_slice(&mut self, value: bits, offset: usize, length: usize) -> Result<(), AArch64LifterError> {
        if offset + length > self.length || value.length != length {
            panic!("Invalid slice range");
        }

        let mask = if length == 128 { u128::MAX } else { ((1 << length) - 1) << offset };
        let cleared_value = self.value & !mask;
        let shifted_slice = value.truncate().value << offset;

        self.value = cleared_value | shifted_slice;

        Ok(())
    }

    pub fn truncate(mut self) -> Self {
        let mask = if self.length == 128 { u128::MAX } else { (1 << self.length) - 1 };
        self.value &= mask;

        self
    }
}

pub trait BigIntExt {
    fn extract_slice(&self, offset: usize, length: usize) -> Result<bits, AArch64LifterError>;
}

impl BigIntExt for integer {
    fn extract_slice(&self, offset: usize, length: usize) -> Result<bits, AArch64LifterError> {
        if offset + length > 64 {
            panic!("3");
            return Err(AArch64LifterError::InvalidSliceRange);
        }

        let mask = (integer::one() << 64) - integer::one();
        let truncated: integer = self & mask;
        let value = truncated.to_u128().ok_or(AArch64LifterError::IntegerTooBig)?;
        let mask = if length == 128 { u128::MAX } else { (1 << length) - 1 };
        let slice_value = (value >> offset) & mask;

        Ok(bits::new(slice_value, length))
    }
}

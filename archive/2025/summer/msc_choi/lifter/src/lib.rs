//! Machine code to AIR lifting module.
//! This module is responsible for translating machine code and proofs to AIR, including parsing
//! proofs.
#![deny(missing_docs)]
extern crate core;

use tnj::air::instructions::CodeRegion;

pub mod arm64;

/// A lifter.
pub trait Lifter {
    /// Error type when lifting fails.
    type E;

    /// Lift from a reader reading machine code and one reading proofs to a Blob.
    fn lift(&self, code: &[u8], proofs: &[u8], decode_only: bool) -> Result<CodeRegion, Self::E>;
}

#![allow(nonstandard_style, unused)]
use crate::arm64::AArch64LifterError;
use crate::arm64::{lift, common};
use crate::arm64::common::types::BigIntExt;
use crate::{integer_to_usize, integer_to_u32, integer_to_u64};
use crate::arm64::lift::types::{AirPackable, BlockSequencer};
use tnj::air::instructions::builder::InstructionBuilder;
use tnj::air::instructions::Value;
use tnj::arch::reg::Reg;
use tnj::types::Type;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive};
use std::collections::BTreeMap;
pub fn lift_aarch64_integer_shift_variable(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: lift::types::Variable,
    d: common::types::integer,
    datasize: common::types::integer,
    m: common::types::integer,
    n: common::types::integer,
    shift_type: common::types::ShiftType,
) -> Result<(), AArch64LifterError> {
    let mut d: lift::types::Variable = d.into();
    let mut datasize: lift::types::Variable = datasize.into();
    let mut m: lift::types::Variable = m.into();
    let mut n: lift::types::Variable = n.into();
    let mut shift_type: lift::types::Variable = shift_type.into();
    let mut assigns_0: BTreeMap<String, lift::types::AirPackable> = BTreeMap::new();
    let mut result: lift::types::Variable = match datasize.clone() {
        lift::types::Variable::Rust(lift::types::RustVariable::integer(i_inner)) => {
            common::types::bits::new(0, integer_to_usize!(i_inner)).into()
        }
        lift::types::Variable::Air(a_inner) => {
            lift::types::Variable::air_from_bits(
                builder,
                common::types::bits::from_bits_literal("0")?,
            )?
        }
        _ => return Err(AArch64LifterError::VariableNotExpectedEnum),
    };
    let mut operand2: lift::types::Variable = {
        let arg_0 = m.clone();
        let arg_1 = datasize.clone();
        lift::helpers::X_read_0(builder, sequencer, pc.clone(), arg_0, arg_1)?
    };
    result = {
        let arg_0 = n.clone();
        let arg_1 = shift_type.clone();
        let arg_2 = {
            let arg_0 = {
                let arg_0 = operand2.clone();
                let arg_1 = datasize.clone();
                lift::helpers::UInt_1(builder, sequencer, pc.clone(), arg_0, arg_1)?
            };
            let arg_1 = datasize.clone();
            lift::helpers::frem_int_0(builder, sequencer, pc.clone(), arg_0, arg_1)?
        };
        let arg_3 = datasize.clone();
        lift::helpers::ShiftReg_0(
            builder,
            sequencer,
            pc.clone(),
            arg_0,
            arg_1,
            arg_2,
            arg_3,
        )?
    };
    {
        let arg_0 = result.clone();
        let arg_1 = d.clone();
        let arg_2 = datasize.clone();
        lift::helpers::X_set_0(builder, sequencer, pc.clone(), arg_0, arg_1, arg_2)?;
    }
    Ok(())
}

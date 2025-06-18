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
pub fn lift_aarch64_integer_ins_ext_extract_immediate(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: lift::types::Variable,
    d: common::types::integer,
    datasize: common::types::integer,
    lsb: common::types::integer,
    m: common::types::integer,
    n: common::types::integer,
) -> Result<(), AArch64LifterError> {
    let mut d: lift::types::Variable = d.into();
    let mut datasize: lift::types::Variable = datasize.into();
    let mut lsb: lift::types::Variable = lsb.into();
    let mut m: lift::types::Variable = m.into();
    let mut n: lift::types::Variable = n.into();
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
    let mut operand1: lift::types::Variable = {
        let arg_0 = n.clone();
        let arg_1 = datasize.clone();
        lift::helpers::X_read_0(builder, sequencer, pc.clone(), arg_0, arg_1)?
    };
    let mut operand2: lift::types::Variable = {
        let arg_0 = m.clone();
        let arg_1 = datasize.clone();
        lift::helpers::X_read_0(builder, sequencer, pc.clone(), arg_0, arg_1)?
    };
    let mut concat: lift::types::Variable = {
        let arg_0 = operand1.clone();
        let arg_1 = operand2.clone();
        let arg_2 = datasize.clone();
        let arg_3 = datasize.clone();
        lift::helpers::append_bits_0(
            builder,
            sequencer,
            pc.clone(),
            arg_0,
            arg_1,
            arg_2,
            arg_3,
        )?
    };
    result = {
        let arg_0 = lsb.clone();
        let arg_1 = {
            let arg_0 = {
                let arg_0 = {
                    let arg_0 = {
                        let arg_0 = lsb.clone();
                        let arg_1 = datasize.clone();
                        lift::helpers::add_int_0(
                            builder,
                            sequencer,
                            pc.clone(),
                            arg_0,
                            arg_1,
                        )?
                    };
                    let arg_1 = lift::types::Variable::from(
                        common::types::integer::from(1),
                    );
                    lift::helpers::sub_int_0(
                        builder,
                        sequencer,
                        pc.clone(),
                        arg_0,
                        arg_1,
                    )?
                };
                let arg_1 = lift::types::Variable::from(common::types::integer::one());
                lift::helpers::add_int_0(builder, sequencer, pc.clone(), arg_0, arg_1)?
            };
            let arg_1 = lsb.clone();
            lift::helpers::sub_int_0(builder, sequencer, pc.clone(), arg_0, arg_1)?
        };
        concat.extract_slice(builder, arg_0, arg_1)?
    };
    {
        let arg_0 = result.clone();
        let arg_1 = d.clone();
        let arg_2 = datasize.clone();
        lift::helpers::X_set_0(builder, sequencer, pc.clone(), arg_0, arg_1, arg_2)?;
    }
    Ok(())
}

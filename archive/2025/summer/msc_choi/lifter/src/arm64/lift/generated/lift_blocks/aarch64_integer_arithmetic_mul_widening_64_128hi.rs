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
pub fn lift_aarch64_integer_arithmetic_mul_widening_64_128hi(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: lift::types::Variable,
    a: common::types::integer,
    d: common::types::integer,
    datasize: common::types::integer,
    destsize: common::types::integer,
    m: common::types::integer,
    n: common::types::integer,
    unsigned: common::types::boolean,
) -> Result<(), AArch64LifterError> {
    let mut a: lift::types::Variable = a.into();
    let mut d: lift::types::Variable = d.into();
    let mut datasize: lift::types::Variable = datasize.into();
    let mut destsize: lift::types::Variable = destsize.into();
    let mut m: lift::types::Variable = m.into();
    let mut n: lift::types::Variable = n.into();
    let mut unsigned: lift::types::Variable = unsigned.into();
    let mut assigns_0: BTreeMap<String, lift::types::AirPackable> = BTreeMap::new();
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
    let mut result: lift::types::Variable = lift::types::Variable::from(
        common::types::integer::default(),
    );
    result = {
        let arg_0 = {
            let arg_0 = operand1.clone();
            let arg_1 = unsigned.clone();
            let arg_2 = datasize.clone();
            lift::helpers::Int_0(builder, sequencer, pc.clone(), arg_0, arg_1, arg_2)?
        };
        let arg_1 = {
            let arg_0 = operand2.clone();
            let arg_1 = unsigned.clone();
            let arg_2 = datasize.clone();
            lift::helpers::Int_0(builder, sequencer, pc.clone(), arg_0, arg_1, arg_2)?
        };
        lift::helpers::mul_int_0(builder, sequencer, pc.clone(), arg_0, arg_1)?
    };
    {
        let arg_0 = {
            let arg_0 = lift::types::Variable::from(common::types::integer::from(64));
            let arg_1 = {
                let arg_0 = {
                    let arg_0 = lift::types::Variable::from(
                        common::types::integer::from(127),
                    );
                    let arg_1 = lift::types::Variable::from(
                        common::types::integer::one(),
                    );
                    lift::helpers::add_int_0(
                        builder,
                        sequencer,
                        pc.clone(),
                        arg_0,
                        arg_1,
                    )?
                };
                let arg_1 = lift::types::Variable::from(
                    common::types::integer::from(64),
                );
                lift::helpers::sub_int_0(builder, sequencer, pc.clone(), arg_0, arg_1)?
            };
            result.extract_slice(builder, arg_0, arg_1)?
        };
        let arg_1 = d.clone();
        let arg_2 = {
            let arg_0 = lift::types::Variable::from(common::types::integer::from(0));
            let arg_1 = {
                let arg_0 = {
                    let arg_0 = lift::types::Variable::from(
                        common::types::integer::from(127),
                    );
                    let arg_1 = lift::types::Variable::from(
                        common::types::integer::from(64),
                    );
                    lift::helpers::sub_int_0(
                        builder,
                        sequencer,
                        pc.clone(),
                        arg_0,
                        arg_1,
                    )?
                };
                let arg_1 = lift::types::Variable::from(common::types::integer::from(1));
                lift::helpers::add_int_0(builder, sequencer, pc.clone(), arg_0, arg_1)?
            };
            lift::helpers::add_int_0(builder, sequencer, pc.clone(), arg_0, arg_1)?
        };
        lift::helpers::X_set_0(builder, sequencer, pc.clone(), arg_0, arg_1, arg_2)?;
    }
    Ok(())
}

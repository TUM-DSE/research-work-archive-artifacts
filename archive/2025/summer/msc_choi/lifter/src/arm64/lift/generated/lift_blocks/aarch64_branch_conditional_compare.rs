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
pub fn lift_aarch64_branch_conditional_compare(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: lift::types::Variable,
    datasize: common::types::integer,
    iszero: common::types::boolean,
    offset: common::types::bits,
    t: common::types::integer,
) -> Result<(), AArch64LifterError> {
    let mut datasize: lift::types::Variable = datasize.into();
    let mut iszero: lift::types::Variable = iszero.into();
    let mut offset: lift::types::Variable = offset.into();
    let mut t: lift::types::Variable = t.into();
    let mut assigns_0: BTreeMap<String, lift::types::AirPackable> = BTreeMap::new();
    let mut operand1: lift::types::Variable = {
        let arg_0 = t.clone();
        let arg_1 = datasize.clone();
        lift::helpers::X_read_0(builder, sequencer, pc.clone(), arg_0, arg_1)?
    };
    {
        let cond = {
            let arg_0 = {
                let arg_0 = operand1.clone();
                let arg_1 = datasize.clone();
                lift::helpers::IsZero_0(builder, sequencer, pc.clone(), arg_0, arg_1)?
            };
            let arg_1 = iszero.clone();
            lift::helpers::eq_enum_0(builder, sequencer, pc.clone(), arg_0, arg_1)?
        };
        match cond {
            lift::types::Variable::Rust(lift::types::RustVariable::boolean(b_inner)) => {
                if b_inner == common::types::boolean::TRUE {
                    let mut assigns_1: BTreeMap<String, lift::types::AirPackable> = assigns_0
                        .clone();
                    {
                        let arg_0 = {
                            let arg_0 = {
                                lift::helpers::PC_read_1(builder, sequencer, pc.clone())?
                            };
                            let arg_1 = offset.clone();
                            let arg_2 = lift::types::Variable::from(
                                common::types::integer::from(64),
                            );
                            lift::helpers::add_bits_0(
                                builder,
                                sequencer,
                                pc.clone(),
                                arg_0,
                                arg_1,
                                arg_2,
                            )?
                        };
                        let arg_1 = lift::types::Variable::from(
                            common::types::BranchType::BranchType_DIR,
                        );
                        let arg_2 = lift::types::Variable::from(
                            common::types::integer::from(64),
                        );
                        lift::helpers::BranchTo_0(
                            builder,
                            sequencer,
                            pc.clone(),
                            arg_0,
                            arg_1,
                            arg_2,
                        )?;
                    }
                } else {
                    let mut assigns_1: BTreeMap<String, lift::types::AirPackable> = assigns_0
                        .clone();
                }
            }
            lift::types::Variable::Air(a_inner) => {
                let current_block = builder.current_block();
                let then_block = sequencer
                    .get_block(
                        pc.to_bits()?.value as u64,
                        lift::types::BlockType::IntraBlock,
                        builder,
                        &vec![],
                    )?;
                builder.set_insert_block(then_block);
                let mut assigns_1: BTreeMap<String, lift::types::AirPackable> = assigns_0
                    .clone();
                {
                    let arg_0 = {
                        let arg_0 = {
                            lift::helpers::PC_read_1(builder, sequencer, pc.clone())?
                        };
                        let arg_1 = match assigns_1.get("offset") {
                            Some(packable) => (*packable).clone().try_into()?,
                            None => offset.clone(),
                        };
                        let arg_2 = lift::types::Variable::from(
                            common::types::integer::from(64),
                        );
                        lift::helpers::add_bits_0(
                            builder,
                            sequencer,
                            pc.clone(),
                            arg_0,
                            arg_1,
                            arg_2,
                        )?
                    };
                    let arg_1 = lift::types::Variable::from(
                        common::types::BranchType::BranchType_DIR,
                    );
                    let arg_2 = lift::types::Variable::from(
                        common::types::integer::from(64),
                    );
                    lift::helpers::BranchTo_0(
                        builder,
                        sequencer,
                        pc.clone(),
                        arg_0,
                        arg_1,
                        arg_2,
                    )?;
                }
                let (then_args, block_param_types): (Vec<Value>, Vec<Type>) = vec![]
                    .into_iter()
                    .flat_map(|(args, tys): (Vec<Value>, Vec<Type>)| {
                        args.into_iter().zip(tys.into_iter())
                    })
                    .unzip();
                let else_block = sequencer
                    .get_block(
                        pc.to_bits()?.value as u64,
                        lift::types::BlockType::IntraBlock,
                        builder,
                        &vec![],
                    )?;
                builder.set_insert_block(current_block);
                builder.jumpif(a_inner.val, then_block, [], else_block, []);
                builder.set_insert_block(else_block);
                let mut assigns_1: BTreeMap<String, lift::types::AirPackable> = assigns_0
                    .clone();
                let (else_args, _): (Vec<Value>, Vec<Type>) = vec![]
                    .into_iter()
                    .flat_map(|(args, tys): (Vec<Value>, Vec<Type>)| {
                        args.into_iter().zip(tys.into_iter())
                    })
                    .unzip();
                let end_block = sequencer
                    .get_block(
                        pc.to_bits()?.value as u64,
                        lift::types::BlockType::IntraBlock,
                        builder,
                        &block_param_types,
                    )?;
                builder.set_insert_block(then_block);
                builder.jump(end_block, then_args);
                builder.set_insert_block(else_block);
                builder.jump(end_block, else_args);
                builder.set_insert_block(end_block);
                let mut end_args = Vec::new();
                for i in 0..block_param_types.len() {
                    end_args
                        .push(Value::from(builder.get_block_param(end_block, i as u32)));
                }
                let mut consumed_total = 0;
            }
            _ => return Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    Ok(())
}

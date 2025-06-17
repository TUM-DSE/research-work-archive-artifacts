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
pub fn lift_aarch64_memory_literal_general(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: lift::types::Variable,
    memop: common::types::MemOp,
    offset: common::types::bits,
    signed: common::types::boolean,
    size: common::types::integer,
    t: common::types::integer,
    tag_checked: common::types::boolean,
) -> Result<(), AArch64LifterError> {
    let mut memop: lift::types::Variable = memop.into();
    let mut offset: lift::types::Variable = offset.into();
    let mut signed: lift::types::Variable = signed.into();
    let mut size: lift::types::Variable = size.into();
    let mut t: lift::types::Variable = t.into();
    let mut tag_checked: lift::types::Variable = tag_checked.into();
    let mut assigns_0: BTreeMap<String, lift::types::AirPackable> = BTreeMap::new();
    let mut address: lift::types::Variable = {
        let arg_0 = { lift::helpers::PC_read_1(builder, sequencer, pc.clone())? };
        let arg_1 = offset.clone();
        let arg_2 = lift::types::Variable::from(common::types::integer::from(64));
        lift::helpers::add_bits_0(builder, sequencer, pc.clone(), arg_0, arg_1, arg_2)?
    };
    let mut data: lift::types::Variable = match {
        let arg_0 = size.clone();
        let arg_1 = lift::types::Variable::from(common::types::integer::from(8));
        lift::helpers::mul_int_0(builder, sequencer, pc.clone(), arg_0, arg_1)?
    } {
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
    {
        let cond = { lift::helpers::HaveMTEExt_0(builder, sequencer, pc.clone())? };
        match cond {
            lift::types::Variable::Rust(lift::types::RustVariable::boolean(b_inner)) => {
                if b_inner == common::types::boolean::TRUE {
                    let mut assigns_1: BTreeMap<String, lift::types::AirPackable> = assigns_0
                        .clone();
                    {
                        let arg_0 = tag_checked.clone();
                        lift::helpers::SetTagCheckedInstruction_0(
                            builder,
                            sequencer,
                            pc.clone(),
                            arg_0,
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
                    let arg_0 = match assigns_1.get("tag_checked") {
                        Some(packable) => (*packable).clone().try_into()?,
                        None => tag_checked.clone(),
                    };
                    lift::helpers::SetTagCheckedInstruction_0(
                        builder,
                        sequencer,
                        pc.clone(),
                        arg_0,
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
    {
        let expr = memop.clone();
        match &expr {
            lift::types::Variable::Rust(_) => {
                if (memop.clone()
                    == lift::types::Variable::from(common::types::MemOp::MemOp_LOAD))
                    && true
                {
                    let mut assigns_1: BTreeMap<String, lift::types::AirPackable> = assigns_0
                        .clone();
                    data = {
                        let arg_0 = address.clone();
                        let arg_1 = size.clone();
                        let arg_2 = lift::types::Variable::from(
                            common::types::AccType::AccType_NORMAL,
                        );
                        let arg_3 = size.clone();
                        lift::helpers::Mem_read_0(
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
                        let cond = signed.clone();
                        match cond {
                            lift::types::Variable::Rust(
                                lift::types::RustVariable::boolean(b_inner),
                            ) => {
                                if b_inner == common::types::boolean::TRUE {
                                    let mut assigns_2: BTreeMap<
                                        String,
                                        lift::types::AirPackable,
                                    > = assigns_1.clone();
                                    {
                                        let arg_0 = {
                                            let arg_0 = data.clone();
                                            let arg_1 = lift::types::Variable::from(
                                                common::types::integer::from(64),
                                            );
                                            let arg_2 = {
                                                let arg_0 = size.clone();
                                                let arg_1 = lift::types::Variable::from(
                                                    common::types::integer::from(8),
                                                );
                                                lift::helpers::mul_int_0(
                                                    builder,
                                                    sequencer,
                                                    pc.clone(),
                                                    arg_0,
                                                    arg_1,
                                                )?
                                            };
                                            let arg_3 = lift::types::Variable::from(
                                                common::types::integer::from(64),
                                            );
                                            lift::helpers::SignExtend_0(
                                                builder,
                                                sequencer,
                                                pc.clone(),
                                                arg_0,
                                                arg_1,
                                                arg_2,
                                                arg_3,
                                            )?
                                        };
                                        let arg_1 = t.clone();
                                        let arg_2 = lift::types::Variable::from(
                                            common::types::integer::from(64),
                                        );
                                        lift::helpers::X_set_0(
                                            builder,
                                            sequencer,
                                            pc.clone(),
                                            arg_0,
                                            arg_1,
                                            arg_2,
                                        )?;
                                    }
                                } else {
                                    let mut assigns_2: BTreeMap<
                                        String,
                                        lift::types::AirPackable,
                                    > = assigns_1.clone();
                                    {
                                        let arg_0 = data.clone();
                                        let arg_1 = t.clone();
                                        let arg_2 = {
                                            let arg_0 = size.clone();
                                            let arg_1 = lift::types::Variable::from(
                                                common::types::integer::from(8),
                                            );
                                            lift::helpers::mul_int_0(
                                                builder,
                                                sequencer,
                                                pc.clone(),
                                                arg_0,
                                                arg_1,
                                            )?
                                        };
                                        lift::helpers::X_set_0(
                                            builder,
                                            sequencer,
                                            pc.clone(),
                                            arg_0,
                                            arg_1,
                                            arg_2,
                                        )?;
                                    }
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
                                let mut assigns_2: BTreeMap<
                                    String,
                                    lift::types::AirPackable,
                                > = assigns_1.clone();
                                {
                                    let arg_0 = {
                                        let arg_0 = match assigns_2.get("data") {
                                            Some(packable) => (*packable).clone().try_into()?,
                                            None => data.clone(),
                                        };
                                        let arg_1 = lift::types::Variable::from(
                                            common::types::integer::from(64),
                                        );
                                        let arg_2 = {
                                            let arg_0 = match assigns_2.get("size") {
                                                Some(packable) => (*packable).clone().try_into()?,
                                                None => size.clone(),
                                            };
                                            let arg_1 = lift::types::Variable::from(
                                                common::types::integer::from(8),
                                            );
                                            lift::helpers::mul_int_0(
                                                builder,
                                                sequencer,
                                                pc.clone(),
                                                arg_0,
                                                arg_1,
                                            )?
                                        };
                                        let arg_3 = lift::types::Variable::from(
                                            common::types::integer::from(64),
                                        );
                                        lift::helpers::SignExtend_0(
                                            builder,
                                            sequencer,
                                            pc.clone(),
                                            arg_0,
                                            arg_1,
                                            arg_2,
                                            arg_3,
                                        )?
                                    };
                                    let arg_1 = match assigns_2.get("t") {
                                        Some(packable) => (*packable).clone().try_into()?,
                                        None => t.clone(),
                                    };
                                    let arg_2 = lift::types::Variable::from(
                                        common::types::integer::from(64),
                                    );
                                    lift::helpers::X_set_0(
                                        builder,
                                        sequencer,
                                        pc.clone(),
                                        arg_0,
                                        arg_1,
                                        arg_2,
                                    )?;
                                }
                                let (
                                    then_args,
                                    block_param_types,
                                ): (Vec<Value>, Vec<Type>) = vec![]
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
                                let mut assigns_2: BTreeMap<
                                    String,
                                    lift::types::AirPackable,
                                > = assigns_1.clone();
                                {
                                    let arg_0 = match assigns_2.get("data") {
                                        Some(packable) => (*packable).clone().try_into()?,
                                        None => data.clone(),
                                    };
                                    let arg_1 = match assigns_2.get("t") {
                                        Some(packable) => (*packable).clone().try_into()?,
                                        None => t.clone(),
                                    };
                                    let arg_2 = {
                                        let arg_0 = match assigns_2.get("size") {
                                            Some(packable) => (*packable).clone().try_into()?,
                                            None => size.clone(),
                                        };
                                        let arg_1 = lift::types::Variable::from(
                                            common::types::integer::from(8),
                                        );
                                        lift::helpers::mul_int_0(
                                            builder,
                                            sequencer,
                                            pc.clone(),
                                            arg_0,
                                            arg_1,
                                        )?
                                    };
                                    lift::helpers::X_set_0(
                                        builder,
                                        sequencer,
                                        pc.clone(),
                                        arg_0,
                                        arg_1,
                                        arg_2,
                                    )?;
                                }
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
                                        .push(
                                            Value::from(builder.get_block_param(end_block, i as u32)),
                                        );
                                }
                                let mut consumed_total = 0;
                            }
                            _ => return Err(AArch64LifterError::VariableNotExpectedEnum),
                        }
                    }
                } else if (memop.clone()
                    == lift::types::Variable::from(common::types::MemOp::MemOp_PREFETCH))
                    && true
                {
                    let mut assigns_1: BTreeMap<String, lift::types::AirPackable> = assigns_0
                        .clone();
                    {
                        let arg_0 = address.clone();
                        let arg_1 = {
                            let arg_0 = lift::types::Variable::from(
                                common::types::integer::from(0),
                            );
                            let arg_1 = {
                                let arg_0 = {
                                    let arg_0 = lift::types::Variable::from(
                                        common::types::integer::from(4),
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
                                    common::types::integer::from(0),
                                );
                                lift::helpers::sub_int_0(
                                    builder,
                                    sequencer,
                                    pc.clone(),
                                    arg_0,
                                    arg_1,
                                )?
                            };
                            t.extract_slice(builder, arg_0, arg_1)?
                        };
                        lift::helpers::Prefetch_0(
                            builder,
                            sequencer,
                            pc.clone(),
                            arg_0,
                            arg_1,
                        )?;
                    }
                } else {
                    let mut assigns_1: BTreeMap<String, lift::types::AirPackable> = assigns_0
                        .clone();
                }
            }
            lift::types::Variable::Air(_) => {
                return Err(AArch64LifterError::NotImplemented(file!(), line!()));
            }
        }
    }
    Ok(())
}

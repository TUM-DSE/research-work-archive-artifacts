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
pub fn lift_aarch64_branch_unconditional_register(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: lift::types::Variable,
    branch_type: common::types::BranchType,
    m: common::types::integer,
    n: common::types::integer,
    pac: common::types::boolean,
    source_is_sp: common::types::boolean,
    use_key_a: common::types::boolean,
) -> Result<(), AArch64LifterError> {
    let mut branch_type: lift::types::Variable = branch_type.into();
    let mut m: lift::types::Variable = m.into();
    let mut n: lift::types::Variable = n.into();
    let mut pac: lift::types::Variable = pac.into();
    let mut source_is_sp: lift::types::Variable = source_is_sp.into();
    let mut use_key_a: lift::types::Variable = use_key_a.into();
    let mut assigns_0: BTreeMap<String, lift::types::AirPackable> = BTreeMap::new();
    let mut target: lift::types::Variable = {
        let arg_0 = n.clone();
        let arg_1 = lift::types::Variable::from(common::types::integer::from(64));
        lift::helpers::X_read_0(builder, sequencer, pc.clone(), arg_0, arg_1)?
    };
    let mut auth_then_branch: lift::types::Variable = lift::types::Variable::from(
        common::types::boolean::TRUE,
    );
    {
        let cond = pac.clone();
        match cond {
            lift::types::Variable::Rust(lift::types::RustVariable::boolean(b_inner)) => {
                if b_inner == common::types::boolean::TRUE {
                    let mut assigns_1: BTreeMap<String, lift::types::AirPackable> = assigns_0
                        .clone();
                    let mut modifier: lift::types::Variable = {
                        let cond = source_is_sp.clone();
                        match cond {
                            lift::types::Variable::Rust(
                                lift::types::RustVariable::boolean(b_inner),
                            ) => {
                                if b_inner == common::types::boolean::TRUE {
                                    {
                                        let arg_0 = lift::types::Variable::from(
                                            common::types::integer::from(64),
                                        );
                                        lift::helpers::SP_read_1(
                                            builder,
                                            sequencer,
                                            pc.clone(),
                                            arg_0,
                                        )?
                                    }
                                } else {
                                    {
                                        let arg_0 = m.clone();
                                        let arg_1 = lift::types::Variable::from(
                                            common::types::integer::from(64),
                                        );
                                        lift::helpers::X_read_0(
                                            builder,
                                            sequencer,
                                            pc.clone(),
                                            arg_0,
                                            arg_1,
                                        )?
                                    }
                                }
                            }
                            lift::types::Variable::Air(a_inner) => {
                                let then_block = sequencer
                                    .get_block(
                                        pc.to_bits()?.value as u64,
                                        lift::types::BlockType::IntraBlock,
                                        builder,
                                        &vec![],
                                    )?;
                                let else_block = sequencer
                                    .get_block(
                                        pc.to_bits()?.value as u64,
                                        lift::types::BlockType::IntraBlock,
                                        builder,
                                        &vec![],
                                    )?;
                                builder.jumpif(a_inner.val, then_block, [], else_block, []);
                                builder.set_insert_block(then_block);
                                let then_body_promoted = {
                                    let arg_0 = lift::types::Variable::from(
                                        common::types::integer::from(64),
                                    );
                                    lift::helpers::SP_read_1(
                                        builder,
                                        sequencer,
                                        pc.clone(),
                                        arg_0,
                                    )?
                                };
                                let (arg_then_vals, arg_tys) = {
                                    let packable: lift::types::AirPackable = then_body_promoted
                                        .clone()
                                        .into();
                                    packable.unpack_to_air_values_and_types(builder)?
                                };
                                let end_block = sequencer
                                    .get_block(
                                        pc.to_bits()?.value as u64,
                                        lift::types::BlockType::IntraBlock,
                                        builder,
                                        &arg_tys,
                                    )?;
                                builder.jump(end_block, arg_then_vals);
                                builder.set_insert_block(else_block);
                                let (arg_else_vals, _) = {
                                    let packable: lift::types::AirPackable = {
                                        let arg_0 = m.clone();
                                        let arg_1 = lift::types::Variable::from(
                                            common::types::integer::from(64),
                                        );
                                        lift::helpers::X_read_0(
                                            builder,
                                            sequencer,
                                            pc.clone(),
                                            arg_0,
                                            arg_1,
                                        )?
                                    }
                                        .into();
                                    packable.unpack_to_air_values_and_types(builder)?
                                };
                                builder.jump(end_block, arg_else_vals);
                                builder.set_insert_block(end_block);
                                let mut end_args = Vec::new();
                                for i in 0..arg_tys.len() {
                                    end_args
                                        .push(
                                            Value::from(builder.get_block_param(end_block, i as u32)),
                                        );
                                }
                                let packable: lift::types::AirPackable = then_body_promoted
                                    .into();
                                let (end_body_packable, _) = packable
                                    .pack_from_air_values_and_types(&end_args, &arg_tys)?;
                                end_body_packable.try_into()?
                            }
                            _ => return Err(AArch64LifterError::VariableNotExpectedEnum),
                        }
                    };
                    {
                        let cond = use_key_a.clone();
                        match cond {
                            lift::types::Variable::Rust(
                                lift::types::RustVariable::boolean(b_inner),
                            ) => {
                                if b_inner == common::types::boolean::TRUE {
                                    let mut assigns_2: BTreeMap<
                                        String,
                                        lift::types::AirPackable,
                                    > = assigns_1.clone();
                                    target = {
                                        let arg_0 = target.clone();
                                        let arg_1 = modifier.clone();
                                        let arg_2 = auth_then_branch.clone();
                                        lift::helpers::AuthIA_0(
                                            builder,
                                            sequencer,
                                            pc.clone(),
                                            arg_0,
                                            arg_1,
                                            arg_2,
                                        )?
                                    };
                                } else {
                                    let mut assigns_2: BTreeMap<
                                        String,
                                        lift::types::AirPackable,
                                    > = assigns_1.clone();
                                    target = {
                                        let arg_0 = target.clone();
                                        let arg_1 = modifier.clone();
                                        let arg_2 = auth_then_branch.clone();
                                        lift::helpers::AuthIB_0(
                                            builder,
                                            sequencer,
                                            pc.clone(),
                                            arg_0,
                                            arg_1,
                                            arg_2,
                                        )?
                                    };
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
                                assigns_2
                                    .insert(
                                        "target".to_string(),
                                        {
                                            let arg_0 = match assigns_2.get("target") {
                                                Some(packable) => (*packable).clone().try_into()?,
                                                None => target.clone(),
                                            };
                                            let arg_1 = match assigns_2.get("modifier") {
                                                Some(packable) => (*packable).clone().try_into()?,
                                                None => modifier.clone(),
                                            };
                                            let arg_2 = match assigns_2.get("auth_then_branch") {
                                                Some(packable) => (*packable).clone().try_into()?,
                                                None => auth_then_branch.clone(),
                                            };
                                            lift::helpers::AuthIA_0(
                                                builder,
                                                sequencer,
                                                pc.clone(),
                                                arg_0,
                                                arg_1,
                                                arg_2,
                                            )?
                                        }
                                            .into(),
                                    );
                                let (
                                    then_args,
                                    block_param_types,
                                ): (Vec<Value>, Vec<Type>) = vec![
                                    assigns_2.get("target").unwrap()
                                    .unpack_to_air_values_and_types(builder) ?
                                ]
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
                                assigns_2
                                    .insert(
                                        "target".to_string(),
                                        {
                                            let arg_0 = match assigns_2.get("target") {
                                                Some(packable) => (*packable).clone().try_into()?,
                                                None => target.clone(),
                                            };
                                            let arg_1 = match assigns_2.get("modifier") {
                                                Some(packable) => (*packable).clone().try_into()?,
                                                None => modifier.clone(),
                                            };
                                            let arg_2 = match assigns_2.get("auth_then_branch") {
                                                Some(packable) => (*packable).clone().try_into()?,
                                                None => auth_then_branch.clone(),
                                            };
                                            lift::helpers::AuthIB_0(
                                                builder,
                                                sequencer,
                                                pc.clone(),
                                                arg_0,
                                                arg_1,
                                                arg_2,
                                            )?
                                        }
                                            .into(),
                                    );
                                let (else_args, _): (Vec<Value>, Vec<Type>) = vec![
                                    assigns_2.get("target").unwrap()
                                    .unpack_to_air_values_and_types(builder) ?
                                ]
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
                                let packable: lift::types::AirPackable = target
                                    .clone()
                                    .into();
                                let (packed, consumed) = packable
                                    .pack_from_air_values_and_types(
                                        &end_args[consumed_total..],
                                        &block_param_types[consumed_total..],
                                    )?;
                                target = packed.try_into()?;
                                consumed_total += consumed;
                            }
                            _ => return Err(AArch64LifterError::VariableNotExpectedEnum),
                        }
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
                let mut modifier: lift::types::Variable = {
                    let cond = match assigns_1.get("source_is_sp") {
                        Some(packable) => (*packable).clone().try_into()?,
                        None => source_is_sp.clone(),
                    };
                    match cond {
                        lift::types::Variable::Rust(
                            lift::types::RustVariable::boolean(b_inner),
                        ) => {
                            if b_inner == common::types::boolean::TRUE {
                                {
                                    let arg_0 = lift::types::Variable::from(
                                        common::types::integer::from(64),
                                    );
                                    lift::helpers::SP_read_1(
                                        builder,
                                        sequencer,
                                        pc.clone(),
                                        arg_0,
                                    )?
                                }
                            } else {
                                {
                                    let arg_0 = match assigns_1.get("m") {
                                        Some(packable) => (*packable).clone().try_into()?,
                                        None => m.clone(),
                                    };
                                    let arg_1 = lift::types::Variable::from(
                                        common::types::integer::from(64),
                                    );
                                    lift::helpers::X_read_0(
                                        builder,
                                        sequencer,
                                        pc.clone(),
                                        arg_0,
                                        arg_1,
                                    )?
                                }
                            }
                        }
                        lift::types::Variable::Air(a_inner) => {
                            let then_block = sequencer
                                .get_block(
                                    pc.to_bits()?.value as u64,
                                    lift::types::BlockType::IntraBlock,
                                    builder,
                                    &vec![],
                                )?;
                            let else_block = sequencer
                                .get_block(
                                    pc.to_bits()?.value as u64,
                                    lift::types::BlockType::IntraBlock,
                                    builder,
                                    &vec![],
                                )?;
                            builder.jumpif(a_inner.val, then_block, [], else_block, []);
                            builder.set_insert_block(then_block);
                            let then_body_promoted = {
                                let arg_0 = lift::types::Variable::from(
                                    common::types::integer::from(64),
                                );
                                lift::helpers::SP_read_1(
                                    builder,
                                    sequencer,
                                    pc.clone(),
                                    arg_0,
                                )?
                            };
                            let (arg_then_vals, arg_tys) = {
                                let packable: lift::types::AirPackable = then_body_promoted
                                    .clone()
                                    .into();
                                packable.unpack_to_air_values_and_types(builder)?
                            };
                            let end_block = sequencer
                                .get_block(
                                    pc.to_bits()?.value as u64,
                                    lift::types::BlockType::IntraBlock,
                                    builder,
                                    &arg_tys,
                                )?;
                            builder.jump(end_block, arg_then_vals);
                            builder.set_insert_block(else_block);
                            let (arg_else_vals, _) = {
                                let packable: lift::types::AirPackable = {
                                    let arg_0 = match assigns_1.get("m") {
                                        Some(packable) => (*packable).clone().try_into()?,
                                        None => m.clone(),
                                    };
                                    let arg_1 = lift::types::Variable::from(
                                        common::types::integer::from(64),
                                    );
                                    lift::helpers::X_read_0(
                                        builder,
                                        sequencer,
                                        pc.clone(),
                                        arg_0,
                                        arg_1,
                                    )?
                                }
                                    .into();
                                packable.unpack_to_air_values_and_types(builder)?
                            };
                            builder.jump(end_block, arg_else_vals);
                            builder.set_insert_block(end_block);
                            let mut end_args = Vec::new();
                            for i in 0..arg_tys.len() {
                                end_args
                                    .push(
                                        Value::from(builder.get_block_param(end_block, i as u32)),
                                    );
                            }
                            let packable: lift::types::AirPackable = then_body_promoted
                                .into();
                            let (end_body_packable, _) = packable
                                .pack_from_air_values_and_types(&end_args, &arg_tys)?;
                            end_body_packable.try_into()?
                        }
                        _ => return Err(AArch64LifterError::VariableNotExpectedEnum),
                    }
                };
                {
                    let cond = match assigns_1.get("use_key_a") {
                        Some(packable) => (*packable).clone().try_into()?,
                        None => use_key_a.clone(),
                    };
                    match cond {
                        lift::types::Variable::Rust(
                            lift::types::RustVariable::boolean(b_inner),
                        ) => {
                            if b_inner == common::types::boolean::TRUE {
                                let mut assigns_2: BTreeMap<
                                    String,
                                    lift::types::AirPackable,
                                > = assigns_1.clone();
                                assigns_2
                                    .insert(
                                        "target".to_string(),
                                        {
                                            let arg_0 = match assigns_2.get("target") {
                                                Some(packable) => (*packable).clone().try_into()?,
                                                None => target.clone(),
                                            };
                                            let arg_1 = match assigns_2.get("modifier") {
                                                Some(packable) => (*packable).clone().try_into()?,
                                                None => modifier.clone(),
                                            };
                                            let arg_2 = match assigns_2.get("auth_then_branch") {
                                                Some(packable) => (*packable).clone().try_into()?,
                                                None => auth_then_branch.clone(),
                                            };
                                            lift::helpers::AuthIA_0(
                                                builder,
                                                sequencer,
                                                pc.clone(),
                                                arg_0,
                                                arg_1,
                                                arg_2,
                                            )?
                                        }
                                            .into(),
                                    );
                                assigns_1
                                    .insert(
                                        "target".to_string(),
                                        assigns_2.get("target").unwrap().clone(),
                                    );
                            } else {
                                let mut assigns_2: BTreeMap<
                                    String,
                                    lift::types::AirPackable,
                                > = assigns_1.clone();
                                assigns_2
                                    .insert(
                                        "target".to_string(),
                                        {
                                            let arg_0 = match assigns_2.get("target") {
                                                Some(packable) => (*packable).clone().try_into()?,
                                                None => target.clone(),
                                            };
                                            let arg_1 = match assigns_2.get("modifier") {
                                                Some(packable) => (*packable).clone().try_into()?,
                                                None => modifier.clone(),
                                            };
                                            let arg_2 = match assigns_2.get("auth_then_branch") {
                                                Some(packable) => (*packable).clone().try_into()?,
                                                None => auth_then_branch.clone(),
                                            };
                                            lift::helpers::AuthIB_0(
                                                builder,
                                                sequencer,
                                                pc.clone(),
                                                arg_0,
                                                arg_1,
                                                arg_2,
                                            )?
                                        }
                                            .into(),
                                    );
                                assigns_1
                                    .insert(
                                        "target".to_string(),
                                        assigns_2.get("target").unwrap().clone(),
                                    );
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
                            assigns_2
                                .insert(
                                    "target".to_string(),
                                    {
                                        let arg_0 = match assigns_2.get("target") {
                                            Some(packable) => (*packable).clone().try_into()?,
                                            None => target.clone(),
                                        };
                                        let arg_1 = match assigns_2.get("modifier") {
                                            Some(packable) => (*packable).clone().try_into()?,
                                            None => modifier.clone(),
                                        };
                                        let arg_2 = match assigns_2.get("auth_then_branch") {
                                            Some(packable) => (*packable).clone().try_into()?,
                                            None => auth_then_branch.clone(),
                                        };
                                        lift::helpers::AuthIA_0(
                                            builder,
                                            sequencer,
                                            pc.clone(),
                                            arg_0,
                                            arg_1,
                                            arg_2,
                                        )?
                                    }
                                        .into(),
                                );
                            let (
                                then_args,
                                block_param_types,
                            ): (Vec<Value>, Vec<Type>) = vec![
                                assigns_2.get("target").unwrap()
                                .unpack_to_air_values_and_types(builder) ?
                            ]
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
                            assigns_2
                                .insert(
                                    "target".to_string(),
                                    {
                                        let arg_0 = match assigns_2.get("target") {
                                            Some(packable) => (*packable).clone().try_into()?,
                                            None => target.clone(),
                                        };
                                        let arg_1 = match assigns_2.get("modifier") {
                                            Some(packable) => (*packable).clone().try_into()?,
                                            None => modifier.clone(),
                                        };
                                        let arg_2 = match assigns_2.get("auth_then_branch") {
                                            Some(packable) => (*packable).clone().try_into()?,
                                            None => auth_then_branch.clone(),
                                        };
                                        lift::helpers::AuthIB_0(
                                            builder,
                                            sequencer,
                                            pc.clone(),
                                            arg_0,
                                            arg_1,
                                            arg_2,
                                        )?
                                    }
                                        .into(),
                                );
                            let (else_args, _): (Vec<Value>, Vec<Type>) = vec![
                                assigns_2.get("target").unwrap()
                                .unpack_to_air_values_and_types(builder) ?
                            ]
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
                            let packable: lift::types::AirPackable = target
                                .clone()
                                .into();
                            let (packed, consumed) = packable
                                .pack_from_air_values_and_types(
                                    &end_args[consumed_total..],
                                    &block_param_types[consumed_total..],
                                )?;
                            assigns_1.insert("target".to_string(), packed);
                            consumed_total += consumed;
                        }
                        _ => return Err(AArch64LifterError::VariableNotExpectedEnum),
                    }
                }
                let (then_args, block_param_types): (Vec<Value>, Vec<Type>) = vec![
                    assigns_1.get("target").unwrap()
                    .unpack_to_air_values_and_types(builder) ?
                ]
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
                let (else_args, _): (Vec<Value>, Vec<Type>) = vec![
                    { let packable : lift::types::AirPackable = target.clone().into();
                    packable.unpack_to_air_values_and_types(builder) ? }
                ]
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
                let packable: lift::types::AirPackable = target.clone().into();
                let (packed, consumed) = packable
                    .pack_from_air_values_and_types(
                        &end_args[consumed_total..],
                        &block_param_types[consumed_total..],
                    )?;
                target = packed.try_into()?;
                consumed_total += consumed;
            }
            _ => return Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    {
        let cond = {
            let arg_0 = branch_type.clone();
            let arg_1 = lift::types::Variable::from(
                common::types::BranchType::BranchType_INDCALL,
            );
            lift::helpers::eq_enum_16(builder, sequencer, pc.clone(), arg_0, arg_1)?
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
                            let arg_1 = lift::types::Variable::from(
                                common::types::integer::from(4),
                            );
                            let arg_2 = lift::types::Variable::from(
                                common::types::integer::from(64),
                            );
                            lift::helpers::add_bits_int_0(
                                builder,
                                sequencer,
                                pc.clone(),
                                arg_0,
                                arg_1,
                                arg_2,
                            )?
                        };
                        let arg_1 = lift::types::Variable::from(
                            common::types::integer::from(30),
                        );
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
                        let arg_1 = lift::types::Variable::from(
                            common::types::integer::from(4),
                        );
                        let arg_2 = lift::types::Variable::from(
                            common::types::integer::from(64),
                        );
                        lift::helpers::add_bits_int_0(
                            builder,
                            sequencer,
                            pc.clone(),
                            arg_0,
                            arg_1,
                            arg_2,
                        )?
                    };
                    let arg_1 = lift::types::Variable::from(
                        common::types::integer::from(30),
                    );
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
        let expr = branch_type.clone();
        match &expr {
            lift::types::Variable::Rust(_) => {
                if (branch_type.clone()
                    == lift::types::Variable::from(
                        common::types::BranchType::BranchType_INDIR,
                    )) && true
                {
                    let mut assigns_1: BTreeMap<String, lift::types::AirPackable> = assigns_0
                        .clone();
                    {
                        let cond = lift::types::Variable::from(
                            common::types::boolean::FALSE,
                        );
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
                                        let cond = {
                                            let arg_0 = {
                                                let arg_0 = n.clone();
                                                let arg_1 = lift::types::Variable::from(
                                                    common::types::integer::from(16),
                                                );
                                                lift::helpers::eq_int_0(
                                                    builder,
                                                    sequencer,
                                                    pc.clone(),
                                                    arg_0,
                                                    arg_1,
                                                )?
                                            };
                                            let arg_1 = {
                                                let arg_0 = n.clone();
                                                let arg_1 = lift::types::Variable::from(
                                                    common::types::integer::from(17),
                                                );
                                                lift::helpers::eq_int_0(
                                                    builder,
                                                    sequencer,
                                                    pc.clone(),
                                                    arg_0,
                                                    arg_1,
                                                )?
                                            };
                                            lift::helpers::or_bool_0(
                                                builder,
                                                sequencer,
                                                pc.clone(),
                                                arg_0,
                                                arg_1,
                                            )?
                                        };
                                        match cond {
                                            lift::types::Variable::Rust(
                                                lift::types::RustVariable::boolean(b_inner),
                                            ) => {
                                                if b_inner == common::types::boolean::TRUE {
                                                    let mut assigns_3: BTreeMap<
                                                        String,
                                                        lift::types::AirPackable,
                                                    > = assigns_2.clone();
                                                    {
                                                        let arg_0 = lift::types::Register::BTypeNext;
                                                        lift::types::Variable::from(
                                                                common::types::bits::from_bits_literal("01")?,
                                                            )
                                                            .to_register(builder, arg_0)?;
                                                    }
                                                } else {
                                                    let mut assigns_3: BTreeMap<
                                                        String,
                                                        lift::types::AirPackable,
                                                    > = assigns_2.clone();
                                                    {
                                                        let arg_0 = lift::types::Register::BTypeNext;
                                                        lift::types::Variable::from(
                                                                common::types::bits::from_bits_literal("11")?,
                                                            )
                                                            .to_register(builder, arg_0)?;
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
                                                let mut assigns_3: BTreeMap<
                                                    String,
                                                    lift::types::AirPackable,
                                                > = assigns_2.clone();
                                                {
                                                    let arg_0 = lift::types::Register::BTypeNext;
                                                    lift::types::Variable::from(
                                                            common::types::bits::from_bits_literal("01")?,
                                                        )
                                                        .to_register(builder, arg_0)?;
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
                                                let mut assigns_3: BTreeMap<
                                                    String,
                                                    lift::types::AirPackable,
                                                > = assigns_2.clone();
                                                {
                                                    let arg_0 = lift::types::Register::BTypeNext;
                                                    lift::types::Variable::from(
                                                            common::types::bits::from_bits_literal("11")?,
                                                        )
                                                        .to_register(builder, arg_0)?;
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
                                } else {
                                    let mut assigns_2: BTreeMap<
                                        String,
                                        lift::types::AirPackable,
                                    > = assigns_1.clone();
                                    {
                                        let arg_0 = lift::types::Register::BTypeNext;
                                        lift::types::Variable::from(
                                                common::types::bits::from_bits_literal("01")?,
                                            )
                                            .to_register(builder, arg_0)?;
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
                                    let cond = {
                                        let arg_0 = {
                                            let arg_0 = match assigns_2.get("n") {
                                                Some(packable) => (*packable).clone().try_into()?,
                                                None => n.clone(),
                                            };
                                            let arg_1 = lift::types::Variable::from(
                                                common::types::integer::from(16),
                                            );
                                            lift::helpers::eq_int_0(
                                                builder,
                                                sequencer,
                                                pc.clone(),
                                                arg_0,
                                                arg_1,
                                            )?
                                        };
                                        let arg_1 = {
                                            let arg_0 = match assigns_2.get("n") {
                                                Some(packable) => (*packable).clone().try_into()?,
                                                None => n.clone(),
                                            };
                                            let arg_1 = lift::types::Variable::from(
                                                common::types::integer::from(17),
                                            );
                                            lift::helpers::eq_int_0(
                                                builder,
                                                sequencer,
                                                pc.clone(),
                                                arg_0,
                                                arg_1,
                                            )?
                                        };
                                        lift::helpers::or_bool_0(
                                            builder,
                                            sequencer,
                                            pc.clone(),
                                            arg_0,
                                            arg_1,
                                        )?
                                    };
                                    match cond {
                                        lift::types::Variable::Rust(
                                            lift::types::RustVariable::boolean(b_inner),
                                        ) => {
                                            if b_inner == common::types::boolean::TRUE {
                                                let mut assigns_3: BTreeMap<
                                                    String,
                                                    lift::types::AirPackable,
                                                > = assigns_2.clone();
                                                {
                                                    let arg_0 = lift::types::Register::BTypeNext;
                                                    lift::types::Variable::from(
                                                            common::types::bits::from_bits_literal("01")?,
                                                        )
                                                        .to_register(builder, arg_0)?;
                                                }
                                            } else {
                                                let mut assigns_3: BTreeMap<
                                                    String,
                                                    lift::types::AirPackable,
                                                > = assigns_2.clone();
                                                {
                                                    let arg_0 = lift::types::Register::BTypeNext;
                                                    lift::types::Variable::from(
                                                            common::types::bits::from_bits_literal("11")?,
                                                        )
                                                        .to_register(builder, arg_0)?;
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
                                            let mut assigns_3: BTreeMap<
                                                String,
                                                lift::types::AirPackable,
                                            > = assigns_2.clone();
                                            {
                                                let arg_0 = lift::types::Register::BTypeNext;
                                                lift::types::Variable::from(
                                                        common::types::bits::from_bits_literal("01")?,
                                                    )
                                                    .to_register(builder, arg_0)?;
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
                                            let mut assigns_3: BTreeMap<
                                                String,
                                                lift::types::AirPackable,
                                            > = assigns_2.clone();
                                            {
                                                let arg_0 = lift::types::Register::BTypeNext;
                                                lift::types::Variable::from(
                                                        common::types::bits::from_bits_literal("11")?,
                                                    )
                                                    .to_register(builder, arg_0)?;
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
                                    let arg_0 = lift::types::Register::BTypeNext;
                                    lift::types::Variable::from(
                                            common::types::bits::from_bits_literal("01")?,
                                        )
                                        .to_register(builder, arg_0)?;
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
                } else if (branch_type.clone()
                    == lift::types::Variable::from(
                        common::types::BranchType::BranchType_INDCALL,
                    )) && true
                {
                    let mut assigns_1: BTreeMap<String, lift::types::AirPackable> = assigns_0
                        .clone();
                    {
                        let arg_0 = lift::types::Register::BTypeNext;
                        lift::types::Variable::from(
                                common::types::bits::from_bits_literal("10")?,
                            )
                            .to_register(builder, arg_0)?;
                    }
                } else if (branch_type.clone()
                    == lift::types::Variable::from(
                        common::types::BranchType::BranchType_RET,
                    )) && true
                {
                    let mut assigns_1: BTreeMap<String, lift::types::AirPackable> = assigns_0
                        .clone();
                    {
                        let arg_0 = lift::types::Register::BTypeNext;
                        lift::types::Variable::from(
                                common::types::bits::from_bits_literal("00")?,
                            )
                            .to_register(builder, arg_0)?;
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
    {
        let arg_0 = target.clone();
        let arg_1 = branch_type.clone();
        let arg_2 = lift::types::Variable::from(common::types::integer::from(64));
        lift::helpers::BranchTo_0(builder, sequencer, pc.clone(), arg_0, arg_1, arg_2)?;
    }
    Ok(())
}

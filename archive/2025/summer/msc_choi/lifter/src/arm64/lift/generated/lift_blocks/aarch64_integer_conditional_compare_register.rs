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
pub fn lift_aarch64_integer_conditional_compare_register(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: lift::types::Variable,
    condition: common::types::bits,
    datasize: common::types::integer,
    flags: common::types::bits,
    m: common::types::integer,
    n: common::types::integer,
    sub_op: common::types::boolean,
) -> Result<(), AArch64LifterError> {
    let mut condition: lift::types::Variable = condition.into();
    let mut datasize: lift::types::Variable = datasize.into();
    let mut flags: lift::types::Variable = flags.into();
    let mut m: lift::types::Variable = m.into();
    let mut n: lift::types::Variable = n.into();
    let mut sub_op: lift::types::Variable = sub_op.into();
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
    let mut carry_in: lift::types::Variable = lift::types::Variable::from(
        common::types::bits::from_bits_literal("0")?,
    );
    {
        let cond = {
            let arg_0 = condition.clone();
            lift::helpers::ConditionHolds_0(builder, sequencer, pc.clone(), arg_0)?
        };
        match cond {
            lift::types::Variable::Rust(lift::types::RustVariable::boolean(b_inner)) => {
                if b_inner == common::types::boolean::TRUE {
                    let mut assigns_1: BTreeMap<String, lift::types::AirPackable> = assigns_0
                        .clone();
                    {
                        let cond = sub_op.clone();
                        match cond {
                            lift::types::Variable::Rust(
                                lift::types::RustVariable::boolean(b_inner),
                            ) => {
                                if b_inner == common::types::boolean::TRUE {
                                    let mut assigns_2: BTreeMap<
                                        String,
                                        lift::types::AirPackable,
                                    > = assigns_1.clone();
                                    operand2 = {
                                        let arg_0 = (operand2.clone());
                                        let arg_1 = datasize.clone();
                                        lift::helpers::not_bits_0(
                                            builder,
                                            sequencer,
                                            pc.clone(),
                                            arg_0,
                                            arg_1,
                                        )?
                                    };
                                    carry_in = lift::types::Variable::from(
                                        common::types::bits::from_bits_literal("1")?,
                                    );
                                } else {
                                    let mut assigns_2: BTreeMap<
                                        String,
                                        lift::types::AirPackable,
                                    > = assigns_1.clone();
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
                                        "operand2".to_string(),
                                        {
                                            let arg_0 = (match assigns_2.get("operand2") {
                                                Some(packable) => (*packable).clone().try_into()?,
                                                None => operand2.clone(),
                                            });
                                            let arg_1 = match assigns_2.get("datasize") {
                                                Some(packable) => (*packable).clone().try_into()?,
                                                None => datasize.clone(),
                                            };
                                            lift::helpers::not_bits_0(
                                                builder,
                                                sequencer,
                                                pc.clone(),
                                                arg_0,
                                                arg_1,
                                            )?
                                        }
                                            .into(),
                                    );
                                assigns_2
                                    .insert(
                                        "carry_in".to_string(),
                                        lift::types::Variable::from(
                                                common::types::bits::from_bits_literal("1")?,
                                            )
                                            .into(),
                                    );
                                let (
                                    then_args,
                                    block_param_types,
                                ): (Vec<Value>, Vec<Type>) = vec![
                                    assigns_2.get("carry_in").unwrap()
                                    .unpack_to_air_values_and_types(builder) ?, assigns_2
                                    .get("operand2").unwrap()
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
                                let (else_args, _): (Vec<Value>, Vec<Type>) = vec![
                                    { let packable : lift::types::AirPackable = carry_in.clone()
                                    .into(); packable.unpack_to_air_values_and_types(builder) ?
                                    }, { let packable : lift::types::AirPackable = operand2
                                    .clone().into(); packable
                                    .unpack_to_air_values_and_types(builder) ? }
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
                                let packable: lift::types::AirPackable = carry_in
                                    .clone()
                                    .into();
                                let (packed, consumed) = packable
                                    .pack_from_air_values_and_types(
                                        &end_args[consumed_total..],
                                        &block_param_types[consumed_total..],
                                    )?;
                                carry_in = packed.try_into()?;
                                consumed_total += consumed;
                                let packable: lift::types::AirPackable = operand2
                                    .clone()
                                    .into();
                                let (packed, consumed) = packable
                                    .pack_from_air_values_and_types(
                                        &end_args[consumed_total..],
                                        &block_param_types[consumed_total..],
                                    )?;
                                operand2 = packed.try_into()?;
                                consumed_total += consumed;
                            }
                            _ => return Err(AArch64LifterError::VariableNotExpectedEnum),
                        }
                    }
                    (_, flags) = {
                        let arg_0 = operand1.clone();
                        let arg_1 = operand2.clone();
                        let arg_2 = carry_in.clone();
                        let arg_3 = datasize.clone();
                        lift::helpers::AddWithCarry_0(
                            builder,
                            sequencer,
                            pc.clone(),
                            arg_0,
                            arg_1,
                            arg_2,
                            arg_3,
                        )?
                    };
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
                    let cond = match assigns_1.get("sub_op") {
                        Some(packable) => (*packable).clone().try_into()?,
                        None => sub_op.clone(),
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
                                        "operand2".to_string(),
                                        {
                                            let arg_0 = (match assigns_2.get("operand2") {
                                                Some(packable) => (*packable).clone().try_into()?,
                                                None => operand2.clone(),
                                            });
                                            let arg_1 = match assigns_2.get("datasize") {
                                                Some(packable) => (*packable).clone().try_into()?,
                                                None => datasize.clone(),
                                            };
                                            lift::helpers::not_bits_0(
                                                builder,
                                                sequencer,
                                                pc.clone(),
                                                arg_0,
                                                arg_1,
                                            )?
                                        }
                                            .into(),
                                    );
                                assigns_2
                                    .insert(
                                        "carry_in".to_string(),
                                        lift::types::Variable::from(
                                                common::types::bits::from_bits_literal("1")?,
                                            )
                                            .into(),
                                    );
                                assigns_1
                                    .insert(
                                        "carry_in".to_string(),
                                        assigns_2.get("carry_in").unwrap().clone(),
                                    );
                                assigns_1
                                    .insert(
                                        "operand2".to_string(),
                                        assigns_2.get("operand2").unwrap().clone(),
                                    );
                            } else {
                                let mut assigns_2: BTreeMap<
                                    String,
                                    lift::types::AirPackable,
                                > = assigns_1.clone();
                                assigns_1
                                    .entry("carry_in".to_string())
                                    .or_insert(carry_in.clone().into());
                                assigns_1
                                    .entry("operand2".to_string())
                                    .or_insert(operand2.clone().into());
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
                                    "operand2".to_string(),
                                    {
                                        let arg_0 = (match assigns_2.get("operand2") {
                                            Some(packable) => (*packable).clone().try_into()?,
                                            None => operand2.clone(),
                                        });
                                        let arg_1 = match assigns_2.get("datasize") {
                                            Some(packable) => (*packable).clone().try_into()?,
                                            None => datasize.clone(),
                                        };
                                        lift::helpers::not_bits_0(
                                            builder,
                                            sequencer,
                                            pc.clone(),
                                            arg_0,
                                            arg_1,
                                        )?
                                    }
                                        .into(),
                                );
                            assigns_2
                                .insert(
                                    "carry_in".to_string(),
                                    lift::types::Variable::from(
                                            common::types::bits::from_bits_literal("1")?,
                                        )
                                        .into(),
                                );
                            let (
                                then_args,
                                block_param_types,
                            ): (Vec<Value>, Vec<Type>) = vec![
                                assigns_2.get("carry_in").unwrap()
                                .unpack_to_air_values_and_types(builder) ?, assigns_2
                                .get("operand2").unwrap()
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
                            let (else_args, _): (Vec<Value>, Vec<Type>) = vec![
                                { let packable : lift::types::AirPackable = carry_in.clone()
                                .into(); packable.unpack_to_air_values_and_types(builder) ?
                                }, { let packable : lift::types::AirPackable = operand2
                                .clone().into(); packable
                                .unpack_to_air_values_and_types(builder) ? }
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
                            let packable: lift::types::AirPackable = carry_in
                                .clone()
                                .into();
                            let (packed, consumed) = packable
                                .pack_from_air_values_and_types(
                                    &end_args[consumed_total..],
                                    &block_param_types[consumed_total..],
                                )?;
                            assigns_1.insert("carry_in".to_string(), packed);
                            consumed_total += consumed;
                            let packable: lift::types::AirPackable = operand2
                                .clone()
                                .into();
                            let (packed, consumed) = packable
                                .pack_from_air_values_and_types(
                                    &end_args[consumed_total..],
                                    &block_param_types[consumed_total..],
                                )?;
                            assigns_1.insert("operand2".to_string(), packed);
                            consumed_total += consumed;
                        }
                        _ => return Err(AArch64LifterError::VariableNotExpectedEnum),
                    }
                }
                {
                    let rhs = {
                        let arg_0 = match assigns_1.get("operand1") {
                            Some(packable) => (*packable).clone().try_into()?,
                            None => operand1.clone(),
                        };
                        let arg_1 = match assigns_1.get("operand2") {
                            Some(packable) => (*packable).clone().try_into()?,
                            None => operand2.clone(),
                        };
                        let arg_2 = match assigns_1.get("carry_in") {
                            Some(packable) => (*packable).clone().try_into()?,
                            None => carry_in.clone(),
                        };
                        let arg_3 = match assigns_1.get("datasize") {
                            Some(packable) => (*packable).clone().try_into()?,
                            None => datasize.clone(),
                        };
                        lift::helpers::AddWithCarry_0(
                            builder,
                            sequencer,
                            pc.clone(),
                            arg_0,
                            arg_1,
                            arg_2,
                            arg_3,
                        )?
                    };
                    assigns_1.insert("flags".to_string(), rhs.1.into());
                }
                let (then_args, block_param_types): (Vec<Value>, Vec<Type>) = vec![
                    assigns_1.get("carry_in").unwrap()
                    .unpack_to_air_values_and_types(builder) ?, assigns_1.get("flags")
                    .unwrap().unpack_to_air_values_and_types(builder) ?, assigns_1
                    .get("operand2").unwrap().unpack_to_air_values_and_types(builder) ?
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
                    { let packable : lift::types::AirPackable = carry_in.clone().into();
                    packable.unpack_to_air_values_and_types(builder) ? }, { let packable
                    : lift::types::AirPackable = flags.clone().into(); packable
                    .unpack_to_air_values_and_types(builder) ? }, { let packable :
                    lift::types::AirPackable = operand2.clone().into(); packable
                    .unpack_to_air_values_and_types(builder) ? }
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
                let packable: lift::types::AirPackable = carry_in.clone().into();
                let (packed, consumed) = packable
                    .pack_from_air_values_and_types(
                        &end_args[consumed_total..],
                        &block_param_types[consumed_total..],
                    )?;
                carry_in = packed.try_into()?;
                consumed_total += consumed;
                let packable: lift::types::AirPackable = flags.clone().into();
                let (packed, consumed) = packable
                    .pack_from_air_values_and_types(
                        &end_args[consumed_total..],
                        &block_param_types[consumed_total..],
                    )?;
                flags = packed.try_into()?;
                consumed_total += consumed;
                let packable: lift::types::AirPackable = operand2.clone().into();
                let (packed, consumed) = packable
                    .pack_from_air_values_and_types(
                        &end_args[consumed_total..],
                        &block_param_types[consumed_total..],
                    )?;
                operand2 = packed.try_into()?;
                consumed_total += consumed;
            }
            _ => return Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    {
        let expr_ref = &(flags);
        let flag_value = expr_ref
            .extract_slice(
                builder,
                common::types::integer::from(0).into(),
                common::types::integer::one().into(),
            )?;
        flag_value.to_flag(builder, lift::types::Flag::V);
        let flag_value = expr_ref
            .extract_slice(
                builder,
                common::types::integer::from(1).into(),
                common::types::integer::one().into(),
            )?;
        flag_value.to_flag(builder, lift::types::Flag::C);
        let flag_value = expr_ref
            .extract_slice(
                builder,
                common::types::integer::from(2).into(),
                common::types::integer::one().into(),
            )?;
        flag_value.to_flag(builder, lift::types::Flag::Z);
        let flag_value = expr_ref
            .extract_slice(
                builder,
                common::types::integer::from(3).into(),
                common::types::integer::one().into(),
            )?;
        flag_value.to_flag(builder, lift::types::Flag::N);
    }
    Ok(())
}

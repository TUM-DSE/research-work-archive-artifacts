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
pub fn lift_aarch64_integer_logical_shiftedreg(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: lift::types::Variable,
    d: common::types::integer,
    datasize: common::types::integer,
    invert: common::types::boolean,
    m: common::types::integer,
    n: common::types::integer,
    op: common::types::LogicalOp,
    setflags: common::types::boolean,
    shift_amount: common::types::integer,
    shift_type: common::types::ShiftType,
) -> Result<(), AArch64LifterError> {
    let mut d: lift::types::Variable = d.into();
    let mut datasize: lift::types::Variable = datasize.into();
    let mut invert: lift::types::Variable = invert.into();
    let mut m: lift::types::Variable = m.into();
    let mut n: lift::types::Variable = n.into();
    let mut op: lift::types::Variable = op.into();
    let mut setflags: lift::types::Variable = setflags.into();
    let mut shift_amount: lift::types::Variable = shift_amount.into();
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
    let mut operand1: lift::types::Variable = {
        let arg_0 = n.clone();
        let arg_1 = datasize.clone();
        lift::helpers::X_read_0(builder, sequencer, pc.clone(), arg_0, arg_1)?
    };
    let mut operand2: lift::types::Variable = {
        let arg_0 = m.clone();
        let arg_1 = shift_type.clone();
        let arg_2 = shift_amount.clone();
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
        let cond = invert.clone();
        match cond {
            lift::types::Variable::Rust(lift::types::RustVariable::boolean(b_inner)) => {
                if b_inner == common::types::boolean::TRUE {
                    let mut assigns_1: BTreeMap<String, lift::types::AirPackable> = assigns_0
                        .clone();
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
                assigns_1
                    .insert(
                        "operand2".to_string(),
                        {
                            let arg_0 = (match assigns_1.get("operand2") {
                                Some(packable) => (*packable).clone().try_into()?,
                                None => operand2.clone(),
                            });
                            let arg_1 = match assigns_1.get("datasize") {
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
                let (then_args, block_param_types): (Vec<Value>, Vec<Type>) = vec![
                    assigns_1.get("operand2").unwrap()
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
                    { let packable : lift::types::AirPackable = operand2.clone().into();
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
        let expr = op.clone();
        match &expr {
            lift::types::Variable::Rust(_) => {
                if (op.clone()
                    == lift::types::Variable::from(
                        common::types::LogicalOp::LogicalOp_AND,
                    )) && true
                {
                    let mut assigns_1: BTreeMap<String, lift::types::AirPackable> = assigns_0
                        .clone();
                    result = {
                        let arg_0 = operand1.clone();
                        let arg_1 = operand2.clone();
                        let arg_2 = datasize.clone();
                        lift::helpers::and_bits_0(
                            builder,
                            sequencer,
                            pc.clone(),
                            arg_0,
                            arg_1,
                            arg_2,
                        )?
                    };
                } else if (op.clone()
                    == lift::types::Variable::from(
                        common::types::LogicalOp::LogicalOp_ORR,
                    )) && true
                {
                    let mut assigns_1: BTreeMap<String, lift::types::AirPackable> = assigns_0
                        .clone();
                    result = {
                        let arg_0 = operand1.clone();
                        let arg_1 = operand2.clone();
                        let arg_2 = datasize.clone();
                        lift::helpers::or_bits_0(
                            builder,
                            sequencer,
                            pc.clone(),
                            arg_0,
                            arg_1,
                            arg_2,
                        )?
                    };
                } else if (op.clone()
                    == lift::types::Variable::from(
                        common::types::LogicalOp::LogicalOp_EOR,
                    )) && true
                {
                    let mut assigns_1: BTreeMap<String, lift::types::AirPackable> = assigns_0
                        .clone();
                    result = {
                        let arg_0 = operand1.clone();
                        let arg_1 = operand2.clone();
                        let arg_2 = datasize.clone();
                        lift::helpers::eor_bits_0(
                            builder,
                            sequencer,
                            pc.clone(),
                            arg_0,
                            arg_1,
                            arg_2,
                        )?
                    };
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
        let cond = setflags.clone();
        match cond {
            lift::types::Variable::Rust(lift::types::RustVariable::boolean(b_inner)) => {
                if b_inner == common::types::boolean::TRUE {
                    let mut assigns_1: BTreeMap<String, lift::types::AirPackable> = assigns_0
                        .clone();
                    {
                        let expr_ref = &({
                            let arg_0 = {
                                let arg_0 = {
                                    let arg_0 = {
                                        let arg_0 = datasize.clone();
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
                                    let arg_1 = lift::types::Variable::from(
                                        common::types::integer::one(),
                                    );
                                    result.extract_slice(builder, arg_0, arg_1)?
                                };
                                let arg_1 = {
                                    let arg_0 = result.clone();
                                    let arg_1 = datasize.clone();
                                    lift::helpers::IsZeroBit_0(
                                        builder,
                                        sequencer,
                                        pc.clone(),
                                        arg_0,
                                        arg_1,
                                    )?
                                };
                                let arg_2 = {
                                    let arg_0 = lift::types::Variable::from(
                                        common::types::integer::from(0),
                                    );
                                    let arg_1 = lift::types::Variable::from(
                                        common::types::integer::from(1),
                                    );
                                    lift::helpers::add_int_0(
                                        builder,
                                        sequencer,
                                        pc.clone(),
                                        arg_0,
                                        arg_1,
                                    )?
                                };
                                let arg_3 = lift::types::Variable::from(
                                    common::types::integer::from(1),
                                );
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
                            let arg_1 = lift::types::Variable::from(
                                common::types::bits::from_bits_literal("00")?,
                            );
                            let arg_2 = {
                                let arg_0 = {
                                    let arg_0 = lift::types::Variable::from(
                                        common::types::integer::from(0),
                                    );
                                    let arg_1 = lift::types::Variable::from(
                                        common::types::integer::from(1),
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
                                    common::types::integer::from(1),
                                );
                                lift::helpers::add_int_0(
                                    builder,
                                    sequencer,
                                    pc.clone(),
                                    arg_0,
                                    arg_1,
                                )?
                            };
                            let arg_3 = lift::types::Variable::from(
                                common::types::integer::from(2),
                            );
                            lift::helpers::append_bits_0(
                                builder,
                                sequencer,
                                pc.clone(),
                                arg_0,
                                arg_1,
                                arg_2,
                                arg_3,
                            )?
                        });
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
                    let expr_ref = &({
                        let arg_0 = {
                            let arg_0 = {
                                let arg_0 = {
                                    let arg_0 = match assigns_1.get("datasize") {
                                        Some(packable) => (*packable).clone().try_into()?,
                                        None => datasize.clone(),
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
                                let arg_1 = lift::types::Variable::from(
                                    common::types::integer::one(),
                                );
                                match assigns_1.get("result") {
                                    Some(packable) => (*packable).clone().try_into()?,
                                    None => result.clone(),
                                }
                                    .extract_slice(builder, arg_0, arg_1)?
                            };
                            let arg_1 = {
                                let arg_0 = match assigns_1.get("result") {
                                    Some(packable) => (*packable).clone().try_into()?,
                                    None => result.clone(),
                                };
                                let arg_1 = match assigns_1.get("datasize") {
                                    Some(packable) => (*packable).clone().try_into()?,
                                    None => datasize.clone(),
                                };
                                lift::helpers::IsZeroBit_0(
                                    builder,
                                    sequencer,
                                    pc.clone(),
                                    arg_0,
                                    arg_1,
                                )?
                            };
                            let arg_2 = {
                                let arg_0 = lift::types::Variable::from(
                                    common::types::integer::from(0),
                                );
                                let arg_1 = lift::types::Variable::from(
                                    common::types::integer::from(1),
                                );
                                lift::helpers::add_int_0(
                                    builder,
                                    sequencer,
                                    pc.clone(),
                                    arg_0,
                                    arg_1,
                                )?
                            };
                            let arg_3 = lift::types::Variable::from(
                                common::types::integer::from(1),
                            );
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
                        let arg_1 = lift::types::Variable::from(
                            common::types::bits::from_bits_literal("00")?,
                        );
                        let arg_2 = {
                            let arg_0 = {
                                let arg_0 = lift::types::Variable::from(
                                    common::types::integer::from(0),
                                );
                                let arg_1 = lift::types::Variable::from(
                                    common::types::integer::from(1),
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
                                common::types::integer::from(1),
                            );
                            lift::helpers::add_int_0(
                                builder,
                                sequencer,
                                pc.clone(),
                                arg_0,
                                arg_1,
                            )?
                        };
                        let arg_3 = lift::types::Variable::from(
                            common::types::integer::from(2),
                        );
                        lift::helpers::append_bits_0(
                            builder,
                            sequencer,
                            pc.clone(),
                            arg_0,
                            arg_1,
                            arg_2,
                            arg_3,
                        )?
                    });
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
        let arg_0 = result.clone();
        let arg_1 = d.clone();
        let arg_2 = datasize.clone();
        lift::helpers::X_set_0(builder, sequencer, pc.clone(), arg_0, arg_1, arg_2)?;
    }
    Ok(())
}

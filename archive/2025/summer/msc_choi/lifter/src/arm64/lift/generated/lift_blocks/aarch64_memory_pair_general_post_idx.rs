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
pub fn lift_aarch64_memory_pair_general_post_idx(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: lift::types::Variable,
    acctype: common::types::AccType,
    datasize: common::types::integer,
    memop: common::types::MemOp,
    n: common::types::integer,
    offset: common::types::bits,
    postindex: common::types::boolean,
    scale: common::types::integer,
    signed: common::types::boolean,
    t: common::types::integer,
    t2: common::types::integer,
    tag_checked: common::types::boolean,
    wback: common::types::boolean,
) -> Result<(), AArch64LifterError> {
    let mut acctype: lift::types::Variable = acctype.into();
    let mut datasize: lift::types::Variable = datasize.into();
    let mut memop: lift::types::Variable = memop.into();
    let mut n: lift::types::Variable = n.into();
    let mut offset: lift::types::Variable = offset.into();
    let mut postindex: lift::types::Variable = postindex.into();
    let mut scale: lift::types::Variable = scale.into();
    let mut signed: lift::types::Variable = signed.into();
    let mut t: lift::types::Variable = t.into();
    let mut t2: lift::types::Variable = t2.into();
    let mut tag_checked: lift::types::Variable = tag_checked.into();
    let mut wback: lift::types::Variable = wback.into();
    let mut assigns_0: BTreeMap<String, lift::types::AirPackable> = BTreeMap::new();
    let mut address: lift::types::Variable = match lift::types::Variable::from(
        common::types::integer::from(64),
    ) {
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
    let mut data1: lift::types::Variable = match datasize.clone() {
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
    let mut data2: lift::types::Variable = match datasize.clone() {
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
    let dbytes: lift::types::Variable = {
        let arg_0 = datasize.clone();
        let arg_1 = lift::types::Variable::from(common::types::integer::from(8));
        lift::helpers::fdiv_int_0(builder, sequencer, pc.clone(), arg_0, arg_1)?
    };
    let mut rt_unknown: lift::types::Variable = lift::types::Variable::from(
        common::types::boolean::FALSE,
    );
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
    let mut wb_unknown: lift::types::Variable = lift::types::Variable::from(
        common::types::boolean::FALSE,
    );
    {
        let cond = {
            let arg_0 = {
                let arg_0 = {
                    let arg_0 = {
                        let arg_0 = memop.clone();
                        let arg_1 = lift::types::Variable::from(
                            common::types::MemOp::MemOp_LOAD,
                        );
                        lift::helpers::eq_enum_37(
                            builder,
                            sequencer,
                            pc.clone(),
                            arg_0,
                            arg_1,
                        )?
                    };
                    let arg_1 = wback.clone();
                    lift::helpers::and_bool_0(
                        builder,
                        sequencer,
                        pc.clone(),
                        arg_0,
                        arg_1,
                    )?
                };
                let arg_1 = ({
                    let arg_0 = {
                        let arg_0 = t.clone();
                        let arg_1 = n.clone();
                        lift::helpers::eq_int_0(
                            builder,
                            sequencer,
                            pc.clone(),
                            arg_0,
                            arg_1,
                        )?
                    };
                    let arg_1 = {
                        let arg_0 = t2.clone();
                        let arg_1 = n.clone();
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
                });
                lift::helpers::and_bool_0(builder, sequencer, pc.clone(), arg_0, arg_1)?
            };
            let arg_1 = {
                let arg_0 = n.clone();
                let arg_1 = lift::types::Variable::from(
                    common::types::integer::from(31),
                );
                lift::helpers::ne_int_0(builder, sequencer, pc.clone(), arg_0, arg_1)?
            };
            lift::helpers::and_bool_0(builder, sequencer, pc.clone(), arg_0, arg_1)?
        };
        match cond {
            lift::types::Variable::Rust(lift::types::RustVariable::boolean(b_inner)) => {
                if b_inner == common::types::boolean::TRUE {
                    let mut assigns_1: BTreeMap<String, lift::types::AirPackable> = assigns_0
                        .clone();
                    let mut c: lift::types::Variable = {
                        let arg_0 = lift::types::Variable::from(
                            common::types::Unpredictable::Unpredictable_WBOVERLAPLD,
                        );
                        lift::helpers::ConstrainUnpredictable_0(
                            builder,
                            sequencer,
                            pc.clone(),
                            arg_0,
                        )?
                    };
                    match {
                        let expr = c.clone();
                        match &expr {
                            lift::types::Variable::Rust(_) => {
                                if [
                                    lift::types::Variable::from(
                                        common::types::Constraint::Constraint_WBSUPPRESS,
                                    ),
                                    lift::types::Variable::from(
                                        common::types::Constraint::Constraint_UNKNOWN,
                                    ),
                                    lift::types::Variable::from(
                                        common::types::Constraint::Constraint_UNDEF,
                                    ),
                                    lift::types::Variable::from(
                                        common::types::Constraint::Constraint_NOP,
                                    ),
                                ]
                                    .contains(&expr)
                                {
                                    lift::types::Variable::from(common::types::boolean::TRUE)
                                } else {
                                    lift::types::Variable::from(common::types::boolean::FALSE)
                                }
                            }
                            lift::types::Variable::Air(_) => {
                                return Err(
                                    AArch64LifterError::NotImplemented(file!(), line!()),
                                );
                            }
                        }
                    } {
                        lift::types::Variable::Rust(
                            lift::types::RustVariable::boolean(b_inner),
                        ) => assert_eq!(b_inner, common::types::boolean::TRUE),
                        lift::types::Variable::Air(_) => {}
                        _ => return Err(AArch64LifterError::VariableNotExpectedEnum),
                    }
                    {
                        let expr = c.clone();
                        match &expr {
                            lift::types::Variable::Rust(_) => {
                                if (c.clone()
                                    == lift::types::Variable::from(
                                        common::types::Constraint::Constraint_WBSUPPRESS,
                                    )) && true
                                {
                                    let mut assigns_2: BTreeMap<
                                        String,
                                        lift::types::AirPackable,
                                    > = assigns_1.clone();
                                    wback = lift::types::Variable::from(
                                        common::types::boolean::FALSE,
                                    );
                                } else if (c.clone()
                                    == lift::types::Variable::from(
                                        common::types::Constraint::Constraint_UNKNOWN,
                                    )) && true
                                {
                                    let mut assigns_2: BTreeMap<
                                        String,
                                        lift::types::AirPackable,
                                    > = assigns_1.clone();
                                    wb_unknown = lift::types::Variable::from(
                                        common::types::boolean::TRUE,
                                    );
                                } else if (c.clone()
                                    == lift::types::Variable::from(
                                        common::types::Constraint::Constraint_UNDEF,
                                    )) && true
                                {
                                    let mut assigns_2: BTreeMap<
                                        String,
                                        lift::types::AirPackable,
                                    > = assigns_1.clone();
                                    panic!(
                                        "Unimplemented instruction in {} at line {}",
                                        "codegen/src/asl/lift_transpiler.rs", 692u32
                                    );
                                } else if (c.clone()
                                    == lift::types::Variable::from(
                                        common::types::Constraint::Constraint_NOP,
                                    )) && true
                                {
                                    let mut assigns_2: BTreeMap<
                                        String,
                                        lift::types::AirPackable,
                                    > = assigns_1.clone();
                                    {
                                        lift::helpers::EndOfInstruction_0(
                                            builder,
                                            sequencer,
                                            pc.clone(),
                                        )?;
                                    }
                                } else {
                                    let mut assigns_2: BTreeMap<
                                        String,
                                        lift::types::AirPackable,
                                    > = assigns_1.clone();
                                }
                            }
                            lift::types::Variable::Air(_) => {
                                return Err(
                                    AArch64LifterError::NotImplemented(file!(), line!()),
                                );
                            }
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
                let mut c: lift::types::Variable = {
                    let arg_0 = lift::types::Variable::from(
                        common::types::Unpredictable::Unpredictable_WBOVERLAPLD,
                    );
                    lift::helpers::ConstrainUnpredictable_0(
                        builder,
                        sequencer,
                        pc.clone(),
                        arg_0,
                    )?
                };
                {
                    let expr = match assigns_1.get("c") {
                        Some(packable) => (*packable).clone().try_into()?,
                        None => c.clone(),
                    };
                    match &expr {
                        lift::types::Variable::Rust(_) => {
                            if (match assigns_1.get("c") {
                                Some(packable) => (*packable).clone().try_into()?,
                                None => c.clone(),
                            }
                                == lift::types::Variable::from(
                                    common::types::Constraint::Constraint_WBSUPPRESS,
                                )) && true
                            {
                                let mut assigns_2: BTreeMap<
                                    String,
                                    lift::types::AirPackable,
                                > = assigns_1.clone();
                                wback = lift::types::Variable::from(
                                    common::types::boolean::FALSE,
                                );
                            } else if (match assigns_1.get("c") {
                                Some(packable) => (*packable).clone().try_into()?,
                                None => c.clone(),
                            }
                                == lift::types::Variable::from(
                                    common::types::Constraint::Constraint_UNKNOWN,
                                )) && true
                            {
                                let mut assigns_2: BTreeMap<
                                    String,
                                    lift::types::AirPackable,
                                > = assigns_1.clone();
                                wb_unknown = lift::types::Variable::from(
                                    common::types::boolean::TRUE,
                                );
                            } else if (match assigns_1.get("c") {
                                Some(packable) => (*packable).clone().try_into()?,
                                None => c.clone(),
                            }
                                == lift::types::Variable::from(
                                    common::types::Constraint::Constraint_UNDEF,
                                )) && true
                            {
                                let mut assigns_2: BTreeMap<
                                    String,
                                    lift::types::AirPackable,
                                > = assigns_1.clone();
                                panic!(
                                    "Unimplemented instruction in {} at line {}",
                                    "codegen/src/asl/lift_transpiler.rs", 692u32
                                );
                            } else if (match assigns_1.get("c") {
                                Some(packable) => (*packable).clone().try_into()?,
                                None => c.clone(),
                            }
                                == lift::types::Variable::from(
                                    common::types::Constraint::Constraint_NOP,
                                )) && true
                            {
                                let mut assigns_2: BTreeMap<
                                    String,
                                    lift::types::AirPackable,
                                > = assigns_1.clone();
                                {
                                    lift::helpers::EndOfInstruction_0(
                                        builder,
                                        sequencer,
                                        pc.clone(),
                                    )?;
                                }
                            } else {
                                let mut assigns_2: BTreeMap<
                                    String,
                                    lift::types::AirPackable,
                                > = assigns_1.clone();
                            }
                        }
                        lift::types::Variable::Air(_) => {
                            return Err(
                                AArch64LifterError::NotImplemented(file!(), line!()),
                            );
                        }
                    }
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
        let cond = {
            let arg_0 = {
                let arg_0 = {
                    let arg_0 = {
                        let arg_0 = memop.clone();
                        let arg_1 = lift::types::Variable::from(
                            common::types::MemOp::MemOp_STORE,
                        );
                        lift::helpers::eq_enum_37(
                            builder,
                            sequencer,
                            pc.clone(),
                            arg_0,
                            arg_1,
                        )?
                    };
                    let arg_1 = wback.clone();
                    lift::helpers::and_bool_0(
                        builder,
                        sequencer,
                        pc.clone(),
                        arg_0,
                        arg_1,
                    )?
                };
                let arg_1 = ({
                    let arg_0 = {
                        let arg_0 = t.clone();
                        let arg_1 = n.clone();
                        lift::helpers::eq_int_0(
                            builder,
                            sequencer,
                            pc.clone(),
                            arg_0,
                            arg_1,
                        )?
                    };
                    let arg_1 = {
                        let arg_0 = t2.clone();
                        let arg_1 = n.clone();
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
                });
                lift::helpers::and_bool_0(builder, sequencer, pc.clone(), arg_0, arg_1)?
            };
            let arg_1 = {
                let arg_0 = n.clone();
                let arg_1 = lift::types::Variable::from(
                    common::types::integer::from(31),
                );
                lift::helpers::ne_int_0(builder, sequencer, pc.clone(), arg_0, arg_1)?
            };
            lift::helpers::and_bool_0(builder, sequencer, pc.clone(), arg_0, arg_1)?
        };
        match cond {
            lift::types::Variable::Rust(lift::types::RustVariable::boolean(b_inner)) => {
                if b_inner == common::types::boolean::TRUE {
                    let mut assigns_1: BTreeMap<String, lift::types::AirPackable> = assigns_0
                        .clone();
                    let mut c: lift::types::Variable = {
                        let arg_0 = lift::types::Variable::from(
                            common::types::Unpredictable::Unpredictable_WBOVERLAPST,
                        );
                        lift::helpers::ConstrainUnpredictable_0(
                            builder,
                            sequencer,
                            pc.clone(),
                            arg_0,
                        )?
                    };
                    match {
                        let expr = c.clone();
                        match &expr {
                            lift::types::Variable::Rust(_) => {
                                if [
                                    lift::types::Variable::from(
                                        common::types::Constraint::Constraint_NONE,
                                    ),
                                    lift::types::Variable::from(
                                        common::types::Constraint::Constraint_UNKNOWN,
                                    ),
                                    lift::types::Variable::from(
                                        common::types::Constraint::Constraint_UNDEF,
                                    ),
                                    lift::types::Variable::from(
                                        common::types::Constraint::Constraint_NOP,
                                    ),
                                ]
                                    .contains(&expr)
                                {
                                    lift::types::Variable::from(common::types::boolean::TRUE)
                                } else {
                                    lift::types::Variable::from(common::types::boolean::FALSE)
                                }
                            }
                            lift::types::Variable::Air(_) => {
                                return Err(
                                    AArch64LifterError::NotImplemented(file!(), line!()),
                                );
                            }
                        }
                    } {
                        lift::types::Variable::Rust(
                            lift::types::RustVariable::boolean(b_inner),
                        ) => assert_eq!(b_inner, common::types::boolean::TRUE),
                        lift::types::Variable::Air(_) => {}
                        _ => return Err(AArch64LifterError::VariableNotExpectedEnum),
                    }
                    {
                        let expr = c.clone();
                        match &expr {
                            lift::types::Variable::Rust(_) => {
                                if (c.clone()
                                    == lift::types::Variable::from(
                                        common::types::Constraint::Constraint_NONE,
                                    )) && true
                                {
                                    let mut assigns_2: BTreeMap<
                                        String,
                                        lift::types::AirPackable,
                                    > = assigns_1.clone();
                                    rt_unknown = lift::types::Variable::from(
                                        common::types::boolean::FALSE,
                                    );
                                } else if (c.clone()
                                    == lift::types::Variable::from(
                                        common::types::Constraint::Constraint_UNKNOWN,
                                    )) && true
                                {
                                    let mut assigns_2: BTreeMap<
                                        String,
                                        lift::types::AirPackable,
                                    > = assigns_1.clone();
                                    rt_unknown = lift::types::Variable::from(
                                        common::types::boolean::TRUE,
                                    );
                                } else if (c.clone()
                                    == lift::types::Variable::from(
                                        common::types::Constraint::Constraint_UNDEF,
                                    )) && true
                                {
                                    let mut assigns_2: BTreeMap<
                                        String,
                                        lift::types::AirPackable,
                                    > = assigns_1.clone();
                                    panic!(
                                        "Unimplemented instruction in {} at line {}",
                                        "codegen/src/asl/lift_transpiler.rs", 692u32
                                    );
                                } else if (c.clone()
                                    == lift::types::Variable::from(
                                        common::types::Constraint::Constraint_NOP,
                                    )) && true
                                {
                                    let mut assigns_2: BTreeMap<
                                        String,
                                        lift::types::AirPackable,
                                    > = assigns_1.clone();
                                    {
                                        lift::helpers::EndOfInstruction_0(
                                            builder,
                                            sequencer,
                                            pc.clone(),
                                        )?;
                                    }
                                } else {
                                    let mut assigns_2: BTreeMap<
                                        String,
                                        lift::types::AirPackable,
                                    > = assigns_1.clone();
                                }
                            }
                            lift::types::Variable::Air(_) => {
                                return Err(
                                    AArch64LifterError::NotImplemented(file!(), line!()),
                                );
                            }
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
                let mut c: lift::types::Variable = {
                    let arg_0 = lift::types::Variable::from(
                        common::types::Unpredictable::Unpredictable_WBOVERLAPST,
                    );
                    lift::helpers::ConstrainUnpredictable_0(
                        builder,
                        sequencer,
                        pc.clone(),
                        arg_0,
                    )?
                };
                {
                    let expr = match assigns_1.get("c") {
                        Some(packable) => (*packable).clone().try_into()?,
                        None => c.clone(),
                    };
                    match &expr {
                        lift::types::Variable::Rust(_) => {
                            if (match assigns_1.get("c") {
                                Some(packable) => (*packable).clone().try_into()?,
                                None => c.clone(),
                            }
                                == lift::types::Variable::from(
                                    common::types::Constraint::Constraint_NONE,
                                )) && true
                            {
                                let mut assigns_2: BTreeMap<
                                    String,
                                    lift::types::AirPackable,
                                > = assigns_1.clone();
                                rt_unknown = lift::types::Variable::from(
                                    common::types::boolean::FALSE,
                                );
                            } else if (match assigns_1.get("c") {
                                Some(packable) => (*packable).clone().try_into()?,
                                None => c.clone(),
                            }
                                == lift::types::Variable::from(
                                    common::types::Constraint::Constraint_UNKNOWN,
                                )) && true
                            {
                                let mut assigns_2: BTreeMap<
                                    String,
                                    lift::types::AirPackable,
                                > = assigns_1.clone();
                                rt_unknown = lift::types::Variable::from(
                                    common::types::boolean::TRUE,
                                );
                            } else if (match assigns_1.get("c") {
                                Some(packable) => (*packable).clone().try_into()?,
                                None => c.clone(),
                            }
                                == lift::types::Variable::from(
                                    common::types::Constraint::Constraint_UNDEF,
                                )) && true
                            {
                                let mut assigns_2: BTreeMap<
                                    String,
                                    lift::types::AirPackable,
                                > = assigns_1.clone();
                                panic!(
                                    "Unimplemented instruction in {} at line {}",
                                    "codegen/src/asl/lift_transpiler.rs", 692u32
                                );
                            } else if (match assigns_1.get("c") {
                                Some(packable) => (*packable).clone().try_into()?,
                                None => c.clone(),
                            }
                                == lift::types::Variable::from(
                                    common::types::Constraint::Constraint_NOP,
                                )) && true
                            {
                                let mut assigns_2: BTreeMap<
                                    String,
                                    lift::types::AirPackable,
                                > = assigns_1.clone();
                                {
                                    lift::helpers::EndOfInstruction_0(
                                        builder,
                                        sequencer,
                                        pc.clone(),
                                    )?;
                                }
                            } else {
                                let mut assigns_2: BTreeMap<
                                    String,
                                    lift::types::AirPackable,
                                > = assigns_1.clone();
                            }
                        }
                        lift::types::Variable::Air(_) => {
                            return Err(
                                AArch64LifterError::NotImplemented(file!(), line!()),
                            );
                        }
                    }
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
        let cond = {
            let arg_0 = {
                let arg_0 = memop.clone();
                let arg_1 = lift::types::Variable::from(
                    common::types::MemOp::MemOp_LOAD,
                );
                lift::helpers::eq_enum_37(builder, sequencer, pc.clone(), arg_0, arg_1)?
            };
            let arg_1 = {
                let arg_0 = t.clone();
                let arg_1 = t2.clone();
                lift::helpers::eq_int_0(builder, sequencer, pc.clone(), arg_0, arg_1)?
            };
            lift::helpers::and_bool_0(builder, sequencer, pc.clone(), arg_0, arg_1)?
        };
        match cond {
            lift::types::Variable::Rust(lift::types::RustVariable::boolean(b_inner)) => {
                if b_inner == common::types::boolean::TRUE {
                    let mut assigns_1: BTreeMap<String, lift::types::AirPackable> = assigns_0
                        .clone();
                    let mut c: lift::types::Variable = {
                        let arg_0 = lift::types::Variable::from(
                            common::types::Unpredictable::Unpredictable_LDPOVERLAP,
                        );
                        lift::helpers::ConstrainUnpredictable_0(
                            builder,
                            sequencer,
                            pc.clone(),
                            arg_0,
                        )?
                    };
                    match {
                        let expr = c.clone();
                        match &expr {
                            lift::types::Variable::Rust(_) => {
                                if [
                                    lift::types::Variable::from(
                                        common::types::Constraint::Constraint_UNKNOWN,
                                    ),
                                    lift::types::Variable::from(
                                        common::types::Constraint::Constraint_UNDEF,
                                    ),
                                    lift::types::Variable::from(
                                        common::types::Constraint::Constraint_NOP,
                                    ),
                                ]
                                    .contains(&expr)
                                {
                                    lift::types::Variable::from(common::types::boolean::TRUE)
                                } else {
                                    lift::types::Variable::from(common::types::boolean::FALSE)
                                }
                            }
                            lift::types::Variable::Air(_) => {
                                return Err(
                                    AArch64LifterError::NotImplemented(file!(), line!()),
                                );
                            }
                        }
                    } {
                        lift::types::Variable::Rust(
                            lift::types::RustVariable::boolean(b_inner),
                        ) => assert_eq!(b_inner, common::types::boolean::TRUE),
                        lift::types::Variable::Air(_) => {}
                        _ => return Err(AArch64LifterError::VariableNotExpectedEnum),
                    }
                    {
                        let expr = c.clone();
                        match &expr {
                            lift::types::Variable::Rust(_) => {
                                if (c.clone()
                                    == lift::types::Variable::from(
                                        common::types::Constraint::Constraint_UNKNOWN,
                                    )) && true
                                {
                                    let mut assigns_2: BTreeMap<
                                        String,
                                        lift::types::AirPackable,
                                    > = assigns_1.clone();
                                    rt_unknown = lift::types::Variable::from(
                                        common::types::boolean::TRUE,
                                    );
                                } else if (c.clone()
                                    == lift::types::Variable::from(
                                        common::types::Constraint::Constraint_UNDEF,
                                    )) && true
                                {
                                    let mut assigns_2: BTreeMap<
                                        String,
                                        lift::types::AirPackable,
                                    > = assigns_1.clone();
                                    panic!(
                                        "Unimplemented instruction in {} at line {}",
                                        "codegen/src/asl/lift_transpiler.rs", 692u32
                                    );
                                } else if (c.clone()
                                    == lift::types::Variable::from(
                                        common::types::Constraint::Constraint_NOP,
                                    )) && true
                                {
                                    let mut assigns_2: BTreeMap<
                                        String,
                                        lift::types::AirPackable,
                                    > = assigns_1.clone();
                                    {
                                        lift::helpers::EndOfInstruction_0(
                                            builder,
                                            sequencer,
                                            pc.clone(),
                                        )?;
                                    }
                                } else {
                                    let mut assigns_2: BTreeMap<
                                        String,
                                        lift::types::AirPackable,
                                    > = assigns_1.clone();
                                }
                            }
                            lift::types::Variable::Air(_) => {
                                return Err(
                                    AArch64LifterError::NotImplemented(file!(), line!()),
                                );
                            }
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
                let mut c: lift::types::Variable = {
                    let arg_0 = lift::types::Variable::from(
                        common::types::Unpredictable::Unpredictable_LDPOVERLAP,
                    );
                    lift::helpers::ConstrainUnpredictable_0(
                        builder,
                        sequencer,
                        pc.clone(),
                        arg_0,
                    )?
                };
                {
                    let expr = match assigns_1.get("c") {
                        Some(packable) => (*packable).clone().try_into()?,
                        None => c.clone(),
                    };
                    match &expr {
                        lift::types::Variable::Rust(_) => {
                            if (match assigns_1.get("c") {
                                Some(packable) => (*packable).clone().try_into()?,
                                None => c.clone(),
                            }
                                == lift::types::Variable::from(
                                    common::types::Constraint::Constraint_UNKNOWN,
                                )) && true
                            {
                                let mut assigns_2: BTreeMap<
                                    String,
                                    lift::types::AirPackable,
                                > = assigns_1.clone();
                                rt_unknown = lift::types::Variable::from(
                                    common::types::boolean::TRUE,
                                );
                            } else if (match assigns_1.get("c") {
                                Some(packable) => (*packable).clone().try_into()?,
                                None => c.clone(),
                            }
                                == lift::types::Variable::from(
                                    common::types::Constraint::Constraint_UNDEF,
                                )) && true
                            {
                                let mut assigns_2: BTreeMap<
                                    String,
                                    lift::types::AirPackable,
                                > = assigns_1.clone();
                                panic!(
                                    "Unimplemented instruction in {} at line {}",
                                    "codegen/src/asl/lift_transpiler.rs", 692u32
                                );
                            } else if (match assigns_1.get("c") {
                                Some(packable) => (*packable).clone().try_into()?,
                                None => c.clone(),
                            }
                                == lift::types::Variable::from(
                                    common::types::Constraint::Constraint_NOP,
                                )) && true
                            {
                                let mut assigns_2: BTreeMap<
                                    String,
                                    lift::types::AirPackable,
                                > = assigns_1.clone();
                                {
                                    lift::helpers::EndOfInstruction_0(
                                        builder,
                                        sequencer,
                                        pc.clone(),
                                    )?;
                                }
                            } else {
                                let mut assigns_2: BTreeMap<
                                    String,
                                    lift::types::AirPackable,
                                > = assigns_1.clone();
                            }
                        }
                        lift::types::Variable::Air(_) => {
                            return Err(
                                AArch64LifterError::NotImplemented(file!(), line!()),
                            );
                        }
                    }
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
        let cond = {
            let arg_0 = n.clone();
            let arg_1 = lift::types::Variable::from(common::types::integer::from(31));
            lift::helpers::eq_int_0(builder, sequencer, pc.clone(), arg_0, arg_1)?
        };
        match cond {
            lift::types::Variable::Rust(lift::types::RustVariable::boolean(b_inner)) => {
                if b_inner == common::types::boolean::TRUE {
                    let mut assigns_1: BTreeMap<String, lift::types::AirPackable> = assigns_0
                        .clone();
                    {
                        lift::helpers::CheckSPAlignment_0(
                            builder,
                            sequencer,
                            pc.clone(),
                        )?;
                    }
                    address = {
                        let arg_0 = lift::types::Variable::from(
                            common::types::integer::from(64),
                        );
                        lift::helpers::SP_read_1(builder, sequencer, pc.clone(), arg_0)?
                    };
                } else {
                    let mut assigns_1: BTreeMap<String, lift::types::AirPackable> = assigns_0
                        .clone();
                    address = {
                        let arg_0 = n.clone();
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
                let mut assigns_1: BTreeMap<String, lift::types::AirPackable> = assigns_0
                    .clone();
                {
                    lift::helpers::CheckSPAlignment_0(builder, sequencer, pc.clone())?;
                }
                assigns_1
                    .insert(
                        "address".to_string(),
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
                            .into(),
                    );
                let (then_args, block_param_types): (Vec<Value>, Vec<Type>) = vec![
                    assigns_1.get("address").unwrap()
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
                assigns_1
                    .insert(
                        "address".to_string(),
                        {
                            let arg_0 = match assigns_1.get("n") {
                                Some(packable) => (*packable).clone().try_into()?,
                                None => n.clone(),
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
                            .into(),
                    );
                let (else_args, _): (Vec<Value>, Vec<Type>) = vec![
                    assigns_1.get("address").unwrap()
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
                        .push(Value::from(builder.get_block_param(end_block, i as u32)));
                }
                let mut consumed_total = 0;
                let packable: lift::types::AirPackable = address.clone().into();
                let (packed, consumed) = packable
                    .pack_from_air_values_and_types(
                        &end_args[consumed_total..],
                        &block_param_types[consumed_total..],
                    )?;
                address = packed.try_into()?;
                consumed_total += consumed;
            }
            _ => return Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    {
        let cond = {
            let arg_0 = postindex.clone();
            lift::helpers::not_bool_0(builder, sequencer, pc.clone(), arg_0)?
        };
        match cond {
            lift::types::Variable::Rust(lift::types::RustVariable::boolean(b_inner)) => {
                if b_inner == common::types::boolean::TRUE {
                    let mut assigns_1: BTreeMap<String, lift::types::AirPackable> = assigns_0
                        .clone();
                    address = {
                        let arg_0 = address.clone();
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
                        "address".to_string(),
                        {
                            let arg_0 = match assigns_1.get("address") {
                                Some(packable) => (*packable).clone().try_into()?,
                                None => address.clone(),
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
                        }
                            .into(),
                    );
                let (then_args, block_param_types): (Vec<Value>, Vec<Type>) = vec![
                    assigns_1.get("address").unwrap()
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
                    { let packable : lift::types::AirPackable = address.clone().into();
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
                let packable: lift::types::AirPackable = address.clone().into();
                let (packed, consumed) = packable
                    .pack_from_air_values_and_types(
                        &end_args[consumed_total..],
                        &block_param_types[consumed_total..],
                    )?;
                address = packed.try_into()?;
                consumed_total += consumed;
            }
            _ => return Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    {
        let expr = memop.clone();
        match &expr {
            lift::types::Variable::Rust(_) => {
                if (memop.clone()
                    == lift::types::Variable::from(common::types::MemOp::MemOp_STORE))
                    && true
                {
                    let mut assigns_1: BTreeMap<String, lift::types::AirPackable> = assigns_0
                        .clone();
                    {
                        let cond = {
                            let arg_0 = rt_unknown.clone();
                            let arg_1 = {
                                let arg_0 = t.clone();
                                let arg_1 = n.clone();
                                lift::helpers::eq_int_0(
                                    builder,
                                    sequencer,
                                    pc.clone(),
                                    arg_0,
                                    arg_1,
                                )?
                            };
                            lift::helpers::and_bool_0(
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
                                    let mut assigns_2: BTreeMap<
                                        String,
                                        lift::types::AirPackable,
                                    > = assigns_1.clone();
                                    data1 = match datasize.clone() {
                                        lift::types::Variable::Rust(
                                            lift::types::RustVariable::integer(i_inner),
                                        ) => {
                                            common::types::bits::new(0, integer_to_usize!(i_inner))
                                                .into()
                                        }
                                        lift::types::Variable::Air(a_inner) => {
                                            lift::types::Variable::air_from_bits(
                                                builder,
                                                common::types::bits::from_bits_literal("0")?,
                                            )?
                                        }
                                        _ => return Err(AArch64LifterError::VariableNotExpectedEnum),
                                    };
                                } else {
                                    let mut assigns_2: BTreeMap<
                                        String,
                                        lift::types::AirPackable,
                                    > = assigns_1.clone();
                                    data1 = {
                                        let arg_0 = t.clone();
                                        let arg_1 = datasize.clone();
                                        lift::helpers::X_read_0(
                                            builder,
                                            sequencer,
                                            pc.clone(),
                                            arg_0,
                                            arg_1,
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
                                        "data1".to_string(),
                                        match match assigns_2.get("datasize") {
                                            Some(packable) => (*packable).clone().try_into()?,
                                            None => datasize.clone(),
                                        } {
                                            lift::types::Variable::Rust(
                                                lift::types::RustVariable::integer(i_inner),
                                            ) => {
                                                common::types::bits::new(0, integer_to_usize!(i_inner))
                                                    .into()
                                            }
                                            lift::types::Variable::Air(a_inner) => {
                                                lift::types::Variable::air_from_bits(
                                                    builder,
                                                    common::types::bits::from_bits_literal("0")?,
                                                )?
                                            }
                                            _ => return Err(AArch64LifterError::VariableNotExpectedEnum),
                                        }
                                            .into(),
                                    );
                                let (
                                    then_args,
                                    block_param_types,
                                ): (Vec<Value>, Vec<Type>) = vec![
                                    assigns_2.get("data1").unwrap()
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
                                        "data1".to_string(),
                                        {
                                            let arg_0 = match assigns_2.get("t") {
                                                Some(packable) => (*packable).clone().try_into()?,
                                                None => t.clone(),
                                            };
                                            let arg_1 = match assigns_2.get("datasize") {
                                                Some(packable) => (*packable).clone().try_into()?,
                                                None => datasize.clone(),
                                            };
                                            lift::helpers::X_read_0(
                                                builder,
                                                sequencer,
                                                pc.clone(),
                                                arg_0,
                                                arg_1,
                                            )?
                                        }
                                            .into(),
                                    );
                                let (else_args, _): (Vec<Value>, Vec<Type>) = vec![
                                    assigns_2.get("data1").unwrap()
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
                                let packable: lift::types::AirPackable = data1
                                    .clone()
                                    .into();
                                let (packed, consumed) = packable
                                    .pack_from_air_values_and_types(
                                        &end_args[consumed_total..],
                                        &block_param_types[consumed_total..],
                                    )?;
                                data1 = packed.try_into()?;
                                consumed_total += consumed;
                            }
                            _ => return Err(AArch64LifterError::VariableNotExpectedEnum),
                        }
                    }
                    {
                        let cond = {
                            let arg_0 = rt_unknown.clone();
                            let arg_1 = {
                                let arg_0 = t2.clone();
                                let arg_1 = n.clone();
                                lift::helpers::eq_int_0(
                                    builder,
                                    sequencer,
                                    pc.clone(),
                                    arg_0,
                                    arg_1,
                                )?
                            };
                            lift::helpers::and_bool_0(
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
                                    let mut assigns_2: BTreeMap<
                                        String,
                                        lift::types::AirPackable,
                                    > = assigns_1.clone();
                                    data2 = match datasize.clone() {
                                        lift::types::Variable::Rust(
                                            lift::types::RustVariable::integer(i_inner),
                                        ) => {
                                            common::types::bits::new(0, integer_to_usize!(i_inner))
                                                .into()
                                        }
                                        lift::types::Variable::Air(a_inner) => {
                                            lift::types::Variable::air_from_bits(
                                                builder,
                                                common::types::bits::from_bits_literal("0")?,
                                            )?
                                        }
                                        _ => return Err(AArch64LifterError::VariableNotExpectedEnum),
                                    };
                                } else {
                                    let mut assigns_2: BTreeMap<
                                        String,
                                        lift::types::AirPackable,
                                    > = assigns_1.clone();
                                    data2 = {
                                        let arg_0 = t2.clone();
                                        let arg_1 = datasize.clone();
                                        lift::helpers::X_read_0(
                                            builder,
                                            sequencer,
                                            pc.clone(),
                                            arg_0,
                                            arg_1,
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
                                        "data2".to_string(),
                                        match match assigns_2.get("datasize") {
                                            Some(packable) => (*packable).clone().try_into()?,
                                            None => datasize.clone(),
                                        } {
                                            lift::types::Variable::Rust(
                                                lift::types::RustVariable::integer(i_inner),
                                            ) => {
                                                common::types::bits::new(0, integer_to_usize!(i_inner))
                                                    .into()
                                            }
                                            lift::types::Variable::Air(a_inner) => {
                                                lift::types::Variable::air_from_bits(
                                                    builder,
                                                    common::types::bits::from_bits_literal("0")?,
                                                )?
                                            }
                                            _ => return Err(AArch64LifterError::VariableNotExpectedEnum),
                                        }
                                            .into(),
                                    );
                                let (
                                    then_args,
                                    block_param_types,
                                ): (Vec<Value>, Vec<Type>) = vec![
                                    assigns_2.get("data2").unwrap()
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
                                        "data2".to_string(),
                                        {
                                            let arg_0 = match assigns_2.get("t2") {
                                                Some(packable) => (*packable).clone().try_into()?,
                                                None => t2.clone(),
                                            };
                                            let arg_1 = match assigns_2.get("datasize") {
                                                Some(packable) => (*packable).clone().try_into()?,
                                                None => datasize.clone(),
                                            };
                                            lift::helpers::X_read_0(
                                                builder,
                                                sequencer,
                                                pc.clone(),
                                                arg_0,
                                                arg_1,
                                            )?
                                        }
                                            .into(),
                                    );
                                let (else_args, _): (Vec<Value>, Vec<Type>) = vec![
                                    assigns_2.get("data2").unwrap()
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
                                let packable: lift::types::AirPackable = data2
                                    .clone()
                                    .into();
                                let (packed, consumed) = packable
                                    .pack_from_air_values_and_types(
                                        &end_args[consumed_total..],
                                        &block_param_types[consumed_total..],
                                    )?;
                                data2 = packed.try_into()?;
                                consumed_total += consumed;
                            }
                            _ => return Err(AArch64LifterError::VariableNotExpectedEnum),
                        }
                    }
                    {
                        let arg_0 = data1.clone();
                        let arg_1 = {
                            let arg_0 = address.clone();
                            let arg_1 = lift::types::Variable::from(
                                common::types::integer::from(0),
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
                        let arg_2 = dbytes.clone();
                        let arg_3 = acctype.clone();
                        let arg_4 = dbytes.clone();
                        lift::helpers::Mem_set_0(
                            builder,
                            sequencer,
                            pc.clone(),
                            arg_0,
                            arg_1,
                            arg_2,
                            arg_3,
                            arg_4,
                        )?;
                    }
                    {
                        let arg_0 = data2.clone();
                        let arg_1 = {
                            let arg_0 = address.clone();
                            let arg_1 = dbytes.clone();
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
                        let arg_2 = dbytes.clone();
                        let arg_3 = acctype.clone();
                        let arg_4 = dbytes.clone();
                        lift::helpers::Mem_set_0(
                            builder,
                            sequencer,
                            pc.clone(),
                            arg_0,
                            arg_1,
                            arg_2,
                            arg_3,
                            arg_4,
                        )?;
                    }
                } else if (memop.clone()
                    == lift::types::Variable::from(common::types::MemOp::MemOp_LOAD))
                    && true
                {
                    let mut assigns_1: BTreeMap<String, lift::types::AirPackable> = assigns_0
                        .clone();
                    data1 = {
                        let arg_0 = {
                            let arg_0 = address.clone();
                            let arg_1 = lift::types::Variable::from(
                                common::types::integer::from(0),
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
                        let arg_1 = dbytes.clone();
                        let arg_2 = acctype.clone();
                        let arg_3 = dbytes.clone();
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
                    data2 = {
                        let arg_0 = {
                            let arg_0 = address.clone();
                            let arg_1 = dbytes.clone();
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
                        let arg_1 = dbytes.clone();
                        let arg_2 = acctype.clone();
                        let arg_3 = dbytes.clone();
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
                        let cond = rt_unknown.clone();
                        match cond {
                            lift::types::Variable::Rust(
                                lift::types::RustVariable::boolean(b_inner),
                            ) => {
                                if b_inner == common::types::boolean::TRUE {
                                    let mut assigns_2: BTreeMap<
                                        String,
                                        lift::types::AirPackable,
                                    > = assigns_1.clone();
                                    data1 = match datasize.clone() {
                                        lift::types::Variable::Rust(
                                            lift::types::RustVariable::integer(i_inner),
                                        ) => {
                                            common::types::bits::new(0, integer_to_usize!(i_inner))
                                                .into()
                                        }
                                        lift::types::Variable::Air(a_inner) => {
                                            lift::types::Variable::air_from_bits(
                                                builder,
                                                common::types::bits::from_bits_literal("0")?,
                                            )?
                                        }
                                        _ => return Err(AArch64LifterError::VariableNotExpectedEnum),
                                    };
                                    data2 = match datasize.clone() {
                                        lift::types::Variable::Rust(
                                            lift::types::RustVariable::integer(i_inner),
                                        ) => {
                                            common::types::bits::new(0, integer_to_usize!(i_inner))
                                                .into()
                                        }
                                        lift::types::Variable::Air(a_inner) => {
                                            lift::types::Variable::air_from_bits(
                                                builder,
                                                common::types::bits::from_bits_literal("0")?,
                                            )?
                                        }
                                        _ => return Err(AArch64LifterError::VariableNotExpectedEnum),
                                    };
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
                                        "data1".to_string(),
                                        match match assigns_2.get("datasize") {
                                            Some(packable) => (*packable).clone().try_into()?,
                                            None => datasize.clone(),
                                        } {
                                            lift::types::Variable::Rust(
                                                lift::types::RustVariable::integer(i_inner),
                                            ) => {
                                                common::types::bits::new(0, integer_to_usize!(i_inner))
                                                    .into()
                                            }
                                            lift::types::Variable::Air(a_inner) => {
                                                lift::types::Variable::air_from_bits(
                                                    builder,
                                                    common::types::bits::from_bits_literal("0")?,
                                                )?
                                            }
                                            _ => return Err(AArch64LifterError::VariableNotExpectedEnum),
                                        }
                                            .into(),
                                    );
                                assigns_2
                                    .insert(
                                        "data2".to_string(),
                                        match match assigns_2.get("datasize") {
                                            Some(packable) => (*packable).clone().try_into()?,
                                            None => datasize.clone(),
                                        } {
                                            lift::types::Variable::Rust(
                                                lift::types::RustVariable::integer(i_inner),
                                            ) => {
                                                common::types::bits::new(0, integer_to_usize!(i_inner))
                                                    .into()
                                            }
                                            lift::types::Variable::Air(a_inner) => {
                                                lift::types::Variable::air_from_bits(
                                                    builder,
                                                    common::types::bits::from_bits_literal("0")?,
                                                )?
                                            }
                                            _ => return Err(AArch64LifterError::VariableNotExpectedEnum),
                                        }
                                            .into(),
                                    );
                                let (
                                    then_args,
                                    block_param_types,
                                ): (Vec<Value>, Vec<Type>) = vec![
                                    assigns_2.get("data1").unwrap()
                                    .unpack_to_air_values_and_types(builder) ?, assigns_2
                                    .get("data2").unwrap()
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
                                    { let packable : lift::types::AirPackable = data1.clone()
                                    .into(); packable.unpack_to_air_values_and_types(builder) ?
                                    }, { let packable : lift::types::AirPackable = data2.clone()
                                    .into(); packable.unpack_to_air_values_and_types(builder) ?
                                    }
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
                                let packable: lift::types::AirPackable = data1
                                    .clone()
                                    .into();
                                let (packed, consumed) = packable
                                    .pack_from_air_values_and_types(
                                        &end_args[consumed_total..],
                                        &block_param_types[consumed_total..],
                                    )?;
                                data1 = packed.try_into()?;
                                consumed_total += consumed;
                                let packable: lift::types::AirPackable = data2
                                    .clone()
                                    .into();
                                let (packed, consumed) = packable
                                    .pack_from_air_values_and_types(
                                        &end_args[consumed_total..],
                                        &block_param_types[consumed_total..],
                                    )?;
                                data2 = packed.try_into()?;
                                consumed_total += consumed;
                            }
                            _ => return Err(AArch64LifterError::VariableNotExpectedEnum),
                        }
                    }
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
                                            let arg_0 = data1.clone();
                                            let arg_1 = lift::types::Variable::from(
                                                common::types::integer::from(64),
                                            );
                                            let arg_2 = datasize.clone();
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
                                    {
                                        let arg_0 = {
                                            let arg_0 = data2.clone();
                                            let arg_1 = lift::types::Variable::from(
                                                common::types::integer::from(64),
                                            );
                                            let arg_2 = datasize.clone();
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
                                        let arg_1 = t2.clone();
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
                                        let arg_0 = data1.clone();
                                        let arg_1 = t.clone();
                                        let arg_2 = datasize.clone();
                                        lift::helpers::X_set_0(
                                            builder,
                                            sequencer,
                                            pc.clone(),
                                            arg_0,
                                            arg_1,
                                            arg_2,
                                        )?;
                                    }
                                    {
                                        let arg_0 = data2.clone();
                                        let arg_1 = t2.clone();
                                        let arg_2 = datasize.clone();
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
                                        let arg_0 = match assigns_2.get("data1") {
                                            Some(packable) => (*packable).clone().try_into()?,
                                            None => data1.clone(),
                                        };
                                        let arg_1 = lift::types::Variable::from(
                                            common::types::integer::from(64),
                                        );
                                        let arg_2 = match assigns_2.get("datasize") {
                                            Some(packable) => (*packable).clone().try_into()?,
                                            None => datasize.clone(),
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
                                {
                                    let arg_0 = {
                                        let arg_0 = match assigns_2.get("data2") {
                                            Some(packable) => (*packable).clone().try_into()?,
                                            None => data2.clone(),
                                        };
                                        let arg_1 = lift::types::Variable::from(
                                            common::types::integer::from(64),
                                        );
                                        let arg_2 = match assigns_2.get("datasize") {
                                            Some(packable) => (*packable).clone().try_into()?,
                                            None => datasize.clone(),
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
                                    let arg_1 = match assigns_2.get("t2") {
                                        Some(packable) => (*packable).clone().try_into()?,
                                        None => t2.clone(),
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
                                    let arg_0 = match assigns_2.get("data1") {
                                        Some(packable) => (*packable).clone().try_into()?,
                                        None => data1.clone(),
                                    };
                                    let arg_1 = match assigns_2.get("t") {
                                        Some(packable) => (*packable).clone().try_into()?,
                                        None => t.clone(),
                                    };
                                    let arg_2 = match assigns_2.get("datasize") {
                                        Some(packable) => (*packable).clone().try_into()?,
                                        None => datasize.clone(),
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
                                {
                                    let arg_0 = match assigns_2.get("data2") {
                                        Some(packable) => (*packable).clone().try_into()?,
                                        None => data2.clone(),
                                    };
                                    let arg_1 = match assigns_2.get("t2") {
                                        Some(packable) => (*packable).clone().try_into()?,
                                        None => t2.clone(),
                                    };
                                    let arg_2 = match assigns_2.get("datasize") {
                                        Some(packable) => (*packable).clone().try_into()?,
                                        None => datasize.clone(),
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
        let cond = wback.clone();
        match cond {
            lift::types::Variable::Rust(lift::types::RustVariable::boolean(b_inner)) => {
                if b_inner == common::types::boolean::TRUE {
                    let mut assigns_1: BTreeMap<String, lift::types::AirPackable> = assigns_0
                        .clone();
                    {
                        let cond = wb_unknown.clone();
                        match cond {
                            lift::types::Variable::Rust(
                                lift::types::RustVariable::boolean(b_inner),
                            ) => {
                                if b_inner == common::types::boolean::TRUE {
                                    let mut assigns_2: BTreeMap<
                                        String,
                                        lift::types::AirPackable,
                                    > = assigns_1.clone();
                                    address = match lift::types::Variable::from(
                                        common::types::integer::from(64),
                                    ) {
                                        lift::types::Variable::Rust(
                                            lift::types::RustVariable::integer(i_inner),
                                        ) => {
                                            common::types::bits::new(0, integer_to_usize!(i_inner))
                                                .into()
                                        }
                                        lift::types::Variable::Air(a_inner) => {
                                            lift::types::Variable::air_from_bits(
                                                builder,
                                                common::types::bits::from_bits_literal("0")?,
                                            )?
                                        }
                                        _ => return Err(AArch64LifterError::VariableNotExpectedEnum),
                                    };
                                } else {
                                    let mut assigns_2: BTreeMap<
                                        String,
                                        lift::types::AirPackable,
                                    > = assigns_1.clone();
                                    {
                                        let cond = postindex.clone();
                                        match cond {
                                            lift::types::Variable::Rust(
                                                lift::types::RustVariable::boolean(b_inner),
                                            ) => {
                                                if b_inner == common::types::boolean::TRUE {
                                                    let mut assigns_3: BTreeMap<
                                                        String,
                                                        lift::types::AirPackable,
                                                    > = assigns_2.clone();
                                                    address = {
                                                        let arg_0 = address.clone();
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
                                                } else {
                                                    let mut assigns_3: BTreeMap<
                                                        String,
                                                        lift::types::AirPackable,
                                                    > = assigns_2.clone();
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
                                                assigns_3
                                                    .insert(
                                                        "address".to_string(),
                                                        {
                                                            let arg_0 = match assigns_3.get("address") {
                                                                Some(packable) => (*packable).clone().try_into()?,
                                                                None => address.clone(),
                                                            };
                                                            let arg_1 = match assigns_3.get("offset") {
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
                                                        }
                                                            .into(),
                                                    );
                                                let (
                                                    then_args,
                                                    block_param_types,
                                                ): (Vec<Value>, Vec<Type>) = vec![
                                                    assigns_3.get("address").unwrap()
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
                                                let mut assigns_3: BTreeMap<
                                                    String,
                                                    lift::types::AirPackable,
                                                > = assigns_2.clone();
                                                let (else_args, _): (Vec<Value>, Vec<Type>) = vec![
                                                    { let packable : lift::types::AirPackable = address.clone()
                                                    .into(); packable.unpack_to_air_values_and_types(builder) ?
                                                    }
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
                                                let packable: lift::types::AirPackable = address
                                                    .clone()
                                                    .into();
                                                let (packed, consumed) = packable
                                                    .pack_from_air_values_and_types(
                                                        &end_args[consumed_total..],
                                                        &block_param_types[consumed_total..],
                                                    )?;
                                                address = packed.try_into()?;
                                                consumed_total += consumed;
                                            }
                                            _ => return Err(AArch64LifterError::VariableNotExpectedEnum),
                                        }
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
                                assigns_2
                                    .insert(
                                        "address".to_string(),
                                        match lift::types::Variable::from(
                                            common::types::integer::from(64),
                                        ) {
                                            lift::types::Variable::Rust(
                                                lift::types::RustVariable::integer(i_inner),
                                            ) => {
                                                common::types::bits::new(0, integer_to_usize!(i_inner))
                                                    .into()
                                            }
                                            lift::types::Variable::Air(a_inner) => {
                                                lift::types::Variable::air_from_bits(
                                                    builder,
                                                    common::types::bits::from_bits_literal("0")?,
                                                )?
                                            }
                                            _ => return Err(AArch64LifterError::VariableNotExpectedEnum),
                                        }
                                            .into(),
                                    );
                                let (
                                    then_args,
                                    block_param_types,
                                ): (Vec<Value>, Vec<Type>) = vec![
                                    assigns_2.get("address").unwrap()
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
                                {
                                    let cond = match assigns_2.get("postindex") {
                                        Some(packable) => (*packable).clone().try_into()?,
                                        None => postindex.clone(),
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
                                                assigns_3
                                                    .insert(
                                                        "address".to_string(),
                                                        {
                                                            let arg_0 = match assigns_3.get("address") {
                                                                Some(packable) => (*packable).clone().try_into()?,
                                                                None => address.clone(),
                                                            };
                                                            let arg_1 = match assigns_3.get("offset") {
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
                                                        }
                                                            .into(),
                                                    );
                                                assigns_2
                                                    .insert(
                                                        "address".to_string(),
                                                        assigns_3.get("address").unwrap().clone(),
                                                    );
                                            } else {
                                                let mut assigns_3: BTreeMap<
                                                    String,
                                                    lift::types::AirPackable,
                                                > = assigns_2.clone();
                                                assigns_2
                                                    .entry("address".to_string())
                                                    .or_insert(address.clone().into());
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
                                            assigns_3
                                                .insert(
                                                    "address".to_string(),
                                                    {
                                                        let arg_0 = match assigns_3.get("address") {
                                                            Some(packable) => (*packable).clone().try_into()?,
                                                            None => address.clone(),
                                                        };
                                                        let arg_1 = match assigns_3.get("offset") {
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
                                                    }
                                                        .into(),
                                                );
                                            let (
                                                then_args,
                                                block_param_types,
                                            ): (Vec<Value>, Vec<Type>) = vec![
                                                assigns_3.get("address").unwrap()
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
                                            let mut assigns_3: BTreeMap<
                                                String,
                                                lift::types::AirPackable,
                                            > = assigns_2.clone();
                                            let (else_args, _): (Vec<Value>, Vec<Type>) = vec![
                                                { let packable : lift::types::AirPackable = address.clone()
                                                .into(); packable.unpack_to_air_values_and_types(builder) ?
                                                }
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
                                            let packable: lift::types::AirPackable = address
                                                .clone()
                                                .into();
                                            let (packed, consumed) = packable
                                                .pack_from_air_values_and_types(
                                                    &end_args[consumed_total..],
                                                    &block_param_types[consumed_total..],
                                                )?;
                                            assigns_2.insert("address".to_string(), packed);
                                            consumed_total += consumed;
                                        }
                                        _ => return Err(AArch64LifterError::VariableNotExpectedEnum),
                                    }
                                }
                                let (else_args, _): (Vec<Value>, Vec<Type>) = vec![
                                    assigns_2.get("address").unwrap()
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
                                let packable: lift::types::AirPackable = address
                                    .clone()
                                    .into();
                                let (packed, consumed) = packable
                                    .pack_from_air_values_and_types(
                                        &end_args[consumed_total..],
                                        &block_param_types[consumed_total..],
                                    )?;
                                address = packed.try_into()?;
                                consumed_total += consumed;
                            }
                            _ => return Err(AArch64LifterError::VariableNotExpectedEnum),
                        }
                    }
                    {
                        let cond = {
                            let arg_0 = n.clone();
                            let arg_1 = lift::types::Variable::from(
                                common::types::integer::from(31),
                            );
                            lift::helpers::eq_int_0(
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
                                    let mut assigns_2: BTreeMap<
                                        String,
                                        lift::types::AirPackable,
                                    > = assigns_1.clone();
                                    {
                                        let arg_0 = address.clone();
                                        let arg_1 = lift::types::Variable::from(
                                            common::types::integer::from(64),
                                        );
                                        lift::helpers::SP_set_0(
                                            builder,
                                            sequencer,
                                            pc.clone(),
                                            arg_0,
                                            arg_1,
                                        )?;
                                    }
                                } else {
                                    let mut assigns_2: BTreeMap<
                                        String,
                                        lift::types::AirPackable,
                                    > = assigns_1.clone();
                                    {
                                        let arg_0 = address.clone();
                                        let arg_1 = n.clone();
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
                                    let arg_0 = match assigns_2.get("address") {
                                        Some(packable) => (*packable).clone().try_into()?,
                                        None => address.clone(),
                                    };
                                    let arg_1 = lift::types::Variable::from(
                                        common::types::integer::from(64),
                                    );
                                    lift::helpers::SP_set_0(
                                        builder,
                                        sequencer,
                                        pc.clone(),
                                        arg_0,
                                        arg_1,
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
                                    let arg_0 = match assigns_2.get("address") {
                                        Some(packable) => (*packable).clone().try_into()?,
                                        None => address.clone(),
                                    };
                                    let arg_1 = match assigns_2.get("n") {
                                        Some(packable) => (*packable).clone().try_into()?,
                                        None => n.clone(),
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
                    let cond = match assigns_1.get("wb_unknown") {
                        Some(packable) => (*packable).clone().try_into()?,
                        None => wb_unknown.clone(),
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
                                        "address".to_string(),
                                        match lift::types::Variable::from(
                                            common::types::integer::from(64),
                                        ) {
                                            lift::types::Variable::Rust(
                                                lift::types::RustVariable::integer(i_inner),
                                            ) => {
                                                common::types::bits::new(0, integer_to_usize!(i_inner))
                                                    .into()
                                            }
                                            lift::types::Variable::Air(a_inner) => {
                                                lift::types::Variable::air_from_bits(
                                                    builder,
                                                    common::types::bits::from_bits_literal("0")?,
                                                )?
                                            }
                                            _ => return Err(AArch64LifterError::VariableNotExpectedEnum),
                                        }
                                            .into(),
                                    );
                                assigns_1
                                    .insert(
                                        "address".to_string(),
                                        assigns_2.get("address").unwrap().clone(),
                                    );
                            } else {
                                let mut assigns_2: BTreeMap<
                                    String,
                                    lift::types::AirPackable,
                                > = assigns_1.clone();
                                {
                                    let cond = match assigns_2.get("postindex") {
                                        Some(packable) => (*packable).clone().try_into()?,
                                        None => postindex.clone(),
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
                                                assigns_3
                                                    .insert(
                                                        "address".to_string(),
                                                        {
                                                            let arg_0 = match assigns_3.get("address") {
                                                                Some(packable) => (*packable).clone().try_into()?,
                                                                None => address.clone(),
                                                            };
                                                            let arg_1 = match assigns_3.get("offset") {
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
                                                        }
                                                            .into(),
                                                    );
                                                assigns_2
                                                    .insert(
                                                        "address".to_string(),
                                                        assigns_3.get("address").unwrap().clone(),
                                                    );
                                            } else {
                                                let mut assigns_3: BTreeMap<
                                                    String,
                                                    lift::types::AirPackable,
                                                > = assigns_2.clone();
                                                assigns_2
                                                    .entry("address".to_string())
                                                    .or_insert(address.clone().into());
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
                                            assigns_3
                                                .insert(
                                                    "address".to_string(),
                                                    {
                                                        let arg_0 = match assigns_3.get("address") {
                                                            Some(packable) => (*packable).clone().try_into()?,
                                                            None => address.clone(),
                                                        };
                                                        let arg_1 = match assigns_3.get("offset") {
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
                                                    }
                                                        .into(),
                                                );
                                            let (
                                                then_args,
                                                block_param_types,
                                            ): (Vec<Value>, Vec<Type>) = vec![
                                                assigns_3.get("address").unwrap()
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
                                            let mut assigns_3: BTreeMap<
                                                String,
                                                lift::types::AirPackable,
                                            > = assigns_2.clone();
                                            let (else_args, _): (Vec<Value>, Vec<Type>) = vec![
                                                { let packable : lift::types::AirPackable = address.clone()
                                                .into(); packable.unpack_to_air_values_and_types(builder) ?
                                                }
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
                                            let packable: lift::types::AirPackable = address
                                                .clone()
                                                .into();
                                            let (packed, consumed) = packable
                                                .pack_from_air_values_and_types(
                                                    &end_args[consumed_total..],
                                                    &block_param_types[consumed_total..],
                                                )?;
                                            assigns_2.insert("address".to_string(), packed);
                                            consumed_total += consumed;
                                        }
                                        _ => return Err(AArch64LifterError::VariableNotExpectedEnum),
                                    }
                                }
                                assigns_1
                                    .insert(
                                        "address".to_string(),
                                        assigns_2.get("address").unwrap().clone(),
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
                                    "address".to_string(),
                                    match lift::types::Variable::from(
                                        common::types::integer::from(64),
                                    ) {
                                        lift::types::Variable::Rust(
                                            lift::types::RustVariable::integer(i_inner),
                                        ) => {
                                            common::types::bits::new(0, integer_to_usize!(i_inner))
                                                .into()
                                        }
                                        lift::types::Variable::Air(a_inner) => {
                                            lift::types::Variable::air_from_bits(
                                                builder,
                                                common::types::bits::from_bits_literal("0")?,
                                            )?
                                        }
                                        _ => return Err(AArch64LifterError::VariableNotExpectedEnum),
                                    }
                                        .into(),
                                );
                            let (
                                then_args,
                                block_param_types,
                            ): (Vec<Value>, Vec<Type>) = vec![
                                assigns_2.get("address").unwrap()
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
                            {
                                let cond = match assigns_2.get("postindex") {
                                    Some(packable) => (*packable).clone().try_into()?,
                                    None => postindex.clone(),
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
                                            assigns_3
                                                .insert(
                                                    "address".to_string(),
                                                    {
                                                        let arg_0 = match assigns_3.get("address") {
                                                            Some(packable) => (*packable).clone().try_into()?,
                                                            None => address.clone(),
                                                        };
                                                        let arg_1 = match assigns_3.get("offset") {
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
                                                    }
                                                        .into(),
                                                );
                                            assigns_2
                                                .insert(
                                                    "address".to_string(),
                                                    assigns_3.get("address").unwrap().clone(),
                                                );
                                        } else {
                                            let mut assigns_3: BTreeMap<
                                                String,
                                                lift::types::AirPackable,
                                            > = assigns_2.clone();
                                            assigns_2
                                                .entry("address".to_string())
                                                .or_insert(address.clone().into());
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
                                        assigns_3
                                            .insert(
                                                "address".to_string(),
                                                {
                                                    let arg_0 = match assigns_3.get("address") {
                                                        Some(packable) => (*packable).clone().try_into()?,
                                                        None => address.clone(),
                                                    };
                                                    let arg_1 = match assigns_3.get("offset") {
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
                                                }
                                                    .into(),
                                            );
                                        let (
                                            then_args,
                                            block_param_types,
                                        ): (Vec<Value>, Vec<Type>) = vec![
                                            assigns_3.get("address").unwrap()
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
                                        let mut assigns_3: BTreeMap<
                                            String,
                                            lift::types::AirPackable,
                                        > = assigns_2.clone();
                                        let (else_args, _): (Vec<Value>, Vec<Type>) = vec![
                                            { let packable : lift::types::AirPackable = address.clone()
                                            .into(); packable.unpack_to_air_values_and_types(builder) ?
                                            }
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
                                        let packable: lift::types::AirPackable = address
                                            .clone()
                                            .into();
                                        let (packed, consumed) = packable
                                            .pack_from_air_values_and_types(
                                                &end_args[consumed_total..],
                                                &block_param_types[consumed_total..],
                                            )?;
                                        assigns_2.insert("address".to_string(), packed);
                                        consumed_total += consumed;
                                    }
                                    _ => return Err(AArch64LifterError::VariableNotExpectedEnum),
                                }
                            }
                            let (else_args, _): (Vec<Value>, Vec<Type>) = vec![
                                assigns_2.get("address").unwrap()
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
                            let packable: lift::types::AirPackable = address
                                .clone()
                                .into();
                            let (packed, consumed) = packable
                                .pack_from_air_values_and_types(
                                    &end_args[consumed_total..],
                                    &block_param_types[consumed_total..],
                                )?;
                            assigns_1.insert("address".to_string(), packed);
                            consumed_total += consumed;
                        }
                        _ => return Err(AArch64LifterError::VariableNotExpectedEnum),
                    }
                }
                {
                    let cond = {
                        let arg_0 = match assigns_1.get("n") {
                            Some(packable) => (*packable).clone().try_into()?,
                            None => n.clone(),
                        };
                        let arg_1 = lift::types::Variable::from(
                            common::types::integer::from(31),
                        );
                        lift::helpers::eq_int_0(
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
                                let mut assigns_2: BTreeMap<
                                    String,
                                    lift::types::AirPackable,
                                > = assigns_1.clone();
                                {
                                    let arg_0 = match assigns_2.get("address") {
                                        Some(packable) => (*packable).clone().try_into()?,
                                        None => address.clone(),
                                    };
                                    let arg_1 = lift::types::Variable::from(
                                        common::types::integer::from(64),
                                    );
                                    lift::helpers::SP_set_0(
                                        builder,
                                        sequencer,
                                        pc.clone(),
                                        arg_0,
                                        arg_1,
                                    )?;
                                }
                            } else {
                                let mut assigns_2: BTreeMap<
                                    String,
                                    lift::types::AirPackable,
                                > = assigns_1.clone();
                                {
                                    let arg_0 = match assigns_2.get("address") {
                                        Some(packable) => (*packable).clone().try_into()?,
                                        None => address.clone(),
                                    };
                                    let arg_1 = match assigns_2.get("n") {
                                        Some(packable) => (*packable).clone().try_into()?,
                                        None => n.clone(),
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
                                let arg_0 = match assigns_2.get("address") {
                                    Some(packable) => (*packable).clone().try_into()?,
                                    None => address.clone(),
                                };
                                let arg_1 = lift::types::Variable::from(
                                    common::types::integer::from(64),
                                );
                                lift::helpers::SP_set_0(
                                    builder,
                                    sequencer,
                                    pc.clone(),
                                    arg_0,
                                    arg_1,
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
                                let arg_0 = match assigns_2.get("address") {
                                    Some(packable) => (*packable).clone().try_into()?,
                                    None => address.clone(),
                                };
                                let arg_1 = match assigns_2.get("n") {
                                    Some(packable) => (*packable).clone().try_into()?,
                                    None => n.clone(),
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
                let (then_args, block_param_types): (Vec<Value>, Vec<Type>) = vec![
                    assigns_1.get("address").unwrap()
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
                    { let packable : lift::types::AirPackable = address.clone().into();
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
                let packable: lift::types::AirPackable = address.clone().into();
                let (packed, consumed) = packable
                    .pack_from_air_values_and_types(
                        &end_args[consumed_total..],
                        &block_param_types[consumed_total..],
                    )?;
                address = packed.try_into()?;
                consumed_total += consumed;
            }
            _ => return Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    Ok(())
}

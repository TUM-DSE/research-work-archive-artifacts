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
pub fn lift_aarch64_integer_bitfield(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: lift::types::Variable,
    R: common::types::integer,
    S: common::types::integer,
    d: common::types::integer,
    datasize: common::types::integer,
    extend: common::types::boolean,
    inzero: common::types::boolean,
    n: common::types::integer,
    tmask: common::types::bits,
    wmask: common::types::bits,
) -> Result<(), AArch64LifterError> {
    let mut R: lift::types::Variable = R.into();
    let mut S: lift::types::Variable = S.into();
    let mut d: lift::types::Variable = d.into();
    let mut datasize: lift::types::Variable = datasize.into();
    let mut extend: lift::types::Variable = extend.into();
    let mut inzero: lift::types::Variable = inzero.into();
    let mut n: lift::types::Variable = n.into();
    let mut tmask: lift::types::Variable = tmask.into();
    let mut wmask: lift::types::Variable = wmask.into();
    let mut assigns_0: BTreeMap<String, lift::types::AirPackable> = BTreeMap::new();
    let mut dst: lift::types::Variable = {
        let cond = inzero.clone();
        match cond {
            lift::types::Variable::Rust(lift::types::RustVariable::boolean(b_inner)) => {
                if b_inner == common::types::boolean::TRUE {
                    {
                        let arg_0 = datasize.clone();
                        lift::helpers::Zeros_1(builder, sequencer, pc.clone(), arg_0)?
                    }
                } else {
                    {
                        let arg_0 = d.clone();
                        let arg_1 = datasize.clone();
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
                    let arg_0 = datasize.clone();
                    lift::helpers::Zeros_1(builder, sequencer, pc.clone(), arg_0)?
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
                        let arg_0 = d.clone();
                        let arg_1 = datasize.clone();
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
                        .push(Value::from(builder.get_block_param(end_block, i as u32)));
                }
                let packable: lift::types::AirPackable = then_body_promoted.into();
                let (end_body_packable, _) = packable
                    .pack_from_air_values_and_types(&end_args, &arg_tys)?;
                end_body_packable.try_into()?
            }
            _ => return Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    };
    let mut src: lift::types::Variable = {
        let arg_0 = n.clone();
        let arg_1 = datasize.clone();
        lift::helpers::X_read_0(builder, sequencer, pc.clone(), arg_0, arg_1)?
    };
    let mut bot: lift::types::Variable = {
        let arg_0 = ({
            let arg_0 = dst.clone();
            let arg_1 = {
                let arg_0 = (wmask.clone());
                let arg_1 = datasize.clone();
                lift::helpers::not_bits_0(builder, sequencer, pc.clone(), arg_0, arg_1)?
            };
            let arg_2 = datasize.clone();
            lift::helpers::and_bits_0(
                builder,
                sequencer,
                pc.clone(),
                arg_0,
                arg_1,
                arg_2,
            )?
        });
        let arg_1 = ({
            let arg_0 = {
                let arg_0 = src.clone();
                let arg_1 = R.clone();
                let arg_2 = datasize.clone();
                lift::helpers::ROR_0(
                    builder,
                    sequencer,
                    pc.clone(),
                    arg_0,
                    arg_1,
                    arg_2,
                )?
            };
            let arg_1 = wmask.clone();
            let arg_2 = datasize.clone();
            lift::helpers::and_bits_0(
                builder,
                sequencer,
                pc.clone(),
                arg_0,
                arg_1,
                arg_2,
            )?
        });
        let arg_2 = datasize.clone();
        lift::helpers::or_bits_0(builder, sequencer, pc.clone(), arg_0, arg_1, arg_2)?
    };
    let mut top: lift::types::Variable = {
        let cond = extend.clone();
        match cond {
            lift::types::Variable::Rust(lift::types::RustVariable::boolean(b_inner)) => {
                if b_inner == common::types::boolean::TRUE {
                    {
                        let arg_0 = {
                            let arg_0 = S.clone();
                            let arg_1 = lift::types::Variable::from(
                                common::types::integer::one(),
                            );
                            src.extract_slice(builder, arg_0, arg_1)?
                        };
                        let arg_1 = {
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
                        let arg_2 = datasize.clone();
                        lift::helpers::Replicate_1(
                            builder,
                            sequencer,
                            pc.clone(),
                            arg_0,
                            arg_1,
                            arg_2,
                        )?
                    }
                } else {
                    dst.clone()
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
                    let arg_0 = {
                        let arg_0 = S.clone();
                        let arg_1 = lift::types::Variable::from(
                            common::types::integer::one(),
                        );
                        src.extract_slice(builder, arg_0, arg_1)?
                    };
                    let arg_1 = {
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
                    let arg_2 = datasize.clone();
                    lift::helpers::Replicate_1(
                        builder,
                        sequencer,
                        pc.clone(),
                        arg_0,
                        arg_1,
                        arg_2,
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
                    let packable: lift::types::AirPackable = dst.clone().into();
                    packable.unpack_to_air_values_and_types(builder)?
                };
                builder.jump(end_block, arg_else_vals);
                builder.set_insert_block(end_block);
                let mut end_args = Vec::new();
                for i in 0..arg_tys.len() {
                    end_args
                        .push(Value::from(builder.get_block_param(end_block, i as u32)));
                }
                let packable: lift::types::AirPackable = then_body_promoted.into();
                let (end_body_packable, _) = packable
                    .pack_from_air_values_and_types(&end_args, &arg_tys)?;
                end_body_packable.try_into()?
            }
            _ => return Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    };
    {
        let arg_0 = {
            let arg_0 = ({
                let arg_0 = top.clone();
                let arg_1 = {
                    let arg_0 = (tmask.clone());
                    let arg_1 = datasize.clone();
                    lift::helpers::not_bits_0(
                        builder,
                        sequencer,
                        pc.clone(),
                        arg_0,
                        arg_1,
                    )?
                };
                let arg_2 = datasize.clone();
                lift::helpers::and_bits_0(
                    builder,
                    sequencer,
                    pc.clone(),
                    arg_0,
                    arg_1,
                    arg_2,
                )?
            });
            let arg_1 = ({
                let arg_0 = bot.clone();
                let arg_1 = tmask.clone();
                let arg_2 = datasize.clone();
                lift::helpers::and_bits_0(
                    builder,
                    sequencer,
                    pc.clone(),
                    arg_0,
                    arg_1,
                    arg_2,
                )?
            });
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
        let arg_1 = d.clone();
        let arg_2 = datasize.clone();
        lift::helpers::X_set_0(builder, sequencer, pc.clone(), arg_0, arg_1, arg_2)?;
    }
    Ok(())
}

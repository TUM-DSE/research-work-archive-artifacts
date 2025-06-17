#![allow(nonstandard_style, unused)]

use crate::arm64::common::types::{bits, boolean, integer, BigIntExt, INSTRUCTION_BYTE_SIZE};
pub use crate::arm64::lift::generated::types::*;
use crate::arm64::{common, decode, lift, AArch64LifterError};
use crate::{integer_to_u32, integer_to_u8, integer_to_usize};
use indicatif::{ProgressBar, ProgressStyle};
use num_traits::ToPrimitive;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::hash::Hash;
use target_lexicon::{Aarch64Architecture, Architecture};
use tnj::air::instructions::builder::InstructionBuilder;
use tnj::air::instructions::{BasicBlock, CodeRegion, Value};
use tnj::arch::get_arch;
use tnj::arch::reg::Reg;
use tnj::types::Type;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Variable {
    Air(AirVariable),
    Rust(RustVariable),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct AirVariable {
    pub val: Value,
    pub ty: Type,
}

pub enum Register {
    _R(Variable),
    SP_EL0,
    SP_EL1,
    SP_EL2,
    SP_EL3,
    TCR_EL1,
    TCR_EL2,
    TCR_EL3,
    SCR,
    SCR_EL3,
    HCR_EL2,
    SCTLR,
    SCTLR_EL1,
    SCTLR_EL2,
    SCTLR_EL3,
    APIAKeyHi_EL1,
    APIAKeyLo_EL1,
    APIBKeyHi_EL1,
    APIBKeyLo_EL1,
    BTypeNext,
    DLR,
    DLR_EL0,
    DSPSR,
    DSPSR_EL0,
    DBGOSDLR,
    OSDLR_EL1,
    DBGPRCR,
    DBGPRCR_EL1,
    MDSCR_EL1,
    EDSCR,
    OSLSR_EL1,
    ID_AA64DFR0_EL1,
    MDCR_EL2,
    MDCR_EL3,
    TFSR_EL1,
    TFSR_EL2,
    TFSR_EL3,
    TFSRE0_EL1,
    MPAMVPM0_EL2,
    MPAMVPMV_EL2,
    MPAM3_EL3,
    MPAMIDR_EL1,
    MPAMHCR_EL2,
    MPAM1_EL1,
    MPAM2_EL2,
    HSCTLR,
    HCR2,
    TTBR0_EL1,
    TTBR0_EL2,
    TTBR0_EL3,
    TTBR1_EL1,
    TTBR1_EL2,
    VTCR_EL2,
    VSTCR_EL2,
    VSTTBR_EL2,
    VTTBR_EL2,
}

pub enum Flag {
    N,
    Z,
    C,
    V,
    D,
    I,
    A,
    F,
    T,
    IT,
    IL,
    SS,
    EL,
    nRW,
    SP,
    PAN,
    UAO,
    BTYPE,
    SSBS,
    TCO,
}

impl From<integer> for Variable {
    fn from(value: integer) -> Self {
        Variable::Rust(RustVariable::integer(value))
    }
}

impl From<bits> for Variable {
    fn from(value: bits) -> Self {
        Variable::Rust(RustVariable::bits(value))
    }
}

impl Variable {
    pub fn new_air(val: Value, ty: Type) -> Self {
        Variable::Air(AirVariable { val, ty })
    }

    pub fn from_register(builder: &mut InstructionBuilder, register_type: Register) -> Result<Self, AArch64LifterError> {
        let ty = Type::new_fixed_int(64).ok_or(AArch64LifterError::InvalidBitsLength)?;

        let register = match register_type {
            Register::_R(index) => Reg::new(integer_to_u32!(index.to_integer()?)),
            Register::SP_EL0 | Register::SP_EL1 | Register::SP_EL2 | Register::SP_EL3 => {
                builder.get_code_region().get_arch().lookup_reg(&"sp".into()).unwrap()
            }
            Register::TCR_EL1 | Register::TCR_EL2 | Register::TCR_EL3 => return Ok(Variable::from(bits::new(0, 64))),
            Register::SCTLR_EL1 | Register::SCTLR_EL2 | Register::SCTLR_EL3 => return Ok(Variable::from(bits::new(0, 64))),
            Register::APIAKeyHi_EL1 | Register::APIAKeyLo_EL1 | Register::APIBKeyHi_EL1 | Register::APIBKeyLo_EL1 => {
                return Ok(Variable::from(bits::new(0, 64)))
            }
            Register::SCR | Register::SCR_EL3 => return Ok(Variable::from(bits::new(1025, 64))), // only RW bit = 1 (64-bit register width) and NS bit = 1 (non-secure)
            Register::HCR_EL2 => return Ok(Variable::from(bits::new(2147483648, 64))),           // only RW bit = 1 (64-bit register width)
            Register::DLR | Register::DSPSR | Register::ID_AA64DFR0_EL1 => return Ok(Variable::from(bits::new(0, 64))),
            Register::DBGOSDLR | Register::OSDLR_EL1 => return Ok(Variable::from(bits::new(1, 32))),
            Register::DBGPRCR
            | Register::DBGPRCR_EL1
            | Register::MDSCR_EL1
            | Register::MDCR_EL2
            | Register::MDCR_EL3
            | Register::EDSCR
            | Register::HSCTLR
            | Register::SCTLR => return Ok(Variable::from(bits::new(0, 32))),
            Register::OSLSR_EL1 => return Ok(Variable::from(bits::new(7, 32))),
            Register::MPAMVPM0_EL2
            | Register::MPAMVPMV_EL2
            | Register::MPAM3_EL3
            | Register::MPAMIDR_EL1
            | Register::MPAMHCR_EL2
            | Register::MPAM1_EL1
            | Register::MPAM2_EL2 => return Ok(Variable::from(bits::new(0, 64))),
            Register::HCR2 => return Ok(Variable::from(bits::new(0, 32))),
            Register::TTBR0_EL1 | Register::TTBR0_EL2 | Register::TTBR0_EL3 | Register::TTBR1_EL1 | Register::TTBR1_EL2 => {
                return Ok(Variable::from(bits::new(0, 64)))
            }
            Register::VTCR_EL2 | Register::VSTCR_EL2 => return Ok(Variable::from(bits::new(0, 32))),
            Register::VSTTBR_EL2 | Register::VTTBR_EL2 => return Ok(Variable::from(bits::new(0, 64))),
            _ => return Err(AArch64LifterError::UnsupportedRegister),
        };

        let val = builder.read_reg(register, ty).into();

        Ok(Self::new_air(val, ty))
    }

    pub fn to_register(&self, builder: &mut InstructionBuilder, register_type: Register) -> Result<(), AArch64LifterError> {
        let register = match register_type {
            Register::_R(index) => Reg::new(integer_to_u32!(index.to_integer()?)),
            Register::SP_EL0 | Register::SP_EL1 | Register::SP_EL2 | Register::SP_EL3 => {
                builder.get_code_region().get_arch().lookup_reg(&"sp".into()).unwrap()
            }
            Register::BTypeNext
            | Register::DLR
            | Register::DSPSR
            | Register::TFSR_EL1
            | Register::TFSR_EL2
            | Register::TFSR_EL3
            | Register::TFSRE0_EL1
            | Register::MPAMVPMV_EL2
            | Register::EDSCR => return Ok(()),
            _ => return Err(AArch64LifterError::UnsupportedRegister),
        };

        match self {
            Variable::Air(a) => {
                builder.write_reg(a.val, register, a.ty);
                Ok(())
            }
            Variable::Rust(RustVariable::bits(b)) => {
                let val = builder.iconst(b.value);
                let ty = Type::new_fixed_int(b.length as u8).ok_or(AArch64LifterError::InvalidBitsLength)?;
                builder.write_reg(val, register, ty);
                Ok(())
            }
            _ => panic!("Variable not bits"),
        }
    }

    pub fn from_flag(builder: &mut InstructionBuilder, flag_type: Flag) -> Result<Self, AArch64LifterError> {
        let reg = match flag_type {
            Flag::N => "nf",
            Flag::Z => "zf",
            Flag::C => "cf",
            Flag::V => "vf",
            Flag::D
            | Flag::I
            | Flag::A
            | Flag::F
            | Flag::T
            | Flag::IT
            | Flag::IL
            | Flag::SS
            | Flag::nRW
            | Flag::PAN
            | Flag::UAO
            | Flag::SSBS
            | Flag::TCO => return Ok(Variable::from(bits::new(0, 1))),
            Flag::EL | Flag::BTYPE => return Ok(Variable::from(bits::new(0, 2))),
            Flag::SP => return Ok(Variable::from(bits::new(1, 1))),
        };

        let ty = Type::new_fixed_int(1).ok_or(AArch64LifterError::InvalidBitsLength)?;

        let val = builder
            .read_reg(builder.get_code_region().get_arch().lookup_reg(&reg.into()).unwrap(), ty)
            .into();

        Ok(Self::new_air(val, ty))
    }

    pub fn to_flag(&self, builder: &mut InstructionBuilder, flag_type: Flag) -> Result<(), AArch64LifterError> {
        let reg = match flag_type {
            Flag::N => "nf",
            Flag::Z => "zf",
            Flag::C => "cf",
            Flag::V => "vf",
            Flag::D
            | Flag::I
            | Flag::A
            | Flag::F
            | Flag::T
            | Flag::IT
            | Flag::IL
            | Flag::SS
            | Flag::EL
            | Flag::nRW
            | Flag::SP
            | Flag::PAN
            | Flag::UAO
            | Flag::BTYPE
            | Flag::SSBS
            | Flag::TCO => return Ok(()),
        };

        match self {
            Variable::Air(a) => {
                builder.write_reg(a.val, builder.get_code_region().get_arch().lookup_reg(&reg.into()).unwrap(), a.ty);
                Ok(())
            }
            Variable::Rust(RustVariable::bits(_b)) => Err(AArch64LifterError::NotImplemented(file!(), line!())),
            _ => panic!("Variable not bits"),
        }
    }

    pub fn to_air(&self) -> Result<AirVariable, AArch64LifterError> {
        match self {
            Variable::Air(n) => Ok(n.clone()),
            _ => Err(AArch64LifterError::VariableNotAir),
        }
    }

    pub fn to_integer(&self) -> Result<integer, AArch64LifterError> {
        match self {
            Variable::Rust(RustVariable::integer(n)) => Ok(n.clone()),
            _ => panic!("Variable not integer"),
        }
    }

    pub fn to_bits(&self) -> Result<bits, AArch64LifterError> {
        match self {
            Variable::Rust(RustVariable::bits(n)) => Ok(n.clone()),
            _ => panic!("Variable not bits"),
        }
    }

    pub fn extract_slice(&self, builder: &mut InstructionBuilder, offset: Variable, length: Variable) -> Result<Self, AArch64LifterError> {
        match length {
            Variable::Air(_) => Err(AArch64LifterError::VariableNotPropagated),
            Variable::Rust(RustVariable::integer(li)) => {
                match offset {
                    Variable::Air(_oa) => {
                        // TODO: extract_slice possible even if offset isn't propagated
                        Err(AArch64LifterError::NotImplemented(file!(), line!()))
                    }
                    Variable::Rust(RustVariable::integer(oi)) => match self {
                        Variable::Air(a) => {
                            let ou8 = integer_to_u8!(oi);
                            let lu8 = integer_to_u8!(li);
                            let new_ty = Type::new_fixed_int(lu8).ok_or(AArch64LifterError::InvalidBitsLength)?;

                            let bits = if a.ty.is_int() {
                                let ty = Type::new_fixed_int(ou8 + lu8).ok_or(AArch64LifterError::InvalidBitsLength)?;
                                AirVariable {
                                    val: builder.to_bits(a.val, ty).into(),
                                    ty,
                                }
                            } else if a.ty.is_fixed_size_int() {
                                *a
                            } else {
                                return Err(AArch64LifterError::InvalidAirType);
                            };

                            let shifted = if oi > integer::from(0) {
                                let oa = builder.iconst(oi);
                                builder.lshr(bits.val, oa, bits.ty).into()
                            } else {
                                bits.val
                            };

                            let truncated = if lu8 < bits.ty.bit_width().ok_or(AArch64LifterError::InvalidAirType)? {
                                builder.trunc(shifted, bits.ty, new_ty).into()
                            } else {
                                shifted
                            };

                            Ok(Self::new_air(truncated, new_ty))
                        }
                        Variable::Rust(RustVariable::integer(i)) => Ok(i.extract_slice(integer_to_usize!(oi), integer_to_usize!(li))?.into()),
                        Variable::Rust(RustVariable::bits(b)) => Ok(b.extract_slice(integer_to_usize!(oi), integer_to_usize!(li))?.into()),
                        _ => Err(AArch64LifterError::VariableNotIntegerOrBits),
                    },
                    _ => panic!("Variable not integer"),
                }
            }
            _ => panic!("Variable not integer"),
        }
    }

    pub fn assign_slice(
        &mut self,
        builder: &mut InstructionBuilder,
        value: Variable,
        offset: Variable,
        length: Variable,
    ) -> Result<(), AArch64LifterError> {
        match length {
            Variable::Air(_) => Err(AArch64LifterError::VariableNotPropagated),
            Variable::Rust(RustVariable::integer(li)) => {
                match offset {
                    Variable::Air(_oa) => {
                        // TODO: extract_slice possible even if offset isn't propagated
                        Err(AArch64LifterError::NotImplemented(file!(), line!()))
                    }
                    Variable::Rust(RustVariable::integer(oi)) => match self {
                        Variable::Air(a) => {
                            if a.ty.is_int() {
                                // TODO: not sure if assigning slice to an integer is possible but just in case
                                return Err(AArch64LifterError::NotImplemented(file!(), line!()));
                            } else if !a.ty.is_fixed_size_int() {
                                return Err(AArch64LifterError::InvalidAirType);
                            }

                            let ou8 = integer_to_u8!(oi);
                            let oa = builder.iconst(ou8);
                            let lu8 = integer_to_u8!(li);
                            let op_length = a.ty.bit_width().ok_or(AArch64LifterError::InvalidBitsLength)?;

                            if lu8 < op_length {
                                let slice_mask = ((1u128 << lu8) - 1) << ou8;
                                let clear_mask = (!slice_mask) & ((1u128 << op_length) - 1);

                                let mask = builder.iconst(clear_mask);
                                let cleared_value = builder.and(a.val, mask, a.ty);
                                let shifted_slice = match value {
                                    Variable::Air(va) => builder.lshl(va.val, oa, va.ty).into(),
                                    Variable::Rust(RustVariable::bits(b)) => builder.iconst(b.value << ou8),
                                    _ => panic!("Variable not bits"),
                                };

                                a.val = builder.or(cleared_value, shifted_slice, a.ty).into();
                            }

                            Ok(())
                        }
                        Variable::Rust(RustVariable::integer(i)) => Err(AArch64LifterError::NotImplemented(file!(), line!())),
                        Variable::Rust(RustVariable::bits(b)) => match value {
                            Variable::Air(va) => {
                                let ou8 = integer_to_u8!(oi);
                                let oa = builder.iconst(ou8);
                                let lu8 = integer_to_u8!(li);
                                let op_length = b.length as u8;

                                if lu8 < op_length {
                                    let slice_mask = ((1u128 << lu8) - 1) << ou8;
                                    let clear_mask = (!slice_mask) & ((1u128 << op_length) - 1);

                                    let cleared_value = builder.iconst(b.value & clear_mask);
                                    let shifted_slice = builder.lshl(va.val, oa, va.ty);

                                    let a_ty = Type::new_fixed_int(b.length as u8).ok_or(AArch64LifterError::InvalidBitsLength)?;
                                    let a_val = builder.or(cleared_value, shifted_slice, a_ty).into();
                                    *self = Variable::new_air(a_val, a_ty);
                                }

                                Ok(())
                            }
                            Variable::Rust(RustVariable::bits(vb)) => Ok(b.assign_slice(vb, integer_to_usize!(oi), integer_to_usize!(li))?),
                            _ => panic!("Variable not bits"),
                        },
                        _ => Err(AArch64LifterError::VariableNotIntegerOrBits),
                    },
                    _ => panic!("Variable not integer"),
                }
            }
            _ => panic!("Variable not integer"),
        }
    }

    pub fn air_from_integer(builder: &mut InstructionBuilder, value: integer) -> Result<Self, AArch64LifterError> {
        Ok(Variable::Air(AirVariable::from_integer(builder, value)?))
    }

    pub fn air_from_bits(builder: &mut InstructionBuilder, value: bits) -> Result<Self, AArch64LifterError> {
        Ok(Variable::Air(AirVariable::from_bits(builder, value)?))
    }

    pub fn air_from_boolean(builder: &mut InstructionBuilder, value: boolean) -> Result<Self, AArch64LifterError> {
        Ok(Variable::Air(AirVariable::from_boolean(builder, value)?))
    }

    pub fn air_from_enum(builder: &mut InstructionBuilder, index: u8) -> Result<Self, AArch64LifterError> {
        Ok(Variable::Air(AirVariable::from_enum(builder, index)?))
    }
}

impl AirVariable {
    pub fn from_integer(builder: &mut InstructionBuilder, value: integer) -> Result<Self, AArch64LifterError> {
        Ok(Self {
            val: builder.iconst(value),
            ty: Type::Int,
        })
    }

    pub fn from_bits(builder: &mut InstructionBuilder, value: bits) -> Result<Self, AArch64LifterError> {
        Ok(Self {
            val: builder.iconst(value.value as u64),
            ty: Type::new_fixed_int(value.length as u8).ok_or(AArch64LifterError::InvalidBitsLength)?,
        })
    }

    pub fn from_boolean(builder: &mut InstructionBuilder, value: boolean) -> Result<Self, AArch64LifterError> {
        Ok(Self {
            val: builder.iconst(if value == boolean::TRUE { 1 } else { 0 }),
            ty: Type::Bool,
        })
    }

    pub fn from_enum(builder: &mut InstructionBuilder, index: u8) -> Result<Self, AArch64LifterError> {
        Ok(Self {
            val: builder.iconst(index),
            ty: Type::new_fixed_int(8).ok_or(AArch64LifterError::InvalidBitsLength)?,
        })
    }
}

pub struct BlockSequencer {
    pass: PassType,
    inter_checkpoints: Vec<u64>,
    intra_checkpoints: HashMap<u64, VecDeque<Vec<Type>>>,
    inter_blocks: HashMap<u64, BasicBlock>,
    intra_blocks: HashMap<u64, VecDeque<BasicBlock>>,
}

pub enum PassType {
    Checkpointing,
    Lifting,
}

pub enum BlockType {
    InterBlock,
    IntraBlock,
}

impl BlockSequencer {
    pub fn new(code: &[u8], decoder: &decode::Decoder, decode_only: bool) -> Result<Self, AArch64LifterError> {
        let mut sequencer = Self {
            pass: PassType::Checkpointing,
            inter_checkpoints: Vec::new(),
            intra_checkpoints: HashMap::new(),
            inter_blocks: HashMap::new(),
            intra_blocks: HashMap::new(),
        };

        if !decode_only {
            sequencer.get_checkpoints(code, decoder)?;
        }

        Ok(sequencer)
    }

    fn get_checkpoints(&mut self, code: &[u8], decoder: &decode::Decoder) -> Result<(), AArch64LifterError> {
        let mut reader = decode::BitReader::new(code);
        let arch = get_arch(Architecture::Aarch64(Aarch64Architecture::Aarch64)).unwrap();
        let mut dummy_blob = CodeRegion::with_entry_block(arch);
        let mut dummy_builder = dummy_blob.insert();

        // let total = code.len() / INSTRUCTION_BYTE_SIZE;
        // let pb = ProgressBar::new(total as u64);
        // pb.set_message("Pass 1: marking block checkpoints");
        // pb.set_style(
        //     ProgressStyle::with_template("\t\t[{bar:20}] {pos}/{len} {msg}")
        //         .unwrap()
        //         .progress_chars("=> "),
        // );

        let mut address = 0u64;

        loop {
            if let Some(instruction) = decoder.decode(&mut reader)? {
                generated_sequencer_logic(instruction, &mut dummy_builder, self, address)?;
                // pb.inc(1);
            } else {
                // pb.finish_and_clear();
                break;
            }

            address += INSTRUCTION_BYTE_SIZE as u64;
        }

        Ok(())
    }

    pub fn get_block(
        &mut self,
        address: u64,
        block_type: BlockType,
        builder: &mut InstructionBuilder,
        block_params: &[Type],
    ) -> Result<BasicBlock, AArch64LifterError> {
        match self.pass {
            PassType::Checkpointing => match block_type {
                BlockType::InterBlock => {
                    if !self.inter_checkpoints.contains(&address) {
                        self.inter_checkpoints.push(address);
                    }
                    if !block_params.is_empty() {
                        panic!("Inter blocks should not have block params")
                    }
                    Ok(builder.create_block(format!("addr_{}", address), vec![]))
                }
                BlockType::IntraBlock => {
                    let checkpoints = self.intra_checkpoints.entry(address).or_insert(VecDeque::new());
                    let index = checkpoints.len();
                    checkpoints.push_back(block_params.to_vec());
                    Ok(builder.create_block(format!("addr_{}_block_{}", address, index), block_params.to_vec()))
                }
            },
            PassType::Lifting => match block_type {
                BlockType::InterBlock => self
                    .inter_blocks
                    .get(&address)
                    .copied()
                    .ok_or(AArch64LifterError::MissingBlockInSequencer),
                BlockType::IntraBlock => Ok(self
                    .intra_blocks
                    .get_mut(&address)
                    .expect("Missing block in sequencer")
                    .pop_front()
                    .expect("Missing block in sequencer")),
            },
        }
    }

    pub fn create_blocks(&mut self, builder: &mut InstructionBuilder) -> Result<(), AArch64LifterError> {
        let mut all_addresses = self.inter_checkpoints.iter().copied().collect::<HashSet<u64>>();
        all_addresses.extend(self.intra_checkpoints.keys().copied());
        let mut sorted_addresses = all_addresses.into_iter().collect::<Vec<u64>>();
        sorted_addresses.sort();

        for address in sorted_addresses {
            if self.inter_checkpoints.contains(&address) {
                let block = builder.create_block(format!("addr_{}", address), vec![]);
                self.inter_blocks.insert(address, block);
            }

            if let Some(checkpoints) = self.intra_checkpoints.get(&address) {
                let mut intra_blocks = VecDeque::new();
                for (i, params) in checkpoints.iter().enumerate() {
                    let block = builder.create_block(format!("addr_{}_block_{}", address, i), params.clone());
                    intra_blocks.push_back(block);
                }
                self.intra_blocks.insert(address, intra_blocks);
            }
        }

        self.pass = PassType::Lifting;

        Ok(())
    }
}

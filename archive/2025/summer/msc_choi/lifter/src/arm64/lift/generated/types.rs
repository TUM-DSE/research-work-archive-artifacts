#![allow(nonstandard_style, unused)]
use crate::arm64::{common, decode, lift, AArch64LifterError};
use crate::arm64::lift::types::BlockSequencer;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, BTreeMap};
use std::convert::TryFrom;
use std::hash::Hash;
use target_lexicon::{Aarch64Architecture, Architecture};
use tnj::air::instructions::{BasicBlock, CodeRegion, Value};
use tnj::air::instructions::builder::InstructionBuilder;
use tnj::types::Type;
use tnj::arch::get_arch;
use num_traits::ToPrimitive;
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RustVariable {
    integer(common::types::integer),
    bits(common::types::bits),
    AccType(common::types::AccType),
    ArchVersion(common::types::ArchVersion),
    BranchType(common::types::BranchType),
    CompareOp(common::types::CompareOp),
    Constraint(common::types::Constraint),
    CountOp(common::types::CountOp),
    CrossTriggerIn(common::types::CrossTriggerIn),
    CrossTriggerOut(common::types::CrossTriggerOut),
    DeviceType(common::types::DeviceType),
    Exception(common::types::Exception),
    ExtendType(common::types::ExtendType),
    FPConvOp(common::types::FPConvOp),
    FPExc(common::types::FPExc),
    FPMaxMinOp(common::types::FPMaxMinOp),
    FPRounding(common::types::FPRounding),
    FPType(common::types::FPType),
    FPUnaryOp(common::types::FPUnaryOp),
    Fault(common::types::Fault),
    ImmediateOp(common::types::ImmediateOp),
    InstrSet(common::types::InstrSet),
    InterruptID(common::types::InterruptID),
    LogicalOp(common::types::LogicalOp),
    MBReqDomain(common::types::MBReqDomain),
    MBReqTypes(common::types::MBReqTypes),
    MemAtomicOp(common::types::MemAtomicOp),
    MemBarrierOp(common::types::MemBarrierOp),
    MemOp(common::types::MemOp),
    MemType(common::types::MemType),
    MoveWideOp(common::types::MoveWideOp),
    OpType(common::types::OpType),
    PSTATEField(common::types::PSTATEField),
    PrefetchHint(common::types::PrefetchHint),
    PrivilegeLevel(common::types::PrivilegeLevel),
    ReduceOp(common::types::ReduceOp),
    SRType(common::types::SRType),
    SVECmp(common::types::SVECmp),
    ShiftType(common::types::ShiftType),
    SysRegAccess(common::types::SysRegAccess),
    SystemHintOp(common::types::SystemHintOp),
    SystemOp(common::types::SystemOp),
    TimeStamp(common::types::TimeStamp),
    Unpredictable(common::types::Unpredictable),
    VBitOp(common::types::VBitOp),
    VBitOps(common::types::VBitOps),
    VCGEtype(common::types::VCGEtype),
    VCGTtype(common::types::VCGTtype),
    VFPNegMul(common::types::VFPNegMul),
    __InstrEnc(common::types::__InstrEnc),
    boolean(common::types::boolean),
    signal(common::types::signal),
}
impl From<common::types::AccType> for lift::types::Variable {
    fn from(value: common::types::AccType) -> Self {
        lift::types::Variable::Rust(RustVariable::AccType(value))
    }
}
impl From<common::types::ArchVersion> for lift::types::Variable {
    fn from(value: common::types::ArchVersion) -> Self {
        lift::types::Variable::Rust(RustVariable::ArchVersion(value))
    }
}
impl From<common::types::BranchType> for lift::types::Variable {
    fn from(value: common::types::BranchType) -> Self {
        lift::types::Variable::Rust(RustVariable::BranchType(value))
    }
}
impl From<common::types::CompareOp> for lift::types::Variable {
    fn from(value: common::types::CompareOp) -> Self {
        lift::types::Variable::Rust(RustVariable::CompareOp(value))
    }
}
impl From<common::types::Constraint> for lift::types::Variable {
    fn from(value: common::types::Constraint) -> Self {
        lift::types::Variable::Rust(RustVariable::Constraint(value))
    }
}
impl From<common::types::CountOp> for lift::types::Variable {
    fn from(value: common::types::CountOp) -> Self {
        lift::types::Variable::Rust(RustVariable::CountOp(value))
    }
}
impl From<common::types::CrossTriggerIn> for lift::types::Variable {
    fn from(value: common::types::CrossTriggerIn) -> Self {
        lift::types::Variable::Rust(RustVariable::CrossTriggerIn(value))
    }
}
impl From<common::types::CrossTriggerOut> for lift::types::Variable {
    fn from(value: common::types::CrossTriggerOut) -> Self {
        lift::types::Variable::Rust(RustVariable::CrossTriggerOut(value))
    }
}
impl From<common::types::DeviceType> for lift::types::Variable {
    fn from(value: common::types::DeviceType) -> Self {
        lift::types::Variable::Rust(RustVariable::DeviceType(value))
    }
}
impl From<common::types::Exception> for lift::types::Variable {
    fn from(value: common::types::Exception) -> Self {
        lift::types::Variable::Rust(RustVariable::Exception(value))
    }
}
impl From<common::types::ExtendType> for lift::types::Variable {
    fn from(value: common::types::ExtendType) -> Self {
        lift::types::Variable::Rust(RustVariable::ExtendType(value))
    }
}
impl From<common::types::FPConvOp> for lift::types::Variable {
    fn from(value: common::types::FPConvOp) -> Self {
        lift::types::Variable::Rust(RustVariable::FPConvOp(value))
    }
}
impl From<common::types::FPExc> for lift::types::Variable {
    fn from(value: common::types::FPExc) -> Self {
        lift::types::Variable::Rust(RustVariable::FPExc(value))
    }
}
impl From<common::types::FPMaxMinOp> for lift::types::Variable {
    fn from(value: common::types::FPMaxMinOp) -> Self {
        lift::types::Variable::Rust(RustVariable::FPMaxMinOp(value))
    }
}
impl From<common::types::FPRounding> for lift::types::Variable {
    fn from(value: common::types::FPRounding) -> Self {
        lift::types::Variable::Rust(RustVariable::FPRounding(value))
    }
}
impl From<common::types::FPType> for lift::types::Variable {
    fn from(value: common::types::FPType) -> Self {
        lift::types::Variable::Rust(RustVariable::FPType(value))
    }
}
impl From<common::types::FPUnaryOp> for lift::types::Variable {
    fn from(value: common::types::FPUnaryOp) -> Self {
        lift::types::Variable::Rust(RustVariable::FPUnaryOp(value))
    }
}
impl From<common::types::Fault> for lift::types::Variable {
    fn from(value: common::types::Fault) -> Self {
        lift::types::Variable::Rust(RustVariable::Fault(value))
    }
}
impl From<common::types::ImmediateOp> for lift::types::Variable {
    fn from(value: common::types::ImmediateOp) -> Self {
        lift::types::Variable::Rust(RustVariable::ImmediateOp(value))
    }
}
impl From<common::types::InstrSet> for lift::types::Variable {
    fn from(value: common::types::InstrSet) -> Self {
        lift::types::Variable::Rust(RustVariable::InstrSet(value))
    }
}
impl From<common::types::InterruptID> for lift::types::Variable {
    fn from(value: common::types::InterruptID) -> Self {
        lift::types::Variable::Rust(RustVariable::InterruptID(value))
    }
}
impl From<common::types::LogicalOp> for lift::types::Variable {
    fn from(value: common::types::LogicalOp) -> Self {
        lift::types::Variable::Rust(RustVariable::LogicalOp(value))
    }
}
impl From<common::types::MBReqDomain> for lift::types::Variable {
    fn from(value: common::types::MBReqDomain) -> Self {
        lift::types::Variable::Rust(RustVariable::MBReqDomain(value))
    }
}
impl From<common::types::MBReqTypes> for lift::types::Variable {
    fn from(value: common::types::MBReqTypes) -> Self {
        lift::types::Variable::Rust(RustVariable::MBReqTypes(value))
    }
}
impl From<common::types::MemAtomicOp> for lift::types::Variable {
    fn from(value: common::types::MemAtomicOp) -> Self {
        lift::types::Variable::Rust(RustVariable::MemAtomicOp(value))
    }
}
impl From<common::types::MemBarrierOp> for lift::types::Variable {
    fn from(value: common::types::MemBarrierOp) -> Self {
        lift::types::Variable::Rust(RustVariable::MemBarrierOp(value))
    }
}
impl From<common::types::MemOp> for lift::types::Variable {
    fn from(value: common::types::MemOp) -> Self {
        lift::types::Variable::Rust(RustVariable::MemOp(value))
    }
}
impl From<common::types::MemType> for lift::types::Variable {
    fn from(value: common::types::MemType) -> Self {
        lift::types::Variable::Rust(RustVariable::MemType(value))
    }
}
impl From<common::types::MoveWideOp> for lift::types::Variable {
    fn from(value: common::types::MoveWideOp) -> Self {
        lift::types::Variable::Rust(RustVariable::MoveWideOp(value))
    }
}
impl From<common::types::OpType> for lift::types::Variable {
    fn from(value: common::types::OpType) -> Self {
        lift::types::Variable::Rust(RustVariable::OpType(value))
    }
}
impl From<common::types::PSTATEField> for lift::types::Variable {
    fn from(value: common::types::PSTATEField) -> Self {
        lift::types::Variable::Rust(RustVariable::PSTATEField(value))
    }
}
impl From<common::types::PrefetchHint> for lift::types::Variable {
    fn from(value: common::types::PrefetchHint) -> Self {
        lift::types::Variable::Rust(RustVariable::PrefetchHint(value))
    }
}
impl From<common::types::PrivilegeLevel> for lift::types::Variable {
    fn from(value: common::types::PrivilegeLevel) -> Self {
        lift::types::Variable::Rust(RustVariable::PrivilegeLevel(value))
    }
}
impl From<common::types::ReduceOp> for lift::types::Variable {
    fn from(value: common::types::ReduceOp) -> Self {
        lift::types::Variable::Rust(RustVariable::ReduceOp(value))
    }
}
impl From<common::types::SRType> for lift::types::Variable {
    fn from(value: common::types::SRType) -> Self {
        lift::types::Variable::Rust(RustVariable::SRType(value))
    }
}
impl From<common::types::SVECmp> for lift::types::Variable {
    fn from(value: common::types::SVECmp) -> Self {
        lift::types::Variable::Rust(RustVariable::SVECmp(value))
    }
}
impl From<common::types::ShiftType> for lift::types::Variable {
    fn from(value: common::types::ShiftType) -> Self {
        lift::types::Variable::Rust(RustVariable::ShiftType(value))
    }
}
impl From<common::types::SysRegAccess> for lift::types::Variable {
    fn from(value: common::types::SysRegAccess) -> Self {
        lift::types::Variable::Rust(RustVariable::SysRegAccess(value))
    }
}
impl From<common::types::SystemHintOp> for lift::types::Variable {
    fn from(value: common::types::SystemHintOp) -> Self {
        lift::types::Variable::Rust(RustVariable::SystemHintOp(value))
    }
}
impl From<common::types::SystemOp> for lift::types::Variable {
    fn from(value: common::types::SystemOp) -> Self {
        lift::types::Variable::Rust(RustVariable::SystemOp(value))
    }
}
impl From<common::types::TimeStamp> for lift::types::Variable {
    fn from(value: common::types::TimeStamp) -> Self {
        lift::types::Variable::Rust(RustVariable::TimeStamp(value))
    }
}
impl From<common::types::Unpredictable> for lift::types::Variable {
    fn from(value: common::types::Unpredictable) -> Self {
        lift::types::Variable::Rust(RustVariable::Unpredictable(value))
    }
}
impl From<common::types::VBitOp> for lift::types::Variable {
    fn from(value: common::types::VBitOp) -> Self {
        lift::types::Variable::Rust(RustVariable::VBitOp(value))
    }
}
impl From<common::types::VBitOps> for lift::types::Variable {
    fn from(value: common::types::VBitOps) -> Self {
        lift::types::Variable::Rust(RustVariable::VBitOps(value))
    }
}
impl From<common::types::VCGEtype> for lift::types::Variable {
    fn from(value: common::types::VCGEtype) -> Self {
        lift::types::Variable::Rust(RustVariable::VCGEtype(value))
    }
}
impl From<common::types::VCGTtype> for lift::types::Variable {
    fn from(value: common::types::VCGTtype) -> Self {
        lift::types::Variable::Rust(RustVariable::VCGTtype(value))
    }
}
impl From<common::types::VFPNegMul> for lift::types::Variable {
    fn from(value: common::types::VFPNegMul) -> Self {
        lift::types::Variable::Rust(RustVariable::VFPNegMul(value))
    }
}
impl From<common::types::__InstrEnc> for lift::types::Variable {
    fn from(value: common::types::__InstrEnc) -> Self {
        lift::types::Variable::Rust(RustVariable::__InstrEnc(value))
    }
}
impl From<common::types::boolean> for lift::types::Variable {
    fn from(value: common::types::boolean) -> Self {
        lift::types::Variable::Rust(RustVariable::boolean(value))
    }
}
impl From<common::types::signal> for lift::types::Variable {
    fn from(value: common::types::signal) -> Self {
        lift::types::Variable::Rust(RustVariable::signal(value))
    }
}
impl lift::types::Variable {
    pub fn promote_to_air(
        &self,
        builder: &mut InstructionBuilder,
    ) -> Result<Self, AArch64LifterError> {
        match self {
            lift::types::Variable::Rust(RustVariable::integer(n)) => {
                Ok(Self::air_from_integer(builder, n.clone())?)
            }
            lift::types::Variable::Rust(RustVariable::bits(n)) => {
                Ok(Self::air_from_bits(builder, *n)?)
            }
            lift::types::Variable::Rust(RustVariable::boolean(n)) => {
                Ok(Self::air_from_boolean(builder, *n)?)
            }
            lift::types::Variable::Rust(RustVariable::AccType(n)) => {
                Ok(Self::air_from_enum(builder, *n as u8)?)
            }
            lift::types::Variable::Rust(RustVariable::ArchVersion(n)) => {
                Ok(Self::air_from_enum(builder, *n as u8)?)
            }
            lift::types::Variable::Rust(RustVariable::BranchType(n)) => {
                Ok(Self::air_from_enum(builder, *n as u8)?)
            }
            lift::types::Variable::Rust(RustVariable::CompareOp(n)) => {
                Ok(Self::air_from_enum(builder, *n as u8)?)
            }
            lift::types::Variable::Rust(RustVariable::Constraint(n)) => {
                Ok(Self::air_from_enum(builder, *n as u8)?)
            }
            lift::types::Variable::Rust(RustVariable::CountOp(n)) => {
                Ok(Self::air_from_enum(builder, *n as u8)?)
            }
            lift::types::Variable::Rust(RustVariable::CrossTriggerIn(n)) => {
                Ok(Self::air_from_enum(builder, *n as u8)?)
            }
            lift::types::Variable::Rust(RustVariable::CrossTriggerOut(n)) => {
                Ok(Self::air_from_enum(builder, *n as u8)?)
            }
            lift::types::Variable::Rust(RustVariable::DeviceType(n)) => {
                Ok(Self::air_from_enum(builder, *n as u8)?)
            }
            lift::types::Variable::Rust(RustVariable::Exception(n)) => {
                Ok(Self::air_from_enum(builder, *n as u8)?)
            }
            lift::types::Variable::Rust(RustVariable::ExtendType(n)) => {
                Ok(Self::air_from_enum(builder, *n as u8)?)
            }
            lift::types::Variable::Rust(RustVariable::FPConvOp(n)) => {
                Ok(Self::air_from_enum(builder, *n as u8)?)
            }
            lift::types::Variable::Rust(RustVariable::FPExc(n)) => {
                Ok(Self::air_from_enum(builder, *n as u8)?)
            }
            lift::types::Variable::Rust(RustVariable::FPMaxMinOp(n)) => {
                Ok(Self::air_from_enum(builder, *n as u8)?)
            }
            lift::types::Variable::Rust(RustVariable::FPRounding(n)) => {
                Ok(Self::air_from_enum(builder, *n as u8)?)
            }
            lift::types::Variable::Rust(RustVariable::FPType(n)) => {
                Ok(Self::air_from_enum(builder, *n as u8)?)
            }
            lift::types::Variable::Rust(RustVariable::FPUnaryOp(n)) => {
                Ok(Self::air_from_enum(builder, *n as u8)?)
            }
            lift::types::Variable::Rust(RustVariable::Fault(n)) => {
                Ok(Self::air_from_enum(builder, *n as u8)?)
            }
            lift::types::Variable::Rust(RustVariable::ImmediateOp(n)) => {
                Ok(Self::air_from_enum(builder, *n as u8)?)
            }
            lift::types::Variable::Rust(RustVariable::InstrSet(n)) => {
                Ok(Self::air_from_enum(builder, *n as u8)?)
            }
            lift::types::Variable::Rust(RustVariable::InterruptID(n)) => {
                Ok(Self::air_from_enum(builder, *n as u8)?)
            }
            lift::types::Variable::Rust(RustVariable::LogicalOp(n)) => {
                Ok(Self::air_from_enum(builder, *n as u8)?)
            }
            lift::types::Variable::Rust(RustVariable::MBReqDomain(n)) => {
                Ok(Self::air_from_enum(builder, *n as u8)?)
            }
            lift::types::Variable::Rust(RustVariable::MBReqTypes(n)) => {
                Ok(Self::air_from_enum(builder, *n as u8)?)
            }
            lift::types::Variable::Rust(RustVariable::MemAtomicOp(n)) => {
                Ok(Self::air_from_enum(builder, *n as u8)?)
            }
            lift::types::Variable::Rust(RustVariable::MemBarrierOp(n)) => {
                Ok(Self::air_from_enum(builder, *n as u8)?)
            }
            lift::types::Variable::Rust(RustVariable::MemOp(n)) => {
                Ok(Self::air_from_enum(builder, *n as u8)?)
            }
            lift::types::Variable::Rust(RustVariable::MemType(n)) => {
                Ok(Self::air_from_enum(builder, *n as u8)?)
            }
            lift::types::Variable::Rust(RustVariable::MoveWideOp(n)) => {
                Ok(Self::air_from_enum(builder, *n as u8)?)
            }
            lift::types::Variable::Rust(RustVariable::OpType(n)) => {
                Ok(Self::air_from_enum(builder, *n as u8)?)
            }
            lift::types::Variable::Rust(RustVariable::PSTATEField(n)) => {
                Ok(Self::air_from_enum(builder, *n as u8)?)
            }
            lift::types::Variable::Rust(RustVariable::PrefetchHint(n)) => {
                Ok(Self::air_from_enum(builder, *n as u8)?)
            }
            lift::types::Variable::Rust(RustVariable::PrivilegeLevel(n)) => {
                Ok(Self::air_from_enum(builder, *n as u8)?)
            }
            lift::types::Variable::Rust(RustVariable::ReduceOp(n)) => {
                Ok(Self::air_from_enum(builder, *n as u8)?)
            }
            lift::types::Variable::Rust(RustVariable::SRType(n)) => {
                Ok(Self::air_from_enum(builder, *n as u8)?)
            }
            lift::types::Variable::Rust(RustVariable::SVECmp(n)) => {
                Ok(Self::air_from_enum(builder, *n as u8)?)
            }
            lift::types::Variable::Rust(RustVariable::ShiftType(n)) => {
                Ok(Self::air_from_enum(builder, *n as u8)?)
            }
            lift::types::Variable::Rust(RustVariable::SysRegAccess(n)) => {
                Ok(Self::air_from_enum(builder, *n as u8)?)
            }
            lift::types::Variable::Rust(RustVariable::SystemHintOp(n)) => {
                Ok(Self::air_from_enum(builder, *n as u8)?)
            }
            lift::types::Variable::Rust(RustVariable::SystemOp(n)) => {
                Ok(Self::air_from_enum(builder, *n as u8)?)
            }
            lift::types::Variable::Rust(RustVariable::TimeStamp(n)) => {
                Ok(Self::air_from_enum(builder, *n as u8)?)
            }
            lift::types::Variable::Rust(RustVariable::Unpredictable(n)) => {
                Ok(Self::air_from_enum(builder, *n as u8)?)
            }
            lift::types::Variable::Rust(RustVariable::VBitOp(n)) => {
                Ok(Self::air_from_enum(builder, *n as u8)?)
            }
            lift::types::Variable::Rust(RustVariable::VBitOps(n)) => {
                Ok(Self::air_from_enum(builder, *n as u8)?)
            }
            lift::types::Variable::Rust(RustVariable::VCGEtype(n)) => {
                Ok(Self::air_from_enum(builder, *n as u8)?)
            }
            lift::types::Variable::Rust(RustVariable::VCGTtype(n)) => {
                Ok(Self::air_from_enum(builder, *n as u8)?)
            }
            lift::types::Variable::Rust(RustVariable::VFPNegMul(n)) => {
                Ok(Self::air_from_enum(builder, *n as u8)?)
            }
            lift::types::Variable::Rust(RustVariable::__InstrEnc(n)) => {
                Ok(Self::air_from_enum(builder, *n as u8)?)
            }
            lift::types::Variable::Rust(RustVariable::boolean(n)) => {
                Ok(Self::air_from_enum(builder, *n as u8)?)
            }
            lift::types::Variable::Rust(RustVariable::signal(n)) => {
                Ok(Self::air_from_enum(builder, *n as u8)?)
            }
            lift::types::Variable::Air(_) => Ok(self.clone()),
            _ => Err(AArch64LifterError::NotImplemented(file!(), line!())),
        }
    }
    pub fn to_AccType(&self) -> Result<common::types::AccType, AArch64LifterError> {
        match self {
            lift::types::Variable::Rust(RustVariable::AccType(n)) => Ok(n.clone()),
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    pub fn to_ArchVersion(
        &self,
    ) -> Result<common::types::ArchVersion, AArch64LifterError> {
        match self {
            lift::types::Variable::Rust(RustVariable::ArchVersion(n)) => Ok(n.clone()),
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    pub fn to_BranchType(
        &self,
    ) -> Result<common::types::BranchType, AArch64LifterError> {
        match self {
            lift::types::Variable::Rust(RustVariable::BranchType(n)) => Ok(n.clone()),
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    pub fn to_CompareOp(&self) -> Result<common::types::CompareOp, AArch64LifterError> {
        match self {
            lift::types::Variable::Rust(RustVariable::CompareOp(n)) => Ok(n.clone()),
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    pub fn to_Constraint(
        &self,
    ) -> Result<common::types::Constraint, AArch64LifterError> {
        match self {
            lift::types::Variable::Rust(RustVariable::Constraint(n)) => Ok(n.clone()),
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    pub fn to_CountOp(&self) -> Result<common::types::CountOp, AArch64LifterError> {
        match self {
            lift::types::Variable::Rust(RustVariable::CountOp(n)) => Ok(n.clone()),
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    pub fn to_CrossTriggerIn(
        &self,
    ) -> Result<common::types::CrossTriggerIn, AArch64LifterError> {
        match self {
            lift::types::Variable::Rust(RustVariable::CrossTriggerIn(n)) => Ok(n.clone()),
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    pub fn to_CrossTriggerOut(
        &self,
    ) -> Result<common::types::CrossTriggerOut, AArch64LifterError> {
        match self {
            lift::types::Variable::Rust(RustVariable::CrossTriggerOut(n)) => {
                Ok(n.clone())
            }
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    pub fn to_DeviceType(
        &self,
    ) -> Result<common::types::DeviceType, AArch64LifterError> {
        match self {
            lift::types::Variable::Rust(RustVariable::DeviceType(n)) => Ok(n.clone()),
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    pub fn to_Exception(&self) -> Result<common::types::Exception, AArch64LifterError> {
        match self {
            lift::types::Variable::Rust(RustVariable::Exception(n)) => Ok(n.clone()),
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    pub fn to_ExtendType(
        &self,
    ) -> Result<common::types::ExtendType, AArch64LifterError> {
        match self {
            lift::types::Variable::Rust(RustVariable::ExtendType(n)) => Ok(n.clone()),
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    pub fn to_FPConvOp(&self) -> Result<common::types::FPConvOp, AArch64LifterError> {
        match self {
            lift::types::Variable::Rust(RustVariable::FPConvOp(n)) => Ok(n.clone()),
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    pub fn to_FPExc(&self) -> Result<common::types::FPExc, AArch64LifterError> {
        match self {
            lift::types::Variable::Rust(RustVariable::FPExc(n)) => Ok(n.clone()),
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    pub fn to_FPMaxMinOp(
        &self,
    ) -> Result<common::types::FPMaxMinOp, AArch64LifterError> {
        match self {
            lift::types::Variable::Rust(RustVariable::FPMaxMinOp(n)) => Ok(n.clone()),
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    pub fn to_FPRounding(
        &self,
    ) -> Result<common::types::FPRounding, AArch64LifterError> {
        match self {
            lift::types::Variable::Rust(RustVariable::FPRounding(n)) => Ok(n.clone()),
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    pub fn to_FPType(&self) -> Result<common::types::FPType, AArch64LifterError> {
        match self {
            lift::types::Variable::Rust(RustVariable::FPType(n)) => Ok(n.clone()),
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    pub fn to_FPUnaryOp(&self) -> Result<common::types::FPUnaryOp, AArch64LifterError> {
        match self {
            lift::types::Variable::Rust(RustVariable::FPUnaryOp(n)) => Ok(n.clone()),
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    pub fn to_Fault(&self) -> Result<common::types::Fault, AArch64LifterError> {
        match self {
            lift::types::Variable::Rust(RustVariable::Fault(n)) => Ok(n.clone()),
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    pub fn to_ImmediateOp(
        &self,
    ) -> Result<common::types::ImmediateOp, AArch64LifterError> {
        match self {
            lift::types::Variable::Rust(RustVariable::ImmediateOp(n)) => Ok(n.clone()),
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    pub fn to_InstrSet(&self) -> Result<common::types::InstrSet, AArch64LifterError> {
        match self {
            lift::types::Variable::Rust(RustVariable::InstrSet(n)) => Ok(n.clone()),
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    pub fn to_InterruptID(
        &self,
    ) -> Result<common::types::InterruptID, AArch64LifterError> {
        match self {
            lift::types::Variable::Rust(RustVariable::InterruptID(n)) => Ok(n.clone()),
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    pub fn to_LogicalOp(&self) -> Result<common::types::LogicalOp, AArch64LifterError> {
        match self {
            lift::types::Variable::Rust(RustVariable::LogicalOp(n)) => Ok(n.clone()),
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    pub fn to_MBReqDomain(
        &self,
    ) -> Result<common::types::MBReqDomain, AArch64LifterError> {
        match self {
            lift::types::Variable::Rust(RustVariable::MBReqDomain(n)) => Ok(n.clone()),
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    pub fn to_MBReqTypes(
        &self,
    ) -> Result<common::types::MBReqTypes, AArch64LifterError> {
        match self {
            lift::types::Variable::Rust(RustVariable::MBReqTypes(n)) => Ok(n.clone()),
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    pub fn to_MemAtomicOp(
        &self,
    ) -> Result<common::types::MemAtomicOp, AArch64LifterError> {
        match self {
            lift::types::Variable::Rust(RustVariable::MemAtomicOp(n)) => Ok(n.clone()),
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    pub fn to_MemBarrierOp(
        &self,
    ) -> Result<common::types::MemBarrierOp, AArch64LifterError> {
        match self {
            lift::types::Variable::Rust(RustVariable::MemBarrierOp(n)) => Ok(n.clone()),
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    pub fn to_MemOp(&self) -> Result<common::types::MemOp, AArch64LifterError> {
        match self {
            lift::types::Variable::Rust(RustVariable::MemOp(n)) => Ok(n.clone()),
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    pub fn to_MemType(&self) -> Result<common::types::MemType, AArch64LifterError> {
        match self {
            lift::types::Variable::Rust(RustVariable::MemType(n)) => Ok(n.clone()),
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    pub fn to_MoveWideOp(
        &self,
    ) -> Result<common::types::MoveWideOp, AArch64LifterError> {
        match self {
            lift::types::Variable::Rust(RustVariable::MoveWideOp(n)) => Ok(n.clone()),
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    pub fn to_OpType(&self) -> Result<common::types::OpType, AArch64LifterError> {
        match self {
            lift::types::Variable::Rust(RustVariable::OpType(n)) => Ok(n.clone()),
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    pub fn to_PSTATEField(
        &self,
    ) -> Result<common::types::PSTATEField, AArch64LifterError> {
        match self {
            lift::types::Variable::Rust(RustVariable::PSTATEField(n)) => Ok(n.clone()),
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    pub fn to_PrefetchHint(
        &self,
    ) -> Result<common::types::PrefetchHint, AArch64LifterError> {
        match self {
            lift::types::Variable::Rust(RustVariable::PrefetchHint(n)) => Ok(n.clone()),
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    pub fn to_PrivilegeLevel(
        &self,
    ) -> Result<common::types::PrivilegeLevel, AArch64LifterError> {
        match self {
            lift::types::Variable::Rust(RustVariable::PrivilegeLevel(n)) => Ok(n.clone()),
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    pub fn to_ReduceOp(&self) -> Result<common::types::ReduceOp, AArch64LifterError> {
        match self {
            lift::types::Variable::Rust(RustVariable::ReduceOp(n)) => Ok(n.clone()),
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    pub fn to_SRType(&self) -> Result<common::types::SRType, AArch64LifterError> {
        match self {
            lift::types::Variable::Rust(RustVariable::SRType(n)) => Ok(n.clone()),
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    pub fn to_SVECmp(&self) -> Result<common::types::SVECmp, AArch64LifterError> {
        match self {
            lift::types::Variable::Rust(RustVariable::SVECmp(n)) => Ok(n.clone()),
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    pub fn to_ShiftType(&self) -> Result<common::types::ShiftType, AArch64LifterError> {
        match self {
            lift::types::Variable::Rust(RustVariable::ShiftType(n)) => Ok(n.clone()),
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    pub fn to_SysRegAccess(
        &self,
    ) -> Result<common::types::SysRegAccess, AArch64LifterError> {
        match self {
            lift::types::Variable::Rust(RustVariable::SysRegAccess(n)) => Ok(n.clone()),
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    pub fn to_SystemHintOp(
        &self,
    ) -> Result<common::types::SystemHintOp, AArch64LifterError> {
        match self {
            lift::types::Variable::Rust(RustVariable::SystemHintOp(n)) => Ok(n.clone()),
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    pub fn to_SystemOp(&self) -> Result<common::types::SystemOp, AArch64LifterError> {
        match self {
            lift::types::Variable::Rust(RustVariable::SystemOp(n)) => Ok(n.clone()),
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    pub fn to_TimeStamp(&self) -> Result<common::types::TimeStamp, AArch64LifterError> {
        match self {
            lift::types::Variable::Rust(RustVariable::TimeStamp(n)) => Ok(n.clone()),
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    pub fn to_Unpredictable(
        &self,
    ) -> Result<common::types::Unpredictable, AArch64LifterError> {
        match self {
            lift::types::Variable::Rust(RustVariable::Unpredictable(n)) => Ok(n.clone()),
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    pub fn to_VBitOp(&self) -> Result<common::types::VBitOp, AArch64LifterError> {
        match self {
            lift::types::Variable::Rust(RustVariable::VBitOp(n)) => Ok(n.clone()),
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    pub fn to_VBitOps(&self) -> Result<common::types::VBitOps, AArch64LifterError> {
        match self {
            lift::types::Variable::Rust(RustVariable::VBitOps(n)) => Ok(n.clone()),
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    pub fn to_VCGEtype(&self) -> Result<common::types::VCGEtype, AArch64LifterError> {
        match self {
            lift::types::Variable::Rust(RustVariable::VCGEtype(n)) => Ok(n.clone()),
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    pub fn to_VCGTtype(&self) -> Result<common::types::VCGTtype, AArch64LifterError> {
        match self {
            lift::types::Variable::Rust(RustVariable::VCGTtype(n)) => Ok(n.clone()),
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    pub fn to_VFPNegMul(&self) -> Result<common::types::VFPNegMul, AArch64LifterError> {
        match self {
            lift::types::Variable::Rust(RustVariable::VFPNegMul(n)) => Ok(n.clone()),
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    pub fn to___InstrEnc(
        &self,
    ) -> Result<common::types::__InstrEnc, AArch64LifterError> {
        match self {
            lift::types::Variable::Rust(RustVariable::__InstrEnc(n)) => Ok(n.clone()),
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    pub fn to_boolean(&self) -> Result<common::types::boolean, AArch64LifterError> {
        match self {
            lift::types::Variable::Rust(RustVariable::boolean(n)) => Ok(n.clone()),
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
    pub fn to_signal(&self) -> Result<common::types::signal, AArch64LifterError> {
        match self {
            lift::types::Variable::Rust(RustVariable::signal(n)) => Ok(n.clone()),
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        }
    }
}
#[derive(Debug, Clone)]
pub struct AArch32_SErrorSyndrome {
    pub AET: Box<lift::types::Variable>,
    pub ExT: Box<lift::types::Variable>,
}
impl Default for lift::types::AArch32_SErrorSyndrome {
    fn default() -> Self {
        lift::types::AArch32_SErrorSyndrome {
            AET: Box::new(lift::types::Variable::from(common::types::bits::new(0, 2))),
            ExT: Box::new(lift::types::Variable::from(common::types::bits::new(0, 1))),
        }
    }
}
#[derive(Debug, Clone)]
pub struct AccessDescriptor {
    pub acctype: Box<lift::types::Variable>,
    pub mpam: Box<lift::types::MPAMinfo>,
    pub page_table_walk: Box<lift::types::Variable>,
    pub secondstage: Box<lift::types::Variable>,
    pub s2fs1walk: Box<lift::types::Variable>,
    pub level: Box<lift::types::Variable>,
}
impl Default for lift::types::AccessDescriptor {
    fn default() -> Self {
        lift::types::AccessDescriptor {
            acctype: Box::new(
                lift::types::Variable::from(common::types::AccType::default()),
            ),
            mpam: Box::new(lift::types::MPAMinfo::default()),
            page_table_walk: Box::new(
                lift::types::Variable::from(common::types::boolean::default()),
            ),
            secondstage: Box::new(
                lift::types::Variable::from(common::types::boolean::default()),
            ),
            s2fs1walk: Box::new(
                lift::types::Variable::from(common::types::boolean::default()),
            ),
            level: Box::new(
                lift::types::Variable::from(common::types::integer::default()),
            ),
        }
    }
}
#[derive(Debug, Clone)]
pub struct AddressDescriptor {
    pub fault: Box<lift::types::FaultRecord>,
    pub memattrs: Box<lift::types::MemoryAttributes>,
    pub paddress: Box<lift::types::FullAddress>,
    pub vaddress: Box<lift::types::Variable>,
}
impl Default for lift::types::AddressDescriptor {
    fn default() -> Self {
        lift::types::AddressDescriptor {
            fault: Box::new(lift::types::FaultRecord::default()),
            memattrs: Box::new(lift::types::MemoryAttributes::default()),
            paddress: Box::new(lift::types::FullAddress::default()),
            vaddress: Box::new(
                lift::types::Variable::from(common::types::bits::new(0, 64)),
            ),
        }
    }
}
#[derive(Debug, Clone)]
pub struct DescriptorUpdate {
    pub AF: Box<lift::types::Variable>,
    pub AP: Box<lift::types::Variable>,
    pub descaddr: Box<lift::types::AddressDescriptor>,
}
impl Default for lift::types::DescriptorUpdate {
    fn default() -> Self {
        lift::types::DescriptorUpdate {
            AF: Box::new(lift::types::Variable::from(common::types::boolean::default())),
            AP: Box::new(lift::types::Variable::from(common::types::boolean::default())),
            descaddr: Box::new(lift::types::AddressDescriptor::default()),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ExceptionRecord {
    pub exceptype: Box<lift::types::Variable>,
    pub syndrome: Box<lift::types::Variable>,
    pub vaddress: Box<lift::types::Variable>,
    pub ipavalid: Box<lift::types::Variable>,
    pub NS: Box<lift::types::Variable>,
    pub ipaddress: Box<lift::types::Variable>,
}
impl Default for lift::types::ExceptionRecord {
    fn default() -> Self {
        lift::types::ExceptionRecord {
            exceptype: Box::new(
                lift::types::Variable::from(common::types::Exception::default()),
            ),
            syndrome: Box::new(
                lift::types::Variable::from(common::types::bits::new(0, 25)),
            ),
            vaddress: Box::new(
                lift::types::Variable::from(common::types::bits::new(0, 64)),
            ),
            ipavalid: Box::new(
                lift::types::Variable::from(common::types::boolean::default()),
            ),
            NS: Box::new(lift::types::Variable::from(common::types::bits::new(0, 1))),
            ipaddress: Box::new(
                lift::types::Variable::from(common::types::bits::new(0, 52)),
            ),
        }
    }
}
#[derive(Debug, Clone)]
pub struct FaultRecord {
    pub statuscode: Box<lift::types::Variable>,
    pub acctype: Box<lift::types::Variable>,
    pub ipaddress: Box<lift::types::FullAddress>,
    pub s2fs1walk: Box<lift::types::Variable>,
    pub write: Box<lift::types::Variable>,
    pub level: Box<lift::types::Variable>,
    pub extflag: Box<lift::types::Variable>,
    pub secondstage: Box<lift::types::Variable>,
    pub domain: Box<lift::types::Variable>,
    pub errortype: Box<lift::types::Variable>,
    pub debugmoe: Box<lift::types::Variable>,
}
impl Default for lift::types::FaultRecord {
    fn default() -> Self {
        lift::types::FaultRecord {
            statuscode: Box::new(
                lift::types::Variable::from(common::types::Fault::default()),
            ),
            acctype: Box::new(
                lift::types::Variable::from(common::types::AccType::default()),
            ),
            ipaddress: Box::new(lift::types::FullAddress::default()),
            s2fs1walk: Box::new(
                lift::types::Variable::from(common::types::boolean::default()),
            ),
            write: Box::new(
                lift::types::Variable::from(common::types::boolean::default()),
            ),
            level: Box::new(
                lift::types::Variable::from(common::types::integer::default()),
            ),
            extflag: Box::new(
                lift::types::Variable::from(common::types::bits::new(0, 1)),
            ),
            secondstage: Box::new(
                lift::types::Variable::from(common::types::boolean::default()),
            ),
            domain: Box::new(
                lift::types::Variable::from(common::types::bits::new(0, 4)),
            ),
            errortype: Box::new(
                lift::types::Variable::from(common::types::bits::new(0, 2)),
            ),
            debugmoe: Box::new(
                lift::types::Variable::from(common::types::bits::new(0, 4)),
            ),
        }
    }
}
#[derive(Debug, Clone)]
pub struct FullAddress {
    pub address: Box<lift::types::Variable>,
    pub NS: Box<lift::types::Variable>,
}
impl Default for lift::types::FullAddress {
    fn default() -> Self {
        lift::types::FullAddress {
            address: Box::new(
                lift::types::Variable::from(common::types::bits::new(0, 52)),
            ),
            NS: Box::new(lift::types::Variable::from(common::types::bits::new(0, 1))),
        }
    }
}
#[derive(Debug, Clone)]
pub struct MPAMinfo {
    pub mpam_ns: Box<lift::types::Variable>,
    pub partid: Box<lift::types::Variable>,
    pub pmg: Box<lift::types::Variable>,
}
impl Default for lift::types::MPAMinfo {
    fn default() -> Self {
        lift::types::MPAMinfo {
            mpam_ns: Box::new(
                lift::types::Variable::from(common::types::bits::new(0, 1)),
            ),
            partid: Box::new(
                lift::types::Variable::from(common::types::bits::new(0, 16)),
            ),
            pmg: Box::new(lift::types::Variable::from(common::types::bits::new(0, 8))),
        }
    }
}
#[derive(Debug, Clone)]
pub struct MemAttrHints {
    pub attrs: Box<lift::types::Variable>,
    pub hints: Box<lift::types::Variable>,
    pub transient: Box<lift::types::Variable>,
}
impl Default for lift::types::MemAttrHints {
    fn default() -> Self {
        lift::types::MemAttrHints {
            attrs: Box::new(lift::types::Variable::from(common::types::bits::new(0, 2))),
            hints: Box::new(lift::types::Variable::from(common::types::bits::new(0, 2))),
            transient: Box::new(
                lift::types::Variable::from(common::types::boolean::default()),
            ),
        }
    }
}
#[derive(Debug, Clone)]
pub struct MemoryAttributes {
    pub memtype: Box<lift::types::Variable>,
    pub device: Box<lift::types::Variable>,
    pub inner: Box<lift::types::MemAttrHints>,
    pub outer: Box<lift::types::MemAttrHints>,
    pub tagged: Box<lift::types::Variable>,
    pub shareable: Box<lift::types::Variable>,
    pub outershareable: Box<lift::types::Variable>,
}
impl Default for lift::types::MemoryAttributes {
    fn default() -> Self {
        lift::types::MemoryAttributes {
            memtype: Box::new(
                lift::types::Variable::from(common::types::MemType::default()),
            ),
            device: Box::new(
                lift::types::Variable::from(common::types::DeviceType::default()),
            ),
            inner: Box::new(lift::types::MemAttrHints::default()),
            outer: Box::new(lift::types::MemAttrHints::default()),
            tagged: Box::new(
                lift::types::Variable::from(common::types::boolean::default()),
            ),
            shareable: Box::new(
                lift::types::Variable::from(common::types::boolean::default()),
            ),
            outershareable: Box::new(
                lift::types::Variable::from(common::types::boolean::default()),
            ),
        }
    }
}
#[derive(Debug, Clone)]
pub struct PCSample {
    pub valid: Box<lift::types::Variable>,
    pub pc: Box<lift::types::Variable>,
    pub el: Box<lift::types::Variable>,
    pub rw: Box<lift::types::Variable>,
    pub ns: Box<lift::types::Variable>,
    pub has_el2: Box<lift::types::Variable>,
    pub contextidr: Box<lift::types::Variable>,
    pub contextidr_el2: Box<lift::types::Variable>,
    pub el0h: Box<lift::types::Variable>,
    pub vmid: Box<lift::types::Variable>,
}
impl Default for lift::types::PCSample {
    fn default() -> Self {
        lift::types::PCSample {
            valid: Box::new(
                lift::types::Variable::from(common::types::boolean::default()),
            ),
            pc: Box::new(lift::types::Variable::from(common::types::bits::new(0, 64))),
            el: Box::new(lift::types::Variable::from(common::types::bits::new(0, 2))),
            rw: Box::new(lift::types::Variable::from(common::types::bits::new(0, 1))),
            ns: Box::new(lift::types::Variable::from(common::types::bits::new(0, 1))),
            has_el2: Box::new(
                lift::types::Variable::from(common::types::boolean::default()),
            ),
            contextidr: Box::new(
                lift::types::Variable::from(common::types::bits::new(0, 32)),
            ),
            contextidr_el2: Box::new(
                lift::types::Variable::from(common::types::bits::new(0, 32)),
            ),
            el0h: Box::new(
                lift::types::Variable::from(common::types::boolean::default()),
            ),
            vmid: Box::new(lift::types::Variable::from(common::types::bits::new(0, 16))),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Permissions {
    pub ap: Box<lift::types::Variable>,
    pub xn: Box<lift::types::Variable>,
    pub xxn: Box<lift::types::Variable>,
    pub pxn: Box<lift::types::Variable>,
}
impl Default for lift::types::Permissions {
    fn default() -> Self {
        lift::types::Permissions {
            ap: Box::new(lift::types::Variable::from(common::types::bits::new(0, 3))),
            xn: Box::new(lift::types::Variable::from(common::types::bits::new(0, 1))),
            xxn: Box::new(lift::types::Variable::from(common::types::bits::new(0, 1))),
            pxn: Box::new(lift::types::Variable::from(common::types::bits::new(0, 1))),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ProcState {
    pub N: Box<lift::types::Variable>,
    pub Z: Box<lift::types::Variable>,
    pub C: Box<lift::types::Variable>,
    pub V: Box<lift::types::Variable>,
    pub D: Box<lift::types::Variable>,
    pub A: Box<lift::types::Variable>,
    pub I: Box<lift::types::Variable>,
    pub F: Box<lift::types::Variable>,
    pub PAN: Box<lift::types::Variable>,
    pub UAO: Box<lift::types::Variable>,
    pub DIT: Box<lift::types::Variable>,
    pub TCO: Box<lift::types::Variable>,
    pub BTYPE: Box<lift::types::Variable>,
    pub SS: Box<lift::types::Variable>,
    pub IL: Box<lift::types::Variable>,
    pub EL: Box<lift::types::Variable>,
    pub nRW: Box<lift::types::Variable>,
    pub SP: Box<lift::types::Variable>,
    pub Q: Box<lift::types::Variable>,
    pub GE: Box<lift::types::Variable>,
    pub SSBS: Box<lift::types::Variable>,
    pub IT: Box<lift::types::Variable>,
    pub J: Box<lift::types::Variable>,
    pub T: Box<lift::types::Variable>,
    pub E: Box<lift::types::Variable>,
    pub M: Box<lift::types::Variable>,
}
impl Default for lift::types::ProcState {
    fn default() -> Self {
        lift::types::ProcState {
            N: Box::new(lift::types::Variable::from(common::types::bits::new(0, 1))),
            Z: Box::new(lift::types::Variable::from(common::types::bits::new(0, 1))),
            C: Box::new(lift::types::Variable::from(common::types::bits::new(0, 1))),
            V: Box::new(lift::types::Variable::from(common::types::bits::new(0, 1))),
            D: Box::new(lift::types::Variable::from(common::types::bits::new(0, 1))),
            A: Box::new(lift::types::Variable::from(common::types::bits::new(0, 1))),
            I: Box::new(lift::types::Variable::from(common::types::bits::new(0, 1))),
            F: Box::new(lift::types::Variable::from(common::types::bits::new(0, 1))),
            PAN: Box::new(lift::types::Variable::from(common::types::bits::new(0, 1))),
            UAO: Box::new(lift::types::Variable::from(common::types::bits::new(0, 1))),
            DIT: Box::new(lift::types::Variable::from(common::types::bits::new(0, 1))),
            TCO: Box::new(lift::types::Variable::from(common::types::bits::new(0, 1))),
            BTYPE: Box::new(lift::types::Variable::from(common::types::bits::new(0, 2))),
            SS: Box::new(lift::types::Variable::from(common::types::bits::new(0, 1))),
            IL: Box::new(lift::types::Variable::from(common::types::bits::new(0, 1))),
            EL: Box::new(lift::types::Variable::from(common::types::bits::new(0, 2))),
            nRW: Box::new(lift::types::Variable::from(common::types::bits::new(0, 1))),
            SP: Box::new(lift::types::Variable::from(common::types::bits::new(0, 1))),
            Q: Box::new(lift::types::Variable::from(common::types::bits::new(0, 1))),
            GE: Box::new(lift::types::Variable::from(common::types::bits::new(0, 4))),
            SSBS: Box::new(lift::types::Variable::from(common::types::bits::new(0, 1))),
            IT: Box::new(lift::types::Variable::from(common::types::bits::new(0, 8))),
            J: Box::new(lift::types::Variable::from(common::types::bits::new(0, 1))),
            T: Box::new(lift::types::Variable::from(common::types::bits::new(0, 1))),
            E: Box::new(lift::types::Variable::from(common::types::bits::new(0, 1))),
            M: Box::new(lift::types::Variable::from(common::types::bits::new(0, 5))),
        }
    }
}
#[derive(Debug, Clone)]
pub struct TLBRecord {
    pub perms: Box<lift::types::Permissions>,
    pub nG: Box<lift::types::Variable>,
    pub domain: Box<lift::types::Variable>,
    pub GP: Box<lift::types::Variable>,
    pub contiguous: Box<lift::types::Variable>,
    pub level: Box<lift::types::Variable>,
    pub blocksize: Box<lift::types::Variable>,
    pub descupdate: Box<lift::types::DescriptorUpdate>,
    pub CnP: Box<lift::types::Variable>,
    pub addrdesc: Box<lift::types::AddressDescriptor>,
}
impl Default for lift::types::TLBRecord {
    fn default() -> Self {
        lift::types::TLBRecord {
            perms: Box::new(lift::types::Permissions::default()),
            nG: Box::new(lift::types::Variable::from(common::types::bits::new(0, 1))),
            domain: Box::new(
                lift::types::Variable::from(common::types::bits::new(0, 4)),
            ),
            GP: Box::new(lift::types::Variable::from(common::types::bits::new(0, 1))),
            contiguous: Box::new(
                lift::types::Variable::from(common::types::boolean::default()),
            ),
            level: Box::new(
                lift::types::Variable::from(common::types::integer::default()),
            ),
            blocksize: Box::new(
                lift::types::Variable::from(common::types::integer::default()),
            ),
            descupdate: Box::new(lift::types::DescriptorUpdate::default()),
            CnP: Box::new(lift::types::Variable::from(common::types::bits::new(0, 1))),
            addrdesc: Box::new(lift::types::AddressDescriptor::default()),
        }
    }
}
#[derive(Debug, Clone)]
pub enum AirPackable {
    Variable(Box<lift::types::Variable>),
    AArch32_SErrorSyndrome(Box<lift::types::AArch32_SErrorSyndrome>),
    AccessDescriptor(Box<lift::types::AccessDescriptor>),
    AddressDescriptor(Box<lift::types::AddressDescriptor>),
    DescriptorUpdate(Box<lift::types::DescriptorUpdate>),
    ExceptionRecord(Box<lift::types::ExceptionRecord>),
    FaultRecord(Box<lift::types::FaultRecord>),
    FullAddress(Box<lift::types::FullAddress>),
    MPAMinfo(Box<lift::types::MPAMinfo>),
    MemAttrHints(Box<lift::types::MemAttrHints>),
    MemoryAttributes(Box<lift::types::MemoryAttributes>),
    PCSample(Box<lift::types::PCSample>),
    Permissions(Box<lift::types::Permissions>),
    ProcState(Box<lift::types::ProcState>),
    TLBRecord(Box<lift::types::TLBRecord>),
}
impl AirPackable {
    pub fn unpack_to_air_values_and_types(
        &self,
        builder: &mut InstructionBuilder,
    ) -> Result<(Vec<Value>, Vec<Type>), AArch64LifterError> {
        match self {
            AirPackable::Variable(Variable) => {
                let air_var = Variable.promote_to_air(builder)?.to_air()?;
                Ok((vec![air_var.val], vec![air_var.ty]))
            }
            AirPackable::AArch32_SErrorSyndrome(AArch32_SErrorSyndrome) => {
                let mut unpacked_values = Vec::new();
                let mut unpacked_types = Vec::new();
                let packable: AirPackable = (*AArch32_SErrorSyndrome.AET).clone().into();
                let (unpacked_values_AET, unpacked_types_AET) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_AET);
                unpacked_types.extend(unpacked_types_AET);
                let packable: AirPackable = (*AArch32_SErrorSyndrome.ExT).clone().into();
                let (unpacked_values_ExT, unpacked_types_ExT) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_ExT);
                unpacked_types.extend(unpacked_types_ExT);
                Ok((unpacked_values, unpacked_types))
            }
            AirPackable::AccessDescriptor(AccessDescriptor) => {
                let mut unpacked_values = Vec::new();
                let mut unpacked_types = Vec::new();
                let packable: AirPackable = (*AccessDescriptor.acctype).clone().into();
                let (unpacked_values_acctype, unpacked_types_acctype) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_acctype);
                unpacked_types.extend(unpacked_types_acctype);
                let packable: AirPackable = (*AccessDescriptor.mpam).clone().into();
                let (unpacked_values_mpam, unpacked_types_mpam) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_mpam);
                unpacked_types.extend(unpacked_types_mpam);
                let packable: AirPackable = (*AccessDescriptor.page_table_walk)
                    .clone()
                    .into();
                let (unpacked_values_page_table_walk, unpacked_types_page_table_walk) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_page_table_walk);
                unpacked_types.extend(unpacked_types_page_table_walk);
                let packable: AirPackable = (*AccessDescriptor.secondstage)
                    .clone()
                    .into();
                let (unpacked_values_secondstage, unpacked_types_secondstage) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_secondstage);
                unpacked_types.extend(unpacked_types_secondstage);
                let packable: AirPackable = (*AccessDescriptor.s2fs1walk).clone().into();
                let (unpacked_values_s2fs1walk, unpacked_types_s2fs1walk) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_s2fs1walk);
                unpacked_types.extend(unpacked_types_s2fs1walk);
                let packable: AirPackable = (*AccessDescriptor.level).clone().into();
                let (unpacked_values_level, unpacked_types_level) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_level);
                unpacked_types.extend(unpacked_types_level);
                Ok((unpacked_values, unpacked_types))
            }
            AirPackable::AddressDescriptor(AddressDescriptor) => {
                let mut unpacked_values = Vec::new();
                let mut unpacked_types = Vec::new();
                let packable: AirPackable = (*AddressDescriptor.fault).clone().into();
                let (unpacked_values_fault, unpacked_types_fault) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_fault);
                unpacked_types.extend(unpacked_types_fault);
                let packable: AirPackable = (*AddressDescriptor.memattrs).clone().into();
                let (unpacked_values_memattrs, unpacked_types_memattrs) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_memattrs);
                unpacked_types.extend(unpacked_types_memattrs);
                let packable: AirPackable = (*AddressDescriptor.paddress).clone().into();
                let (unpacked_values_paddress, unpacked_types_paddress) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_paddress);
                unpacked_types.extend(unpacked_types_paddress);
                let packable: AirPackable = (*AddressDescriptor.vaddress).clone().into();
                let (unpacked_values_vaddress, unpacked_types_vaddress) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_vaddress);
                unpacked_types.extend(unpacked_types_vaddress);
                Ok((unpacked_values, unpacked_types))
            }
            AirPackable::DescriptorUpdate(DescriptorUpdate) => {
                let mut unpacked_values = Vec::new();
                let mut unpacked_types = Vec::new();
                let packable: AirPackable = (*DescriptorUpdate.AF).clone().into();
                let (unpacked_values_AF, unpacked_types_AF) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_AF);
                unpacked_types.extend(unpacked_types_AF);
                let packable: AirPackable = (*DescriptorUpdate.AP).clone().into();
                let (unpacked_values_AP, unpacked_types_AP) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_AP);
                unpacked_types.extend(unpacked_types_AP);
                let packable: AirPackable = (*DescriptorUpdate.descaddr).clone().into();
                let (unpacked_values_descaddr, unpacked_types_descaddr) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_descaddr);
                unpacked_types.extend(unpacked_types_descaddr);
                Ok((unpacked_values, unpacked_types))
            }
            AirPackable::ExceptionRecord(ExceptionRecord) => {
                let mut unpacked_values = Vec::new();
                let mut unpacked_types = Vec::new();
                let packable: AirPackable = (*ExceptionRecord.exceptype).clone().into();
                let (unpacked_values_exceptype, unpacked_types_exceptype) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_exceptype);
                unpacked_types.extend(unpacked_types_exceptype);
                let packable: AirPackable = (*ExceptionRecord.syndrome).clone().into();
                let (unpacked_values_syndrome, unpacked_types_syndrome) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_syndrome);
                unpacked_types.extend(unpacked_types_syndrome);
                let packable: AirPackable = (*ExceptionRecord.vaddress).clone().into();
                let (unpacked_values_vaddress, unpacked_types_vaddress) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_vaddress);
                unpacked_types.extend(unpacked_types_vaddress);
                let packable: AirPackable = (*ExceptionRecord.ipavalid).clone().into();
                let (unpacked_values_ipavalid, unpacked_types_ipavalid) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_ipavalid);
                unpacked_types.extend(unpacked_types_ipavalid);
                let packable: AirPackable = (*ExceptionRecord.NS).clone().into();
                let (unpacked_values_NS, unpacked_types_NS) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_NS);
                unpacked_types.extend(unpacked_types_NS);
                let packable: AirPackable = (*ExceptionRecord.ipaddress).clone().into();
                let (unpacked_values_ipaddress, unpacked_types_ipaddress) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_ipaddress);
                unpacked_types.extend(unpacked_types_ipaddress);
                Ok((unpacked_values, unpacked_types))
            }
            AirPackable::FaultRecord(FaultRecord) => {
                let mut unpacked_values = Vec::new();
                let mut unpacked_types = Vec::new();
                let packable: AirPackable = (*FaultRecord.statuscode).clone().into();
                let (unpacked_values_statuscode, unpacked_types_statuscode) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_statuscode);
                unpacked_types.extend(unpacked_types_statuscode);
                let packable: AirPackable = (*FaultRecord.acctype).clone().into();
                let (unpacked_values_acctype, unpacked_types_acctype) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_acctype);
                unpacked_types.extend(unpacked_types_acctype);
                let packable: AirPackable = (*FaultRecord.ipaddress).clone().into();
                let (unpacked_values_ipaddress, unpacked_types_ipaddress) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_ipaddress);
                unpacked_types.extend(unpacked_types_ipaddress);
                let packable: AirPackable = (*FaultRecord.s2fs1walk).clone().into();
                let (unpacked_values_s2fs1walk, unpacked_types_s2fs1walk) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_s2fs1walk);
                unpacked_types.extend(unpacked_types_s2fs1walk);
                let packable: AirPackable = (*FaultRecord.write).clone().into();
                let (unpacked_values_write, unpacked_types_write) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_write);
                unpacked_types.extend(unpacked_types_write);
                let packable: AirPackable = (*FaultRecord.level).clone().into();
                let (unpacked_values_level, unpacked_types_level) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_level);
                unpacked_types.extend(unpacked_types_level);
                let packable: AirPackable = (*FaultRecord.extflag).clone().into();
                let (unpacked_values_extflag, unpacked_types_extflag) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_extflag);
                unpacked_types.extend(unpacked_types_extflag);
                let packable: AirPackable = (*FaultRecord.secondstage).clone().into();
                let (unpacked_values_secondstage, unpacked_types_secondstage) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_secondstage);
                unpacked_types.extend(unpacked_types_secondstage);
                let packable: AirPackable = (*FaultRecord.domain).clone().into();
                let (unpacked_values_domain, unpacked_types_domain) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_domain);
                unpacked_types.extend(unpacked_types_domain);
                let packable: AirPackable = (*FaultRecord.errortype).clone().into();
                let (unpacked_values_errortype, unpacked_types_errortype) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_errortype);
                unpacked_types.extend(unpacked_types_errortype);
                let packable: AirPackable = (*FaultRecord.debugmoe).clone().into();
                let (unpacked_values_debugmoe, unpacked_types_debugmoe) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_debugmoe);
                unpacked_types.extend(unpacked_types_debugmoe);
                Ok((unpacked_values, unpacked_types))
            }
            AirPackable::FullAddress(FullAddress) => {
                let mut unpacked_values = Vec::new();
                let mut unpacked_types = Vec::new();
                let packable: AirPackable = (*FullAddress.address).clone().into();
                let (unpacked_values_address, unpacked_types_address) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_address);
                unpacked_types.extend(unpacked_types_address);
                let packable: AirPackable = (*FullAddress.NS).clone().into();
                let (unpacked_values_NS, unpacked_types_NS) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_NS);
                unpacked_types.extend(unpacked_types_NS);
                Ok((unpacked_values, unpacked_types))
            }
            AirPackable::MPAMinfo(MPAMinfo) => {
                let mut unpacked_values = Vec::new();
                let mut unpacked_types = Vec::new();
                let packable: AirPackable = (*MPAMinfo.mpam_ns).clone().into();
                let (unpacked_values_mpam_ns, unpacked_types_mpam_ns) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_mpam_ns);
                unpacked_types.extend(unpacked_types_mpam_ns);
                let packable: AirPackable = (*MPAMinfo.partid).clone().into();
                let (unpacked_values_partid, unpacked_types_partid) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_partid);
                unpacked_types.extend(unpacked_types_partid);
                let packable: AirPackable = (*MPAMinfo.pmg).clone().into();
                let (unpacked_values_pmg, unpacked_types_pmg) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_pmg);
                unpacked_types.extend(unpacked_types_pmg);
                Ok((unpacked_values, unpacked_types))
            }
            AirPackable::MemAttrHints(MemAttrHints) => {
                let mut unpacked_values = Vec::new();
                let mut unpacked_types = Vec::new();
                let packable: AirPackable = (*MemAttrHints.attrs).clone().into();
                let (unpacked_values_attrs, unpacked_types_attrs) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_attrs);
                unpacked_types.extend(unpacked_types_attrs);
                let packable: AirPackable = (*MemAttrHints.hints).clone().into();
                let (unpacked_values_hints, unpacked_types_hints) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_hints);
                unpacked_types.extend(unpacked_types_hints);
                let packable: AirPackable = (*MemAttrHints.transient).clone().into();
                let (unpacked_values_transient, unpacked_types_transient) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_transient);
                unpacked_types.extend(unpacked_types_transient);
                Ok((unpacked_values, unpacked_types))
            }
            AirPackable::MemoryAttributes(MemoryAttributes) => {
                let mut unpacked_values = Vec::new();
                let mut unpacked_types = Vec::new();
                let packable: AirPackable = (*MemoryAttributes.memtype).clone().into();
                let (unpacked_values_memtype, unpacked_types_memtype) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_memtype);
                unpacked_types.extend(unpacked_types_memtype);
                let packable: AirPackable = (*MemoryAttributes.device).clone().into();
                let (unpacked_values_device, unpacked_types_device) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_device);
                unpacked_types.extend(unpacked_types_device);
                let packable: AirPackable = (*MemoryAttributes.inner).clone().into();
                let (unpacked_values_inner, unpacked_types_inner) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_inner);
                unpacked_types.extend(unpacked_types_inner);
                let packable: AirPackable = (*MemoryAttributes.outer).clone().into();
                let (unpacked_values_outer, unpacked_types_outer) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_outer);
                unpacked_types.extend(unpacked_types_outer);
                let packable: AirPackable = (*MemoryAttributes.tagged).clone().into();
                let (unpacked_values_tagged, unpacked_types_tagged) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_tagged);
                unpacked_types.extend(unpacked_types_tagged);
                let packable: AirPackable = (*MemoryAttributes.shareable).clone().into();
                let (unpacked_values_shareable, unpacked_types_shareable) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_shareable);
                unpacked_types.extend(unpacked_types_shareable);
                let packable: AirPackable = (*MemoryAttributes.outershareable)
                    .clone()
                    .into();
                let (unpacked_values_outershareable, unpacked_types_outershareable) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_outershareable);
                unpacked_types.extend(unpacked_types_outershareable);
                Ok((unpacked_values, unpacked_types))
            }
            AirPackable::PCSample(PCSample) => {
                let mut unpacked_values = Vec::new();
                let mut unpacked_types = Vec::new();
                let packable: AirPackable = (*PCSample.valid).clone().into();
                let (unpacked_values_valid, unpacked_types_valid) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_valid);
                unpacked_types.extend(unpacked_types_valid);
                let packable: AirPackable = (*PCSample.pc).clone().into();
                let (unpacked_values_pc, unpacked_types_pc) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_pc);
                unpacked_types.extend(unpacked_types_pc);
                let packable: AirPackable = (*PCSample.el).clone().into();
                let (unpacked_values_el, unpacked_types_el) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_el);
                unpacked_types.extend(unpacked_types_el);
                let packable: AirPackable = (*PCSample.rw).clone().into();
                let (unpacked_values_rw, unpacked_types_rw) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_rw);
                unpacked_types.extend(unpacked_types_rw);
                let packable: AirPackable = (*PCSample.ns).clone().into();
                let (unpacked_values_ns, unpacked_types_ns) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_ns);
                unpacked_types.extend(unpacked_types_ns);
                let packable: AirPackable = (*PCSample.has_el2).clone().into();
                let (unpacked_values_has_el2, unpacked_types_has_el2) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_has_el2);
                unpacked_types.extend(unpacked_types_has_el2);
                let packable: AirPackable = (*PCSample.contextidr).clone().into();
                let (unpacked_values_contextidr, unpacked_types_contextidr) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_contextidr);
                unpacked_types.extend(unpacked_types_contextidr);
                let packable: AirPackable = (*PCSample.contextidr_el2).clone().into();
                let (unpacked_values_contextidr_el2, unpacked_types_contextidr_el2) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_contextidr_el2);
                unpacked_types.extend(unpacked_types_contextidr_el2);
                let packable: AirPackable = (*PCSample.el0h).clone().into();
                let (unpacked_values_el0h, unpacked_types_el0h) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_el0h);
                unpacked_types.extend(unpacked_types_el0h);
                let packable: AirPackable = (*PCSample.vmid).clone().into();
                let (unpacked_values_vmid, unpacked_types_vmid) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_vmid);
                unpacked_types.extend(unpacked_types_vmid);
                Ok((unpacked_values, unpacked_types))
            }
            AirPackable::Permissions(Permissions) => {
                let mut unpacked_values = Vec::new();
                let mut unpacked_types = Vec::new();
                let packable: AirPackable = (*Permissions.ap).clone().into();
                let (unpacked_values_ap, unpacked_types_ap) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_ap);
                unpacked_types.extend(unpacked_types_ap);
                let packable: AirPackable = (*Permissions.xn).clone().into();
                let (unpacked_values_xn, unpacked_types_xn) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_xn);
                unpacked_types.extend(unpacked_types_xn);
                let packable: AirPackable = (*Permissions.xxn).clone().into();
                let (unpacked_values_xxn, unpacked_types_xxn) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_xxn);
                unpacked_types.extend(unpacked_types_xxn);
                let packable: AirPackable = (*Permissions.pxn).clone().into();
                let (unpacked_values_pxn, unpacked_types_pxn) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_pxn);
                unpacked_types.extend(unpacked_types_pxn);
                Ok((unpacked_values, unpacked_types))
            }
            AirPackable::ProcState(ProcState) => {
                let mut unpacked_values = Vec::new();
                let mut unpacked_types = Vec::new();
                let packable: AirPackable = (*ProcState.N).clone().into();
                let (unpacked_values_N, unpacked_types_N) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_N);
                unpacked_types.extend(unpacked_types_N);
                let packable: AirPackable = (*ProcState.Z).clone().into();
                let (unpacked_values_Z, unpacked_types_Z) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_Z);
                unpacked_types.extend(unpacked_types_Z);
                let packable: AirPackable = (*ProcState.C).clone().into();
                let (unpacked_values_C, unpacked_types_C) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_C);
                unpacked_types.extend(unpacked_types_C);
                let packable: AirPackable = (*ProcState.V).clone().into();
                let (unpacked_values_V, unpacked_types_V) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_V);
                unpacked_types.extend(unpacked_types_V);
                let packable: AirPackable = (*ProcState.D).clone().into();
                let (unpacked_values_D, unpacked_types_D) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_D);
                unpacked_types.extend(unpacked_types_D);
                let packable: AirPackable = (*ProcState.A).clone().into();
                let (unpacked_values_A, unpacked_types_A) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_A);
                unpacked_types.extend(unpacked_types_A);
                let packable: AirPackable = (*ProcState.I).clone().into();
                let (unpacked_values_I, unpacked_types_I) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_I);
                unpacked_types.extend(unpacked_types_I);
                let packable: AirPackable = (*ProcState.F).clone().into();
                let (unpacked_values_F, unpacked_types_F) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_F);
                unpacked_types.extend(unpacked_types_F);
                let packable: AirPackable = (*ProcState.PAN).clone().into();
                let (unpacked_values_PAN, unpacked_types_PAN) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_PAN);
                unpacked_types.extend(unpacked_types_PAN);
                let packable: AirPackable = (*ProcState.UAO).clone().into();
                let (unpacked_values_UAO, unpacked_types_UAO) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_UAO);
                unpacked_types.extend(unpacked_types_UAO);
                let packable: AirPackable = (*ProcState.DIT).clone().into();
                let (unpacked_values_DIT, unpacked_types_DIT) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_DIT);
                unpacked_types.extend(unpacked_types_DIT);
                let packable: AirPackable = (*ProcState.TCO).clone().into();
                let (unpacked_values_TCO, unpacked_types_TCO) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_TCO);
                unpacked_types.extend(unpacked_types_TCO);
                let packable: AirPackable = (*ProcState.BTYPE).clone().into();
                let (unpacked_values_BTYPE, unpacked_types_BTYPE) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_BTYPE);
                unpacked_types.extend(unpacked_types_BTYPE);
                let packable: AirPackable = (*ProcState.SS).clone().into();
                let (unpacked_values_SS, unpacked_types_SS) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_SS);
                unpacked_types.extend(unpacked_types_SS);
                let packable: AirPackable = (*ProcState.IL).clone().into();
                let (unpacked_values_IL, unpacked_types_IL) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_IL);
                unpacked_types.extend(unpacked_types_IL);
                let packable: AirPackable = (*ProcState.EL).clone().into();
                let (unpacked_values_EL, unpacked_types_EL) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_EL);
                unpacked_types.extend(unpacked_types_EL);
                let packable: AirPackable = (*ProcState.nRW).clone().into();
                let (unpacked_values_nRW, unpacked_types_nRW) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_nRW);
                unpacked_types.extend(unpacked_types_nRW);
                let packable: AirPackable = (*ProcState.SP).clone().into();
                let (unpacked_values_SP, unpacked_types_SP) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_SP);
                unpacked_types.extend(unpacked_types_SP);
                let packable: AirPackable = (*ProcState.Q).clone().into();
                let (unpacked_values_Q, unpacked_types_Q) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_Q);
                unpacked_types.extend(unpacked_types_Q);
                let packable: AirPackable = (*ProcState.GE).clone().into();
                let (unpacked_values_GE, unpacked_types_GE) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_GE);
                unpacked_types.extend(unpacked_types_GE);
                let packable: AirPackable = (*ProcState.SSBS).clone().into();
                let (unpacked_values_SSBS, unpacked_types_SSBS) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_SSBS);
                unpacked_types.extend(unpacked_types_SSBS);
                let packable: AirPackable = (*ProcState.IT).clone().into();
                let (unpacked_values_IT, unpacked_types_IT) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_IT);
                unpacked_types.extend(unpacked_types_IT);
                let packable: AirPackable = (*ProcState.J).clone().into();
                let (unpacked_values_J, unpacked_types_J) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_J);
                unpacked_types.extend(unpacked_types_J);
                let packable: AirPackable = (*ProcState.T).clone().into();
                let (unpacked_values_T, unpacked_types_T) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_T);
                unpacked_types.extend(unpacked_types_T);
                let packable: AirPackable = (*ProcState.E).clone().into();
                let (unpacked_values_E, unpacked_types_E) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_E);
                unpacked_types.extend(unpacked_types_E);
                let packable: AirPackable = (*ProcState.M).clone().into();
                let (unpacked_values_M, unpacked_types_M) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_M);
                unpacked_types.extend(unpacked_types_M);
                Ok((unpacked_values, unpacked_types))
            }
            AirPackable::TLBRecord(TLBRecord) => {
                let mut unpacked_values = Vec::new();
                let mut unpacked_types = Vec::new();
                let packable: AirPackable = (*TLBRecord.perms).clone().into();
                let (unpacked_values_perms, unpacked_types_perms) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_perms);
                unpacked_types.extend(unpacked_types_perms);
                let packable: AirPackable = (*TLBRecord.nG).clone().into();
                let (unpacked_values_nG, unpacked_types_nG) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_nG);
                unpacked_types.extend(unpacked_types_nG);
                let packable: AirPackable = (*TLBRecord.domain).clone().into();
                let (unpacked_values_domain, unpacked_types_domain) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_domain);
                unpacked_types.extend(unpacked_types_domain);
                let packable: AirPackable = (*TLBRecord.GP).clone().into();
                let (unpacked_values_GP, unpacked_types_GP) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_GP);
                unpacked_types.extend(unpacked_types_GP);
                let packable: AirPackable = (*TLBRecord.contiguous).clone().into();
                let (unpacked_values_contiguous, unpacked_types_contiguous) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_contiguous);
                unpacked_types.extend(unpacked_types_contiguous);
                let packable: AirPackable = (*TLBRecord.level).clone().into();
                let (unpacked_values_level, unpacked_types_level) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_level);
                unpacked_types.extend(unpacked_types_level);
                let packable: AirPackable = (*TLBRecord.blocksize).clone().into();
                let (unpacked_values_blocksize, unpacked_types_blocksize) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_blocksize);
                unpacked_types.extend(unpacked_types_blocksize);
                let packable: AirPackable = (*TLBRecord.descupdate).clone().into();
                let (unpacked_values_descupdate, unpacked_types_descupdate) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_descupdate);
                unpacked_types.extend(unpacked_types_descupdate);
                let packable: AirPackable = (*TLBRecord.CnP).clone().into();
                let (unpacked_values_CnP, unpacked_types_CnP) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_CnP);
                unpacked_types.extend(unpacked_types_CnP);
                let packable: AirPackable = (*TLBRecord.addrdesc).clone().into();
                let (unpacked_values_addrdesc, unpacked_types_addrdesc) = packable
                    .unpack_to_air_values_and_types(builder)?;
                unpacked_values.extend(unpacked_values_addrdesc);
                unpacked_types.extend(unpacked_types_addrdesc);
                Ok((unpacked_values, unpacked_types))
            }
        }
    }
    pub fn pack_from_air_values_and_types(
        &self,
        values: &[Value],
        types: &[Type],
    ) -> Result<(Self, usize), AArch64LifterError> {
        match self {
            AirPackable::Variable(Variable) => {
                Ok((lift::types::Variable::new_air(values[0], types[0]).into(), 1))
            }
            AirPackable::AArch32_SErrorSyndrome(AArch32_SErrorSyndrome) => {
                let AArch32_SErrorSyndrome = &*AArch32_SErrorSyndrome;
                let mut consumed = 0;
                let packable: AirPackable = (*AArch32_SErrorSyndrome.AET).clone().into();
                let (packed_AET, consumed_AET) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_AET;
                let packable: AirPackable = (*AArch32_SErrorSyndrome.ExT).clone().into();
                let (packed_ExT, consumed_ExT) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_ExT;
                Ok((
                    AArch32_SErrorSyndrome {
                        AET: Box::new(packed_AET.try_into()?),
                        ExT: Box::new(packed_ExT.try_into()?),
                    }
                        .into(),
                    consumed,
                ))
            }
            AirPackable::AccessDescriptor(AccessDescriptor) => {
                let AccessDescriptor = &*AccessDescriptor;
                let mut consumed = 0;
                let packable: AirPackable = (*AccessDescriptor.acctype).clone().into();
                let (packed_acctype, consumed_acctype) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_acctype;
                let packable: AirPackable = (*AccessDescriptor.mpam).clone().into();
                let (packed_mpam, consumed_mpam) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_mpam;
                let packable: AirPackable = (*AccessDescriptor.page_table_walk)
                    .clone()
                    .into();
                let (packed_page_table_walk, consumed_page_table_walk) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_page_table_walk;
                let packable: AirPackable = (*AccessDescriptor.secondstage)
                    .clone()
                    .into();
                let (packed_secondstage, consumed_secondstage) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_secondstage;
                let packable: AirPackable = (*AccessDescriptor.s2fs1walk).clone().into();
                let (packed_s2fs1walk, consumed_s2fs1walk) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_s2fs1walk;
                let packable: AirPackable = (*AccessDescriptor.level).clone().into();
                let (packed_level, consumed_level) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_level;
                Ok((
                    AccessDescriptor {
                        acctype: Box::new(packed_acctype.try_into()?),
                        mpam: Box::new(packed_mpam.try_into()?),
                        page_table_walk: Box::new(packed_page_table_walk.try_into()?),
                        secondstage: Box::new(packed_secondstage.try_into()?),
                        s2fs1walk: Box::new(packed_s2fs1walk.try_into()?),
                        level: Box::new(packed_level.try_into()?),
                    }
                        .into(),
                    consumed,
                ))
            }
            AirPackable::AddressDescriptor(AddressDescriptor) => {
                let AddressDescriptor = &*AddressDescriptor;
                let mut consumed = 0;
                let packable: AirPackable = (*AddressDescriptor.fault).clone().into();
                let (packed_fault, consumed_fault) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_fault;
                let packable: AirPackable = (*AddressDescriptor.memattrs).clone().into();
                let (packed_memattrs, consumed_memattrs) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_memattrs;
                let packable: AirPackable = (*AddressDescriptor.paddress).clone().into();
                let (packed_paddress, consumed_paddress) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_paddress;
                let packable: AirPackable = (*AddressDescriptor.vaddress).clone().into();
                let (packed_vaddress, consumed_vaddress) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_vaddress;
                Ok((
                    AddressDescriptor {
                        fault: Box::new(packed_fault.try_into()?),
                        memattrs: Box::new(packed_memattrs.try_into()?),
                        paddress: Box::new(packed_paddress.try_into()?),
                        vaddress: Box::new(packed_vaddress.try_into()?),
                    }
                        .into(),
                    consumed,
                ))
            }
            AirPackable::DescriptorUpdate(DescriptorUpdate) => {
                let DescriptorUpdate = &*DescriptorUpdate;
                let mut consumed = 0;
                let packable: AirPackable = (*DescriptorUpdate.AF).clone().into();
                let (packed_AF, consumed_AF) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_AF;
                let packable: AirPackable = (*DescriptorUpdate.AP).clone().into();
                let (packed_AP, consumed_AP) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_AP;
                let packable: AirPackable = (*DescriptorUpdate.descaddr).clone().into();
                let (packed_descaddr, consumed_descaddr) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_descaddr;
                Ok((
                    DescriptorUpdate {
                        AF: Box::new(packed_AF.try_into()?),
                        AP: Box::new(packed_AP.try_into()?),
                        descaddr: Box::new(packed_descaddr.try_into()?),
                    }
                        .into(),
                    consumed,
                ))
            }
            AirPackable::ExceptionRecord(ExceptionRecord) => {
                let ExceptionRecord = &*ExceptionRecord;
                let mut consumed = 0;
                let packable: AirPackable = (*ExceptionRecord.exceptype).clone().into();
                let (packed_exceptype, consumed_exceptype) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_exceptype;
                let packable: AirPackable = (*ExceptionRecord.syndrome).clone().into();
                let (packed_syndrome, consumed_syndrome) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_syndrome;
                let packable: AirPackable = (*ExceptionRecord.vaddress).clone().into();
                let (packed_vaddress, consumed_vaddress) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_vaddress;
                let packable: AirPackable = (*ExceptionRecord.ipavalid).clone().into();
                let (packed_ipavalid, consumed_ipavalid) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_ipavalid;
                let packable: AirPackable = (*ExceptionRecord.NS).clone().into();
                let (packed_NS, consumed_NS) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_NS;
                let packable: AirPackable = (*ExceptionRecord.ipaddress).clone().into();
                let (packed_ipaddress, consumed_ipaddress) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_ipaddress;
                Ok((
                    ExceptionRecord {
                        exceptype: Box::new(packed_exceptype.try_into()?),
                        syndrome: Box::new(packed_syndrome.try_into()?),
                        vaddress: Box::new(packed_vaddress.try_into()?),
                        ipavalid: Box::new(packed_ipavalid.try_into()?),
                        NS: Box::new(packed_NS.try_into()?),
                        ipaddress: Box::new(packed_ipaddress.try_into()?),
                    }
                        .into(),
                    consumed,
                ))
            }
            AirPackable::FaultRecord(FaultRecord) => {
                let FaultRecord = &*FaultRecord;
                let mut consumed = 0;
                let packable: AirPackable = (*FaultRecord.statuscode).clone().into();
                let (packed_statuscode, consumed_statuscode) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_statuscode;
                let packable: AirPackable = (*FaultRecord.acctype).clone().into();
                let (packed_acctype, consumed_acctype) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_acctype;
                let packable: AirPackable = (*FaultRecord.ipaddress).clone().into();
                let (packed_ipaddress, consumed_ipaddress) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_ipaddress;
                let packable: AirPackable = (*FaultRecord.s2fs1walk).clone().into();
                let (packed_s2fs1walk, consumed_s2fs1walk) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_s2fs1walk;
                let packable: AirPackable = (*FaultRecord.write).clone().into();
                let (packed_write, consumed_write) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_write;
                let packable: AirPackable = (*FaultRecord.level).clone().into();
                let (packed_level, consumed_level) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_level;
                let packable: AirPackable = (*FaultRecord.extflag).clone().into();
                let (packed_extflag, consumed_extflag) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_extflag;
                let packable: AirPackable = (*FaultRecord.secondstage).clone().into();
                let (packed_secondstage, consumed_secondstage) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_secondstage;
                let packable: AirPackable = (*FaultRecord.domain).clone().into();
                let (packed_domain, consumed_domain) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_domain;
                let packable: AirPackable = (*FaultRecord.errortype).clone().into();
                let (packed_errortype, consumed_errortype) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_errortype;
                let packable: AirPackable = (*FaultRecord.debugmoe).clone().into();
                let (packed_debugmoe, consumed_debugmoe) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_debugmoe;
                Ok((
                    FaultRecord {
                        statuscode: Box::new(packed_statuscode.try_into()?),
                        acctype: Box::new(packed_acctype.try_into()?),
                        ipaddress: Box::new(packed_ipaddress.try_into()?),
                        s2fs1walk: Box::new(packed_s2fs1walk.try_into()?),
                        write: Box::new(packed_write.try_into()?),
                        level: Box::new(packed_level.try_into()?),
                        extflag: Box::new(packed_extflag.try_into()?),
                        secondstage: Box::new(packed_secondstage.try_into()?),
                        domain: Box::new(packed_domain.try_into()?),
                        errortype: Box::new(packed_errortype.try_into()?),
                        debugmoe: Box::new(packed_debugmoe.try_into()?),
                    }
                        .into(),
                    consumed,
                ))
            }
            AirPackable::FullAddress(FullAddress) => {
                let FullAddress = &*FullAddress;
                let mut consumed = 0;
                let packable: AirPackable = (*FullAddress.address).clone().into();
                let (packed_address, consumed_address) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_address;
                let packable: AirPackable = (*FullAddress.NS).clone().into();
                let (packed_NS, consumed_NS) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_NS;
                Ok((
                    FullAddress {
                        address: Box::new(packed_address.try_into()?),
                        NS: Box::new(packed_NS.try_into()?),
                    }
                        .into(),
                    consumed,
                ))
            }
            AirPackable::MPAMinfo(MPAMinfo) => {
                let MPAMinfo = &*MPAMinfo;
                let mut consumed = 0;
                let packable: AirPackable = (*MPAMinfo.mpam_ns).clone().into();
                let (packed_mpam_ns, consumed_mpam_ns) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_mpam_ns;
                let packable: AirPackable = (*MPAMinfo.partid).clone().into();
                let (packed_partid, consumed_partid) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_partid;
                let packable: AirPackable = (*MPAMinfo.pmg).clone().into();
                let (packed_pmg, consumed_pmg) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_pmg;
                Ok((
                    MPAMinfo {
                        mpam_ns: Box::new(packed_mpam_ns.try_into()?),
                        partid: Box::new(packed_partid.try_into()?),
                        pmg: Box::new(packed_pmg.try_into()?),
                    }
                        .into(),
                    consumed,
                ))
            }
            AirPackable::MemAttrHints(MemAttrHints) => {
                let MemAttrHints = &*MemAttrHints;
                let mut consumed = 0;
                let packable: AirPackable = (*MemAttrHints.attrs).clone().into();
                let (packed_attrs, consumed_attrs) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_attrs;
                let packable: AirPackable = (*MemAttrHints.hints).clone().into();
                let (packed_hints, consumed_hints) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_hints;
                let packable: AirPackable = (*MemAttrHints.transient).clone().into();
                let (packed_transient, consumed_transient) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_transient;
                Ok((
                    MemAttrHints {
                        attrs: Box::new(packed_attrs.try_into()?),
                        hints: Box::new(packed_hints.try_into()?),
                        transient: Box::new(packed_transient.try_into()?),
                    }
                        .into(),
                    consumed,
                ))
            }
            AirPackable::MemoryAttributes(MemoryAttributes) => {
                let MemoryAttributes = &*MemoryAttributes;
                let mut consumed = 0;
                let packable: AirPackable = (*MemoryAttributes.memtype).clone().into();
                let (packed_memtype, consumed_memtype) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_memtype;
                let packable: AirPackable = (*MemoryAttributes.device).clone().into();
                let (packed_device, consumed_device) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_device;
                let packable: AirPackable = (*MemoryAttributes.inner).clone().into();
                let (packed_inner, consumed_inner) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_inner;
                let packable: AirPackable = (*MemoryAttributes.outer).clone().into();
                let (packed_outer, consumed_outer) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_outer;
                let packable: AirPackable = (*MemoryAttributes.tagged).clone().into();
                let (packed_tagged, consumed_tagged) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_tagged;
                let packable: AirPackable = (*MemoryAttributes.shareable).clone().into();
                let (packed_shareable, consumed_shareable) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_shareable;
                let packable: AirPackable = (*MemoryAttributes.outershareable)
                    .clone()
                    .into();
                let (packed_outershareable, consumed_outershareable) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_outershareable;
                Ok((
                    MemoryAttributes {
                        memtype: Box::new(packed_memtype.try_into()?),
                        device: Box::new(packed_device.try_into()?),
                        inner: Box::new(packed_inner.try_into()?),
                        outer: Box::new(packed_outer.try_into()?),
                        tagged: Box::new(packed_tagged.try_into()?),
                        shareable: Box::new(packed_shareable.try_into()?),
                        outershareable: Box::new(packed_outershareable.try_into()?),
                    }
                        .into(),
                    consumed,
                ))
            }
            AirPackable::PCSample(PCSample) => {
                let PCSample = &*PCSample;
                let mut consumed = 0;
                let packable: AirPackable = (*PCSample.valid).clone().into();
                let (packed_valid, consumed_valid) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_valid;
                let packable: AirPackable = (*PCSample.pc).clone().into();
                let (packed_pc, consumed_pc) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_pc;
                let packable: AirPackable = (*PCSample.el).clone().into();
                let (packed_el, consumed_el) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_el;
                let packable: AirPackable = (*PCSample.rw).clone().into();
                let (packed_rw, consumed_rw) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_rw;
                let packable: AirPackable = (*PCSample.ns).clone().into();
                let (packed_ns, consumed_ns) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_ns;
                let packable: AirPackable = (*PCSample.has_el2).clone().into();
                let (packed_has_el2, consumed_has_el2) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_has_el2;
                let packable: AirPackable = (*PCSample.contextidr).clone().into();
                let (packed_contextidr, consumed_contextidr) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_contextidr;
                let packable: AirPackable = (*PCSample.contextidr_el2).clone().into();
                let (packed_contextidr_el2, consumed_contextidr_el2) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_contextidr_el2;
                let packable: AirPackable = (*PCSample.el0h).clone().into();
                let (packed_el0h, consumed_el0h) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_el0h;
                let packable: AirPackable = (*PCSample.vmid).clone().into();
                let (packed_vmid, consumed_vmid) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_vmid;
                Ok((
                    PCSample {
                        valid: Box::new(packed_valid.try_into()?),
                        pc: Box::new(packed_pc.try_into()?),
                        el: Box::new(packed_el.try_into()?),
                        rw: Box::new(packed_rw.try_into()?),
                        ns: Box::new(packed_ns.try_into()?),
                        has_el2: Box::new(packed_has_el2.try_into()?),
                        contextidr: Box::new(packed_contextidr.try_into()?),
                        contextidr_el2: Box::new(packed_contextidr_el2.try_into()?),
                        el0h: Box::new(packed_el0h.try_into()?),
                        vmid: Box::new(packed_vmid.try_into()?),
                    }
                        .into(),
                    consumed,
                ))
            }
            AirPackable::Permissions(Permissions) => {
                let Permissions = &*Permissions;
                let mut consumed = 0;
                let packable: AirPackable = (*Permissions.ap).clone().into();
                let (packed_ap, consumed_ap) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_ap;
                let packable: AirPackable = (*Permissions.xn).clone().into();
                let (packed_xn, consumed_xn) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_xn;
                let packable: AirPackable = (*Permissions.xxn).clone().into();
                let (packed_xxn, consumed_xxn) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_xxn;
                let packable: AirPackable = (*Permissions.pxn).clone().into();
                let (packed_pxn, consumed_pxn) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_pxn;
                Ok((
                    Permissions {
                        ap: Box::new(packed_ap.try_into()?),
                        xn: Box::new(packed_xn.try_into()?),
                        xxn: Box::new(packed_xxn.try_into()?),
                        pxn: Box::new(packed_pxn.try_into()?),
                    }
                        .into(),
                    consumed,
                ))
            }
            AirPackable::ProcState(ProcState) => {
                let ProcState = &*ProcState;
                let mut consumed = 0;
                let packable: AirPackable = (*ProcState.N).clone().into();
                let (packed_N, consumed_N) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_N;
                let packable: AirPackable = (*ProcState.Z).clone().into();
                let (packed_Z, consumed_Z) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_Z;
                let packable: AirPackable = (*ProcState.C).clone().into();
                let (packed_C, consumed_C) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_C;
                let packable: AirPackable = (*ProcState.V).clone().into();
                let (packed_V, consumed_V) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_V;
                let packable: AirPackable = (*ProcState.D).clone().into();
                let (packed_D, consumed_D) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_D;
                let packable: AirPackable = (*ProcState.A).clone().into();
                let (packed_A, consumed_A) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_A;
                let packable: AirPackable = (*ProcState.I).clone().into();
                let (packed_I, consumed_I) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_I;
                let packable: AirPackable = (*ProcState.F).clone().into();
                let (packed_F, consumed_F) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_F;
                let packable: AirPackable = (*ProcState.PAN).clone().into();
                let (packed_PAN, consumed_PAN) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_PAN;
                let packable: AirPackable = (*ProcState.UAO).clone().into();
                let (packed_UAO, consumed_UAO) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_UAO;
                let packable: AirPackable = (*ProcState.DIT).clone().into();
                let (packed_DIT, consumed_DIT) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_DIT;
                let packable: AirPackable = (*ProcState.TCO).clone().into();
                let (packed_TCO, consumed_TCO) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_TCO;
                let packable: AirPackable = (*ProcState.BTYPE).clone().into();
                let (packed_BTYPE, consumed_BTYPE) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_BTYPE;
                let packable: AirPackable = (*ProcState.SS).clone().into();
                let (packed_SS, consumed_SS) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_SS;
                let packable: AirPackable = (*ProcState.IL).clone().into();
                let (packed_IL, consumed_IL) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_IL;
                let packable: AirPackable = (*ProcState.EL).clone().into();
                let (packed_EL, consumed_EL) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_EL;
                let packable: AirPackable = (*ProcState.nRW).clone().into();
                let (packed_nRW, consumed_nRW) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_nRW;
                let packable: AirPackable = (*ProcState.SP).clone().into();
                let (packed_SP, consumed_SP) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_SP;
                let packable: AirPackable = (*ProcState.Q).clone().into();
                let (packed_Q, consumed_Q) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_Q;
                let packable: AirPackable = (*ProcState.GE).clone().into();
                let (packed_GE, consumed_GE) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_GE;
                let packable: AirPackable = (*ProcState.SSBS).clone().into();
                let (packed_SSBS, consumed_SSBS) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_SSBS;
                let packable: AirPackable = (*ProcState.IT).clone().into();
                let (packed_IT, consumed_IT) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_IT;
                let packable: AirPackable = (*ProcState.J).clone().into();
                let (packed_J, consumed_J) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_J;
                let packable: AirPackable = (*ProcState.T).clone().into();
                let (packed_T, consumed_T) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_T;
                let packable: AirPackable = (*ProcState.E).clone().into();
                let (packed_E, consumed_E) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_E;
                let packable: AirPackable = (*ProcState.M).clone().into();
                let (packed_M, consumed_M) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_M;
                Ok((
                    ProcState {
                        N: Box::new(packed_N.try_into()?),
                        Z: Box::new(packed_Z.try_into()?),
                        C: Box::new(packed_C.try_into()?),
                        V: Box::new(packed_V.try_into()?),
                        D: Box::new(packed_D.try_into()?),
                        A: Box::new(packed_A.try_into()?),
                        I: Box::new(packed_I.try_into()?),
                        F: Box::new(packed_F.try_into()?),
                        PAN: Box::new(packed_PAN.try_into()?),
                        UAO: Box::new(packed_UAO.try_into()?),
                        DIT: Box::new(packed_DIT.try_into()?),
                        TCO: Box::new(packed_TCO.try_into()?),
                        BTYPE: Box::new(packed_BTYPE.try_into()?),
                        SS: Box::new(packed_SS.try_into()?),
                        IL: Box::new(packed_IL.try_into()?),
                        EL: Box::new(packed_EL.try_into()?),
                        nRW: Box::new(packed_nRW.try_into()?),
                        SP: Box::new(packed_SP.try_into()?),
                        Q: Box::new(packed_Q.try_into()?),
                        GE: Box::new(packed_GE.try_into()?),
                        SSBS: Box::new(packed_SSBS.try_into()?),
                        IT: Box::new(packed_IT.try_into()?),
                        J: Box::new(packed_J.try_into()?),
                        T: Box::new(packed_T.try_into()?),
                        E: Box::new(packed_E.try_into()?),
                        M: Box::new(packed_M.try_into()?),
                    }
                        .into(),
                    consumed,
                ))
            }
            AirPackable::TLBRecord(TLBRecord) => {
                let TLBRecord = &*TLBRecord;
                let mut consumed = 0;
                let packable: AirPackable = (*TLBRecord.perms).clone().into();
                let (packed_perms, consumed_perms) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_perms;
                let packable: AirPackable = (*TLBRecord.nG).clone().into();
                let (packed_nG, consumed_nG) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_nG;
                let packable: AirPackable = (*TLBRecord.domain).clone().into();
                let (packed_domain, consumed_domain) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_domain;
                let packable: AirPackable = (*TLBRecord.GP).clone().into();
                let (packed_GP, consumed_GP) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_GP;
                let packable: AirPackable = (*TLBRecord.contiguous).clone().into();
                let (packed_contiguous, consumed_contiguous) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_contiguous;
                let packable: AirPackable = (*TLBRecord.level).clone().into();
                let (packed_level, consumed_level) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_level;
                let packable: AirPackable = (*TLBRecord.blocksize).clone().into();
                let (packed_blocksize, consumed_blocksize) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_blocksize;
                let packable: AirPackable = (*TLBRecord.descupdate).clone().into();
                let (packed_descupdate, consumed_descupdate) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_descupdate;
                let packable: AirPackable = (*TLBRecord.CnP).clone().into();
                let (packed_CnP, consumed_CnP) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_CnP;
                let packable: AirPackable = (*TLBRecord.addrdesc).clone().into();
                let (packed_addrdesc, consumed_addrdesc) = packable
                    .pack_from_air_values_and_types(
                        &values[consumed..],
                        &types[consumed..],
                    )?;
                consumed += consumed_addrdesc;
                Ok((
                    TLBRecord {
                        perms: Box::new(packed_perms.try_into()?),
                        nG: Box::new(packed_nG.try_into()?),
                        domain: Box::new(packed_domain.try_into()?),
                        GP: Box::new(packed_GP.try_into()?),
                        contiguous: Box::new(packed_contiguous.try_into()?),
                        level: Box::new(packed_level.try_into()?),
                        blocksize: Box::new(packed_blocksize.try_into()?),
                        descupdate: Box::new(packed_descupdate.try_into()?),
                        CnP: Box::new(packed_CnP.try_into()?),
                        addrdesc: Box::new(packed_addrdesc.try_into()?),
                    }
                        .into(),
                    consumed,
                ))
            }
        }
    }
}
impl From<lift::types::Variable> for AirPackable {
    fn from(inner: lift::types::Variable) -> Self {
        AirPackable::Variable(Box::new(inner))
    }
}
impl TryFrom<AirPackable> for lift::types::Variable {
    type Error = AArch64LifterError;
    fn try_from(value: AirPackable) -> Result<Self, Self::Error> {
        if let AirPackable::Variable(inner) = value {
            Ok(*inner)
        } else {
            Err(
                AArch64LifterError::MismatchedAirPackableType(
                    "Variable".to_string(),
                    format!("{:?}", value),
                ),
            )
        }
    }
}
impl From<lift::types::AArch32_SErrorSyndrome> for AirPackable {
    fn from(inner: lift::types::AArch32_SErrorSyndrome) -> Self {
        AirPackable::AArch32_SErrorSyndrome(Box::new(inner))
    }
}
impl TryFrom<AirPackable> for lift::types::AArch32_SErrorSyndrome {
    type Error = AArch64LifterError;
    fn try_from(value: AirPackable) -> Result<Self, Self::Error> {
        if let AirPackable::AArch32_SErrorSyndrome(inner) = value {
            Ok(*inner)
        } else {
            Err(
                AArch64LifterError::MismatchedAirPackableType(
                    "AArch32_SErrorSyndrome".to_string(),
                    format!("{:?}", value),
                ),
            )
        }
    }
}
impl From<lift::types::AccessDescriptor> for AirPackable {
    fn from(inner: lift::types::AccessDescriptor) -> Self {
        AirPackable::AccessDescriptor(Box::new(inner))
    }
}
impl TryFrom<AirPackable> for lift::types::AccessDescriptor {
    type Error = AArch64LifterError;
    fn try_from(value: AirPackable) -> Result<Self, Self::Error> {
        if let AirPackable::AccessDescriptor(inner) = value {
            Ok(*inner)
        } else {
            Err(
                AArch64LifterError::MismatchedAirPackableType(
                    "AccessDescriptor".to_string(),
                    format!("{:?}", value),
                ),
            )
        }
    }
}
impl From<lift::types::AddressDescriptor> for AirPackable {
    fn from(inner: lift::types::AddressDescriptor) -> Self {
        AirPackable::AddressDescriptor(Box::new(inner))
    }
}
impl TryFrom<AirPackable> for lift::types::AddressDescriptor {
    type Error = AArch64LifterError;
    fn try_from(value: AirPackable) -> Result<Self, Self::Error> {
        if let AirPackable::AddressDescriptor(inner) = value {
            Ok(*inner)
        } else {
            Err(
                AArch64LifterError::MismatchedAirPackableType(
                    "AddressDescriptor".to_string(),
                    format!("{:?}", value),
                ),
            )
        }
    }
}
impl From<lift::types::DescriptorUpdate> for AirPackable {
    fn from(inner: lift::types::DescriptorUpdate) -> Self {
        AirPackable::DescriptorUpdate(Box::new(inner))
    }
}
impl TryFrom<AirPackable> for lift::types::DescriptorUpdate {
    type Error = AArch64LifterError;
    fn try_from(value: AirPackable) -> Result<Self, Self::Error> {
        if let AirPackable::DescriptorUpdate(inner) = value {
            Ok(*inner)
        } else {
            Err(
                AArch64LifterError::MismatchedAirPackableType(
                    "DescriptorUpdate".to_string(),
                    format!("{:?}", value),
                ),
            )
        }
    }
}
impl From<lift::types::ExceptionRecord> for AirPackable {
    fn from(inner: lift::types::ExceptionRecord) -> Self {
        AirPackable::ExceptionRecord(Box::new(inner))
    }
}
impl TryFrom<AirPackable> for lift::types::ExceptionRecord {
    type Error = AArch64LifterError;
    fn try_from(value: AirPackable) -> Result<Self, Self::Error> {
        if let AirPackable::ExceptionRecord(inner) = value {
            Ok(*inner)
        } else {
            Err(
                AArch64LifterError::MismatchedAirPackableType(
                    "ExceptionRecord".to_string(),
                    format!("{:?}", value),
                ),
            )
        }
    }
}
impl From<lift::types::FaultRecord> for AirPackable {
    fn from(inner: lift::types::FaultRecord) -> Self {
        AirPackable::FaultRecord(Box::new(inner))
    }
}
impl TryFrom<AirPackable> for lift::types::FaultRecord {
    type Error = AArch64LifterError;
    fn try_from(value: AirPackable) -> Result<Self, Self::Error> {
        if let AirPackable::FaultRecord(inner) = value {
            Ok(*inner)
        } else {
            Err(
                AArch64LifterError::MismatchedAirPackableType(
                    "FaultRecord".to_string(),
                    format!("{:?}", value),
                ),
            )
        }
    }
}
impl From<lift::types::FullAddress> for AirPackable {
    fn from(inner: lift::types::FullAddress) -> Self {
        AirPackable::FullAddress(Box::new(inner))
    }
}
impl TryFrom<AirPackable> for lift::types::FullAddress {
    type Error = AArch64LifterError;
    fn try_from(value: AirPackable) -> Result<Self, Self::Error> {
        if let AirPackable::FullAddress(inner) = value {
            Ok(*inner)
        } else {
            Err(
                AArch64LifterError::MismatchedAirPackableType(
                    "FullAddress".to_string(),
                    format!("{:?}", value),
                ),
            )
        }
    }
}
impl From<lift::types::MPAMinfo> for AirPackable {
    fn from(inner: lift::types::MPAMinfo) -> Self {
        AirPackable::MPAMinfo(Box::new(inner))
    }
}
impl TryFrom<AirPackable> for lift::types::MPAMinfo {
    type Error = AArch64LifterError;
    fn try_from(value: AirPackable) -> Result<Self, Self::Error> {
        if let AirPackable::MPAMinfo(inner) = value {
            Ok(*inner)
        } else {
            Err(
                AArch64LifterError::MismatchedAirPackableType(
                    "MPAMinfo".to_string(),
                    format!("{:?}", value),
                ),
            )
        }
    }
}
impl From<lift::types::MemAttrHints> for AirPackable {
    fn from(inner: lift::types::MemAttrHints) -> Self {
        AirPackable::MemAttrHints(Box::new(inner))
    }
}
impl TryFrom<AirPackable> for lift::types::MemAttrHints {
    type Error = AArch64LifterError;
    fn try_from(value: AirPackable) -> Result<Self, Self::Error> {
        if let AirPackable::MemAttrHints(inner) = value {
            Ok(*inner)
        } else {
            Err(
                AArch64LifterError::MismatchedAirPackableType(
                    "MemAttrHints".to_string(),
                    format!("{:?}", value),
                ),
            )
        }
    }
}
impl From<lift::types::MemoryAttributes> for AirPackable {
    fn from(inner: lift::types::MemoryAttributes) -> Self {
        AirPackable::MemoryAttributes(Box::new(inner))
    }
}
impl TryFrom<AirPackable> for lift::types::MemoryAttributes {
    type Error = AArch64LifterError;
    fn try_from(value: AirPackable) -> Result<Self, Self::Error> {
        if let AirPackable::MemoryAttributes(inner) = value {
            Ok(*inner)
        } else {
            Err(
                AArch64LifterError::MismatchedAirPackableType(
                    "MemoryAttributes".to_string(),
                    format!("{:?}", value),
                ),
            )
        }
    }
}
impl From<lift::types::PCSample> for AirPackable {
    fn from(inner: lift::types::PCSample) -> Self {
        AirPackable::PCSample(Box::new(inner))
    }
}
impl TryFrom<AirPackable> for lift::types::PCSample {
    type Error = AArch64LifterError;
    fn try_from(value: AirPackable) -> Result<Self, Self::Error> {
        if let AirPackable::PCSample(inner) = value {
            Ok(*inner)
        } else {
            Err(
                AArch64LifterError::MismatchedAirPackableType(
                    "PCSample".to_string(),
                    format!("{:?}", value),
                ),
            )
        }
    }
}
impl From<lift::types::Permissions> for AirPackable {
    fn from(inner: lift::types::Permissions) -> Self {
        AirPackable::Permissions(Box::new(inner))
    }
}
impl TryFrom<AirPackable> for lift::types::Permissions {
    type Error = AArch64LifterError;
    fn try_from(value: AirPackable) -> Result<Self, Self::Error> {
        if let AirPackable::Permissions(inner) = value {
            Ok(*inner)
        } else {
            Err(
                AArch64LifterError::MismatchedAirPackableType(
                    "Permissions".to_string(),
                    format!("{:?}", value),
                ),
            )
        }
    }
}
impl From<lift::types::ProcState> for AirPackable {
    fn from(inner: lift::types::ProcState) -> Self {
        AirPackable::ProcState(Box::new(inner))
    }
}
impl TryFrom<AirPackable> for lift::types::ProcState {
    type Error = AArch64LifterError;
    fn try_from(value: AirPackable) -> Result<Self, Self::Error> {
        if let AirPackable::ProcState(inner) = value {
            Ok(*inner)
        } else {
            Err(
                AArch64LifterError::MismatchedAirPackableType(
                    "ProcState".to_string(),
                    format!("{:?}", value),
                ),
            )
        }
    }
}
impl From<lift::types::TLBRecord> for AirPackable {
    fn from(inner: lift::types::TLBRecord) -> Self {
        AirPackable::TLBRecord(Box::new(inner))
    }
}
impl TryFrom<AirPackable> for lift::types::TLBRecord {
    type Error = AArch64LifterError;
    fn try_from(value: AirPackable) -> Result<Self, Self::Error> {
        if let AirPackable::TLBRecord(inner) = value {
            Ok(*inner)
        } else {
            Err(
                AArch64LifterError::MismatchedAirPackableType(
                    "TLBRecord".to_string(),
                    format!("{:?}", value),
                ),
            )
        }
    }
}
pub fn generated_sequencer_logic(
    instruction: common::types::Instruction,
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    address: u64,
) -> Result<(), AArch64LifterError> {
    match instruction {
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ABS_Z_P_Z__(operands) => {
            let common::types::ABS_Z_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_ABS_Z_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ADDPL_R_RI__(operands) => {
            let common::types::ADDPL_R_RI___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_ADDPL_R_RI__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ADDVL_R_RI__(operands) => {
            let common::types::ADDVL_R_RI___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_ADDVL_R_RI__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ADD_Z_P_ZZ__(operands) => {
            let common::types::ADD_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_ADD_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ADD_Z_ZI__(operands) => {
            let common::types::ADD_Z_ZI___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_ADD_Z_ZI__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ADD_Z_ZZ__(operands) => {
            let common::types::ADD_Z_ZZ___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_ADD_Z_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ADR_Z_AZ_SD_same_scaled(operands) => {
            let common::types::ADR_Z_AZ_SD_same_scaled_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_ADR_Z_AZ_SD_same_scaled(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ANDV_R_P_Z__(operands) => {
            let common::types::ANDV_R_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_ANDV_R_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::AND_P_P_PP_Z(operands) => {
            let common::types::AND_P_P_PP_Z_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_AND_P_P_PP_Z(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::AND_Z_P_ZZ__(operands) => {
            let common::types::AND_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_AND_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::AND_Z_ZI__(operands) => {
            let common::types::AND_Z_ZI___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_AND_Z_ZI__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::AND_Z_ZZ__(operands) => {
            let common::types::AND_Z_ZZ___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_AND_Z_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ASRD_Z_P_ZI__(operands) => {
            let common::types::ASRD_Z_P_ZI___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_ASRD_Z_P_ZI__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ASRR_Z_P_ZZ__(operands) => {
            let common::types::ASRR_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_ASRR_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ASR_Z_P_ZI__(operands) => {
            let common::types::ASR_Z_P_ZI___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_ASR_Z_P_ZI__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ASR_Z_P_ZW__(operands) => {
            let common::types::ASR_Z_P_ZW___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_ASR_Z_P_ZW__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ASR_Z_P_ZZ__(operands) => {
            let common::types::ASR_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_ASR_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ASR_Z_ZI__(operands) => {
            let common::types::ASR_Z_ZI___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_ASR_Z_ZI__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ASR_Z_ZW__(operands) => {
            let common::types::ASR_Z_ZW___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_ASR_Z_ZW__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::BFCVTNT_Z_P_Z_S2BF(operands) => {
            let common::types::BFCVTNT_Z_P_Z_S2BF_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_BFCVTNT_Z_P_Z_S2BF(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::BFCVT_Z_P_Z_S2BF(operands) => {
            let common::types::BFCVT_Z_P_Z_S2BF_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_BFCVT_Z_P_Z_S2BF(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::BFDOT_Z_ZZZ__(operands) => {
            let common::types::BFDOT_Z_ZZZ___operands { n } = *operands;
            lift::generated::lift_blocks::lift_BFDOT_Z_ZZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::BFDOT_Z_ZZZi__(operands) => {
            let common::types::BFDOT_Z_ZZZi___operands { n } = *operands;
            lift::generated::lift_blocks::lift_BFDOT_Z_ZZZi__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::BFMLALB_Z_ZZZ__(operands) => {
            let common::types::BFMLALB_Z_ZZZ___operands { n } = *operands;
            lift::generated::lift_blocks::lift_BFMLALB_Z_ZZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::BFMLALB_Z_ZZZi__(operands) => {
            let common::types::BFMLALB_Z_ZZZi___operands { n } = *operands;
            lift::generated::lift_blocks::lift_BFMLALB_Z_ZZZi__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::BFMLALT_Z_ZZZ__(operands) => {
            let common::types::BFMLALT_Z_ZZZ___operands { n } = *operands;
            lift::generated::lift_blocks::lift_BFMLALT_Z_ZZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::BFMLALT_Z_ZZZi__(operands) => {
            let common::types::BFMLALT_Z_ZZZi___operands { n } = *operands;
            lift::generated::lift_blocks::lift_BFMLALT_Z_ZZZi__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::BFMMLA_Z_ZZZ__(operands) => {
            let common::types::BFMMLA_Z_ZZZ___operands { n } = *operands;
            lift::generated::lift_blocks::lift_BFMMLA_Z_ZZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::BIC_P_P_PP_Z(operands) => {
            let common::types::BIC_P_P_PP_Z_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_BIC_P_P_PP_Z(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::BIC_Z_P_ZZ__(operands) => {
            let common::types::BIC_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_BIC_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::BIC_Z_ZZ__(operands) => {
            let common::types::BIC_Z_ZZ___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_BIC_Z_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::BRKA_P_P_P__(operands) => {
            let common::types::BRKA_P_P_P___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_BRKA_P_P_P__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::BRKB_P_P_P__(operands) => {
            let common::types::BRKB_P_P_P___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_BRKB_P_P_P__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::BRKN_P_P_PP__(operands) => {
            let common::types::BRKN_P_P_PP___operands { n } = *operands;
            lift::generated::lift_blocks::lift_BRKN_P_P_PP__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::BRKPA_P_P_PP__(operands) => {
            let common::types::BRKPA_P_P_PP___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_BRKPA_P_P_PP__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::BRKPB_P_P_PP__(operands) => {
            let common::types::BRKPB_P_P_PP___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_BRKPB_P_P_PP__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::CLASTA_R_P_Z__(operands) => {
            let common::types::CLASTA_R_P_Z___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_CLASTA_R_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::CLASTA_V_P_Z__(operands) => {
            let common::types::CLASTA_V_P_Z___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_CLASTA_V_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::CLASTA_Z_P_ZZ__(operands) => {
            let common::types::CLASTA_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_CLASTA_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::CLASTB_R_P_Z__(operands) => {
            let common::types::CLASTB_R_P_Z___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_CLASTB_R_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::CLASTB_V_P_Z__(operands) => {
            let common::types::CLASTB_V_P_Z___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_CLASTB_V_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::CLASTB_Z_P_ZZ__(operands) => {
            let common::types::CLASTB_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_CLASTB_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::CLS_Z_P_Z__(operands) => {
            let common::types::CLS_Z_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_CLS_Z_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::CLZ_Z_P_Z__(operands) => {
            let common::types::CLZ_Z_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_CLZ_Z_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::CMPEQ_P_P_ZI__(operands) => {
            let common::types::CMPEQ_P_P_ZI___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_CMPEQ_P_P_ZI__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::CMPEQ_P_P_ZW__(operands) => {
            let common::types::CMPEQ_P_P_ZW___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_CMPEQ_P_P_ZW__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::CMPEQ_P_P_ZZ__(operands) => {
            let common::types::CMPEQ_P_P_ZZ___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_CMPEQ_P_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::CNOT_Z_P_Z__(operands) => {
            let common::types::CNOT_Z_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_CNOT_Z_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::CNTB_R_S__(operands) => {
            let common::types::CNTB_R_S___operands { d } = *operands;
            lift::generated::lift_blocks::lift_CNTB_R_S__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::CNTP_R_P_P__(operands) => {
            let common::types::CNTP_R_P_P___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_CNTP_R_P_P__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::CNT_Z_P_Z__(operands) => {
            let common::types::CNT_Z_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_CNT_Z_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::COMPACT_Z_P_Z__(operands) => {
            let common::types::COMPACT_Z_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_COMPACT_Z_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::CPY_Z_O_I__(operands) => {
            let common::types::CPY_Z_O_I___operands { d } = *operands;
            lift::generated::lift_blocks::lift_CPY_Z_O_I__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::CPY_Z_P_I__(operands) => {
            let common::types::CPY_Z_P_I___operands { d } = *operands;
            lift::generated::lift_blocks::lift_CPY_Z_P_I__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::CPY_Z_P_R__(operands) => {
            let common::types::CPY_Z_P_R___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_CPY_Z_P_R__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::CPY_Z_P_V__(operands) => {
            let common::types::CPY_Z_P_V___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_CPY_Z_P_V__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::CTERMEQ_RR__(operands) => {
            let common::types::CTERMEQ_RR___operands { n } = *operands;
            lift::generated::lift_blocks::lift_CTERMEQ_RR__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::DECB_R_RS__(operands) => {
            let common::types::DECB_R_RS___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_DECB_R_RS__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::DECD_Z_ZS__(operands) => {
            let common::types::DECD_Z_ZS___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_DECD_Z_ZS__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::DECP_R_P_R__(operands) => {
            let common::types::DECP_R_P_R___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_DECP_R_P_R__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::DECP_Z_P_Z__(operands) => {
            let common::types::DECP_Z_P_Z___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_DECP_Z_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::DUPM_Z_I__(operands) => {
            let common::types::DUPM_Z_I___operands { d } = *operands;
            lift::generated::lift_blocks::lift_DUPM_Z_I__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::DUP_Z_I__(operands) => {
            let common::types::DUP_Z_I___operands { d } = *operands;
            lift::generated::lift_blocks::lift_DUP_Z_I__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::DUP_Z_R__(operands) => {
            let common::types::DUP_Z_R___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_DUP_Z_R__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::DUP_Z_Zi__(operands) => {
            let common::types::DUP_Z_Zi___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_DUP_Z_Zi__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::EORV_R_P_Z__(operands) => {
            let common::types::EORV_R_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_EORV_R_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::EOR_P_P_PP_Z(operands) => {
            let common::types::EOR_P_P_PP_Z_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_EOR_P_P_PP_Z(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::EOR_Z_P_ZZ__(operands) => {
            let common::types::EOR_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_EOR_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::EOR_Z_ZI__(operands) => {
            let common::types::EOR_Z_ZI___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_EOR_Z_ZI__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::EOR_Z_ZZ__(operands) => {
            let common::types::EOR_Z_ZZ___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_EOR_Z_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::EXT_Z_ZI_Des(operands) => {
            let common::types::EXT_Z_ZI_Des_operands { dn } = *operands;
            lift::generated::lift_blocks::lift_EXT_Z_ZI_Des(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FABD_Z_P_ZZ__(operands) => {
            let common::types::FABD_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FABD_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FABS_Z_P_Z__(operands) => {
            let common::types::FABS_Z_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_FABS_Z_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FACGT_P_P_ZZ__(operands) => {
            let common::types::FACGT_P_P_ZZ___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_FACGT_P_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FADDA_V_P_Z__(operands) => {
            let common::types::FADDA_V_P_Z___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FADDA_V_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FADDV_V_P_Z__(operands) => {
            let common::types::FADDV_V_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_FADDV_V_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FADD_Z_P_ZS__(operands) => {
            let common::types::FADD_Z_P_ZS___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FADD_Z_P_ZS__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FADD_Z_P_ZZ__(operands) => {
            let common::types::FADD_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FADD_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FADD_Z_ZZ__(operands) => {
            let common::types::FADD_Z_ZZ___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_FADD_Z_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FCADD_Z_P_ZZ__(operands) => {
            let common::types::FCADD_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FCADD_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FCMEQ_P_P_Z0__(operands) => {
            let common::types::FCMEQ_P_P_Z0___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_FCMEQ_P_P_Z0__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FCMEQ_P_P_ZZ__(operands) => {
            let common::types::FCMEQ_P_P_ZZ___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_FCMEQ_P_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FCMLA_Z_P_ZZZ__(operands) => {
            let common::types::FCMLA_Z_P_ZZZ___operands { n } = *operands;
            lift::generated::lift_blocks::lift_FCMLA_Z_P_ZZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FCMLA_Z_ZZZi_H(operands) => {
            let common::types::FCMLA_Z_ZZZi_H_operands { n } = *operands;
            lift::generated::lift_blocks::lift_FCMLA_Z_ZZZi_H(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FCPY_Z_P_I__(operands) => {
            let common::types::FCPY_Z_P_I___operands { d } = *operands;
            lift::generated::lift_blocks::lift_FCPY_Z_P_I__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FCVTZS_Z_P_Z_FP162H(operands) => {
            let common::types::FCVTZS_Z_P_Z_FP162H_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_FCVTZS_Z_P_Z_FP162H(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FCVTZU_Z_P_Z_FP162H(operands) => {
            let common::types::FCVTZU_Z_P_Z_FP162H_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_FCVTZU_Z_P_Z_FP162H(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FCVT_Z_P_Z_H2S(operands) => {
            let common::types::FCVT_Z_P_Z_H2S_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_FCVT_Z_P_Z_H2S(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FDIVR_Z_P_ZZ__(operands) => {
            let common::types::FDIVR_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FDIVR_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FDIV_Z_P_ZZ__(operands) => {
            let common::types::FDIV_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FDIV_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FDUP_Z_I__(operands) => {
            let common::types::FDUP_Z_I___operands { d } = *operands;
            lift::generated::lift_blocks::lift_FDUP_Z_I__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FEXPA_Z_Z__(operands) => {
            let common::types::FEXPA_Z_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_FEXPA_Z_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FMAD_Z_P_ZZZ__(operands) => {
            let common::types::FMAD_Z_P_ZZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FMAD_Z_P_ZZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FMAXNMV_V_P_Z__(operands) => {
            let common::types::FMAXNMV_V_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_FMAXNMV_V_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FMAXNM_Z_P_ZS__(operands) => {
            let common::types::FMAXNM_Z_P_ZS___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FMAXNM_Z_P_ZS__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FMAXNM_Z_P_ZZ__(operands) => {
            let common::types::FMAXNM_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FMAXNM_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FMAXV_V_P_Z__(operands) => {
            let common::types::FMAXV_V_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_FMAXV_V_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FMAX_Z_P_ZS__(operands) => {
            let common::types::FMAX_Z_P_ZS___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FMAX_Z_P_ZS__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FMAX_Z_P_ZZ__(operands) => {
            let common::types::FMAX_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FMAX_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FMINNMV_V_P_Z__(operands) => {
            let common::types::FMINNMV_V_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_FMINNMV_V_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FMINNM_Z_P_ZS__(operands) => {
            let common::types::FMINNM_Z_P_ZS___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FMINNM_Z_P_ZS__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FMINNM_Z_P_ZZ__(operands) => {
            let common::types::FMINNM_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FMINNM_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FMINV_V_P_Z__(operands) => {
            let common::types::FMINV_V_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_FMINV_V_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FMIN_Z_P_ZS__(operands) => {
            let common::types::FMIN_Z_P_ZS___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FMIN_Z_P_ZS__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FMIN_Z_P_ZZ__(operands) => {
            let common::types::FMIN_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FMIN_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FMLA_Z_P_ZZZ__(operands) => {
            let common::types::FMLA_Z_P_ZZZ___operands { n } = *operands;
            lift::generated::lift_blocks::lift_FMLA_Z_P_ZZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FMLA_Z_ZZZi_H(operands) => {
            let common::types::FMLA_Z_ZZZi_H_operands { n } = *operands;
            lift::generated::lift_blocks::lift_FMLA_Z_ZZZi_H(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FMLS_Z_P_ZZZ__(operands) => {
            let common::types::FMLS_Z_P_ZZZ___operands { n } = *operands;
            lift::generated::lift_blocks::lift_FMLS_Z_P_ZZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FMLS_Z_ZZZi_H(operands) => {
            let common::types::FMLS_Z_ZZZi_H_operands { n } = *operands;
            lift::generated::lift_blocks::lift_FMLS_Z_ZZZi_H(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FMMLA_Z_ZZZ_S(operands) => {
            let common::types::FMMLA_Z_ZZZ_S_operands { n } = *operands;
            lift::generated::lift_blocks::lift_FMMLA_Z_ZZZ_S(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FMSB_Z_P_ZZZ__(operands) => {
            let common::types::FMSB_Z_P_ZZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FMSB_Z_P_ZZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FMULX_Z_P_ZZ__(operands) => {
            let common::types::FMULX_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FMULX_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FMUL_Z_P_ZS__(operands) => {
            let common::types::FMUL_Z_P_ZS___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FMUL_Z_P_ZS__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FMUL_Z_P_ZZ__(operands) => {
            let common::types::FMUL_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FMUL_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FMUL_Z_ZZ__(operands) => {
            let common::types::FMUL_Z_ZZ___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_FMUL_Z_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FMUL_Z_ZZi_H(operands) => {
            let common::types::FMUL_Z_ZZi_H_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_FMUL_Z_ZZi_H(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FNEG_Z_P_Z__(operands) => {
            let common::types::FNEG_Z_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_FNEG_Z_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FNMAD_Z_P_ZZZ__(operands) => {
            let common::types::FNMAD_Z_P_ZZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FNMAD_Z_P_ZZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FNMLA_Z_P_ZZZ__(operands) => {
            let common::types::FNMLA_Z_P_ZZZ___operands { n } = *operands;
            lift::generated::lift_blocks::lift_FNMLA_Z_P_ZZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FNMLS_Z_P_ZZZ__(operands) => {
            let common::types::FNMLS_Z_P_ZZZ___operands { n } = *operands;
            lift::generated::lift_blocks::lift_FNMLS_Z_P_ZZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FNMSB_Z_P_ZZZ__(operands) => {
            let common::types::FNMSB_Z_P_ZZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FNMSB_Z_P_ZZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FRECPE_Z_Z__(operands) => {
            let common::types::FRECPE_Z_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_FRECPE_Z_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FRECPS_Z_ZZ__(operands) => {
            let common::types::FRECPS_Z_ZZ___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_FRECPS_Z_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FRECPX_Z_P_Z__(operands) => {
            let common::types::FRECPX_Z_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_FRECPX_Z_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FRINTI_Z_P_Z__(operands) => {
            let common::types::FRINTI_Z_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_FRINTI_Z_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FRSQRTE_Z_Z__(operands) => {
            let common::types::FRSQRTE_Z_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_FRSQRTE_Z_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FRSQRTS_Z_ZZ__(operands) => {
            let common::types::FRSQRTS_Z_ZZ___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_FRSQRTS_Z_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FSCALE_Z_P_ZZ__(operands) => {
            let common::types::FSCALE_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FSCALE_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FSQRT_Z_P_Z__(operands) => {
            let common::types::FSQRT_Z_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_FSQRT_Z_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FSUBR_Z_P_ZS__(operands) => {
            let common::types::FSUBR_Z_P_ZS___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FSUBR_Z_P_ZS__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FSUBR_Z_P_ZZ__(operands) => {
            let common::types::FSUBR_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FSUBR_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FSUB_Z_P_ZS__(operands) => {
            let common::types::FSUB_Z_P_ZS___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FSUB_Z_P_ZS__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FSUB_Z_P_ZZ__(operands) => {
            let common::types::FSUB_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FSUB_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FSUB_Z_ZZ__(operands) => {
            let common::types::FSUB_Z_ZZ___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_FSUB_Z_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FTMAD_Z_ZZI__(operands) => {
            let common::types::FTMAD_Z_ZZI___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_FTMAD_Z_ZZI__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FTSMUL_Z_ZZ__(operands) => {
            let common::types::FTSMUL_Z_ZZ___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_FTSMUL_Z_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::FTSSEL_Z_ZZ__(operands) => {
            let common::types::FTSSEL_Z_ZZ___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_FTSSEL_Z_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::INCB_R_RS__(operands) => {
            let common::types::INCB_R_RS___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_INCB_R_RS__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::INCD_Z_ZS__(operands) => {
            let common::types::INCD_Z_ZS___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_INCD_Z_ZS__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::INCP_R_P_R__(operands) => {
            let common::types::INCP_R_P_R___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_INCP_R_P_R__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::INCP_Z_P_Z__(operands) => {
            let common::types::INCP_Z_P_Z___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_INCP_Z_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::INDEX_Z_II__(operands) => {
            let common::types::INDEX_Z_II___operands { d } = *operands;
            lift::generated::lift_blocks::lift_INDEX_Z_II__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::INDEX_Z_IR__(operands) => {
            let common::types::INDEX_Z_IR___operands { d } = *operands;
            lift::generated::lift_blocks::lift_INDEX_Z_IR__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::INDEX_Z_RI__(operands) => {
            let common::types::INDEX_Z_RI___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_INDEX_Z_RI__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::INDEX_Z_RR__(operands) => {
            let common::types::INDEX_Z_RR___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_INDEX_Z_RR__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::INSR_Z_R__(operands) => {
            let common::types::INSR_Z_R___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_INSR_Z_R__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::INSR_Z_V__(operands) => {
            let common::types::INSR_Z_V___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_INSR_Z_V__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LASTA_R_P_Z__(operands) => {
            let common::types::LASTA_R_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_LASTA_R_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LASTA_V_P_Z__(operands) => {
            let common::types::LASTA_V_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_LASTA_V_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LASTB_R_P_Z__(operands) => {
            let common::types::LASTB_R_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_LASTB_R_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LASTB_V_P_Z__(operands) => {
            let common::types::LASTB_V_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_LASTB_V_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD1B_Z_P_AI_S(operands) => {
            let common::types::LD1B_Z_P_AI_S_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1B_Z_P_AI_S(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD1B_Z_P_BI_U8(operands) => {
            let common::types::LD1B_Z_P_BI_U8_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1B_Z_P_BI_U8(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD1B_Z_P_BR_U8(operands) => {
            let common::types::LD1B_Z_P_BR_U8_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1B_Z_P_BR_U8(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD1B_Z_P_BZ_D_x32_unscaled(operands) => {
            let common::types::LD1B_Z_P_BZ_D_x32_unscaled_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1B_Z_P_BZ_D_x32_unscaled(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD1D_Z_P_AI_D(operands) => {
            let common::types::LD1D_Z_P_AI_D_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1D_Z_P_AI_D(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD1D_Z_P_BI_U64(operands) => {
            let common::types::LD1D_Z_P_BI_U64_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1D_Z_P_BI_U64(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD1D_Z_P_BR_U64(operands) => {
            let common::types::LD1D_Z_P_BR_U64_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1D_Z_P_BR_U64(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD1D_Z_P_BZ_D_x32_scaled(operands) => {
            let common::types::LD1D_Z_P_BZ_D_x32_scaled_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1D_Z_P_BZ_D_x32_scaled(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD1H_Z_P_AI_S(operands) => {
            let common::types::LD1H_Z_P_AI_S_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1H_Z_P_AI_S(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD1H_Z_P_BI_U16(operands) => {
            let common::types::LD1H_Z_P_BI_U16_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1H_Z_P_BI_U16(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD1H_Z_P_BR_U16(operands) => {
            let common::types::LD1H_Z_P_BR_U16_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1H_Z_P_BR_U16(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD1H_Z_P_BZ_S_x32_scaled(operands) => {
            let common::types::LD1H_Z_P_BZ_S_x32_scaled_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1H_Z_P_BZ_S_x32_scaled(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD1RB_Z_P_BI_U8(operands) => {
            let common::types::LD1RB_Z_P_BI_U8_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1RB_Z_P_BI_U8(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD1RD_Z_P_BI_U64(operands) => {
            let common::types::LD1RD_Z_P_BI_U64_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1RD_Z_P_BI_U64(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD1RH_Z_P_BI_U16(operands) => {
            let common::types::LD1RH_Z_P_BI_U16_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1RH_Z_P_BI_U16(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD1ROB_Z_P_BI_U8(operands) => {
            let common::types::LD1ROB_Z_P_BI_U8_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1ROB_Z_P_BI_U8(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD1ROB_Z_P_BR_Contiguous(operands) => {
            let common::types::LD1ROB_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1ROB_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD1ROD_Z_P_BI_U64(operands) => {
            let common::types::LD1ROD_Z_P_BI_U64_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1ROD_Z_P_BI_U64(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD1ROD_Z_P_BR_Contiguous(operands) => {
            let common::types::LD1ROD_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1ROD_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD1ROH_Z_P_BI_U16(operands) => {
            let common::types::LD1ROH_Z_P_BI_U16_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1ROH_Z_P_BI_U16(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD1ROH_Z_P_BR_Contiguous(operands) => {
            let common::types::LD1ROH_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1ROH_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD1ROW_Z_P_BI_U32(operands) => {
            let common::types::LD1ROW_Z_P_BI_U32_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1ROW_Z_P_BI_U32(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD1ROW_Z_P_BR_Contiguous(operands) => {
            let common::types::LD1ROW_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1ROW_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD1RQB_Z_P_BI_U8(operands) => {
            let common::types::LD1RQB_Z_P_BI_U8_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1RQB_Z_P_BI_U8(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD1RQB_Z_P_BR_Contiguous(operands) => {
            let common::types::LD1RQB_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1RQB_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD1RQD_Z_P_BI_U64(operands) => {
            let common::types::LD1RQD_Z_P_BI_U64_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1RQD_Z_P_BI_U64(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD1RQD_Z_P_BR_Contiguous(operands) => {
            let common::types::LD1RQD_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1RQD_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD1RQH_Z_P_BI_U16(operands) => {
            let common::types::LD1RQH_Z_P_BI_U16_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1RQH_Z_P_BI_U16(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD1RQH_Z_P_BR_Contiguous(operands) => {
            let common::types::LD1RQH_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1RQH_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD1RQW_Z_P_BI_U32(operands) => {
            let common::types::LD1RQW_Z_P_BI_U32_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1RQW_Z_P_BI_U32(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD1RQW_Z_P_BR_Contiguous(operands) => {
            let common::types::LD1RQW_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1RQW_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD1RSB_Z_P_BI_S16(operands) => {
            let common::types::LD1RSB_Z_P_BI_S16_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1RSB_Z_P_BI_S16(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD1RSH_Z_P_BI_S32(operands) => {
            let common::types::LD1RSH_Z_P_BI_S32_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1RSH_Z_P_BI_S32(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD1RSW_Z_P_BI_S64(operands) => {
            let common::types::LD1RSW_Z_P_BI_S64_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1RSW_Z_P_BI_S64(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD1RW_Z_P_BI_U32(operands) => {
            let common::types::LD1RW_Z_P_BI_U32_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1RW_Z_P_BI_U32(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD1SB_Z_P_AI_S(operands) => {
            let common::types::LD1SB_Z_P_AI_S_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1SB_Z_P_AI_S(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD1SB_Z_P_BI_S16(operands) => {
            let common::types::LD1SB_Z_P_BI_S16_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1SB_Z_P_BI_S16(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD1SB_Z_P_BR_S16(operands) => {
            let common::types::LD1SB_Z_P_BR_S16_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1SB_Z_P_BR_S16(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD1SB_Z_P_BZ_D_x32_unscaled(operands) => {
            let common::types::LD1SB_Z_P_BZ_D_x32_unscaled_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1SB_Z_P_BZ_D_x32_unscaled(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD1SH_Z_P_AI_S(operands) => {
            let common::types::LD1SH_Z_P_AI_S_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1SH_Z_P_AI_S(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD1SH_Z_P_BI_S32(operands) => {
            let common::types::LD1SH_Z_P_BI_S32_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1SH_Z_P_BI_S32(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD1SH_Z_P_BR_S32(operands) => {
            let common::types::LD1SH_Z_P_BR_S32_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1SH_Z_P_BR_S32(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD1SH_Z_P_BZ_S_x32_scaled(operands) => {
            let common::types::LD1SH_Z_P_BZ_S_x32_scaled_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1SH_Z_P_BZ_S_x32_scaled(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD1SW_Z_P_AI_D(operands) => {
            let common::types::LD1SW_Z_P_AI_D_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1SW_Z_P_AI_D(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD1SW_Z_P_BI_S64(operands) => {
            let common::types::LD1SW_Z_P_BI_S64_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1SW_Z_P_BI_S64(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD1SW_Z_P_BR_S64(operands) => {
            let common::types::LD1SW_Z_P_BR_S64_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1SW_Z_P_BR_S64(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD1SW_Z_P_BZ_D_x32_scaled(operands) => {
            let common::types::LD1SW_Z_P_BZ_D_x32_scaled_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1SW_Z_P_BZ_D_x32_scaled(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD1W_Z_P_AI_S(operands) => {
            let common::types::LD1W_Z_P_AI_S_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1W_Z_P_AI_S(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD1W_Z_P_BI_U32(operands) => {
            let common::types::LD1W_Z_P_BI_U32_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1W_Z_P_BI_U32(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD1W_Z_P_BR_U32(operands) => {
            let common::types::LD1W_Z_P_BR_U32_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1W_Z_P_BR_U32(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD1W_Z_P_BZ_S_x32_scaled(operands) => {
            let common::types::LD1W_Z_P_BZ_S_x32_scaled_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD1W_Z_P_BZ_S_x32_scaled(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD2B_Z_P_BI_Contiguous(operands) => {
            let common::types::LD2B_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD2B_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD2B_Z_P_BR_Contiguous(operands) => {
            let common::types::LD2B_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD2B_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD2D_Z_P_BI_Contiguous(operands) => {
            let common::types::LD2D_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD2D_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD2D_Z_P_BR_Contiguous(operands) => {
            let common::types::LD2D_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD2D_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD2H_Z_P_BI_Contiguous(operands) => {
            let common::types::LD2H_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD2H_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD2H_Z_P_BR_Contiguous(operands) => {
            let common::types::LD2H_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD2H_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD2W_Z_P_BI_Contiguous(operands) => {
            let common::types::LD2W_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD2W_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD2W_Z_P_BR_Contiguous(operands) => {
            let common::types::LD2W_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD2W_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD3B_Z_P_BI_Contiguous(operands) => {
            let common::types::LD3B_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD3B_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD3B_Z_P_BR_Contiguous(operands) => {
            let common::types::LD3B_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD3B_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD3D_Z_P_BI_Contiguous(operands) => {
            let common::types::LD3D_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD3D_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD3D_Z_P_BR_Contiguous(operands) => {
            let common::types::LD3D_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD3D_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD3H_Z_P_BI_Contiguous(operands) => {
            let common::types::LD3H_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD3H_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD3H_Z_P_BR_Contiguous(operands) => {
            let common::types::LD3H_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD3H_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD3W_Z_P_BI_Contiguous(operands) => {
            let common::types::LD3W_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD3W_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD3W_Z_P_BR_Contiguous(operands) => {
            let common::types::LD3W_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD3W_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD4B_Z_P_BI_Contiguous(operands) => {
            let common::types::LD4B_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD4B_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD4B_Z_P_BR_Contiguous(operands) => {
            let common::types::LD4B_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD4B_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD4D_Z_P_BI_Contiguous(operands) => {
            let common::types::LD4D_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD4D_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD4D_Z_P_BR_Contiguous(operands) => {
            let common::types::LD4D_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD4D_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD4H_Z_P_BI_Contiguous(operands) => {
            let common::types::LD4H_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD4H_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD4H_Z_P_BR_Contiguous(operands) => {
            let common::types::LD4H_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD4H_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD4W_Z_P_BI_Contiguous(operands) => {
            let common::types::LD4W_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD4W_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LD4W_Z_P_BR_Contiguous(operands) => {
            let common::types::LD4W_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LD4W_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LDFF1B_Z_P_AI_S(operands) => {
            let common::types::LDFF1B_Z_P_AI_S_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDFF1B_Z_P_AI_S(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LDFF1B_Z_P_BR_U8(operands) => {
            let common::types::LDFF1B_Z_P_BR_U8_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDFF1B_Z_P_BR_U8(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LDFF1B_Z_P_BZ_D_x32_unscaled(operands) => {
            let common::types::LDFF1B_Z_P_BZ_D_x32_unscaled_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDFF1B_Z_P_BZ_D_x32_unscaled(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LDFF1D_Z_P_AI_D(operands) => {
            let common::types::LDFF1D_Z_P_AI_D_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDFF1D_Z_P_AI_D(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LDFF1D_Z_P_BR_U64(operands) => {
            let common::types::LDFF1D_Z_P_BR_U64_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDFF1D_Z_P_BR_U64(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LDFF1D_Z_P_BZ_D_x32_scaled(operands) => {
            let common::types::LDFF1D_Z_P_BZ_D_x32_scaled_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDFF1D_Z_P_BZ_D_x32_scaled(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LDFF1H_Z_P_AI_S(operands) => {
            let common::types::LDFF1H_Z_P_AI_S_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDFF1H_Z_P_AI_S(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LDFF1H_Z_P_BR_U16(operands) => {
            let common::types::LDFF1H_Z_P_BR_U16_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDFF1H_Z_P_BR_U16(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LDFF1H_Z_P_BZ_S_x32_scaled(operands) => {
            let common::types::LDFF1H_Z_P_BZ_S_x32_scaled_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDFF1H_Z_P_BZ_S_x32_scaled(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LDFF1SB_Z_P_AI_S(operands) => {
            let common::types::LDFF1SB_Z_P_AI_S_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDFF1SB_Z_P_AI_S(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LDFF1SB_Z_P_BR_S16(operands) => {
            let common::types::LDFF1SB_Z_P_BR_S16_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDFF1SB_Z_P_BR_S16(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LDFF1SB_Z_P_BZ_D_x32_unscaled(operands) => {
            let common::types::LDFF1SB_Z_P_BZ_D_x32_unscaled_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDFF1SB_Z_P_BZ_D_x32_unscaled(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LDFF1SH_Z_P_AI_S(operands) => {
            let common::types::LDFF1SH_Z_P_AI_S_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDFF1SH_Z_P_AI_S(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LDFF1SH_Z_P_BR_S32(operands) => {
            let common::types::LDFF1SH_Z_P_BR_S32_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDFF1SH_Z_P_BR_S32(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LDFF1SH_Z_P_BZ_S_x32_scaled(operands) => {
            let common::types::LDFF1SH_Z_P_BZ_S_x32_scaled_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDFF1SH_Z_P_BZ_S_x32_scaled(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LDFF1SW_Z_P_AI_D(operands) => {
            let common::types::LDFF1SW_Z_P_AI_D_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDFF1SW_Z_P_AI_D(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LDFF1SW_Z_P_BR_S64(operands) => {
            let common::types::LDFF1SW_Z_P_BR_S64_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDFF1SW_Z_P_BR_S64(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LDFF1SW_Z_P_BZ_D_x32_scaled(operands) => {
            let common::types::LDFF1SW_Z_P_BZ_D_x32_scaled_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDFF1SW_Z_P_BZ_D_x32_scaled(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LDFF1W_Z_P_AI_S(operands) => {
            let common::types::LDFF1W_Z_P_AI_S_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDFF1W_Z_P_AI_S(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LDFF1W_Z_P_BR_U32(operands) => {
            let common::types::LDFF1W_Z_P_BR_U32_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDFF1W_Z_P_BR_U32(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LDFF1W_Z_P_BZ_S_x32_scaled(operands) => {
            let common::types::LDFF1W_Z_P_BZ_S_x32_scaled_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDFF1W_Z_P_BZ_S_x32_scaled(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LDNF1B_Z_P_BI_U8(operands) => {
            let common::types::LDNF1B_Z_P_BI_U8_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDNF1B_Z_P_BI_U8(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LDNF1D_Z_P_BI_U64(operands) => {
            let common::types::LDNF1D_Z_P_BI_U64_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDNF1D_Z_P_BI_U64(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LDNF1H_Z_P_BI_U16(operands) => {
            let common::types::LDNF1H_Z_P_BI_U16_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDNF1H_Z_P_BI_U16(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LDNF1SB_Z_P_BI_S16(operands) => {
            let common::types::LDNF1SB_Z_P_BI_S16_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDNF1SB_Z_P_BI_S16(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LDNF1SH_Z_P_BI_S32(operands) => {
            let common::types::LDNF1SH_Z_P_BI_S32_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDNF1SH_Z_P_BI_S32(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LDNF1SW_Z_P_BI_S64(operands) => {
            let common::types::LDNF1SW_Z_P_BI_S64_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDNF1SW_Z_P_BI_S64(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LDNF1W_Z_P_BI_U32(operands) => {
            let common::types::LDNF1W_Z_P_BI_U32_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDNF1W_Z_P_BI_U32(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LDNT1B_Z_P_BI_Contiguous(operands) => {
            let common::types::LDNT1B_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDNT1B_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LDNT1B_Z_P_BR_Contiguous(operands) => {
            let common::types::LDNT1B_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDNT1B_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LDNT1D_Z_P_BI_Contiguous(operands) => {
            let common::types::LDNT1D_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDNT1D_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LDNT1D_Z_P_BR_Contiguous(operands) => {
            let common::types::LDNT1D_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDNT1D_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LDNT1H_Z_P_BI_Contiguous(operands) => {
            let common::types::LDNT1H_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDNT1H_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LDNT1H_Z_P_BR_Contiguous(operands) => {
            let common::types::LDNT1H_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDNT1H_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LDNT1W_Z_P_BI_Contiguous(operands) => {
            let common::types::LDNT1W_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDNT1W_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LDNT1W_Z_P_BR_Contiguous(operands) => {
            let common::types::LDNT1W_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDNT1W_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LDR_P_BI__(operands) => {
            let common::types::LDR_P_BI___operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDR_P_BI__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LDR_Z_BI__(operands) => {
            let common::types::LDR_Z_BI___operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_LDR_Z_BI__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LSLR_Z_P_ZZ__(operands) => {
            let common::types::LSLR_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_LSLR_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LSL_Z_P_ZI__(operands) => {
            let common::types::LSL_Z_P_ZI___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_LSL_Z_P_ZI__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LSL_Z_P_ZW__(operands) => {
            let common::types::LSL_Z_P_ZW___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_LSL_Z_P_ZW__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LSL_Z_P_ZZ__(operands) => {
            let common::types::LSL_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_LSL_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LSL_Z_ZI__(operands) => {
            let common::types::LSL_Z_ZI___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_LSL_Z_ZI__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LSL_Z_ZW__(operands) => {
            let common::types::LSL_Z_ZW___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_LSL_Z_ZW__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LSRR_Z_P_ZZ__(operands) => {
            let common::types::LSRR_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_LSRR_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LSR_Z_P_ZI__(operands) => {
            let common::types::LSR_Z_P_ZI___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_LSR_Z_P_ZI__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LSR_Z_P_ZW__(operands) => {
            let common::types::LSR_Z_P_ZW___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_LSR_Z_P_ZW__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LSR_Z_P_ZZ__(operands) => {
            let common::types::LSR_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_LSR_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LSR_Z_ZI__(operands) => {
            let common::types::LSR_Z_ZI___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_LSR_Z_ZI__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::LSR_Z_ZW__(operands) => {
            let common::types::LSR_Z_ZW___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_LSR_Z_ZW__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::MAD_Z_P_ZZZ__(operands) => {
            let common::types::MAD_Z_P_ZZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_MAD_Z_P_ZZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::MLA_Z_P_ZZZ__(operands) => {
            let common::types::MLA_Z_P_ZZZ___operands { n } = *operands;
            lift::generated::lift_blocks::lift_MLA_Z_P_ZZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::MLS_Z_P_ZZZ__(operands) => {
            let common::types::MLS_Z_P_ZZZ___operands { n } = *operands;
            lift::generated::lift_blocks::lift_MLS_Z_P_ZZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::MOVPRFX_Z_P_Z__(operands) => {
            let common::types::MOVPRFX_Z_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_MOVPRFX_Z_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::MOVPRFX_Z_Z__(operands) => {
            let common::types::MOVPRFX_Z_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_MOVPRFX_Z_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::MSB_Z_P_ZZZ__(operands) => {
            let common::types::MSB_Z_P_ZZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_MSB_Z_P_ZZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::MUL_Z_P_ZZ__(operands) => {
            let common::types::MUL_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_MUL_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::MUL_Z_ZI__(operands) => {
            let common::types::MUL_Z_ZI___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_MUL_Z_ZI__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::NAND_P_P_PP_Z(operands) => {
            let common::types::NAND_P_P_PP_Z_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_NAND_P_P_PP_Z(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::NEG_Z_P_Z__(operands) => {
            let common::types::NEG_Z_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_NEG_Z_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::NOR_P_P_PP_Z(operands) => {
            let common::types::NOR_P_P_PP_Z_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_NOR_P_P_PP_Z(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::NOT_Z_P_Z__(operands) => {
            let common::types::NOT_Z_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_NOT_Z_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ORN_P_P_PP_Z(operands) => {
            let common::types::ORN_P_P_PP_Z_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_ORN_P_P_PP_Z(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ORR_P_P_PP_Z(operands) => {
            let common::types::ORR_P_P_PP_Z_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_ORR_P_P_PP_Z(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ORR_Z_P_ZZ__(operands) => {
            let common::types::ORR_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_ORR_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ORR_Z_ZI__(operands) => {
            let common::types::ORR_Z_ZI___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_ORR_Z_ZI__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ORR_Z_ZZ__(operands) => {
            let common::types::ORR_Z_ZZ___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_ORR_Z_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ORV_R_P_Z__(operands) => {
            let common::types::ORV_R_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_ORV_R_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::PFALSE_P__(operands) => {
            let common::types::PFALSE_P___operands { d } = *operands;
            lift::generated::lift_blocks::lift_PFALSE_P__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::PFIRST_P_P_P__(operands) => {
            let common::types::PFIRST_P_P_P___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_PFIRST_P_P_P__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::PNEXT_P_P_P__(operands) => {
            let common::types::PNEXT_P_P_P___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_PNEXT_P_P_P__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::PRFB_I_P_AI_S(operands) => {
            let common::types::PRFB_I_P_AI_S_operands { n } = *operands;
            lift::generated::lift_blocks::lift_PRFB_I_P_AI_S(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::PRFB_I_P_BI_S(operands) => {
            let common::types::PRFB_I_P_BI_S_operands { n } = *operands;
            lift::generated::lift_blocks::lift_PRFB_I_P_BI_S(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::PRFB_I_P_BR_S(operands) => {
            let common::types::PRFB_I_P_BR_S_operands { n } = *operands;
            lift::generated::lift_blocks::lift_PRFB_I_P_BR_S(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::PRFB_I_P_BZ_S_x32_scaled(operands) => {
            let common::types::PRFB_I_P_BZ_S_x32_scaled_operands { n } = *operands;
            lift::generated::lift_blocks::lift_PRFB_I_P_BZ_S_x32_scaled(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::PRFD_I_P_AI_S(operands) => {
            let common::types::PRFD_I_P_AI_S_operands { n } = *operands;
            lift::generated::lift_blocks::lift_PRFD_I_P_AI_S(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::PRFD_I_P_BI_S(operands) => {
            let common::types::PRFD_I_P_BI_S_operands { n } = *operands;
            lift::generated::lift_blocks::lift_PRFD_I_P_BI_S(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::PRFD_I_P_BR_S(operands) => {
            let common::types::PRFD_I_P_BR_S_operands { n } = *operands;
            lift::generated::lift_blocks::lift_PRFD_I_P_BR_S(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::PRFD_I_P_BZ_S_x32_scaled(operands) => {
            let common::types::PRFD_I_P_BZ_S_x32_scaled_operands { n } = *operands;
            lift::generated::lift_blocks::lift_PRFD_I_P_BZ_S_x32_scaled(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::PRFH_I_P_AI_S(operands) => {
            let common::types::PRFH_I_P_AI_S_operands { n } = *operands;
            lift::generated::lift_blocks::lift_PRFH_I_P_AI_S(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::PRFH_I_P_BI_S(operands) => {
            let common::types::PRFH_I_P_BI_S_operands { n } = *operands;
            lift::generated::lift_blocks::lift_PRFH_I_P_BI_S(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::PRFH_I_P_BR_S(operands) => {
            let common::types::PRFH_I_P_BR_S_operands { n } = *operands;
            lift::generated::lift_blocks::lift_PRFH_I_P_BR_S(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::PRFH_I_P_BZ_S_x32_scaled(operands) => {
            let common::types::PRFH_I_P_BZ_S_x32_scaled_operands { n } = *operands;
            lift::generated::lift_blocks::lift_PRFH_I_P_BZ_S_x32_scaled(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::PRFW_I_P_AI_S(operands) => {
            let common::types::PRFW_I_P_AI_S_operands { n } = *operands;
            lift::generated::lift_blocks::lift_PRFW_I_P_AI_S(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::PRFW_I_P_BI_S(operands) => {
            let common::types::PRFW_I_P_BI_S_operands { n } = *operands;
            lift::generated::lift_blocks::lift_PRFW_I_P_BI_S(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::PRFW_I_P_BR_S(operands) => {
            let common::types::PRFW_I_P_BR_S_operands { n } = *operands;
            lift::generated::lift_blocks::lift_PRFW_I_P_BR_S(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::PRFW_I_P_BZ_S_x32_scaled(operands) => {
            let common::types::PRFW_I_P_BZ_S_x32_scaled_operands { n } = *operands;
            lift::generated::lift_blocks::lift_PRFW_I_P_BZ_S_x32_scaled(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::PTEST__P_P__(operands) => {
            let common::types::PTEST__P_P___operands { n } = *operands;
            lift::generated::lift_blocks::lift_PTEST__P_P__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::PTRUE_P_S__(operands) => {
            let common::types::PTRUE_P_S___operands { d } = *operands;
            lift::generated::lift_blocks::lift_PTRUE_P_S__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::PUNPKHI_P_P__(operands) => {
            let common::types::PUNPKHI_P_P___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_PUNPKHI_P_P__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::RBIT_Z_P_Z__(operands) => {
            let common::types::RBIT_Z_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_RBIT_Z_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::RDFFR_P_F__(operands) => {
            let common::types::RDFFR_P_F___operands { d } = *operands;
            lift::generated::lift_blocks::lift_RDFFR_P_F__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::RDFFR_P_P_F__(operands) => {
            let common::types::RDFFR_P_P_F___operands { d } = *operands;
            lift::generated::lift_blocks::lift_RDFFR_P_P_F__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::RDVL_R_I__(operands) => {
            let common::types::RDVL_R_I___operands { d } = *operands;
            lift::generated::lift_blocks::lift_RDVL_R_I__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::REVB_Z_Z__(operands) => {
            let common::types::REVB_Z_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_REVB_Z_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::REV_P_P__(operands) => {
            let common::types::REV_P_P___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_REV_P_P__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::REV_Z_Z__(operands) => {
            let common::types::REV_Z_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_REV_Z_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::SABD_Z_P_ZZ__(operands) => {
            let common::types::SABD_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SABD_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::SADDV_R_P_Z__(operands) => {
            let common::types::SADDV_R_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_SADDV_R_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::SCVTF_Z_P_Z_H2FP16(operands) => {
            let common::types::SCVTF_Z_P_Z_H2FP16_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_SCVTF_Z_P_Z_H2FP16(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::SDIVR_Z_P_ZZ__(operands) => {
            let common::types::SDIVR_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SDIVR_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::SDIV_Z_P_ZZ__(operands) => {
            let common::types::SDIV_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SDIV_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::SDOT_Z_ZZZ__(operands) => {
            let common::types::SDOT_Z_ZZZ___operands { n } = *operands;
            lift::generated::lift_blocks::lift_SDOT_Z_ZZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::SDOT_Z_ZZZi_S(operands) => {
            let common::types::SDOT_Z_ZZZi_S_operands { n } = *operands;
            lift::generated::lift_blocks::lift_SDOT_Z_ZZZi_S(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::SEL_P_P_PP__(operands) => {
            let common::types::SEL_P_P_PP___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_SEL_P_P_PP__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::SEL_Z_P_ZZ__(operands) => {
            let common::types::SEL_Z_P_ZZ___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_SEL_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::SETFFR_F__(operands) => {
            let common::types::SETFFR_F___operands {} = *operands;
            lift::generated::lift_blocks::lift_SETFFR_F__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::SMAXV_R_P_Z__(operands) => {
            let common::types::SMAXV_R_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_SMAXV_R_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::SMAX_Z_P_ZZ__(operands) => {
            let common::types::SMAX_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SMAX_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::SMAX_Z_ZI__(operands) => {
            let common::types::SMAX_Z_ZI___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SMAX_Z_ZI__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::SMINV_R_P_Z__(operands) => {
            let common::types::SMINV_R_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_SMINV_R_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::SMIN_Z_P_ZZ__(operands) => {
            let common::types::SMIN_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SMIN_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::SMIN_Z_ZI__(operands) => {
            let common::types::SMIN_Z_ZI___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SMIN_Z_ZI__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::SMMLA_Z_ZZZ__(operands) => {
            let common::types::SMMLA_Z_ZZZ___operands { n } = *operands;
            lift::generated::lift_blocks::lift_SMMLA_Z_ZZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::SMULH_Z_P_ZZ__(operands) => {
            let common::types::SMULH_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SMULH_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::SPLICE_Z_P_ZZ_Des(operands) => {
            let common::types::SPLICE_Z_P_ZZ_Des_operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SPLICE_Z_P_ZZ_Des(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::SQADD_Z_ZI__(operands) => {
            let common::types::SQADD_Z_ZI___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SQADD_Z_ZI__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::SQADD_Z_ZZ__(operands) => {
            let common::types::SQADD_Z_ZZ___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_SQADD_Z_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::SQDECB_R_RS_SX(operands) => {
            let common::types::SQDECB_R_RS_SX_operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SQDECB_R_RS_SX(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::SQDECD_R_RS_SX(operands) => {
            let common::types::SQDECD_R_RS_SX_operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SQDECD_R_RS_SX(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::SQDECD_Z_ZS__(operands) => {
            let common::types::SQDECD_Z_ZS___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SQDECD_Z_ZS__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::SQDECH_R_RS_SX(operands) => {
            let common::types::SQDECH_R_RS_SX_operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SQDECH_R_RS_SX(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::SQDECH_Z_ZS__(operands) => {
            let common::types::SQDECH_Z_ZS___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SQDECH_Z_ZS__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::SQDECP_R_P_R_SX(operands) => {
            let common::types::SQDECP_R_P_R_SX_operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SQDECP_R_P_R_SX(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::SQDECP_Z_P_Z__(operands) => {
            let common::types::SQDECP_Z_P_Z___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SQDECP_Z_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::SQDECW_R_RS_SX(operands) => {
            let common::types::SQDECW_R_RS_SX_operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SQDECW_R_RS_SX(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::SQDECW_Z_ZS__(operands) => {
            let common::types::SQDECW_Z_ZS___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SQDECW_Z_ZS__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::SQINCB_R_RS_SX(operands) => {
            let common::types::SQINCB_R_RS_SX_operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SQINCB_R_RS_SX(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::SQINCD_R_RS_SX(operands) => {
            let common::types::SQINCD_R_RS_SX_operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SQINCD_R_RS_SX(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::SQINCD_Z_ZS__(operands) => {
            let common::types::SQINCD_Z_ZS___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SQINCD_Z_ZS__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::SQINCH_R_RS_SX(operands) => {
            let common::types::SQINCH_R_RS_SX_operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SQINCH_R_RS_SX(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::SQINCH_Z_ZS__(operands) => {
            let common::types::SQINCH_Z_ZS___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SQINCH_Z_ZS__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::SQINCP_R_P_R_SX(operands) => {
            let common::types::SQINCP_R_P_R_SX_operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SQINCP_R_P_R_SX(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::SQINCP_Z_P_Z__(operands) => {
            let common::types::SQINCP_Z_P_Z___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SQINCP_Z_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::SQINCW_R_RS_SX(operands) => {
            let common::types::SQINCW_R_RS_SX_operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SQINCW_R_RS_SX(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::SQINCW_Z_ZS__(operands) => {
            let common::types::SQINCW_Z_ZS___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SQINCW_Z_ZS__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::SQSUB_Z_ZI__(operands) => {
            let common::types::SQSUB_Z_ZI___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SQSUB_Z_ZI__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::SQSUB_Z_ZZ__(operands) => {
            let common::types::SQSUB_Z_ZZ___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_SQSUB_Z_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ST1B_Z_P_AI_S(operands) => {
            let common::types::ST1B_Z_P_AI_S_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST1B_Z_P_AI_S(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ST1B_Z_P_BI__(operands) => {
            let common::types::ST1B_Z_P_BI___operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST1B_Z_P_BI__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ST1B_Z_P_BR__(operands) => {
            let common::types::ST1B_Z_P_BR___operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST1B_Z_P_BR__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ST1B_Z_P_BZ_D_x32_unscaled(operands) => {
            let common::types::ST1B_Z_P_BZ_D_x32_unscaled_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST1B_Z_P_BZ_D_x32_unscaled(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ST1D_Z_P_AI_D(operands) => {
            let common::types::ST1D_Z_P_AI_D_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST1D_Z_P_AI_D(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ST1D_Z_P_BI__(operands) => {
            let common::types::ST1D_Z_P_BI___operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST1D_Z_P_BI__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ST1D_Z_P_BR__(operands) => {
            let common::types::ST1D_Z_P_BR___operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST1D_Z_P_BR__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ST1D_Z_P_BZ_D_x32_scaled(operands) => {
            let common::types::ST1D_Z_P_BZ_D_x32_scaled_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST1D_Z_P_BZ_D_x32_scaled(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ST1H_Z_P_AI_S(operands) => {
            let common::types::ST1H_Z_P_AI_S_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST1H_Z_P_AI_S(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ST1H_Z_P_BI__(operands) => {
            let common::types::ST1H_Z_P_BI___operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST1H_Z_P_BI__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ST1H_Z_P_BR__(operands) => {
            let common::types::ST1H_Z_P_BR___operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST1H_Z_P_BR__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ST1H_Z_P_BZ_S_x32_scaled(operands) => {
            let common::types::ST1H_Z_P_BZ_S_x32_scaled_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST1H_Z_P_BZ_S_x32_scaled(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ST1W_Z_P_AI_S(operands) => {
            let common::types::ST1W_Z_P_AI_S_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST1W_Z_P_AI_S(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ST1W_Z_P_BI__(operands) => {
            let common::types::ST1W_Z_P_BI___operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST1W_Z_P_BI__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ST1W_Z_P_BR__(operands) => {
            let common::types::ST1W_Z_P_BR___operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST1W_Z_P_BR__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ST1W_Z_P_BZ_S_x32_scaled(operands) => {
            let common::types::ST1W_Z_P_BZ_S_x32_scaled_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST1W_Z_P_BZ_S_x32_scaled(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ST2B_Z_P_BI_Contiguous(operands) => {
            let common::types::ST2B_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST2B_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ST2B_Z_P_BR_Contiguous(operands) => {
            let common::types::ST2B_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST2B_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ST2D_Z_P_BI_Contiguous(operands) => {
            let common::types::ST2D_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST2D_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ST2D_Z_P_BR_Contiguous(operands) => {
            let common::types::ST2D_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST2D_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ST2H_Z_P_BI_Contiguous(operands) => {
            let common::types::ST2H_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST2H_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ST2H_Z_P_BR_Contiguous(operands) => {
            let common::types::ST2H_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST2H_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ST2W_Z_P_BI_Contiguous(operands) => {
            let common::types::ST2W_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST2W_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ST2W_Z_P_BR_Contiguous(operands) => {
            let common::types::ST2W_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST2W_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ST3B_Z_P_BI_Contiguous(operands) => {
            let common::types::ST3B_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST3B_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ST3B_Z_P_BR_Contiguous(operands) => {
            let common::types::ST3B_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST3B_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ST3D_Z_P_BI_Contiguous(operands) => {
            let common::types::ST3D_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST3D_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ST3D_Z_P_BR_Contiguous(operands) => {
            let common::types::ST3D_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST3D_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ST3H_Z_P_BI_Contiguous(operands) => {
            let common::types::ST3H_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST3H_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ST3H_Z_P_BR_Contiguous(operands) => {
            let common::types::ST3H_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST3H_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ST3W_Z_P_BI_Contiguous(operands) => {
            let common::types::ST3W_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST3W_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ST3W_Z_P_BR_Contiguous(operands) => {
            let common::types::ST3W_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST3W_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ST4B_Z_P_BI_Contiguous(operands) => {
            let common::types::ST4B_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST4B_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ST4B_Z_P_BR_Contiguous(operands) => {
            let common::types::ST4B_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST4B_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ST4D_Z_P_BI_Contiguous(operands) => {
            let common::types::ST4D_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST4D_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ST4D_Z_P_BR_Contiguous(operands) => {
            let common::types::ST4D_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST4D_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ST4H_Z_P_BI_Contiguous(operands) => {
            let common::types::ST4H_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST4H_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ST4H_Z_P_BR_Contiguous(operands) => {
            let common::types::ST4H_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST4H_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ST4W_Z_P_BI_Contiguous(operands) => {
            let common::types::ST4W_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST4W_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ST4W_Z_P_BR_Contiguous(operands) => {
            let common::types::ST4W_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_ST4W_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::STNT1B_Z_P_BI_Contiguous(operands) => {
            let common::types::STNT1B_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_STNT1B_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::STNT1B_Z_P_BR_Contiguous(operands) => {
            let common::types::STNT1B_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_STNT1B_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::STNT1D_Z_P_BI_Contiguous(operands) => {
            let common::types::STNT1D_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_STNT1D_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::STNT1D_Z_P_BR_Contiguous(operands) => {
            let common::types::STNT1D_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_STNT1D_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::STNT1H_Z_P_BI_Contiguous(operands) => {
            let common::types::STNT1H_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_STNT1H_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::STNT1H_Z_P_BR_Contiguous(operands) => {
            let common::types::STNT1H_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_STNT1H_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::STNT1W_Z_P_BI_Contiguous(operands) => {
            let common::types::STNT1W_Z_P_BI_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_STNT1W_Z_P_BI_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::STNT1W_Z_P_BR_Contiguous(operands) => {
            let common::types::STNT1W_Z_P_BR_Contiguous_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_STNT1W_Z_P_BR_Contiguous(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::STR_P_BI__(operands) => {
            let common::types::STR_P_BI___operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_STR_P_BI__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::STR_Z_BI__(operands) => {
            let common::types::STR_Z_BI___operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_STR_Z_BI__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::SUBR_Z_P_ZZ__(operands) => {
            let common::types::SUBR_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SUBR_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::SUBR_Z_ZI__(operands) => {
            let common::types::SUBR_Z_ZI___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SUBR_Z_ZI__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::SUB_Z_P_ZZ__(operands) => {
            let common::types::SUB_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SUB_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::SUB_Z_ZI__(operands) => {
            let common::types::SUB_Z_ZI___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_SUB_Z_ZI__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::SUB_Z_ZZ__(operands) => {
            let common::types::SUB_Z_ZZ___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_SUB_Z_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::SUDOT_Z_ZZZi_S(operands) => {
            let common::types::SUDOT_Z_ZZZi_S_operands { n } = *operands;
            lift::generated::lift_blocks::lift_SUDOT_Z_ZZZi_S(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::SUNPKHI_Z_Z__(operands) => {
            let common::types::SUNPKHI_Z_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_SUNPKHI_Z_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::SXTB_Z_P_Z__(operands) => {
            let common::types::SXTB_Z_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_SXTB_Z_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::TBL_Z_ZZ_1(operands) => {
            let common::types::TBL_Z_ZZ_1_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_TBL_Z_ZZ_1(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::TRN1_P_PP__(operands) => {
            let common::types::TRN1_P_PP___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_TRN1_P_PP__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::TRN1_Z_ZZ__(operands) => {
            let common::types::TRN1_Z_ZZ___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_TRN1_Z_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::UABD_Z_P_ZZ__(operands) => {
            let common::types::UABD_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UABD_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::UADDV_R_P_Z__(operands) => {
            let common::types::UADDV_R_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_UADDV_R_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::UCVTF_Z_P_Z_H2FP16(operands) => {
            let common::types::UCVTF_Z_P_Z_H2FP16_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_UCVTF_Z_P_Z_H2FP16(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::UDIVR_Z_P_ZZ__(operands) => {
            let common::types::UDIVR_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UDIVR_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::UDIV_Z_P_ZZ__(operands) => {
            let common::types::UDIV_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UDIV_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::UDOT_Z_ZZZ__(operands) => {
            let common::types::UDOT_Z_ZZZ___operands { n } = *operands;
            lift::generated::lift_blocks::lift_UDOT_Z_ZZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::UDOT_Z_ZZZi_S(operands) => {
            let common::types::UDOT_Z_ZZZi_S_operands { n } = *operands;
            lift::generated::lift_blocks::lift_UDOT_Z_ZZZi_S(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::UMAXV_R_P_Z__(operands) => {
            let common::types::UMAXV_R_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_UMAXV_R_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::UMAX_Z_P_ZZ__(operands) => {
            let common::types::UMAX_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UMAX_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::UMAX_Z_ZI__(operands) => {
            let common::types::UMAX_Z_ZI___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UMAX_Z_ZI__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::UMINV_R_P_Z__(operands) => {
            let common::types::UMINV_R_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_UMINV_R_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::UMIN_Z_P_ZZ__(operands) => {
            let common::types::UMIN_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UMIN_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::UMIN_Z_ZI__(operands) => {
            let common::types::UMIN_Z_ZI___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UMIN_Z_ZI__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::UMMLA_Z_ZZZ__(operands) => {
            let common::types::UMMLA_Z_ZZZ___operands { n } = *operands;
            lift::generated::lift_blocks::lift_UMMLA_Z_ZZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::UMULH_Z_P_ZZ__(operands) => {
            let common::types::UMULH_Z_P_ZZ___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UMULH_Z_P_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::UQADD_Z_ZI__(operands) => {
            let common::types::UQADD_Z_ZI___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UQADD_Z_ZI__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::UQADD_Z_ZZ__(operands) => {
            let common::types::UQADD_Z_ZZ___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_UQADD_Z_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::UQDECB_R_RS_UW(operands) => {
            let common::types::UQDECB_R_RS_UW_operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UQDECB_R_RS_UW(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::UQDECD_R_RS_UW(operands) => {
            let common::types::UQDECD_R_RS_UW_operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UQDECD_R_RS_UW(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::UQDECD_Z_ZS__(operands) => {
            let common::types::UQDECD_Z_ZS___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UQDECD_Z_ZS__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::UQDECH_R_RS_UW(operands) => {
            let common::types::UQDECH_R_RS_UW_operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UQDECH_R_RS_UW(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::UQDECH_Z_ZS__(operands) => {
            let common::types::UQDECH_Z_ZS___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UQDECH_Z_ZS__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::UQDECP_R_P_R_UW(operands) => {
            let common::types::UQDECP_R_P_R_UW_operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UQDECP_R_P_R_UW(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::UQDECP_Z_P_Z__(operands) => {
            let common::types::UQDECP_Z_P_Z___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UQDECP_Z_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::UQDECW_R_RS_UW(operands) => {
            let common::types::UQDECW_R_RS_UW_operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UQDECW_R_RS_UW(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::UQDECW_Z_ZS__(operands) => {
            let common::types::UQDECW_Z_ZS___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UQDECW_Z_ZS__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::UQINCB_R_RS_UW(operands) => {
            let common::types::UQINCB_R_RS_UW_operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UQINCB_R_RS_UW(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::UQINCD_R_RS_UW(operands) => {
            let common::types::UQINCD_R_RS_UW_operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UQINCD_R_RS_UW(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::UQINCD_Z_ZS__(operands) => {
            let common::types::UQINCD_Z_ZS___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UQINCD_Z_ZS__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::UQINCH_R_RS_UW(operands) => {
            let common::types::UQINCH_R_RS_UW_operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UQINCH_R_RS_UW(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::UQINCH_Z_ZS__(operands) => {
            let common::types::UQINCH_Z_ZS___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UQINCH_Z_ZS__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::UQINCP_R_P_R_UW(operands) => {
            let common::types::UQINCP_R_P_R_UW_operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UQINCP_R_P_R_UW(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::UQINCP_Z_P_Z__(operands) => {
            let common::types::UQINCP_Z_P_Z___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UQINCP_Z_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::UQINCW_R_RS_UW(operands) => {
            let common::types::UQINCW_R_RS_UW_operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UQINCW_R_RS_UW(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::UQINCW_Z_ZS__(operands) => {
            let common::types::UQINCW_Z_ZS___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UQINCW_Z_ZS__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::UQSUB_Z_ZI__(operands) => {
            let common::types::UQSUB_Z_ZI___operands { dn } = *operands;
            lift::generated::lift_blocks::lift_UQSUB_Z_ZI__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                dn,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::UQSUB_Z_ZZ__(operands) => {
            let common::types::UQSUB_Z_ZZ___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_UQSUB_Z_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::USDOT_Z_ZZZ_S(operands) => {
            let common::types::USDOT_Z_ZZZ_S_operands { n } = *operands;
            lift::generated::lift_blocks::lift_USDOT_Z_ZZZ_S(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::USDOT_Z_ZZZi_S(operands) => {
            let common::types::USDOT_Z_ZZZi_S_operands { n } = *operands;
            lift::generated::lift_blocks::lift_USDOT_Z_ZZZi_S(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::USMMLA_Z_ZZZ__(operands) => {
            let common::types::USMMLA_Z_ZZZ___operands { n } = *operands;
            lift::generated::lift_blocks::lift_USMMLA_Z_ZZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::UUNPKHI_Z_Z__(operands) => {
            let common::types::UUNPKHI_Z_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_UUNPKHI_Z_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::UXTB_Z_P_Z__(operands) => {
            let common::types::UXTB_Z_P_Z___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_UXTB_Z_P_Z__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::UZP1_P_PP__(operands) => {
            let common::types::UZP1_P_PP___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_UZP1_P_PP__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::UZP1_Z_ZZ__(operands) => {
            let common::types::UZP1_Z_ZZ___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_UZP1_Z_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::WHILELE_P_P_RR__(operands) => {
            let common::types::WHILELE_P_P_RR___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_WHILELE_P_P_RR__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::WHILELO_P_P_RR__(operands) => {
            let common::types::WHILELO_P_P_RR___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_WHILELO_P_P_RR__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::WHILELS_P_P_RR__(operands) => {
            let common::types::WHILELS_P_P_RR___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_WHILELS_P_P_RR__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::WHILELT_P_P_RR__(operands) => {
            let common::types::WHILELT_P_P_RR___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_WHILELT_P_P_RR__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::WRFFR_F_P__(operands) => {
            let common::types::WRFFR_F_P___operands { n } = *operands;
            lift::generated::lift_blocks::lift_WRFFR_F_P__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ZIP2_P_PP__(operands) => {
            let common::types::ZIP2_P_PP___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_ZIP2_P_PP__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::ZIP2_Z_ZZ__(operands) => {
            let common::types::ZIP2_Z_ZZ___operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_ZIP2_Z_ZZ__(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_branch_conditional_compare(operands) => {
            let common::types::aarch64_branch_conditional_compare_operands {
                datasize,
                iszero,
                offset,
                t,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_branch_conditional_compare(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                datasize,
                iszero,
                offset,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_branch_conditional_cond(operands) => {
            let common::types::aarch64_branch_conditional_cond_operands {
                condition,
                offset,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_branch_conditional_cond(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                condition,
                offset,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_branch_conditional_test(operands) => {
            let common::types::aarch64_branch_conditional_test_operands {
                bit_pos,
                bit_val,
                datasize,
                offset,
                t,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_branch_conditional_test(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                bit_pos,
                bit_val,
                datasize,
                offset,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_branch_unconditional_dret(operands) => {
            let common::types::aarch64_branch_unconditional_dret_operands {} = *operands;
            lift::generated::lift_blocks::lift_aarch64_branch_unconditional_dret(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_branch_unconditional_eret(operands) => {
            let common::types::aarch64_branch_unconditional_eret_operands {} = *operands;
            lift::generated::lift_blocks::lift_aarch64_branch_unconditional_eret(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_branch_unconditional_immediate(operands) => {
            let common::types::aarch64_branch_unconditional_immediate_operands {
                branch_type,
                offset,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_branch_unconditional_immediate(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                branch_type,
                offset,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_branch_unconditional_register(operands) => {
            let common::types::aarch64_branch_unconditional_register_operands {
                branch_type,
                m,
                n,
                pac,
                source_is_sp,
                use_key_a,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_branch_unconditional_register(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                branch_type,
                m,
                n,
                pac,
                source_is_sp,
                use_key_a,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_float_arithmetic_add_sub(operands) => {
            let common::types::aarch64_float_arithmetic_add_sub_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_float_arithmetic_add_sub(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_float_arithmetic_div(operands) => {
            let common::types::aarch64_float_arithmetic_div_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_float_arithmetic_div(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_float_arithmetic_max_min(operands) => {
            let common::types::aarch64_float_arithmetic_max_min_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_float_arithmetic_max_min(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_float_arithmetic_mul_add_sub(operands) => {
            let common::types::aarch64_float_arithmetic_mul_add_sub_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_float_arithmetic_mul_add_sub(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_float_arithmetic_mul_product(operands) => {
            let common::types::aarch64_float_arithmetic_mul_product_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_float_arithmetic_mul_product(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_float_arithmetic_round_frint(operands) => {
            let common::types::aarch64_float_arithmetic_round_frint_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_float_arithmetic_round_frint(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_float_arithmetic_round_frint_32_64(
            operands,
        ) => {
            let common::types::aarch64_float_arithmetic_round_frint_32_64_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_float_arithmetic_round_frint_32_64(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_float_arithmetic_unary(operands) => {
            let common::types::aarch64_float_arithmetic_unary_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_float_arithmetic_unary(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_float_compare_cond(operands) => {
            let common::types::aarch64_float_compare_cond_operands { n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_float_compare_cond(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_float_compare_uncond(operands) => {
            let common::types::aarch64_float_compare_uncond_operands { n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_float_compare_uncond(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_float_convert_fix(operands) => {
            let common::types::aarch64_float_convert_fix_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_float_convert_fix(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_float_convert_fp(operands) => {
            let common::types::aarch64_float_convert_fp_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_float_convert_fp(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_float_convert_int(operands) => {
            let common::types::aarch64_float_convert_int_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_float_convert_int(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_float_move_fp_imm(operands) => {
            let common::types::aarch64_float_move_fp_imm_operands { d } = *operands;
            lift::generated::lift_blocks::lift_aarch64_float_move_fp_imm(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_float_move_fp_select(operands) => {
            let common::types::aarch64_float_move_fp_select_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_float_move_fp_select(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_integer_arithmetic_add_sub_carry(
            operands,
        ) => {
            let common::types::aarch64_integer_arithmetic_add_sub_carry_operands {
                d,
                datasize,
                m,
                n,
                setflags,
                sub_op,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_arithmetic_add_sub_carry(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                datasize,
                m,
                n,
                setflags,
                sub_op,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_integer_arithmetic_add_sub_extendedreg(
            operands,
        ) => {
            let common::types::aarch64_integer_arithmetic_add_sub_extendedreg_operands {
                d,
                datasize,
                extend_type,
                m,
                n,
                setflags,
                shift,
                sub_op,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_arithmetic_add_sub_extendedreg(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                datasize,
                extend_type,
                m,
                n,
                setflags,
                shift,
                sub_op,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_integer_arithmetic_add_sub_immediate(
            operands,
        ) => {
            let common::types::aarch64_integer_arithmetic_add_sub_immediate_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_arithmetic_add_sub_immediate(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_integer_arithmetic_add_sub_shiftedreg(
            operands,
        ) => {
            let common::types::aarch64_integer_arithmetic_add_sub_shiftedreg_operands {
                d,
                datasize,
                m,
                n,
                setflags,
                shift_amount,
                shift_type,
                sub_op,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_arithmetic_add_sub_shiftedreg(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                datasize,
                m,
                n,
                setflags,
                shift_amount,
                shift_type,
                sub_op,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_integer_arithmetic_address_pc_rel(
            operands,
        ) => {
            let common::types::aarch64_integer_arithmetic_address_pc_rel_operands {
                d,
                imm,
                page,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_arithmetic_address_pc_rel(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                imm,
                page,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_integer_arithmetic_cnt(operands) => {
            let common::types::aarch64_integer_arithmetic_cnt_operands {
                d,
                datasize,
                n,
                opcode,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_arithmetic_cnt(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                datasize,
                n,
                opcode,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_integer_arithmetic_div(operands) => {
            let common::types::aarch64_integer_arithmetic_div_operands {
                d,
                datasize,
                m,
                n,
                unsigned,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_arithmetic_div(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                datasize,
                m,
                n,
                unsigned,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_integer_arithmetic_mul_uniform_add_sub(
            operands,
        ) => {
            let common::types::aarch64_integer_arithmetic_mul_uniform_add_sub_operands {
                a,
                d,
                datasize,
                destsize,
                m,
                n,
                sub_op,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_arithmetic_mul_uniform_add_sub(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                a,
                d,
                datasize,
                destsize,
                m,
                n,
                sub_op,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_integer_arithmetic_mul_widening_32_64(
            operands,
        ) => {
            let common::types::aarch64_integer_arithmetic_mul_widening_32_64_operands {
                a,
                d,
                datasize,
                destsize,
                m,
                n,
                sub_op,
                unsigned,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_arithmetic_mul_widening_32_64(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                a,
                d,
                datasize,
                destsize,
                m,
                n,
                sub_op,
                unsigned,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_integer_arithmetic_mul_widening_64_128hi(
            operands,
        ) => {
            let common::types::aarch64_integer_arithmetic_mul_widening_64_128hi_operands {
                a,
                d,
                datasize,
                destsize,
                m,
                n,
                unsigned,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_arithmetic_mul_widening_64_128hi(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                a,
                d,
                datasize,
                destsize,
                m,
                n,
                unsigned,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress(
            operands,
        ) => {
            let common::types::aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags(
            operands,
        ) => {
            let common::types::aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_integer_arithmetic_rbit(operands) => {
            let common::types::aarch64_integer_arithmetic_rbit_operands {
                d,
                datasize,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_arithmetic_rbit(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                datasize,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_integer_arithmetic_rev(operands) => {
            let common::types::aarch64_integer_arithmetic_rev_operands {
                container_size,
                d,
                datasize,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_arithmetic_rev(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                container_size,
                d,
                datasize,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_integer_bitfield(operands) => {
            let common::types::aarch64_integer_bitfield_operands {
                R,
                S,
                d,
                datasize,
                extend,
                inzero,
                n,
                tmask,
                wmask,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_bitfield(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                R,
                S,
                d,
                datasize,
                extend,
                inzero,
                n,
                tmask,
                wmask,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_integer_conditional_compare_immediate(
            operands,
        ) => {
            let common::types::aarch64_integer_conditional_compare_immediate_operands {
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_conditional_compare_immediate(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_integer_conditional_compare_register(
            operands,
        ) => {
            let common::types::aarch64_integer_conditional_compare_register_operands {
                condition,
                datasize,
                flags,
                m,
                n,
                sub_op,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_conditional_compare_register(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                condition,
                datasize,
                flags,
                m,
                n,
                sub_op,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_integer_conditional_select(operands) => {
            let common::types::aarch64_integer_conditional_select_operands {
                condition,
                d,
                datasize,
                else_inc,
                else_inv,
                m,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_conditional_select(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                condition,
                d,
                datasize,
                else_inc,
                else_inv,
                m,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_integer_crc(operands) => {
            let common::types::aarch64_integer_crc_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_crc(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_integer_flags_axflag(operands) => {
            let common::types::aarch64_integer_flags_axflag_operands {} = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_flags_axflag(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_integer_flags_cfinv(operands) => {
            let common::types::aarch64_integer_flags_cfinv_operands {} = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_flags_cfinv(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_integer_flags_rmif(operands) => {
            let common::types::aarch64_integer_flags_rmif_operands { n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_flags_rmif(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_integer_flags_setf(operands) => {
            let common::types::aarch64_integer_flags_setf_operands { n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_flags_setf(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_integer_flags_xaflag(operands) => {
            let common::types::aarch64_integer_flags_xaflag_operands {} = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_flags_xaflag(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_integer_ins_ext_extract_immediate(
            operands,
        ) => {
            let common::types::aarch64_integer_ins_ext_extract_immediate_operands {
                d,
                datasize,
                lsb,
                m,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_ins_ext_extract_immediate(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                datasize,
                lsb,
                m,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_integer_ins_ext_insert_movewide(
            operands,
        ) => {
            let common::types::aarch64_integer_ins_ext_insert_movewide_operands {
                d,
                datasize,
                imm,
                opcode,
                pos,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_ins_ext_insert_movewide(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                datasize,
                imm,
                opcode,
                pos,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_integer_logical_immediate(operands) => {
            let common::types::aarch64_integer_logical_immediate_operands {
                d,
                datasize,
                imm,
                n,
                op,
                setflags,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_logical_immediate(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                datasize,
                imm,
                n,
                op,
                setflags,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_integer_logical_shiftedreg(operands) => {
            let common::types::aarch64_integer_logical_shiftedreg_operands {
                d,
                datasize,
                invert,
                m,
                n,
                op,
                setflags,
                shift_amount,
                shift_type,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_logical_shiftedreg(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                datasize,
                invert,
                m,
                n,
                op,
                setflags,
                shift_amount,
                shift_type,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_integer_pac_autda_dp_1src(operands) => {
            let common::types::aarch64_integer_pac_autda_dp_1src_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_pac_autda_dp_1src(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_integer_pac_autdb_dp_1src(operands) => {
            let common::types::aarch64_integer_pac_autdb_dp_1src_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_pac_autdb_dp_1src(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_integer_pac_autia_dp_1src(operands) => {
            let common::types::aarch64_integer_pac_autia_dp_1src_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_pac_autia_dp_1src(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_integer_pac_autib_dp_1src(operands) => {
            let common::types::aarch64_integer_pac_autib_dp_1src_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_pac_autib_dp_1src(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_integer_pac_pacda_dp_1src(operands) => {
            let common::types::aarch64_integer_pac_pacda_dp_1src_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_pac_pacda_dp_1src(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_integer_pac_pacdb_dp_1src(operands) => {
            let common::types::aarch64_integer_pac_pacdb_dp_1src_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_pac_pacdb_dp_1src(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_integer_pac_pacga_dp_2src(operands) => {
            let common::types::aarch64_integer_pac_pacga_dp_2src_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_pac_pacga_dp_2src(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_integer_pac_pacia_dp_1src(operands) => {
            let common::types::aarch64_integer_pac_pacia_dp_1src_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_pac_pacia_dp_1src(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_integer_pac_pacib_dp_1src(operands) => {
            let common::types::aarch64_integer_pac_pacib_dp_1src_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_pac_pacib_dp_1src(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_integer_pac_strip_dp_1src(operands) => {
            let common::types::aarch64_integer_pac_strip_dp_1src_operands { d } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_pac_strip_dp_1src(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_integer_shift_variable(operands) => {
            let common::types::aarch64_integer_shift_variable_operands {
                d,
                datasize,
                m,
                n,
                shift_type,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_shift_variable(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                datasize,
                m,
                n,
                shift_type,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_integer_tags_mcaddtag(operands) => {
            let common::types::aarch64_integer_tags_mcaddtag_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_tags_mcaddtag(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_integer_tags_mcgettag(operands) => {
            let common::types::aarch64_integer_tags_mcgettag_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_tags_mcgettag(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_integer_tags_mcgettagarray(operands) => {
            let common::types::aarch64_integer_tags_mcgettagarray_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_tags_mcgettagarray(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_integer_tags_mcinsertrandomtag(operands) => {
            let common::types::aarch64_integer_tags_mcinsertrandomtag_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_tags_mcinsertrandomtag(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_integer_tags_mcinserttagmask(operands) => {
            let common::types::aarch64_integer_tags_mcinserttagmask_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_tags_mcinserttagmask(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_integer_tags_mcsettaganddatapairpost(
            operands,
        ) => {
            let common::types::aarch64_integer_tags_mcsettaganddatapairpost_operands {
                n,
                t,
                t2,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_tags_mcsettaganddatapairpost(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
                t2,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_integer_tags_mcsettagandzeroarray(
            operands,
        ) => {
            let common::types::aarch64_integer_tags_mcsettagandzeroarray_operands {
                n,
                t,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_tags_mcsettagandzeroarray(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_integer_tags_mcsettagandzerodatapost(
            operands,
        ) => {
            let common::types::aarch64_integer_tags_mcsettagandzerodatapost_operands {
                n,
                t,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_tags_mcsettagandzerodatapost(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_integer_tags_mcsettagarray(operands) => {
            let common::types::aarch64_integer_tags_mcsettagarray_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_tags_mcsettagarray(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_integer_tags_mcsettagpairandzerodatapost(
            operands,
        ) => {
            let common::types::aarch64_integer_tags_mcsettagpairandzerodatapost_operands {
                n,
                t,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_tags_mcsettagpairandzerodatapost(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_integer_tags_mcsettagpairpost(operands) => {
            let common::types::aarch64_integer_tags_mcsettagpairpost_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_tags_mcsettagpairpost(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_integer_tags_mcsettagpost(operands) => {
            let common::types::aarch64_integer_tags_mcsettagpost_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_tags_mcsettagpost(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_integer_tags_mcsubtag(operands) => {
            let common::types::aarch64_integer_tags_mcsubtag_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_integer_tags_mcsubtag(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_memory_atomicops_cas_pair(operands) => {
            let common::types::aarch64_memory_atomicops_cas_pair_operands { n, s, t } = *operands;
            lift::generated::lift_blocks::lift_aarch64_memory_atomicops_cas_pair(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                s,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_memory_atomicops_cas_single(operands) => {
            let common::types::aarch64_memory_atomicops_cas_single_operands {
                n,
                s,
                t,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_memory_atomicops_cas_single(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                s,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_memory_atomicops_ld(operands) => {
            let common::types::aarch64_memory_atomicops_ld_operands { n, s, t } = *operands;
            lift::generated::lift_blocks::lift_aarch64_memory_atomicops_ld(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                s,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_memory_atomicops_swp(operands) => {
            let common::types::aarch64_memory_atomicops_swp_operands { n, s, t } = *operands;
            lift::generated::lift_blocks::lift_aarch64_memory_atomicops_swp(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                s,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_memory_exclusive_pair(operands) => {
            let common::types::aarch64_memory_exclusive_pair_operands { n, s, t, t2 } = *operands;
            lift::generated::lift_blocks::lift_aarch64_memory_exclusive_pair(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                s,
                t,
                t2,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_memory_exclusive_single(operands) => {
            let common::types::aarch64_memory_exclusive_single_operands {
                n,
                s,
                t,
                t2,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_memory_exclusive_single(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                s,
                t,
                t2,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_memory_literal_general(operands) => {
            let common::types::aarch64_memory_literal_general_operands {
                memop,
                offset,
                signed,
                size,
                t,
                tag_checked,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_memory_literal_general(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                memop,
                offset,
                signed,
                size,
                t,
                tag_checked,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_memory_literal_simdfp(operands) => {
            let common::types::aarch64_memory_literal_simdfp_operands { t } = *operands;
            lift::generated::lift_blocks::lift_aarch64_memory_literal_simdfp(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_memory_ordered(operands) => {
            let common::types::aarch64_memory_ordered_operands {
                acctype,
                datasize,
                elsize,
                memop,
                n,
                regsize,
                s,
                t,
                t2,
                tag_checked,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_memory_ordered(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                acctype,
                datasize,
                elsize,
                memop,
                n,
                regsize,
                s,
                t,
                t2,
                tag_checked,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_memory_ordered_rcpc(operands) => {
            let common::types::aarch64_memory_ordered_rcpc_operands { n, s, t } = *operands;
            lift::generated::lift_blocks::lift_aarch64_memory_ordered_rcpc(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                s,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_memory_pair_general_no_alloc(operands) => {
            let common::types::aarch64_memory_pair_general_no_alloc_operands {
                n,
                t,
                t2,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_memory_pair_general_no_alloc(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
                t2,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_memory_pair_general_post_idx(operands) => {
            let common::types::aarch64_memory_pair_general_post_idx_operands {
                acctype,
                datasize,
                memop,
                n,
                offset,
                postindex,
                scale,
                signed,
                t,
                t2,
                tag_checked,
                wback,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_memory_pair_general_post_idx(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                acctype,
                datasize,
                memop,
                n,
                offset,
                postindex,
                scale,
                signed,
                t,
                t2,
                tag_checked,
                wback,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_memory_pair_simdfp_no_alloc(operands) => {
            let common::types::aarch64_memory_pair_simdfp_no_alloc_operands {
                n,
                t,
                t2,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_memory_pair_simdfp_no_alloc(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
                t2,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_memory_pair_simdfp_post_idx(operands) => {
            let common::types::aarch64_memory_pair_simdfp_post_idx_operands {
                n,
                t,
                t2,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_memory_pair_simdfp_post_idx(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
                t2,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_memory_single_general_immediate_signed_offset_lda_stl(
            operands,
        ) => {
            let common::types::aarch64_memory_single_general_immediate_signed_offset_lda_stl_operands {
                n,
                t,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_memory_single_general_immediate_signed_offset_lda_stl(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_memory_single_general_immediate_signed_offset_normal(
            operands,
        ) => {
            let common::types::aarch64_memory_single_general_immediate_signed_offset_normal_operands {
                acctype,
                datasize,
                memop,
                n,
                offset,
                postindex,
                regsize,
                scale,
                signed,
                t,
                tag_checked,
                wback,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_memory_single_general_immediate_signed_offset_normal(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                acctype,
                datasize,
                memop,
                n,
                offset,
                postindex,
                regsize,
                scale,
                signed,
                t,
                tag_checked,
                wback,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_memory_single_general_immediate_signed_offset_unpriv(
            operands,
        ) => {
            let common::types::aarch64_memory_single_general_immediate_signed_offset_unpriv_operands {
                n,
                t,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_memory_single_general_immediate_signed_offset_unpriv(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_memory_single_general_immediate_signed_pac(
            operands,
        ) => {
            let common::types::aarch64_memory_single_general_immediate_signed_pac_operands {
                n,
                t,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_memory_single_general_immediate_signed_pac(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_memory_single_general_immediate_signed_post_idx(
            operands,
        ) => {
            let common::types::aarch64_memory_single_general_immediate_signed_post_idx_operands {
                acctype,
                datasize,
                memop,
                n,
                offset,
                postindex,
                regsize,
                scale,
                signed,
                t,
                tag_checked,
                wback,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_memory_single_general_immediate_signed_post_idx(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                acctype,
                datasize,
                memop,
                n,
                offset,
                postindex,
                regsize,
                scale,
                signed,
                t,
                tag_checked,
                wback,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_memory_single_general_immediate_unsigned(
            operands,
        ) => {
            let common::types::aarch64_memory_single_general_immediate_unsigned_operands {
                n,
                t,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_memory_single_general_immediate_unsigned(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_memory_single_general_register(operands) => {
            let common::types::aarch64_memory_single_general_register_operands {
                acctype,
                datasize,
                extend_type,
                m,
                memop,
                n,
                postindex,
                regsize,
                scale,
                shift,
                signed,
                t,
                tag_checked,
                wback,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_memory_single_general_register(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                acctype,
                datasize,
                extend_type,
                m,
                memop,
                n,
                postindex,
                regsize,
                scale,
                shift,
                signed,
                t,
                tag_checked,
                wback,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_memory_single_simdfp_immediate_signed_offset_normal(
            operands,
        ) => {
            let common::types::aarch64_memory_single_simdfp_immediate_signed_offset_normal_operands {
                n,
                t,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_memory_single_simdfp_immediate_signed_offset_normal(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_memory_single_simdfp_immediate_signed_post_idx(
            operands,
        ) => {
            let common::types::aarch64_memory_single_simdfp_immediate_signed_post_idx_operands {
                n,
                t,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_memory_single_simdfp_immediate_signed_post_idx(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_memory_single_simdfp_register(operands) => {
            let common::types::aarch64_memory_single_simdfp_register_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_aarch64_memory_single_simdfp_register(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_memory_vector_multiple_no_wb(operands) => {
            let common::types::aarch64_memory_vector_multiple_no_wb_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_aarch64_memory_vector_multiple_no_wb(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_memory_vector_single_no_wb(operands) => {
            let common::types::aarch64_memory_vector_single_no_wb_operands { n, t } = *operands;
            lift::generated::lift_blocks::lift_aarch64_memory_vector_single_no_wb(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                n,
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_system_barriers_dmb(operands) => {
            let common::types::aarch64_system_barriers_dmb_operands {} = *operands;
            lift::generated::lift_blocks::lift_aarch64_system_barriers_dmb(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_system_barriers_dsb(operands) => {
            let common::types::aarch64_system_barriers_dsb_operands {} = *operands;
            lift::generated::lift_blocks::lift_aarch64_system_barriers_dsb(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_system_barriers_isb(operands) => {
            let common::types::aarch64_system_barriers_isb_operands {} = *operands;
            lift::generated::lift_blocks::lift_aarch64_system_barriers_isb(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_system_barriers_pssbb(operands) => {
            let common::types::aarch64_system_barriers_pssbb_operands {} = *operands;
            lift::generated::lift_blocks::lift_aarch64_system_barriers_pssbb(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_system_barriers_sb(operands) => {
            let common::types::aarch64_system_barriers_sb_operands {} = *operands;
            lift::generated::lift_blocks::lift_aarch64_system_barriers_sb(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_system_barriers_ssbb(operands) => {
            let common::types::aarch64_system_barriers_ssbb_operands {} = *operands;
            lift::generated::lift_blocks::lift_aarch64_system_barriers_ssbb(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_system_exceptions_debug_breakpoint(
            operands,
        ) => {
            let common::types::aarch64_system_exceptions_debug_breakpoint_operands {} = *operands;
            lift::generated::lift_blocks::lift_aarch64_system_exceptions_debug_breakpoint(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_system_exceptions_debug_exception(
            operands,
        ) => {
            let common::types::aarch64_system_exceptions_debug_exception_operands {} = *operands;
            lift::generated::lift_blocks::lift_aarch64_system_exceptions_debug_exception(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_system_exceptions_debug_halt(operands) => {
            let common::types::aarch64_system_exceptions_debug_halt_operands {} = *operands;
            lift::generated::lift_blocks::lift_aarch64_system_exceptions_debug_halt(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_system_exceptions_runtime_hvc(operands) => {
            let common::types::aarch64_system_exceptions_runtime_hvc_operands {} = *operands;
            lift::generated::lift_blocks::lift_aarch64_system_exceptions_runtime_hvc(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_system_exceptions_runtime_smc(operands) => {
            let common::types::aarch64_system_exceptions_runtime_smc_operands {} = *operands;
            lift::generated::lift_blocks::lift_aarch64_system_exceptions_runtime_smc(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_system_exceptions_runtime_svc(operands) => {
            let common::types::aarch64_system_exceptions_runtime_svc_operands {} = *operands;
            lift::generated::lift_blocks::lift_aarch64_system_exceptions_runtime_svc(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_system_hints(operands) => {
            let common::types::aarch64_system_hints_operands {} = *operands;
            lift::generated::lift_blocks::lift_aarch64_system_hints(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_system_monitors(operands) => {
            let common::types::aarch64_system_monitors_operands {} = *operands;
            lift::generated::lift_blocks::lift_aarch64_system_monitors(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_system_register_cpsr(operands) => {
            let common::types::aarch64_system_register_cpsr_operands {} = *operands;
            lift::generated::lift_blocks::lift_aarch64_system_register_cpsr(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_system_register_system(operands) => {
            let common::types::aarch64_system_register_system_operands { t } = *operands;
            lift::generated::lift_blocks::lift_aarch64_system_register_system(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_system_sysops(operands) => {
            let common::types::aarch64_system_sysops_operands { t } = *operands;
            lift::generated::lift_blocks::lift_aarch64_system_sysops(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                t,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_udf(operands) => {
            let common::types::aarch64_udf_operands {} = *operands;
            lift::generated::lift_blocks::lift_aarch64_udf(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_disparate_add_sub_long(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_disparate_add_sub_long_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_disparate_add_sub_long(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_disparate_add_sub_narrow(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_disparate_add_sub_narrow_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_disparate_add_sub_narrow(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_disparate_add_sub_wide(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_disparate_add_sub_wide_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_disparate_add_sub_wide(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_disparate_diff(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_disparate_diff_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_disparate_diff(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_disparate_mul_accum(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_disparate_mul_accum_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_disparate_mul_accum(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_disparate_mul_dmacc_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_disparate_mul_dmacc_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_disparate_mul_dmacc_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_disparate_mul_double_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_disparate_mul_double_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_disparate_mul_double_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_disparate_mul_poly(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_disparate_mul_poly_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_disparate_mul_poly(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_disparate_mul_product(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_disparate_mul_product_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_disparate_mul_product(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_element_bfdot(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_element_bfdot_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_element_bfdot(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_element_dotp(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_element_dotp_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_element_dotp(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_element_mat_mul_int_dotp(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_element_mat_mul_int_dotp_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_element_mat_mul_int_dotp(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_element_mul_acc_bf16_long(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_element_mul_acc_bf16_long_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_element_mul_acc_bf16_long(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_element_mul_acc_complex(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_element_mul_acc_complex_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_element_mul_acc_complex(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_element_mul_acc_double_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_element_mul_acc_double_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_element_mul_acc_double_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_element_mul_acc_fp16_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_element_mul_acc_fp16_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_element_mul_acc_fp16_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_element_mul_acc_high_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_element_mul_acc_high_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_element_mul_acc_high_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_element_mul_acc_int(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_element_mul_acc_int_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_element_mul_acc_int(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_element_mul_acc_long(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_element_mul_acc_long_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_element_mul_acc_long(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_element_mul_acc_mul_norounding_i_lower(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_element_mul_acc_mul_norounding_i_lower_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_element_mul_acc_mul_norounding_i_lower(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_element_mul_double_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_element_mul_double_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_element_mul_double_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_element_mul_fp16_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_element_mul_fp16_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_element_mul_fp16_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_element_mul_high_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_element_mul_high_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_element_mul_high_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_element_mul_int(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_element_mul_int_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_element_mul_int(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_element_mul_long(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_element_mul_long_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_element_mul_long(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_add_fp16(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_add_fp16_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_add_fp16(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_add_fp_complex(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_add_fp_complex_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_add_fp_complex(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_add_halving_rounding(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_add_halving_truncating(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_diff(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_diff_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_diff(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_div_fp16(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_div_fp16_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_div_fp16(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_logical_and_orr(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_logical_and_orr_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_logical_and_orr(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_logical_bsl_eor(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_logical_bsl_eor_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_logical_bsl_eor(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_mat_mul_int_mla(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_mat_mul_int_mla_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_mat_mul_int_mla(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_mat_mul_int_usdot(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_mat_mul_int_usdot_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_mat_mul_int_usdot(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_max_min_fp16_1985(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_max_min_fp16_1985_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_max_min_fp16_1985(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_max_min_fp16_2008(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_max_min_fp16_2008_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_max_min_fp16_2008(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_max_min_pair(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_max_min_pair_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_max_min_pair(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_max_min_single(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_max_min_single_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_max_min_single(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_mul_acc_bf16_long(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_mul_acc_bf16_long_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_mul_acc_bf16_long(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_mul_fp16_extended_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_mul_fp16_extended_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_mul_fp16_extended_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_mul_fp16_fused(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_mul_fp16_fused_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_mul_fp16_fused(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_mul_fp16_product(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_mul_fp16_product_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_mul_fp16_product(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_mul_fp_complex(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_mul_fp_complex_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_mul_fp_complex(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_mul_fp_mul_norounding_lower(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_mul_fp_mul_norounding_lower_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_mul_fp_mul_norounding_lower(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_mul_int_accum(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_mul_int_accum_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_mul_int_accum(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_mul_int_bfdot(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_mul_int_bfdot_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_mul_int_bfdot(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_mul_int_dotp(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_mul_int_dotp_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_mul_int_dotp(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_accum_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_accum_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_accum_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_mul_int_product(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_mul_int_product_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_mul_int_product(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_recps_fp16_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_recps_fp16_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_recps_fp16_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_rsqrts_fp16_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_rsqrts_fp16_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_rsqrts_fp16_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_shift_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_shift_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_shift_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_sub_int(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_sub_int_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_sub_int(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_add_pairwise(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_unary_add_pairwise_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_add_pairwise(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_add_saturating_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_unary_add_saturating_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_add_saturating_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_clsz(operands) => {
            let common::types::aarch64_vector_arithmetic_unary_clsz_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_clsz(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_cnt(operands) => {
            let common::types::aarch64_vector_arithmetic_unary_cnt_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_cnt(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_diff_neg_fp16(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_unary_diff_neg_fp16_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_diff_neg_fp16(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_diff_neg_int_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_unary_diff_neg_int_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_diff_neg_int_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_diff_neg_sat_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_unary_diff_neg_sat_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_diff_neg_sat_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_extract_nosat(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_unary_extract_nosat_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_extract_nosat(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_extract_sat_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_unary_extract_sat_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_extract_sat_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_extract_sqxtun_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_unary_extract_sqxtun_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_extract_sqxtun_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_float_narrow(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_unary_float_narrow_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_float_narrow(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_float_round_frint_32_64(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_unary_float_round_frint_32_64_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_float_round_frint_32_64(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_float_widen(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_unary_float_widen_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_float_widen(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_float_xtn_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_unary_float_xtn_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_float_xtn_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_fp16_conv_float_tieaway_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_unary_fp16_conv_float_tieaway_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_fp16_conv_float_tieaway_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_fp16_conv_int_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_unary_fp16_conv_int_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_fp16_conv_int_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_fp16_round(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_unary_fp16_round_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_fp16_round(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_not(operands) => {
            let common::types::aarch64_vector_arithmetic_unary_not_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_not(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_rbit(operands) => {
            let common::types::aarch64_vector_arithmetic_unary_rbit_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_rbit(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_rev(operands) => {
            let common::types::aarch64_vector_arithmetic_unary_rev_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_rev(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_shift(operands) => {
            let common::types::aarch64_vector_arithmetic_unary_shift_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_shift(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_special_frecpx_fp16(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_unary_special_frecpx_fp16_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_special_frecpx_fp16(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_special_recip_fp16_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_unary_special_recip_fp16_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_special_recip_fp16_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_special_recip_int(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_unary_special_recip_int_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_special_recip_int(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_special_sqrt_est_fp16_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_unary_special_sqrt_est_fp16_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_special_sqrt_est_fp16_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_special_sqrt_est_int(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_unary_special_sqrt_est_int_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_special_sqrt_est_int(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_arithmetic_unary_special_sqrt_fp16(
            operands,
        ) => {
            let common::types::aarch64_vector_arithmetic_unary_special_sqrt_fp16_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_arithmetic_unary_special_sqrt_fp16(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_bfmmla(operands) => {
            let common::types::aarch64_vector_bfmmla_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_bfmmla(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_crypto_aes_mix(operands) => {
            let common::types::aarch64_vector_crypto_aes_mix_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_aes_mix(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_crypto_aes_round(operands) => {
            let common::types::aarch64_vector_crypto_aes_round_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_aes_round(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_crypto_sha2op_sha1_hash(operands) => {
            let common::types::aarch64_vector_crypto_sha2op_sha1_hash_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_sha2op_sha1_hash(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_crypto_sha2op_sha1_sched1(
            operands,
        ) => {
            let common::types::aarch64_vector_crypto_sha2op_sha1_sched1_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_sha2op_sha1_sched1(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_crypto_sha2op_sha256_sched0(
            operands,
        ) => {
            let common::types::aarch64_vector_crypto_sha2op_sha256_sched0_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_sha2op_sha256_sched0(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_crypto_sha3_bcax(operands) => {
            let common::types::aarch64_vector_crypto_sha3_bcax_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_sha3_bcax(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_crypto_sha3_eor3(operands) => {
            let common::types::aarch64_vector_crypto_sha3_eor3_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_sha3_eor3(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_crypto_sha3_rax1(operands) => {
            let common::types::aarch64_vector_crypto_sha3_rax1_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_sha3_rax1(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_crypto_sha3_xar(operands) => {
            let common::types::aarch64_vector_crypto_sha3_xar_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_sha3_xar(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_crypto_sha3op_sha1_hash_choose(
            operands,
        ) => {
            let common::types::aarch64_vector_crypto_sha3op_sha1_hash_choose_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_sha3op_sha1_hash_choose(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_crypto_sha3op_sha1_hash_majority(
            operands,
        ) => {
            let common::types::aarch64_vector_crypto_sha3op_sha1_hash_majority_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_sha3op_sha1_hash_majority(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_crypto_sha3op_sha1_hash_parity(
            operands,
        ) => {
            let common::types::aarch64_vector_crypto_sha3op_sha1_hash_parity_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_sha3op_sha1_hash_parity(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_crypto_sha3op_sha1_sched0(
            operands,
        ) => {
            let common::types::aarch64_vector_crypto_sha3op_sha1_sched0_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_sha3op_sha1_sched0(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_crypto_sha3op_sha256_hash(
            operands,
        ) => {
            let common::types::aarch64_vector_crypto_sha3op_sha256_hash_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_sha3op_sha256_hash(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_crypto_sha3op_sha256_sched1(
            operands,
        ) => {
            let common::types::aarch64_vector_crypto_sha3op_sha256_sched1_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_sha3op_sha256_sched1(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_crypto_sha512_sha512h(operands) => {
            let common::types::aarch64_vector_crypto_sha512_sha512h_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_sha512_sha512h(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_crypto_sha512_sha512h2(operands) => {
            let common::types::aarch64_vector_crypto_sha512_sha512h2_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_sha512_sha512h2(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_crypto_sha512_sha512su0(operands) => {
            let common::types::aarch64_vector_crypto_sha512_sha512su0_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_sha512_sha512su0(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_crypto_sha512_sha512su1(operands) => {
            let common::types::aarch64_vector_crypto_sha512_sha512su1_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_sha512_sha512su1(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_crypto_sm3_sm3partw1(operands) => {
            let common::types::aarch64_vector_crypto_sm3_sm3partw1_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_sm3_sm3partw1(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_crypto_sm3_sm3partw2(operands) => {
            let common::types::aarch64_vector_crypto_sm3_sm3partw2_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_sm3_sm3partw2(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_crypto_sm3_sm3ss1(operands) => {
            let common::types::aarch64_vector_crypto_sm3_sm3ss1_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_sm3_sm3ss1(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_crypto_sm3_sm3tt1a(operands) => {
            let common::types::aarch64_vector_crypto_sm3_sm3tt1a_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_sm3_sm3tt1a(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_crypto_sm3_sm3tt1b(operands) => {
            let common::types::aarch64_vector_crypto_sm3_sm3tt1b_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_sm3_sm3tt1b(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_crypto_sm3_sm3tt2a(operands) => {
            let common::types::aarch64_vector_crypto_sm3_sm3tt2a_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_sm3_sm3tt2a(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_crypto_sm3_sm3tt2b(operands) => {
            let common::types::aarch64_vector_crypto_sm3_sm3tt2b_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_sm3_sm3tt2b(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_crypto_sm4_sm4enc(operands) => {
            let common::types::aarch64_vector_crypto_sm4_sm4enc_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_sm4_sm4enc(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_crypto_sm4_sm4enckey(operands) => {
            let common::types::aarch64_vector_crypto_sm4_sm4enckey_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_crypto_sm4_sm4enckey(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_cvt_bf16_scalar(operands) => {
            let common::types::aarch64_vector_cvt_bf16_scalar_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_cvt_bf16_scalar(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_cvt_bf16_vector(operands) => {
            let common::types::aarch64_vector_cvt_bf16_vector_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_cvt_bf16_vector(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_fp16_movi(operands) => {
            let common::types::aarch64_vector_fp16_movi_operands {} = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_fp16_movi(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_logical(operands) => {
            let common::types::aarch64_vector_logical_operands {} = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_logical(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_reduce_add_long(operands) => {
            let common::types::aarch64_vector_reduce_add_long_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_reduce_add_long(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_reduce_add_simd(operands) => {
            let common::types::aarch64_vector_reduce_add_simd_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_reduce_add_simd(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_reduce_add_sisd(operands) => {
            let common::types::aarch64_vector_reduce_add_sisd_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_reduce_add_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_reduce_fp16_add_sisd(operands) => {
            let common::types::aarch64_vector_reduce_fp16_add_sisd_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_reduce_fp16_add_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_reduce_fp16_max_simd(operands) => {
            let common::types::aarch64_vector_reduce_fp16_max_simd_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_reduce_fp16_max_simd(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_reduce_fp16_max_sisd(operands) => {
            let common::types::aarch64_vector_reduce_fp16_max_sisd_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_reduce_fp16_max_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_reduce_fp16_maxnm_simd(operands) => {
            let common::types::aarch64_vector_reduce_fp16_maxnm_simd_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_reduce_fp16_maxnm_simd(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_reduce_fp16_maxnm_sisd(operands) => {
            let common::types::aarch64_vector_reduce_fp16_maxnm_sisd_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_reduce_fp16_maxnm_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_reduce_int_max(operands) => {
            let common::types::aarch64_vector_reduce_int_max_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_reduce_int_max(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_shift_conv_float_sisd(operands) => {
            let common::types::aarch64_vector_shift_conv_float_sisd_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_shift_conv_float_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_shift_conv_int_sisd(operands) => {
            let common::types::aarch64_vector_shift_conv_int_sisd_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_shift_conv_int_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_shift_left_insert_sisd(operands) => {
            let common::types::aarch64_vector_shift_left_insert_sisd_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_shift_left_insert_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_shift_left_long(operands) => {
            let common::types::aarch64_vector_shift_left_long_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_shift_left_long(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_shift_left_sat_sisd(operands) => {
            let common::types::aarch64_vector_shift_left_sat_sisd_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_shift_left_sat_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_shift_left_sisd(operands) => {
            let common::types::aarch64_vector_shift_left_sisd_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_shift_left_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_shift_right_insert_sisd(operands) => {
            let common::types::aarch64_vector_shift_right_insert_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_shift_right_insert_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_shift_right_narrow_logical(
            operands,
        ) => {
            let common::types::aarch64_vector_shift_right_narrow_logical_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_shift_right_narrow_logical(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_shift_right_narrow_nonuniform_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_shift_right_narrow_nonuniform_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_shift_right_narrow_nonuniform_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_shift_right_narrow_uniform_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_shift_right_narrow_uniform_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_shift_right_narrow_uniform_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_shift_right_sisd(operands) => {
            let common::types::aarch64_vector_shift_right_sisd_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_shift_right_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_transfer_integer_dup(operands) => {
            let common::types::aarch64_vector_transfer_integer_dup_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_transfer_integer_dup(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_transfer_integer_insert(operands) => {
            let common::types::aarch64_vector_transfer_integer_insert_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_transfer_integer_insert(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_transfer_integer_move_signed(
            operands,
        ) => {
            let common::types::aarch64_vector_transfer_integer_move_signed_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_transfer_integer_move_signed(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_transfer_integer_move_unsigned(
            operands,
        ) => {
            let common::types::aarch64_vector_transfer_integer_move_unsigned_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_transfer_integer_move_unsigned(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_transfer_vector_cpy_dup_sisd(
            operands,
        ) => {
            let common::types::aarch64_vector_transfer_vector_cpy_dup_sisd_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_transfer_vector_cpy_dup_sisd(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_transfer_vector_extract(operands) => {
            let common::types::aarch64_vector_transfer_vector_extract_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_transfer_vector_extract(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_transfer_vector_insert(operands) => {
            let common::types::aarch64_vector_transfer_vector_insert_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_transfer_vector_insert(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_transfer_vector_permute_transpose(
            operands,
        ) => {
            let common::types::aarch64_vector_transfer_vector_permute_transpose_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_transfer_vector_permute_transpose(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_transfer_vector_permute_unzip(
            operands,
        ) => {
            let common::types::aarch64_vector_transfer_vector_permute_unzip_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_transfer_vector_permute_unzip(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_transfer_vector_permute_zip(
            operands,
        ) => {
            let common::types::aarch64_vector_transfer_vector_permute_zip_operands {
                d,
                n,
            } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_transfer_vector_permute_zip(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        common::types::Instruction::NOP => {}
        common::types::Instruction::UNPRED
        | common::types::Instruction::UNALLOC
        | common::types::Instruction::UNDEF => {
            builder.trap();
        }
        common::types::Instruction::aarch64_vector_transfer_vector_table(operands) => {
            let common::types::aarch64_vector_transfer_vector_table_operands { d, n } = *operands;
            lift::generated::lift_blocks::lift_aarch64_vector_transfer_vector_table(
                builder,
                sequencer,
                lift::types::Variable::from(
                    common::types::bits::new(address as u128, 64),
                ),
                d,
                n,
            )?;
        }
        _ => return Err(AArch64LifterError::UnspecifiedInstruction),
    };
    Ok(())
}

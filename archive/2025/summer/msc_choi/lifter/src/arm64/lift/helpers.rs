#![allow(nonstandard_style, unused)]

use crate::arm64::common::types::{bits, boolean, integer, ArchVersion, BranchType, Dummy};
use crate::arm64::lift::types::{AirVariable, BlockSequencer, ExceptionRecord, RustVariable, Variable};
use crate::arm64::{common, decode, lift, AArch64LifterError};
use crate::{integer_to_i128, integer_to_u128, integer_to_u64, integer_to_u8, integer_to_usize};
use num_traits::{ToPrimitive, Zero};
use tnj::air::instructions::builder::InstructionBuilder;
use tnj::arch::reg::Reg;
use tnj::types::cmp::CmpTy;
use tnj::types::Type;

pub use crate::arm64::lift::generated::helpers::*;

pub fn eq_enum(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
) -> Result<Variable, AArch64LifterError> {
    match &x {
        Variable::Air(xa) => match &y {
            Variable::Air(ya) => {
                let ty = Type::new_fixed_int(8).ok_or(AArch64LifterError::InvalidBitsLength)?;
                Ok(Variable::new_air(builder.icmp(CmpTy::Eq, xa.val, ya.val, ty).into(), ty))
            }
            Variable::Rust(_) => {
                let ty = Type::new_fixed_int(8).ok_or(AArch64LifterError::InvalidBitsLength)?;
                let ya = y.promote_to_air(builder)?.to_air()?;
                Ok(Variable::new_air(builder.icmp(CmpTy::Eq, xa.val, ya.val, ty).into(), ty))
            }
            _ => panic!("Variable not enum"),
        },
        Variable::Rust(_) => match y {
            Variable::Air(ya) => {
                let ty = Type::new_fixed_int(8).ok_or(AArch64LifterError::InvalidBitsLength)?;
                let xa = x.promote_to_air(builder)?.to_air()?;
                Ok(Variable::new_air(builder.icmp(CmpTy::Eq, xa.val, ya.val, ty).into(), ty))
            }
            Variable::Rust(_) => Ok(if x == y {
                Variable::from(boolean::TRUE)
            } else {
                Variable::from(boolean::FALSE)
            }),
            _ => panic!("Variable not enum"),
        },
        _ => panic!("Variable not enum"),
    }
}

pub fn ne_enum(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
) -> Result<Variable, AArch64LifterError> {
    match &x {
        Variable::Air(xa) => match &y {
            Variable::Air(ya) => {
                let ty = Type::new_fixed_int(8).ok_or(AArch64LifterError::InvalidBitsLength)?;
                Ok(Variable::new_air(builder.icmp(CmpTy::Ne, xa.val, ya.val, ty).into(), ty))
            }
            Variable::Rust(_) => {
                let ty = Type::new_fixed_int(8).ok_or(AArch64LifterError::InvalidBitsLength)?;
                let ya = y.promote_to_air(builder)?.to_air()?;
                Ok(Variable::new_air(builder.icmp(CmpTy::Ne, xa.val, ya.val, ty).into(), ty))
            }
            _ => panic!("Variable not enum"),
        },
        Variable::Rust(_) => match y {
            Variable::Air(ya) => {
                let ty = Type::new_fixed_int(8).ok_or(AArch64LifterError::InvalidBitsLength)?;
                let xa = x.promote_to_air(builder)?.to_air()?;
                Ok(Variable::new_air(builder.icmp(CmpTy::Ne, xa.val, ya.val, ty).into(), ty))
            }
            Variable::Rust(_) => Ok(if x != y {
                Variable::from(boolean::TRUE)
            } else {
                Variable::from(boolean::FALSE)
            }),
            _ => panic!("Variable not enum"),
        },
        _ => panic!("Variable not enum"),
    }
}

pub fn eq_enum_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
) -> Result<Variable, AArch64LifterError> {
    eq_bool_0(builder, sequencer, pc, x, y)
}

pub fn ne_enum_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
) -> Result<Variable, AArch64LifterError> {
    ne_bool_0(builder, sequencer, pc, x, y)
}

pub fn eq_enum_1(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
) -> Result<Variable, AArch64LifterError> {
    eq_enum(builder, sequencer, pc, x, y)
}

pub fn ne_enum_1(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
) -> Result<Variable, AArch64LifterError> {
    ne_enum(builder, sequencer, pc, x, y)
}

pub fn eq_enum_6(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
) -> Result<Variable, AArch64LifterError> {
    eq_enum(builder, sequencer, pc, x, y)
}

pub fn ne_enum_6(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
) -> Result<Variable, AArch64LifterError> {
    ne_enum(builder, sequencer, pc, x, y)
}

pub fn eq_enum_7(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
) -> Result<Variable, AArch64LifterError> {
    eq_enum(builder, sequencer, pc, x, y)
}

pub fn ne_enum_7(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
) -> Result<Variable, AArch64LifterError> {
    ne_enum(builder, sequencer, pc, x, y)
}

pub fn eq_enum_8(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
) -> Result<Variable, AArch64LifterError> {
    eq_enum(builder, sequencer, pc, x, y)
}

pub fn ne_enum_8(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
) -> Result<Variable, AArch64LifterError> {
    ne_enum(builder, sequencer, pc, x, y)
}

pub fn eq_enum_13(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
) -> Result<Variable, AArch64LifterError> {
    eq_enum(builder, sequencer, pc, x, y)
}

pub fn ne_enum_13(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
) -> Result<Variable, AArch64LifterError> {
    ne_enum(builder, sequencer, pc, x, y)
}

pub fn eq_enum_14(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
) -> Result<Variable, AArch64LifterError> {
    eq_enum(builder, sequencer, pc, x, y)
}

pub fn ne_enum_14(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
) -> Result<Variable, AArch64LifterError> {
    ne_enum(builder, sequencer, pc, x, y)
}

pub fn eq_enum_15(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
) -> Result<Variable, AArch64LifterError> {
    eq_enum(builder, sequencer, pc, x, y)
}

pub fn ne_enum_15(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
) -> Result<Variable, AArch64LifterError> {
    ne_enum(builder, sequencer, pc, x, y)
}

pub fn eq_enum_16(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
) -> Result<Variable, AArch64LifterError> {
    eq_enum(builder, sequencer, pc, x, y)
}

pub fn ne_enum_16(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
) -> Result<Variable, AArch64LifterError> {
    ne_enum(builder, sequencer, pc, x, y)
}

pub fn eq_enum_17(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
) -> Result<Variable, AArch64LifterError> {
    eq_enum(builder, sequencer, pc, x, y)
}

pub fn ne_enum_17(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
) -> Result<Variable, AArch64LifterError> {
    ne_enum(builder, sequencer, pc, x, y)
}

pub fn eq_enum_18(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
) -> Result<Variable, AArch64LifterError> {
    eq_enum(builder, sequencer, pc, x, y)
}

pub fn ne_enum_18(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
) -> Result<Variable, AArch64LifterError> {
    ne_enum(builder, sequencer, pc, x, y)
}

pub fn eq_enum_23(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
) -> Result<Variable, AArch64LifterError> {
    eq_enum(builder, sequencer, pc, x, y)
}

pub fn ne_enum_23(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
) -> Result<Variable, AArch64LifterError> {
    ne_enum(builder, sequencer, pc, x, y)
}

pub fn eq_enum_34(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
) -> Result<Variable, AArch64LifterError> {
    eq_enum(builder, sequencer, pc, x, y)
}

pub fn ne_enum_34(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
) -> Result<Variable, AArch64LifterError> {
    ne_enum(builder, sequencer, pc, x, y)
}

pub fn eq_enum_37(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
) -> Result<Variable, AArch64LifterError> {
    eq_enum(builder, sequencer, pc, x, y)
}

pub fn ne_enum_37(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
) -> Result<Variable, AArch64LifterError> {
    ne_enum(builder, sequencer, pc, x, y)
}

pub fn eq_enum_49(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
) -> Result<Variable, AArch64LifterError> {
    eq_enum(builder, sequencer, pc, x, y)
}

pub fn ne_enum_49(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
) -> Result<Variable, AArch64LifterError> {
    ne_enum(builder, sequencer, pc, x, y)
}

pub fn eq_bool_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
) -> Result<Variable, AArch64LifterError> {
    match x {
        Variable::Air(xa) => match y {
            Variable::Air(ya) => Ok(Variable::new_air(builder.icmp(CmpTy::Eq, xa.val, ya.val, Type::Bool).into(), Type::Bool)),
            Variable::Rust(RustVariable::boolean(yb)) => {
                let ya = AirVariable::from_boolean(builder, yb)?;
                Ok(Variable::new_air(builder.icmp(CmpTy::Eq, xa.val, ya.val, Type::Bool).into(), Type::Bool))
            }
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        },
        Variable::Rust(RustVariable::boolean(xb)) => match y {
            Variable::Air(ya) => {
                let xa = AirVariable::from_boolean(builder, xb)?;
                Ok(Variable::new_air(builder.icmp(CmpTy::Eq, xa.val, ya.val, Type::Bool).into(), Type::Bool))
            }
            Variable::Rust(RustVariable::boolean(yb)) => Ok(common::helpers::eq_bool_0(xb, yb)?.into()),
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        },
        _ => Err(AArch64LifterError::VariableNotExpectedEnum),
    }
}

pub fn ne_bool_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
) -> Result<Variable, AArch64LifterError> {
    match x {
        Variable::Air(xa) => match y {
            Variable::Air(ya) => Ok(Variable::new_air(builder.icmp(CmpTy::Ne, xa.val, ya.val, Type::Bool).into(), Type::Bool)),
            Variable::Rust(RustVariable::boolean(yb)) => {
                let ya = AirVariable::from_boolean(builder, yb)?;
                Ok(Variable::new_air(builder.icmp(CmpTy::Ne, xa.val, ya.val, Type::Bool).into(), Type::Bool))
            }
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        },
        Variable::Rust(RustVariable::boolean(xb)) => match y {
            Variable::Air(ya) => {
                let xa = AirVariable::from_boolean(builder, xb)?;
                Ok(Variable::new_air(builder.icmp(CmpTy::Ne, xa.val, ya.val, Type::Bool).into(), Type::Bool))
            }
            Variable::Rust(RustVariable::boolean(yb)) => Ok(common::helpers::ne_bool_0(xb, yb)?.into()),
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        },
        _ => Err(AArch64LifterError::VariableNotExpectedEnum),
    }
}

pub fn not_bool_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
) -> Result<Variable, AArch64LifterError> {
    match x {
        Variable::Air(xa) => Ok(Variable::new_air(builder.bitwise_not(xa.val, Type::Bool).into(), Type::Bool)),
        Variable::Rust(RustVariable::boolean(xb)) => Ok(common::helpers::not_bool_0(xb)?.into()),
        _ => Err(AArch64LifterError::VariableNotExpectedEnum),
    }
}

pub fn and_bool_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
) -> Result<Variable, AArch64LifterError> {
    match x {
        Variable::Air(xa) => match y {
            Variable::Air(ya) => Ok(Variable::new_air(builder.and(xa.val, ya.val, Type::Bool).into(), Type::Bool)),
            Variable::Rust(RustVariable::boolean(yb)) => {
                if yb == boolean::FALSE {
                    return Ok(Variable::from(boolean::FALSE));
                }
                let ya = AirVariable::from_boolean(builder, yb)?;
                Ok(Variable::new_air(builder.and(xa.val, ya.val, Type::Bool).into(), Type::Bool))
            }
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        },
        Variable::Rust(RustVariable::boolean(xb)) => match y {
            Variable::Air(ya) => {
                if xb == boolean::FALSE {
                    return Ok(Variable::from(boolean::FALSE));
                }
                let xa = AirVariable::from_boolean(builder, xb)?;
                Ok(Variable::new_air(builder.and(xa.val, ya.val, Type::Bool).into(), Type::Bool))
            }
            Variable::Rust(RustVariable::boolean(yb)) => Ok(common::helpers::and_bool_0(xb, yb)?.into()),
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        },
        _ => Err(AArch64LifterError::VariableNotExpectedEnum),
    }
}

pub fn or_bool_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
) -> Result<Variable, AArch64LifterError> {
    match x {
        Variable::Air(xa) => match y {
            Variable::Air(ya) => Ok(Variable::new_air(builder.or(xa.val, ya.val, Type::Bool).into(), Type::Bool)),
            Variable::Rust(RustVariable::boolean(yb)) => {
                if yb == boolean::TRUE {
                    return Ok(Variable::from(boolean::TRUE));
                }
                let ya = AirVariable::from_boolean(builder, yb)?;
                Ok(Variable::new_air(builder.or(xa.val, ya.val, Type::Bool).into(), Type::Bool))
            }
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        },
        Variable::Rust(RustVariable::boolean(xb)) => match y {
            Variable::Air(ya) => {
                if xb == boolean::TRUE {
                    return Ok(Variable::from(boolean::TRUE));
                }
                let xa = AirVariable::from_boolean(builder, xb)?;
                Ok(Variable::new_air(builder.or(xa.val, ya.val, Type::Bool).into(), Type::Bool))
            }
            Variable::Rust(RustVariable::boolean(yb)) => Ok(common::helpers::or_bool_0(xb, yb)?.into()),
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        },
        _ => Err(AArch64LifterError::VariableNotExpectedEnum),
    }
}

pub fn equiv_bool_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
) -> Result<Variable, AArch64LifterError> {
    match x {
        Variable::Air(xa) => match y {
            Variable::Air(ya) => Ok(Variable::new_air(builder.icmp(CmpTy::Eq, xa.val, ya.val, Type::Bool).into(), Type::Bool)),
            Variable::Rust(RustVariable::boolean(yb)) => {
                let ya = AirVariable::from_boolean(builder, yb)?;
                Ok(Variable::new_air(builder.icmp(CmpTy::Eq, xa.val, ya.val, Type::Bool).into(), Type::Bool))
            }
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        },
        Variable::Rust(RustVariable::boolean(xb)) => match y {
            Variable::Air(ya) => {
                let xa = AirVariable::from_boolean(builder, xb)?;
                Ok(Variable::new_air(builder.icmp(CmpTy::Eq, xa.val, ya.val, Type::Bool).into(), Type::Bool))
            }
            Variable::Rust(RustVariable::boolean(yb)) => Ok(common::helpers::equiv_bool_0(xb, yb)?.into()),
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        },
        _ => Err(AArch64LifterError::VariableNotExpectedEnum),
    }
}

pub fn implies_bool_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
) -> Result<Variable, AArch64LifterError> {
    match x {
        Variable::Air(xa) => match y {
            Variable::Air(ya) => {
                let not_x = builder.bitwise_not(xa.val, Type::Bool);
                Ok(Variable::new_air(builder.or(not_x, ya.val, Type::Bool).into(), Type::Bool))
            }
            Variable::Rust(RustVariable::boolean(yb)) => {
                let ya = AirVariable::from_boolean(builder, yb)?;
                let not_x = builder.bitwise_not(xa.val, Type::Bool);
                Ok(Variable::new_air(builder.or(not_x, ya.val, Type::Bool).into(), Type::Bool))
            }
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        },
        Variable::Rust(RustVariable::boolean(xb)) => match y {
            Variable::Air(ya) => {
                let xa = AirVariable::from_boolean(builder, xb)?;
                let not_x = builder.bitwise_not(xa.val, Type::Bool);
                Ok(Variable::new_air(builder.or(not_x, ya.val, Type::Bool).into(), Type::Bool))
            }
            Variable::Rust(RustVariable::boolean(yb)) => Ok(common::helpers::implies_bool_0(xb, yb)?.into()),
            _ => Err(AArch64LifterError::VariableNotExpectedEnum),
        },
        _ => Err(AArch64LifterError::VariableNotExpectedEnum),
    }
}

pub fn eq_int_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
) -> Result<Variable, AArch64LifterError> {
    match x {
        Variable::Air(xa) => match y {
            Variable::Air(ya) => Ok(Variable::new_air(builder.icmp(CmpTy::Eq, xa.val, ya.val, Type::Int).into(), Type::Int)),
            Variable::Rust(RustVariable::integer(yi)) => {
                let ya = AirVariable::from_integer(builder, yi)?;
                Ok(Variable::new_air(builder.icmp(CmpTy::Eq, xa.val, ya.val, Type::Int).into(), Type::Int))
            }
            _ => panic!("Variable not integer"),
        },
        Variable::Rust(RustVariable::integer(xi)) => match y {
            Variable::Air(ya) => {
                let xa = AirVariable::from_integer(builder, xi)?;
                Ok(Variable::new_air(builder.icmp(CmpTy::Eq, xa.val, ya.val, Type::Int).into(), Type::Int))
            }
            Variable::Rust(RustVariable::integer(yi)) => Ok(common::helpers::eq_int_0(xi, yi)?.into()),
            _ => panic!("Variable not integer"),
        },
        _ => panic!("Variable not integer"),
    }
}

pub fn ne_int_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
) -> Result<Variable, AArch64LifterError> {
    match x {
        Variable::Air(xa) => match y {
            Variable::Air(ya) => Ok(Variable::new_air(builder.icmp(CmpTy::Ne, xa.val, ya.val, Type::Int).into(), Type::Int)),
            Variable::Rust(RustVariable::integer(yi)) => {
                let ya = AirVariable::from_integer(builder, yi)?;
                Ok(Variable::new_air(builder.icmp(CmpTy::Ne, xa.val, ya.val, Type::Int).into(), Type::Int))
            }
            _ => panic!("Variable not integer"),
        },
        Variable::Rust(RustVariable::integer(xi)) => match y {
            Variable::Air(ya) => {
                let xa = AirVariable::from_integer(builder, xi)?;
                Ok(Variable::new_air(builder.icmp(CmpTy::Ne, xa.val, ya.val, Type::Int).into(), Type::Int))
            }
            Variable::Rust(RustVariable::integer(yi)) => Ok(common::helpers::ne_int_0(xi, yi)?.into()),
            _ => panic!("Variable not integer"),
        },
        _ => panic!("Variable not integer"),
    }
}

pub fn gt_int_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
) -> Result<Variable, AArch64LifterError> {
    match x {
        Variable::Air(xa) => match y {
            Variable::Air(ya) => Ok(Variable::new_air(builder.icmp(CmpTy::Gt, xa.val, ya.val, Type::Int).into(), Type::Int)),
            Variable::Rust(RustVariable::integer(yi)) => {
                let ya = AirVariable::from_integer(builder, yi)?;
                Ok(Variable::new_air(builder.icmp(CmpTy::Gt, xa.val, ya.val, Type::Int).into(), Type::Int))
            }
            _ => panic!("Variable not integer"),
        },
        Variable::Rust(RustVariable::integer(xi)) => match y {
            Variable::Air(ya) => {
                let xa = AirVariable::from_integer(builder, xi)?;
                Ok(Variable::new_air(builder.icmp(CmpTy::Gt, xa.val, ya.val, Type::Int).into(), Type::Int))
            }
            Variable::Rust(RustVariable::integer(yi)) => Ok(common::helpers::gt_int_0(xi, yi)?.into()),
            _ => panic!("Variable not integer"),
        },
        _ => panic!("Variable not integer"),
    }
}

pub fn ge_int_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
) -> Result<Variable, AArch64LifterError> {
    match x {
        Variable::Air(xa) => match y {
            Variable::Air(ya) => Ok(Variable::new_air(builder.icmp(CmpTy::Ge, xa.val, ya.val, Type::Int).into(), Type::Int)),
            Variable::Rust(RustVariable::integer(yi)) => {
                let ya = AirVariable::from_integer(builder, yi)?;
                Ok(Variable::new_air(builder.icmp(CmpTy::Ge, xa.val, ya.val, Type::Int).into(), Type::Int))
            }
            _ => panic!("Variable not integer"),
        },
        Variable::Rust(RustVariable::integer(xi)) => match y {
            Variable::Air(ya) => {
                let xa = AirVariable::from_integer(builder, xi)?;
                Ok(Variable::new_air(builder.icmp(CmpTy::Ge, xa.val, ya.val, Type::Int).into(), Type::Int))
            }
            Variable::Rust(RustVariable::integer(yi)) => Ok(common::helpers::ge_int_0(xi, yi)?.into()),
            _ => panic!("Variable not integer"),
        },
        _ => panic!("Variable not integer"),
    }
}

pub fn le_int_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
) -> Result<Variable, AArch64LifterError> {
    match x {
        Variable::Air(xa) => match y {
            Variable::Air(ya) => Ok(Variable::new_air(builder.icmp(CmpTy::Le, xa.val, ya.val, Type::Int).into(), Type::Int)),
            Variable::Rust(RustVariable::integer(yi)) => {
                let ya = AirVariable::from_integer(builder, yi)?;
                Ok(Variable::new_air(builder.icmp(CmpTy::Le, xa.val, ya.val, Type::Int).into(), Type::Int))
            }
            _ => panic!("Variable not integer"),
        },
        Variable::Rust(RustVariable::integer(xi)) => match y {
            Variable::Air(ya) => {
                let xa = AirVariable::from_integer(builder, xi)?;
                Ok(Variable::new_air(builder.icmp(CmpTy::Le, xa.val, ya.val, Type::Int).into(), Type::Int))
            }
            Variable::Rust(RustVariable::integer(yi)) => Ok(common::helpers::le_int_0(xi, yi)?.into()),
            _ => panic!("Variable not integer"),
        },
        _ => panic!("Variable not integer"),
    }
}

pub fn lt_int_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
) -> Result<Variable, AArch64LifterError> {
    match x {
        Variable::Air(xa) => match y {
            Variable::Air(ya) => Ok(Variable::new_air(builder.icmp(CmpTy::Lt, xa.val, ya.val, Type::Int).into(), Type::Int)),
            Variable::Rust(RustVariable::integer(yi)) => {
                let ya = AirVariable::from_integer(builder, yi)?;
                Ok(Variable::new_air(builder.icmp(CmpTy::Lt, xa.val, ya.val, Type::Int).into(), Type::Int))
            }
            _ => panic!("Variable not integer"),
        },
        Variable::Rust(RustVariable::integer(xi)) => match y {
            Variable::Air(ya) => {
                let xa = AirVariable::from_integer(builder, xi)?;
                Ok(Variable::new_air(builder.icmp(CmpTy::Lt, xa.val, ya.val, Type::Int).into(), Type::Int))
            }
            Variable::Rust(RustVariable::integer(yi)) => Ok(common::helpers::lt_int_0(xi, yi)?.into()),
            _ => panic!("Variable not integer"),
        },
        _ => panic!("Variable not integer"),
    }
}

pub fn add_int_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
) -> Result<Variable, AArch64LifterError> {
    match x {
        Variable::Air(xa) => match y {
            Variable::Air(ya) => Ok(Variable::new_air(builder.add(xa.val, ya.val, Type::Int).into(), Type::Int)),
            Variable::Rust(RustVariable::integer(yi)) => {
                let ya = AirVariable::from_integer(builder, yi)?;
                Ok(Variable::new_air(builder.add(xa.val, ya.val, Type::Int).into(), Type::Int))
            }
            _ => panic!("Variable not integer"),
        },
        Variable::Rust(RustVariable::integer(xi)) => match y {
            Variable::Air(ya) => {
                let xa = AirVariable::from_integer(builder, xi)?;
                Ok(Variable::new_air(builder.add(xa.val, ya.val, Type::Int).into(), Type::Int))
            }
            Variable::Rust(RustVariable::integer(yi)) => Ok(common::helpers::add_int_0(xi, yi)?.into()),
            _ => panic!("Variable not integer"),
        },
        _ => panic!("Variable not integer"),
    }
}

pub fn neg_int_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
) -> Result<Variable, AArch64LifterError> {
    match x {
        Variable::Air(a) => {
            let zero = builder.iconst(0);
            Ok(Variable::new_air(builder.sub(zero, a.val, a.ty).into(), a.ty))
        }
        Variable::Rust(RustVariable::integer(i)) => Ok(common::helpers::neg_int_0(i)?.into()),
        _ => panic!("Variable not integer"),
    }
}

pub fn sub_int_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
) -> Result<Variable, AArch64LifterError> {
    match x {
        Variable::Air(xa) => match y {
            Variable::Air(ya) => Ok(Variable::new_air(builder.sub(xa.val, ya.val, Type::Int).into(), Type::Int)),
            Variable::Rust(RustVariable::integer(yi)) => {
                let ya = AirVariable::from_integer(builder, yi)?;
                Ok(Variable::new_air(builder.sub(xa.val, ya.val, Type::Int).into(), Type::Int))
            }
            _ => panic!("Variable not integer"),
        },
        Variable::Rust(RustVariable::integer(xi)) => match y {
            Variable::Air(ya) => {
                let xa = AirVariable::from_integer(builder, xi)?;
                Ok(Variable::new_air(builder.sub(xa.val, ya.val, Type::Int).into(), Type::Int))
            }
            Variable::Rust(RustVariable::integer(yi)) => Ok(common::helpers::sub_int_0(xi, yi)?.into()),
            _ => panic!("Variable not integer"),
        },
        _ => panic!("Variable not integer"),
    }
}

pub fn mul_int_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
) -> Result<Variable, AArch64LifterError> {
    match x {
        Variable::Air(xa) => match y {
            Variable::Air(ya) => Ok(Variable::new_air(builder.mul(xa.val, ya.val, Type::Int).into(), Type::Int)),
            Variable::Rust(RustVariable::integer(yi)) => {
                let ya = AirVariable::from_integer(builder, yi)?;
                Ok(Variable::new_air(builder.mul(xa.val, ya.val, Type::Int).into(), Type::Int))
            }
            _ => panic!("Variable not integer"),
        },
        Variable::Rust(RustVariable::integer(xi)) => match y {
            Variable::Air(ya) => {
                let xa = AirVariable::from_integer(builder, xi)?;
                Ok(Variable::new_air(builder.mul(xa.val, ya.val, Type::Int).into(), Type::Int))
            }
            Variable::Rust(RustVariable::integer(yi)) => Ok(common::helpers::mul_int_0(xi, yi)?.into()),
            _ => panic!("Variable not integer"),
        },
        _ => panic!("Variable not integer"),
    }
}

pub fn zdiv_int_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
) -> Result<Variable, AArch64LifterError> {
    match x {
        Variable::Air(xa) => match y {
            Variable::Air(ya) => Ok(Variable::new_air(builder.div(xa.val, ya.val, Type::Int).into(), Type::Int)),
            Variable::Rust(RustVariable::integer(yi)) => {
                let ya = AirVariable::from_integer(builder, yi)?;
                Ok(Variable::new_air(builder.div(xa.val, ya.val, Type::Int).into(), Type::Int))
            }
            _ => panic!("Variable not integer"),
        },
        Variable::Rust(RustVariable::integer(xi)) => match y {
            Variable::Air(ya) => {
                let xa = AirVariable::from_integer(builder, xi)?;
                Ok(Variable::new_air(builder.div(xa.val, ya.val, Type::Int).into(), Type::Int))
            }
            Variable::Rust(RustVariable::integer(yi)) => Ok(common::helpers::zdiv_int_0(xi, yi)?.into()),
            _ => panic!("Variable not integer"),
        },
        _ => panic!("Variable not integer"),
    }
}

pub fn fdiv_int_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
) -> Result<Variable, AArch64LifterError> {
    match x {
        Variable::Air(xa) => match y {
            Variable::Air(ya) => Err(AArch64LifterError::NotImplemented(file!(), line!())),
            Variable::Rust(RustVariable::integer(yi)) => Err(AArch64LifterError::NotImplemented(file!(), line!())),
            _ => panic!("Variable not integer"),
        },
        Variable::Rust(RustVariable::integer(xi)) => match y {
            Variable::Air(ya) => Err(AArch64LifterError::NotImplemented(file!(), line!())),
            Variable::Rust(RustVariable::integer(yi)) => Ok(common::helpers::fdiv_int_0(xi, yi)?.into()),
            _ => panic!("Variable not integer"),
        },
        _ => panic!("Variable not integer"),
    }
}

pub fn frem_int_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
) -> Result<Variable, AArch64LifterError> {
    match x {
        Variable::Air(xa) => match y {
            Variable::Air(ya) => Ok(Variable::new_air(builder.modulo(xa.val, ya.val, Type::Int).into(), Type::Int)),
            Variable::Rust(RustVariable::integer(yi)) => {
                let ya = AirVariable::from_integer(builder, yi)?;
                Ok(Variable::new_air(builder.modulo(xa.val, ya.val, Type::Int).into(), Type::Int))
            }
            _ => panic!("Variable not integer"),
        },
        Variable::Rust(RustVariable::integer(xi)) => match y {
            Variable::Air(ya) => {
                let xa = AirVariable::from_integer(builder, xi)?;
                Ok(Variable::new_air(builder.modulo(xa.val, ya.val, Type::Int).into(), Type::Int))
            }
            Variable::Rust(RustVariable::integer(yi)) => Ok(common::helpers::frem_int_0(xi, yi)?.into()),
            _ => panic!("Variable not integer"),
        },
        _ => panic!("Variable not integer"),
    }
}

pub fn align_int_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
) -> Result<Variable, AArch64LifterError> {
    match x {
        Variable::Air(xa) => match y {
            Variable::Air(ya) => {
                let rem = builder.modulo(xa.val, ya.val, Type::Int);
                Ok(Variable::new_air(builder.sub(xa.val, rem, Type::Int).into(), Type::Int))
            }
            Variable::Rust(RustVariable::integer(yi)) => {
                let ya = AirVariable::from_integer(builder, yi)?;
                let rem = builder.modulo(xa.val, ya.val, Type::Int);
                Ok(Variable::new_air(builder.sub(xa.val, rem, Type::Int).into(), Type::Int))
            }
            _ => panic!("Variable not integer"),
        },
        Variable::Rust(RustVariable::integer(xi)) => match y {
            Variable::Air(ya) => {
                let xa = AirVariable::from_integer(builder, xi)?;
                let rem = builder.modulo(xa.val, ya.val, Type::Int);
                Ok(Variable::new_air(builder.sub(xa.val, rem, Type::Int).into(), Type::Int))
            }
            Variable::Rust(RustVariable::integer(yi)) => Ok(common::helpers::align_int_0(xi, yi)?.into()),
            _ => panic!("Variable not integer"),
        },
        _ => panic!("Variable not integer"),
    }
}

pub fn cvt_int_real_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
) -> Result<Variable, AArch64LifterError> {
    Ok(x)
}

pub fn divide_real_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
) -> Result<Variable, AArch64LifterError> {
    zdiv_int_0(builder, sequencer, pc, x, y)
}

pub fn round_tozero_real_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
) -> Result<Variable, AArch64LifterError> {
    Ok(x)
}

pub fn cvt_int_bits_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    N: Variable,
    _N: Variable,
) -> Result<Variable, AArch64LifterError> {
    let size = N.to_integer()?;
    match x {
        Variable::Air(a) => {
            let ty = Type::new_fixed_int(integer_to_u8!(size)).ok_or(AArch64LifterError::InvalidBitsLength)?;
            Ok(Variable::new_air(builder.to_bits(a.val, ty).into(), ty))
        }
        Variable::Rust(RustVariable::integer(i)) => Ok(common::helpers::cvt_int_bits_0(i, size)?.into()),
        _ => panic!("Variable not integer"),
    }
}

pub fn cvt_bits_sint_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    N: Variable,
) -> Result<Variable, AArch64LifterError> {
    match x {
        Variable::Air(a) => Ok(Variable::new_air(builder.signed_from_bits(a.val, a.ty).into(), a.ty)),
        Variable::Rust(RustVariable::bits(b)) => Ok(common::helpers::cvt_bits_sint_0(b)?.into()),
        _ => panic!("Variable not bits"),
    }
}

pub fn cvt_bits_uint_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    N: Variable,
) -> Result<Variable, AArch64LifterError> {
    match x {
        Variable::Air(a) => Ok(Variable::new_air(builder.unsigned_from_bits(a.val, a.ty).into(), a.ty)),
        Variable::Rust(RustVariable::bits(b)) => Ok(common::helpers::cvt_bits_uint_0(b)?.into()),
        _ => panic!("Variable not bits"),
    }
}

pub fn eq_bits_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
    N: Variable,
) -> Result<Variable, AArch64LifterError> {
    match x {
        Variable::Air(xa) => match y {
            Variable::Air(ya) => Ok(Variable::new_air(builder.icmp(CmpTy::Eq, xa.val, ya.val, xa.ty).into(), xa.ty)),
            Variable::Rust(RustVariable::bits(yb)) => {
                let ya = AirVariable::from_bits(builder, yb)?;
                Ok(Variable::new_air(builder.icmp(CmpTy::Eq, xa.val, ya.val, xa.ty).into(), xa.ty))
            }
            _ => panic!("Variable not bits"),
        },
        Variable::Rust(RustVariable::bits(xb)) => match y {
            Variable::Air(ya) => {
                let xa = AirVariable::from_bits(builder, xb)?;
                Ok(Variable::new_air(builder.icmp(CmpTy::Eq, xa.val, ya.val, xa.ty).into(), xa.ty))
            }
            Variable::Rust(RustVariable::bits(yb)) => Ok(common::helpers::eq_bits_0(xb, yb, N.to_integer()?)?.into()),
            _ => panic!("Variable not bits"),
        },
        _ => panic!("Variable not bits"),
    }
}

pub fn ne_bits_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
    N: Variable,
) -> Result<Variable, AArch64LifterError> {
    match x {
        Variable::Air(xa) => match y {
            Variable::Air(ya) => Ok(Variable::new_air(builder.icmp(CmpTy::Ne, xa.val, ya.val, xa.ty).into(), xa.ty)),
            Variable::Rust(RustVariable::bits(yb)) => {
                let ya = AirVariable::from_bits(builder, yb)?;
                Ok(Variable::new_air(builder.icmp(CmpTy::Ne, xa.val, ya.val, xa.ty).into(), xa.ty))
            }
            _ => panic!("Variable not bits"),
        },
        Variable::Rust(RustVariable::bits(xb)) => match y {
            Variable::Air(ya) => {
                let xa = AirVariable::from_bits(builder, xb)?;
                Ok(Variable::new_air(builder.icmp(CmpTy::Ne, xa.val, ya.val, xa.ty).into(), xa.ty))
            }
            Variable::Rust(RustVariable::bits(yb)) => Ok(common::helpers::ne_bits_0(xb, yb, N.to_integer()?)?.into()),
            _ => panic!("Variable not bits"),
        },
        _ => panic!("Variable not bits"),
    }
}

pub fn add_bits_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
    N: Variable,
) -> Result<Variable, AArch64LifterError> {
    match x {
        Variable::Air(xa) => match y {
            Variable::Air(ya) => Ok(Variable::new_air(builder.wrapping_add(xa.val, ya.val, xa.ty).into(), xa.ty)),
            Variable::Rust(RustVariable::bits(yb)) => {
                let ya = AirVariable::from_bits(builder, yb)?;
                Ok(Variable::new_air(builder.wrapping_add(xa.val, ya.val, xa.ty).into(), xa.ty))
            }
            _ => panic!("Variable not bits"),
        },
        Variable::Rust(RustVariable::bits(xb)) => match y {
            Variable::Air(ya) => {
                let xa = AirVariable::from_bits(builder, xb)?;
                Ok(Variable::new_air(builder.wrapping_add(xa.val, ya.val, xa.ty).into(), xa.ty))
            }
            Variable::Rust(RustVariable::bits(yb)) => Ok(common::helpers::add_bits_0(xb, yb, N.to_integer()?)?.into()),
            _ => panic!("Variable not bits"),
        },
        _ => panic!("Variable not bits"),
    }
}

pub fn sub_bits_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
    N: Variable,
) -> Result<Variable, AArch64LifterError> {
    match x {
        Variable::Air(xa) => match y {
            Variable::Air(ya) => Ok(Variable::new_air(builder.wrapping_sub(xa.val, ya.val, xa.ty).into(), xa.ty)),
            Variable::Rust(RustVariable::bits(yb)) => {
                let ya = AirVariable::from_bits(builder, yb)?;
                Ok(Variable::new_air(builder.wrapping_sub(xa.val, ya.val, xa.ty).into(), xa.ty))
            }
            _ => panic!("Variable not bits"),
        },
        Variable::Rust(RustVariable::bits(xb)) => match y {
            Variable::Air(ya) => {
                let xa = AirVariable::from_bits(builder, xb)?;
                Ok(Variable::new_air(builder.wrapping_sub(xa.val, ya.val, xa.ty).into(), xa.ty))
            }
            Variable::Rust(RustVariable::bits(yb)) => Ok(common::helpers::sub_bits_0(xb, yb, N.to_integer()?)?.into()),
            _ => panic!("Variable not bits"),
        },
        _ => panic!("Variable not bits"),
    }
}

pub fn mul_bits_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
    N: Variable,
) -> Result<Variable, AArch64LifterError> {
    match x {
        Variable::Air(xa) => match y {
            Variable::Air(ya) => Ok(Variable::new_air(builder.umul(xa.val, ya.val, xa.ty).into(), xa.ty)),
            Variable::Rust(RustVariable::bits(yb)) => {
                let ya = AirVariable::from_bits(builder, yb)?;
                Ok(Variable::new_air(builder.umul(xa.val, ya.val, xa.ty).into(), xa.ty))
            }
            _ => panic!("Variable not bits"),
        },
        Variable::Rust(RustVariable::bits(xb)) => match y {
            Variable::Air(ya) => {
                let xa = AirVariable::from_bits(builder, xb)?;
                Ok(Variable::new_air(builder.umul(xa.val, ya.val, xa.ty).into(), xa.ty))
            }
            Variable::Rust(RustVariable::bits(yb)) => Ok(common::helpers::mul_bits_0(xb, yb, N.to_integer()?)?.into()),
            _ => panic!("Variable not bits"),
        },
        _ => panic!("Variable not bits"),
    }
}

pub fn and_bits_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
    N: Variable,
) -> Result<Variable, AArch64LifterError> {
    match x {
        Variable::Air(xa) => match y {
            Variable::Air(ya) => Ok(Variable::new_air(builder.and(xa.val, ya.val, xa.ty).into(), xa.ty)),
            Variable::Rust(RustVariable::bits(yb)) => {
                let ya = AirVariable::from_bits(builder, yb)?;
                Ok(Variable::new_air(builder.and(xa.val, ya.val, xa.ty).into(), xa.ty))
            }
            _ => panic!("Variable not bits"),
        },
        Variable::Rust(RustVariable::bits(xb)) => match y {
            Variable::Air(ya) => {
                let xa = AirVariable::from_bits(builder, xb)?;
                Ok(Variable::new_air(builder.and(xa.val, ya.val, xa.ty).into(), xa.ty))
            }
            Variable::Rust(RustVariable::bits(yb)) => Ok(common::helpers::and_bits_0(xb, yb, N.to_integer()?)?.into()),
            _ => panic!("Variable not bits"),
        },
        _ => panic!("Variable not bits"),
    }
}

pub fn or_bits_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
    N: Variable,
) -> Result<Variable, AArch64LifterError> {
    match x {
        Variable::Air(xa) => match y {
            Variable::Air(ya) => Ok(Variable::new_air(builder.or(xa.val, ya.val, xa.ty).into(), xa.ty)),
            Variable::Rust(RustVariable::bits(yb)) => {
                let ya = AirVariable::from_bits(builder, yb)?;
                Ok(Variable::new_air(builder.or(xa.val, ya.val, xa.ty).into(), xa.ty))
            }
            _ => panic!("Variable not bits"),
        },
        Variable::Rust(RustVariable::bits(xb)) => match y {
            Variable::Air(ya) => {
                let xa = AirVariable::from_bits(builder, xb)?;
                Ok(Variable::new_air(builder.or(xa.val, ya.val, xa.ty).into(), xa.ty))
            }
            Variable::Rust(RustVariable::bits(yb)) => Ok(common::helpers::or_bits_0(xb, yb, N.to_integer()?)?.into()),
            _ => panic!("Variable not bits"),
        },
        _ => panic!("Variable not bits"),
    }
}

pub fn eor_bits_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
    N: Variable,
) -> Result<Variable, AArch64LifterError> {
    match x {
        Variable::Air(xa) => match y {
            Variable::Air(ya) => Ok(Variable::new_air(builder.xor(xa.val, ya.val, xa.ty).into(), xa.ty)),
            Variable::Rust(RustVariable::bits(yb)) => {
                let ya = AirVariable::from_bits(builder, yb)?;
                Ok(Variable::new_air(builder.xor(xa.val, ya.val, xa.ty).into(), xa.ty))
            }
            _ => panic!("Variable not bits"),
        },
        Variable::Rust(RustVariable::bits(xb)) => match y {
            Variable::Air(ya) => {
                let xa = AirVariable::from_bits(builder, xb)?;
                Ok(Variable::new_air(builder.xor(xa.val, ya.val, xa.ty).into(), xa.ty))
            }
            Variable::Rust(RustVariable::bits(yb)) => Ok(common::helpers::eor_bits_0(xb, yb, N.to_integer()?)?.into()),
            _ => panic!("Variable not bits"),
        },
        _ => panic!("Variable not bits"),
    }
}

pub fn not_bits_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    N: Variable,
) -> Result<Variable, AArch64LifterError> {
    match x {
        Variable::Air(a) => Ok(Variable::new_air(builder.bitwise_not(a.val, a.ty).into(), a.ty)),
        Variable::Rust(RustVariable::bits(b)) => Ok(common::helpers::not_bits_0(b)?.into()),
        _ => panic!("Variable not bits"),
    }
}

pub fn zeros_bits_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    N: Variable,
) -> Result<Variable, AArch64LifterError> {
    Ok(common::helpers::zeros_bits_0(N.to_integer()?)?.into())
}

pub fn ones_bits_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    N: Variable,
) -> Result<Variable, AArch64LifterError> {
    Ok(common::helpers::ones_bits_0(N.to_integer()?)?.into())
}

pub fn replicate_bits_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    N: Variable,
    M: Variable,
    _N: Variable,
) -> Result<Variable, AArch64LifterError> {
    match x {
        Variable::Air(a) => {
            let mut result = x.clone();
            let size = integer_to_usize!(M.to_integer()?);
            match N {
                Variable::Rust(RustVariable::integer(n)) => {
                    let rep = integer_to_usize!(n);
                    for i in 0..rep {
                        result = append_bits_0(
                            builder,
                            sequencer,
                            pc.clone(),
                            result,
                            x.clone(),
                            Variable::from(integer::from(size * (i + 1))),
                            M.clone(),
                        )?;
                    }
                    Ok(result)
                }
                Variable::Air(n) => Err(AArch64LifterError::NotImplemented(file!(), line!())),
                _ => panic!("Variable not integer"),
            }
        }
        Variable::Rust(RustVariable::bits(b)) => Ok(common::helpers::replicate_bits_0(b, N.to_integer()?)?.into()),
        _ => panic!("Variable not bits"),
    }
}

pub fn append_bits_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    y: Variable,
    M: Variable,
    N: Variable,
) -> Result<Variable, AArch64LifterError> {
    if M.to_integer()? == integer::zero() {
        return Ok(y);
    }
    if N.to_integer()? == integer::zero() {
        return Ok(x);
    }

    match x {
        Variable::Air(xa) => match y {
            Variable::Air(ya) => {
                let x_size = xa.ty.bit_width().ok_or(AArch64LifterError::InvalidAirType)?;
                let y_size = ya.ty.bit_width().ok_or(AArch64LifterError::InvalidAirType)?;
                let new_ty = Type::new_fixed_int(x_size + y_size).ok_or(AArch64LifterError::InvalidBitsLength)?;
                let x_ext = builder.zext(xa.val, xa.ty, new_ty);
                let sh_amt = builder.iconst(y_size);
                let x_shl = builder.lshl(x_ext, sh_amt, new_ty);
                let y_ext = builder.zext(ya.val, ya.ty, new_ty);
                Ok(Variable::new_air(builder.or(x_shl, y_ext, new_ty).into(), new_ty))
            }
            Variable::Rust(RustVariable::bits(yb)) => {
                let ya = AirVariable::from_bits(builder, yb)?;
                let x_size = xa.ty.bit_width().ok_or(AArch64LifterError::InvalidAirType)?;
                let y_size = ya.ty.bit_width().ok_or(AArch64LifterError::InvalidAirType)?;
                let new_ty = Type::new_fixed_int(x_size + y_size).ok_or(AArch64LifterError::InvalidBitsLength)?;
                let x_ext = builder.zext(xa.val, xa.ty, new_ty);
                let sh_amt = builder.iconst(y_size);
                let x_shl = builder.lshl(x_ext, sh_amt, new_ty);
                let y_ext = builder.zext(ya.val, ya.ty, new_ty);
                Ok(Variable::new_air(builder.or(x_shl, y_ext, new_ty).into(), new_ty))
            }
            _ => panic!("Variable not bits"),
        },
        Variable::Rust(RustVariable::bits(xb)) => match y {
            Variable::Air(ya) => {
                let xa = AirVariable::from_bits(builder, xb)?;
                let x_size = xa.ty.bit_width().ok_or(AArch64LifterError::InvalidAirType)?;
                let y_size = ya.ty.bit_width().ok_or(AArch64LifterError::InvalidAirType)?;
                let new_ty = Type::new_fixed_int(x_size + y_size).ok_or(AArch64LifterError::InvalidBitsLength)?;
                let x_ext = builder.zext(xa.val, xa.ty, new_ty);
                let sh_amt = builder.iconst(y_size);
                let x_shl = builder.lshl(x_ext, sh_amt, new_ty);
                let y_ext = builder.zext(ya.val, ya.ty, new_ty);
                Ok(Variable::new_air(builder.or(x_shl, y_ext, new_ty).into(), new_ty))
            }
            Variable::Rust(RustVariable::bits(yb)) => Ok(common::helpers::append_bits_0(xb, yb)?.into()),
            _ => panic!("Variable not bits"),
        },
        _ => panic!("Variable not bits"),
    }
}

pub fn ram_read_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    A: Variable,
    N: Variable,
    ram: Dummy,
    address: Variable,
    _A: Variable,
    _N: Variable,
) -> Result<Variable, AArch64LifterError> {
    let ty = Type::new_fixed_int(8 * integer_to_u8!(N.to_integer()?)).ok_or(AArch64LifterError::InvalidBitsLength)?;
    let address_in_air = address.promote_to_air(builder)?.to_air()?;
    let loaded = builder.load(address_in_air.val, ty);

    Ok(Variable::new_air(loaded.into(), ty))
}

pub fn ram_write_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    A: Variable,
    N: Variable,
    ram: Dummy,
    address: Variable,
    val: Variable,
    _A: Variable,
    _N: Variable,
) -> Result<(), AArch64LifterError> {
    let ty = Type::new_fixed_int(8 * integer_to_u8!(N.to_integer()?)).ok_or(AArch64LifterError::InvalidBitsLength)?;
    let address_in_air = address.promote_to_air(builder)?.to_air()?;
    let val_in_air = val.promote_to_air(builder)?.to_air()?;
    builder.store(val_in_air.val, address_in_air.val, ty);

    Ok(())
}

pub fn print_str_0(builder: &mut InstructionBuilder, sequencer: &mut BlockSequencer, pc: Variable, x: String) -> Result<(), AArch64LifterError> {
    // println!("{}", x);

    Ok(())
}

pub fn print_char_0(builder: &mut InstructionBuilder, sequencer: &mut BlockSequencer, pc: Variable, x: Variable) -> Result<(), AArch64LifterError> {
    // println!("{:?}", x);

    Ok(())
}

pub fn program_end_0(builder: &mut InstructionBuilder, sequencer: &mut BlockSequencer, pc: Variable) -> Result<(), AArch64LifterError> {
    Ok(())
}

pub fn SetTagCheckedInstruction_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    checked: Variable,
) -> Result<(), AArch64LifterError> {
    Ok(())
}

pub fn ASR_C_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    shift: Variable,
    N: Variable,
) -> Result<(Variable, Variable), AArch64LifterError> {
    match x {
        Variable::Air(xa) => match shift {
            Variable::Air(sa) => {
                let result = Variable::new_air(builder.ashr(xa.val, sa.val, xa.ty).into(), xa.ty);
                let one = builder.iconst(1);
                let index = builder.sub(sa.val, one, Type::Int);
                let shifted = builder.lshr(xa.val, index, xa.ty);
                let carry_out_ty = Type::new_fixed_int(1).ok_or(AArch64LifterError::InvalidBitsLength)?;
                let carry_out = Variable::new_air(builder.trunc(shifted, xa.ty, carry_out_ty).into(), carry_out_ty);
                Ok((result, carry_out))
            }
            Variable::Rust(RustVariable::integer(si)) => {
                let sa = AirVariable::from_integer(builder, si)?;
                let result = Variable::new_air(builder.ashr(xa.val, sa.val, xa.ty).into(), xa.ty);
                let one = builder.iconst(1);
                let index = builder.sub(sa.val, one, Type::Int);
                let shifted = builder.lshr(xa.val, index, xa.ty);
                let carry_out_ty = Type::new_fixed_int(1).ok_or(AArch64LifterError::InvalidBitsLength)?;
                let carry_out = Variable::new_air(builder.trunc(shifted, xa.ty, carry_out_ty).into(), carry_out_ty);
                Ok((result, carry_out))
            }
            _ => panic!("Variable not integer"),
        },
        Variable::Rust(RustVariable::bits(xb)) => match shift {
            Variable::Air(sa) => {
                let xa = AirVariable::from_bits(builder, xb)?;
                let result = Variable::new_air(builder.ashr(xa.val, sa.val, xa.ty).into(), xa.ty);
                let one = builder.iconst(1);
                let index = builder.sub(sa.val, one, Type::Int);
                let shifted = builder.lshr(xa.val, index, xa.ty);
                let carry_out_ty = Type::new_fixed_int(1).ok_or(AArch64LifterError::InvalidBitsLength)?;
                let carry_out = Variable::new_air(builder.trunc(shifted, xa.ty, carry_out_ty).into(), carry_out_ty);
                Ok((result, carry_out))
            }
            Variable::Rust(RustVariable::integer(si)) => {
                assert!(si > integer::zero());
                let extended_x = decode::helpers::SignExtend_0(xb, &si + N.to_integer()?, integer::from(xb.length), &si + N.to_integer()?)?;
                let result = extended_x.extract_slice(integer_to_usize!(si), integer_to_usize!(N.to_integer()?))?;
                let carry_out = extended_x.extract_slice(integer_to_usize!(si) - 1, 1)?;
                Ok((result.into(), carry_out.into()))
            }
            _ => panic!("Variable not integer"),
        },
        _ => panic!("Variable not bits"),
    }
}

pub fn LSR_C_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    shift: Variable,
    N: Variable,
) -> Result<(Variable, Variable), AArch64LifterError> {
    match x {
        Variable::Air(xa) => match shift {
            Variable::Air(sa) => {
                let result = Variable::new_air(builder.lshr(xa.val, sa.val, xa.ty).into(), xa.ty);
                let one = builder.iconst(1);
                let index = builder.sub(sa.val, one, Type::Int);
                let shifted = builder.lshr(xa.val, index, xa.ty);
                let carry_out_ty = Type::new_fixed_int(1).ok_or(AArch64LifterError::InvalidBitsLength)?;
                let carry_out = Variable::new_air(builder.trunc(shifted, xa.ty, carry_out_ty).into(), carry_out_ty);
                Ok((result, carry_out))
            }
            Variable::Rust(RustVariable::integer(si)) => {
                let sa = AirVariable::from_integer(builder, si)?;
                let result = Variable::new_air(builder.lshr(xa.val, sa.val, xa.ty).into(), xa.ty);
                let one = builder.iconst(1);
                let index = builder.sub(sa.val, one, Type::Int);
                let shifted = builder.lshr(xa.val, index, xa.ty);
                let carry_out_ty = Type::new_fixed_int(1).ok_or(AArch64LifterError::InvalidBitsLength)?;
                let carry_out = Variable::new_air(builder.trunc(shifted, xa.ty, carry_out_ty).into(), carry_out_ty);
                Ok((result, carry_out))
            }
            _ => panic!("Variable not integer"),
        },
        Variable::Rust(RustVariable::bits(xb)) => match shift {
            Variable::Air(sa) => {
                let xa = AirVariable::from_bits(builder, xb)?;
                let result = Variable::new_air(builder.lshr(xa.val, sa.val, xa.ty).into(), xa.ty);
                let one = builder.iconst(1);
                let index = builder.sub(sa.val, one, Type::Int);
                let shifted = builder.lshr(xa.val, index, xa.ty);
                let carry_out_ty = Type::new_fixed_int(1).ok_or(AArch64LifterError::InvalidBitsLength)?;
                let carry_out = Variable::new_air(builder.trunc(shifted, xa.ty, carry_out_ty).into(), carry_out_ty);
                Ok((result, carry_out))
            }
            Variable::Rust(RustVariable::integer(si)) => {
                assert!(si > integer::zero());
                let extended_x = decode::helpers::ZeroExtend_0(xb, &si + N.to_integer()?, integer::from(xb.length), &si + N.to_integer()?)?;
                let result = extended_x.extract_slice(integer_to_usize!(si), integer_to_usize!(N.to_integer()?))?;
                let carry_out = extended_x.extract_slice(integer_to_usize!(si) - 1, 1)?;
                Ok((result.into(), carry_out.into()))
            }
            _ => panic!("Variable not integer"),
        },
        _ => panic!("Variable not bits"),
    }
}

pub fn LSL_C_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    x: Variable,
    shift: Variable,
    N: Variable,
) -> Result<(Variable, Variable), AArch64LifterError> {
    match x {
        Variable::Air(xa) => match shift {
            Variable::Air(sa) => {
                let result = Variable::new_air(builder.lshl(xa.val, sa.val, xa.ty).into(), xa.ty);
                let Na = builder.iconst(integer_to_u64!(N.to_integer()?));
                let index = builder.sub(Na, sa.val, Type::Int);
                let shifted = builder.lshr(xa.val, index, xa.ty);
                let carry_out_ty = Type::new_fixed_int(1).ok_or(AArch64LifterError::InvalidBitsLength)?;
                let carry_out = Variable::new_air(builder.trunc(shifted, xa.ty, carry_out_ty).into(), carry_out_ty);
                Ok((result, carry_out))
            }
            Variable::Rust(RustVariable::integer(si)) => {
                let sa = AirVariable::from_integer(builder, si)?;
                let result = Variable::new_air(builder.lshl(xa.val, sa.val, xa.ty).into(), xa.ty);
                let Na = builder.iconst(integer_to_u64!(N.to_integer()?));
                let index = builder.sub(Na, sa.val, Type::Int);
                let shifted = builder.lshr(xa.val, index, xa.ty);
                let carry_out_ty = Type::new_fixed_int(1).ok_or(AArch64LifterError::InvalidBitsLength)?;
                let carry_out = Variable::new_air(builder.trunc(shifted, xa.ty, carry_out_ty).into(), carry_out_ty);
                Ok((result, carry_out))
            }
            _ => panic!("Variable not integer"),
        },
        Variable::Rust(RustVariable::bits(xb)) => match shift {
            Variable::Air(sa) => {
                let xa = AirVariable::from_bits(builder, xb)?;
                let result = Variable::new_air(builder.lshl(xa.val, sa.val, xa.ty).into(), xa.ty);
                let Na = builder.iconst(integer_to_u64!(N.to_integer()?));
                let index = builder.sub(Na, sa.val, Type::Int);
                let shifted = builder.lshr(xa.val, index, xa.ty);
                let carry_out_ty = Type::new_fixed_int(1).ok_or(AArch64LifterError::InvalidBitsLength)?;
                let carry_out = Variable::new_air(builder.trunc(shifted, xa.ty, carry_out_ty).into(), carry_out_ty);
                Ok((result, carry_out))
            }
            Variable::Rust(RustVariable::integer(si)) => {
                assert!(si > integer::zero());
                let extended_x = decode::helpers::append_bits_0(
                    xb,
                    decode::helpers::Zeros_0(si.clone(), si.clone())?,
                    integer::from(xb.length),
                    si.clone(),
                )?;
                let result = extended_x.extract_slice(0, integer_to_usize!(N.to_integer()?))?;
                let carry_out = extended_x.extract_slice(integer_to_usize!(N.to_integer()? - si), 1)?;
                Ok((result.into(), carry_out.into()))
            }
            _ => panic!("Variable not integer"),
        },
        _ => panic!("Variable not bits"),
    }
}

pub fn AddrTop_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    address: Variable,
    IsInstr: Variable,
    el: Variable,
) -> Result<Variable, AArch64LifterError> {
    Ok(Variable::from(integer::from(63)))
}

pub fn TrapPACUse_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    target_el: Variable,
) -> Result<(), AArch64LifterError> {
    Ok(())
}

pub fn AArch64_PACFailException_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    syndrome: Variable,
) -> Result<(), AArch64LifterError> {
    Ok(())
}

pub fn AArch64_SPAlignmentFault_0(builder: &mut InstructionBuilder, sequencer: &mut BlockSequencer, pc: Variable) -> Result<(), AArch64LifterError> {
    Ok(())
}

pub fn AArch64_Abort_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    vaddress: Variable,
    fault: lift::types::FaultRecord,
) -> Result<(), AArch64LifterError> {
    Ok(())
}

pub fn AArch32_ExecutingLSMInstr_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
) -> Result<Variable, AArch64LifterError> {
    Ok(Variable::from(boolean::FALSE))
}

pub fn AArch64_ExecutingATS1xPInstr_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
) -> Result<Variable, AArch64LifterError> {
    Ok(Variable::from(boolean::FALSE))
}

pub fn InsertIESBBeforeException_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    el: Variable,
) -> Result<Variable, AArch64LifterError> {
    Ok(Variable::from(boolean::FALSE))
}

pub fn SynchronizeErrors_0(builder: &mut InstructionBuilder, sequencer: &mut BlockSequencer, pc: Variable) -> Result<(), AArch64LifterError> {
    Ok(())
}

pub fn AArch64_ReportException_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    exception: ExceptionRecord,
    target_el: Variable,
) -> Result<(), AArch64LifterError> {
    Ok(())
}

pub fn AArch64_TakeException_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    target_el: Variable,
    exception: ExceptionRecord,
    preferred_exception_return: Variable,
    vect_offset: Variable,
) -> Result<(), AArch64LifterError> {
    Ok(())
}

pub fn AArch64_TagCheckFault_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    va: Variable,
    write: Variable,
) -> Result<(), AArch64LifterError> {
    Ok(())
}

pub fn AArch64_TagCheckFail_0(
    builder: &mut InstructionBuilder,
    sequencer: &mut BlockSequencer,
    pc: Variable,
    vaddress: Variable,
    iswrite: Variable,
) -> Result<(), AArch64LifterError> {
    Ok(())
}

use crate::{
    assembly::{BitSize, Data, Instruction, Memory},
    ast::{BitOp, Expr},
    compiler::{
        registers::{RegisterDescriptor, RegisterManager, UsageContext},
        stack::StackManager,
        AssemblyOutput,
    },
};

use super::compile_expr;

pub fn compile_bit_op(
    bitop: BitOp,
    target: RegisterDescriptor,
    lhs: Expr,
    rhs: Expr,
    registers: &mut RegisterManager,
    stack: &mut StackManager,
    var_ctx: &[Memory],
    is_ignored: bool,
) -> AssemblyOutput {
    // kind of same stuff as arithmetic operations
    let (lhs, rhs) = match (lhs, rhs) {
        (Expr::Constant(b), a) if bitop.is_commutative() => (a, Expr::Constant(b)),
        other => other,
    };
    compile_expr(lhs, target, registers, stack, var_ctx, is_ignored)
        .unwrap()
        .chain(if !is_ignored {
            let (compute_rhs, rhs_data) = if let Expr::Constant(b) = rhs {
                (AssemblyOutput::new(), Data::Immediate(b as u64))
            } else {
                registers.locking_register(target, |registers| {
                    let rhs_target = registers.get_suitable_register(UsageContext::Normal);
                    let compute_rhs =
                        compile_expr(rhs, rhs_target, registers, stack, var_ctx, false).unwrap();
                    (
                        compute_rhs,
                        Data::Register(rhs_target.as_immutable(BitSize::Bit32)),
                    )
                })
            };
            compute_rhs.chain(registers.using_register_mutably(
                stack,
                target,
                BitSize::Bit32,
                |_stack, _registers, target| match bitop {
                    BitOp::And => Instruction::And {
                        target,
                        lhs: target.into(),
                        rhs: rhs_data,
                    },
                    BitOp::Or => Instruction::Orr {
                        target,
                        lhs: target.into(),
                        rhs: rhs_data,
                    },
                    BitOp::Xor => Instruction::Eor {
                        target,
                        lhs: target.into(),
                        rhs: rhs_data,
                        bitmask: BitSize::Bit32.full_bits(),
                    },
                    BitOp::LeftShift => Instruction::Lsl {
                        target,
                        lhs: target.into(),
                        rhs: rhs_data,
                    },
                    BitOp::RightShift => Instruction::Lsr {
                        target,
                        lhs: target.into(),
                        rhs: rhs_data,
                    },
                },
            ))
        } else {
            compile_expr(rhs, target, registers, stack, var_ctx, true).unwrap()
        })
}

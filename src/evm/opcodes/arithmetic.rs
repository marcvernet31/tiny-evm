//! Arithmetic opcodes
//! 
//! This module implements arithmetic opcodes like ADD, SUB, MUL, DIV, etc.

use crate::{evm::{opcodes::traits::EVMOperation, EVM}, types::*};
use super::Opcode;

// ADD
pub struct AddOp;

impl EVMOperation for AddOp {
    fn execute(&self, evm: &mut EVM) -> Result<()> {
        let a = evm.stack.pop()?;
        let b = evm.stack.pop()?;
        // In case of ovrflow, the output is returned mod 256
        // u8 example: 250 + 10 = 260, but 260 % 256 = 4
        // This is specified in EVM specification for all arithmetic opcodes.
        let (result, _) = a.overflowing_add(b);
        evm.stack.push(result)?;
        Ok(())
    }
}

// MUL
pub struct MulOp;

impl EVMOperation for MulOp {
    fn execute(&self, evm: &mut crate::evm::EVM) -> Result<()> {
        let a = evm.stack.pop()?;
        let b = evm.stack.pop()?;
        let (result, _) = a.overflowing_mul(b);
        evm.stack.push(result)?;
        Ok(())
    }
}

// SUB
pub struct SubOp;

impl EVMOperation for SubOp {
    fn execute(&self, evm: &mut crate::evm::EVM) -> Result<()> {
        let a = evm.stack.pop()?;  // First pop = top of stack
        let b = evm.stack.pop()?;  // Second pop = second item
        // EVM: SUB computes b - a (second item - top item)
        let (result, _) = b.overflowing_sub(a);
        evm.stack.push(result)?;
        Ok(())
    }
}

// DIV
pub struct DivOp;

impl EVMOperation for DivOp {
    fn execute(&self, evm: &mut crate::evm::EVM) -> Result<()> {
        let a = evm.stack.pop()?;  // First pop = top of stack (dividend)
        let b = evm.stack.pop()?;  // Second pop = second item (divisor)

        // EVM Spec: Division by zero returns 0
        // EVM: DIV computes a / b (top item / second item)
        let result = if b.is_zero() {
            Word::zero()
        } else {
            a / b
        };
        evm.stack.push(result)?;
        Ok(())
    }
}

// MOD
pub struct ModOp;

impl EVMOperation for ModOp {
    fn execute(&self, evm: &mut crate::evm::EVM) -> Result<()> {
        let a = evm.stack.pop()?;  // First pop = top of stack (value)
        let b = evm.stack.pop()?;  // Second pop = second item (modulus)

        // EVM Spec: Modulo by zero returns 0
        // EVM: MOD computes a % b (top item % second item)
        let result = if b.is_zero() {
            Word::zero()
        } else {
            a % b
        };
        evm.stack.push(result)?;
        Ok(())
    }
}

// SDIV
// SMOD
// ADDMOD
// MULMOD
// EXP


pub fn execute_arithmetic_opcode(_opcode: Opcode, _evm: &mut crate::evm::EVM) -> Result<()> {
   match _opcode {
    Opcode::ADD => {    
        let op = AddOp;
        op.execute(_evm)
    }
    Opcode::MUL => {
        let op = MulOp;
        op.execute(_evm)
    }
    Opcode::SUB => {
        let op = SubOp;
        op.execute(_evm)
    }
    Opcode::DIV => {
        let op = DivOp;
        op.execute(_evm)
    }
    Opcode::MOD => {
        let op = ModOp;
        op.execute(_evm)
    }
    _ => Err(Error::InvalidOpcode(_opcode as u8)),
   }
}
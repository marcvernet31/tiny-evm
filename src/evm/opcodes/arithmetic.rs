//! Arithmetic opcodes
//! 
//! This module implements arithmetic opcodes like ADD, SUB, MUL, DIV, etc.

use crate::{evm::{opcodes::traits::EVMOperation, EVM}, types::*};
use super::Opcode;

pub struct AddOp;

impl EVMOperation for AddOp {
    fn execute(&self, evm: &mut EVM) -> Result<()> {
        let a = evm.stack.pop()?;
        let b = evm.stack.pop()?;
        // In case of ovrflow, the output is returned mod 256
        // u8 example: 250 + 10 = 260, but 260 % 256 = 4
        // This is specified in EVM specification for all arithmetic opcodes.
        let result = a.overflowing_add(b).0;
        evm.stack.push(result)?;
        Ok(())
    }
}

pub fn execute_arithmetic_opcode(_opcode: Opcode, _evm: &mut crate::evm::EVM) -> Result<()> {
   match _opcode {
    Opcode::ADD => {    
        let op = AddOp;
        op.execute(_evm)
    }
    _ => Err(Error::InvalidOpcode(_opcode as u8)),
   }
}
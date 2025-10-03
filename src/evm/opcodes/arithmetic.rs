//! Arithmetic opcodes
//! 
//! This module implements arithmetic opcodes like ADD, SUB, MUL, DIV, etc.

use crate::types::*;
use super::Opcode;

// Placeholder for arithmetic opcodes - will be implemented later
pub fn execute_arithmetic_opcode(_opcode: Opcode, _evm: &mut crate::evm::EVM) -> Result<()> {
    Err(Error::InvalidOpcode(0))
}
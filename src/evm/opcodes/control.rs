//! Control flow opcodes
//! 
//! This module implements control flow opcodes like JUMP, JUMPI, STOP, etc.

use crate::types::*;
use super::Opcode;

// Placeholder for control opcodes - will be implemented later
pub fn execute_control_opcode(_opcode: Opcode, _evm: &mut crate::evm::EVM) -> Result<()> {
    Err(Error::InvalidOpcode(0))
}
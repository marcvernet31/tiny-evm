//! System opcodes
//! 
//! This module implements system opcodes like CALL, CREATE, etc.

use crate::types::*;
use super::Opcode;

// Placeholder for system opcodes - will be implemented later
pub fn execute_system_opcode(_opcode: Opcode, _evm: &mut crate::evm::EVM) -> Result<()> {
    Err(Error::InvalidOpcode(0))
}
//! Storage opcodes
//! 
//! This module implements storage opcodes like SLOAD, SSTORE.

use crate::types::*;
use super::Opcode;

// Placeholder for storage opcodes - will be implemented later
pub fn execute_storage_opcode(_opcode: Opcode, _evm: &mut crate::evm::EVM) -> Result<()> {
    Err(Error::InvalidOpcode(0))
}
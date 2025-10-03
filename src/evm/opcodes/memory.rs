//! Memory opcodes
//! 
//! This module implements memory opcodes like MLOAD, MSTORE, etc.

use crate::types::*;
use super::Opcode;

// Placeholder for memory opcodes - will be implemented later
pub fn execute_memory_opcode(_opcode: Opcode, _evm: &mut crate::evm::EVM) -> Result<()> {
    Err(Error::InvalidOpcode(0))
}
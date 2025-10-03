//! Context opcodes
//! 
//! This module implements context opcodes like CALLER, CALLVALUE, etc.

use crate::types::*;
use super::Opcode;

// Placeholder for context opcodes - will be implemented later
pub fn execute_context_opcode(_opcode: Opcode, _evm: &mut crate::evm::EVM) -> Result<()> {
    Err(Error::InvalidOpcode(0))
}
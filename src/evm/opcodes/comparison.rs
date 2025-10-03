//! Comparison opcodes
//! 
//! This module implements comparison opcodes like LT, GT, EQ, etc.

use crate::types::*;
use super::Opcode;

// Placeholder for comparison opcodes - will be implemented later
pub fn execute_comparison_opcode(_opcode: Opcode, _evm: &mut crate::evm::EVM) -> Result<()> {
    Err(Error::InvalidOpcode(0))
}
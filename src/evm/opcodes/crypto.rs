//! Cryptographic opcodes
//! 
//! This module implements cryptographic opcodes like SHA3.

use crate::types::*;
use super::Opcode;

// Placeholder for crypto opcodes - will be implemented later
pub fn execute_crypto_opcode(_opcode: Opcode, _evm: &mut crate::evm::EVM) -> Result<()> {
    Err(Error::InvalidOpcode(0))
}
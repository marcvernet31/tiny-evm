//! Stack manipulation opcodes
//! 
//! This module implements stack-related opcodes including PUSH operations.

use crate::types::*;
use super::Opcode;
use super::traits::*;

/// PUSH1 opcode implementation
/// 
/// PUSH1 reads 1 byte of immediate data from the bytecode and pushes it onto the stack.
/// The immediate byte follows the PUSH1 opcode (0x60) in the bytecode.
/// 
/// Example: PUSH1 0x42
/// - Bytecode: [0x60, 0x42]
/// - Result: Pushes 0x42 (66 in decimal) onto the stack
pub struct Push1Op;

impl EVMOperation for Push1Op {
    fn execute(&self, evm: &mut crate::evm::EVM) -> Result<()> {
        // PUSH1 reads 1 byte of immediate data
        let immediate_bytes = 1;
        
        // Check if we have enough code to read the immediate data
        if evm.pc + immediate_bytes >= evm.context.code.len() {
            return Err(Error::InvalidJump(evm.pc + immediate_bytes));
        }
        
        // Read the immediate byte (the byte after PUSH1)
        let immediate_byte = evm.context.code[evm.pc + 1];
        
        // Convert to Word (256-bit value) - single byte becomes a 256-bit word
        let value = Word::from(immediate_byte);
        
        // Push onto stack
        evm.stack.push(value)?;
        
        // Update PC to skip the immediate data
        evm.pc += immediate_bytes;
        
        Ok(())
    }
}


/// Execute a stack opcode
/// 
/// This function routes stack opcodes to their appropriate implementations.
/// Currently only supports PUSH1 and PUSH2.
pub fn execute_stack_opcode(opcode: Opcode, evm: &mut crate::evm::EVM) -> Result<()> {
    match opcode {
        Opcode::PUSH1 => {
            let op = Push1Op;
            op.execute(evm)
        }
        // TODO: Add PUSH2, PUSH3, etc.
        _ => Err(Error::InvalidOpcode(opcode as u8)),
    }
}
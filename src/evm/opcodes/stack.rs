//! Stack manipulation opcodes
//! 
//! This module implements stack-related opcodes including PUSH operations.

use crate::types::*;
use super::Opcode;
use super::traits::*;

use crate::evm::EVM;

/// PUSH opcode implementation
/// 
/// PUSH1 reads 1 byte of immediate data from the bytecode and pushes it onto the stack.
/// The immediate byte follows the PUSH1 opcode (0x60) in the bytecode.
/// bytes_to_read defines how many bytes to read from the bytecode. (PUSH1 = 1, PUSH2 = 2, etc.)
/// 
/// Example: PUSH1 0x42
/// - Bytecode: [0x60, 0x42]
/// - Result: Pushes 0x42 (66 in decimal) onto the stack
pub struct PushOp {
    bytes_to_read: usize,
}

impl EVMOperation for PushOp {
    fn execute(&self, evm: &mut EVM) -> Result<()> {        
        // Check if we have enough code to read the immediate data
        if evm.pc + self.bytes_to_read >= evm.context.code.len() {
            return Err(Error::InvalidJump(evm.pc + self.bytes_to_read));
        }
        
        // Read the immediate bytes (the bytes after the PUSH opcode)
        let start_idx = evm.pc + 1;
        let end_idx = start_idx + self.bytes_to_read;
        let immediate_bytes = &evm.context.code[start_idx..end_idx];
        
        // Convert bytes to Word (256-bit value)
        let mut value = Word::zero();
        for &byte in immediate_bytes {
            value = (value << 8) | Word::from(byte);
        }
        
        // Push onto stack
        evm.stack.push(value)?;
        
        // Update PC to skip the immediate data (opcode + immediate bytes)
        evm.pc += 1 + self.bytes_to_read;
        
        Ok(())
    }
}

pub struct SwapOp {
    swap_index: usize,
}

impl EVMOperation for SwapOp {
    fn execute(&self, evm: &mut EVM) -> Result<()> {
        if self.swap_index >= evm.stack.depth() {
            return Err(Error::InvalidJump(evm.pc + self.swap_index));
        }

        evm.stack.swap(self.swap_index)?;

        Ok(())
    }
}

pub struct DupOp {
    dup_index: usize,
}

impl EVMOperation for DupOp {
    fn execute(&self, evm: &mut EVM) -> Result<()> {
        if self.dup_index >= evm.stack.depth() {
            return Err(Error::InvalidJump(evm.pc + self.dup_index));
        }

        evm.stack.dup(self.dup_index)?;
        Ok(())
    }
}

pub struct PopOp;

impl EVMOperation for PopOp {
    fn execute(&self, evm: &mut EVM) -> Result<()> {
        evm.stack.pop()?;
        Ok(())
    }
}

pub fn execute_stack_opcode(opcode: Opcode, evm: &mut crate::evm::EVM) -> Result<()> {
    match opcode {
        opcode if opcode.is_push() => {
            let op = PushOp { bytes_to_read: opcode.immediate_bytes() };
            op.execute(evm)
        }
        opcode if opcode.is_swap() => {
            let op = SwapOp { swap_index: opcode.access_depth_bytes() };
            op.execute(evm)
        }
        opcode if opcode.is_dup() => {
            let op = DupOp { dup_index: opcode.access_depth_bytes() };
            op.execute(evm)
        }
        Opcode::POP => {
            let op = PopOp;
            op.execute(evm)
        }
        _ => Err(Error::InvalidOpcode(opcode as u8)),
    }
}

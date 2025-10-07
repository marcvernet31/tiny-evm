//! Arithmetic opcodes
//! 
//! This module implements arithmetic opcodes like ADD, SUB, MUL, DIV, etc.

use crate::{evm::{opcodes::traits::EVMOperation, EVM}, types::*};
use super::Opcode;
use ethereum_types::U512;

// ADD
pub struct AddOp;

impl EVMOperation for AddOp {
    fn execute(&self, evm: &mut EVM) -> Result<()> {
        let a = evm.stack.pop()?;
        let b = evm.stack.pop()?;
        // In case of ovrflow, the output is returned mod 256
        // u8 example: 250 + 10 = 260, but 260 % 256 = 4
        // This is specified in EVM specification for all arithmetic opcodes.
        let (result, _) = a.overflowing_add(b);
        evm.stack.push(result)?;
        Ok(())
    }
}

// MUL
pub struct MulOp;

impl EVMOperation for MulOp {
    fn execute(&self, evm: &mut crate::evm::EVM) -> Result<()> {
        let a = evm.stack.pop()?;
        let b = evm.stack.pop()?;
        let (result, _) = a.overflowing_mul(b);
        evm.stack.push(result)?;
        Ok(())
    }
}

// SUB
pub struct SubOp;

impl EVMOperation for SubOp {
    fn execute(&self, evm: &mut crate::evm::EVM) -> Result<()> {
        let a = evm.stack.pop()?;  // First pop = top of stack
        let b = evm.stack.pop()?;  // Second pop = second item
        // EVM: SUB computes b - a (second item - top item)
        let (result, _) = b.overflowing_sub(a);
        evm.stack.push(result)?;
        Ok(())
    }
}

// DIV
pub struct DivOp;

impl EVMOperation for DivOp {
    fn execute(&self, evm: &mut crate::evm::EVM) -> Result<()> {
        let a = evm.stack.pop()?;  // First pop = top of stack (dividend)
        let b = evm.stack.pop()?;  // Second pop = second item (divisor)

        // EVM Spec: Division by zero returns 0
        // EVM: DIV computes a / b (top item / second item)
        let result = if b.is_zero() {
            Word::zero()
        } else {
            a / b
        };
        evm.stack.push(result)?;
        Ok(())
    }
}

// MOD
pub struct ModOp;

impl EVMOperation for ModOp {
    fn execute(&self, evm: &mut crate::evm::EVM) -> Result<()> {
        let a = evm.stack.pop()?;  // First pop = top of stack (value)
        let b = evm.stack.pop()?;  // Second pop = second item (modulus)

        // EVM Spec: Modulo by zero returns 0
        // EVM: MOD computes a % b (top item % second item)
        let result = if b.is_zero() {
            Word::zero()
        } else {
            a % b
        };
        evm.stack.push(result)?;
        Ok(())
    }
}

// TODO: Implement signed operations
// SDIV
// SMOD


// ADDMOD - Modular Addition with extended precision
pub struct AddModOp;

impl EVMOperation for AddModOp {
    fn execute(&self, evm: &mut crate::evm::EVM) -> Result<()> {
        let n = evm.stack.pop()?;  // Modulus (top of stack)
        let b = evm.stack.pop()?;  // Second operand
        let a = evm.stack.pop()?;  // First operand (bottom of the 3)

        // EVM Spec: If modulus is 0, return 0
        if n.is_zero() {
            evm.stack.push(Word::zero())?;
            return Ok(());
        }

        // Compute (a + b) % n with extended precision
        // Convert to U512 to avoid overflow in intermediate addition
        let a_512 = U512::from(a);
        let b_512 = U512::from(b);
        let n_512 = U512::from(n);

        // Perform addition in U512 space
        let sum_512 = a_512 + b_512;
        
        // Take modulo and convert back to U256
        let result_512 = sum_512 % n_512;
        let result = u512_to_u256(result_512);

        evm.stack.push(result)?;
        Ok(())
    }
}

// MULMOD - Modular Multiplication with extended precision
pub struct MulModOp;

// All Modular Operations are implemented using U512 to handle overflows on intermediate calculations
// This is defined in EVM specification (https://cypherpunks-core.github.io/ethereumbook/13evm.html)
// Actually the existence of these intermediate calculations is to avoid the potential overflow that could 
// happen if the multiplication and modulo where done separately. (I guess multiplication + modulo is a quite common operation)
// Vitalik really thought about all the details lol.
impl EVMOperation for MulModOp {
    fn execute(&self, evm: &mut crate::evm::EVM) -> Result<()> {
        let n = evm.stack.pop()?;  // Modulus (top of stack)
        let b = evm.stack.pop()?;  // Second operand
        let a = evm.stack.pop()?;  // First operand (bottom of the 3)

        // EVM Spec: If modulus is 0, return 0
        if n.is_zero() {
            evm.stack.push(Word::zero())?;
            return Ok(());
        }

        // Compute (a * b) % n with extended precision
        // Convert to U512 to handle 256-bit multiplication (can produce 512-bit result)
        let a_512 = U512::from(a);
        let b_512 = U512::from(b);
        let n_512 = U512::from(n);

        // Perform multiplication in U512 space
        let product_512 = a_512 * b_512;
        
        // Take modulo and convert back to U256
        let result_512 = product_512 % n_512;
        let result = u512_to_u256(result_512);

        evm.stack.push(result)?;
        Ok(())
    }
}

// EXP


/// TODO: Move to utilities file
fn u512_to_u256(value: U512) -> Word {
    // U512 is stored as [u64; 8], U256 is [u64; 4]
    // Take the lower 4 u64 values (first 32 bytes in little endian)
    let mut buffer = [0u8; 64];
    value.to_little_endian(&mut buffer);
    
    let mut bytes = [0u8; 32];
    bytes.copy_from_slice(&buffer[..32]);
    
    Word::from_little_endian(&bytes)
}

pub fn execute_arithmetic_opcode(_opcode: Opcode, _evm: &mut crate::evm::EVM) -> Result<()> {
   match _opcode {
    Opcode::ADD => {    
        let op = AddOp;
        op.execute(_evm)
    }
    Opcode::MUL => {
        let op = MulOp;
        op.execute(_evm)
    }
    Opcode::SUB => {
        let op = SubOp;
        op.execute(_evm)
    }
    Opcode::DIV => {
        let op = DivOp;
        op.execute(_evm)
    }
    Opcode::MOD => {
        let op = ModOp;
        op.execute(_evm)
    }
    Opcode::ADDMOD => {
        let op = AddModOp;
        op.execute(_evm)
    }
    Opcode::MULMOD => {
        let op = MulModOp;
        op.execute(_evm)
    }
    _ => Err(Error::InvalidOpcode(_opcode as u8)),
   }
}
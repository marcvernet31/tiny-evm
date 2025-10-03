//! Common traits for EVM operations
//! 
//! This module defines shared traits used across all opcode implementations.

use crate::types::*;

/// Trait for EVM operations.
/// All opcode implementations should implement this trait to provide a common
/// interface for execution. This allows the EVM to execute any opcode through
/// the same `execute` method.
/// 
/// # Example
/// 
/// ```rust
/// use crate::evm::opcodes::traits::EVMOperation;
/// 
/// struct AddOp;
/// 
/// impl EVMOperation for AddOp {
///     fn execute(&self, evm: &mut crate::evm::EVM) -> Result<()> {
///         let a = evm.stack.pop()?;
///         let b = evm.stack.pop()?;
///         let result = a.overflowing_add(b).0;
///         evm.stack.push(result)?;
///         Ok(())
///     }
/// }
/// ```
pub trait EVMOperation {
    fn execute(&self, evm: &mut crate::evm::EVM) -> Result<()>;
}

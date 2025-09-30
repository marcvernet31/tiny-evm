//! EVM Core - The heart of the Ethereum Virtual Machine
//! 
//! This module contains the main EVM struct and execution loop that
//! processes bytecode instructions and maintains execution state.

use crate::types::*;
use crate::evm::stack::Stack;
use crate::evm::memory::Memory;
use crate::evm::storage::Storage;
use crate::evm::context::ExecutionContext;

#[derive(Debug)]
pub struct EVM {
    /// Execution stack (max 1024 items)
    pub stack: Stack,
    
    /// Linear memory (byte-addressable)
    pub memory: Memory,
    
    /// Persistent storage (word -> word mapping)
    pub storage: Storage,
    
    /// Program counter (current instruction index)
    pub pc: usize,
    
    /// Gas remaining for execution
    pub gas: Gas,
    
    /// Initial gas limit
    pub initial_gas: Gas,
    
    /// Current execution context
    pub context: ExecutionContext,
    
    /// Return data from last call
    pub return_data: Bytes,
    
    /// Execution state flags
    pub stopped: bool,
    pub reverted: bool,
    
    /// Event logs emitted during execution
    pub logs: Vec<Log>,
}

impl EVM {
    /// Create a new EVM instance
    pub fn new(context: ExecutionContext, gas_limit: Gas) -> Self {
        Self {
            stack: Stack::new(),
            memory: Memory::new(),
            storage: Storage::new(),
            pc: 0,
            gas: gas_limit,
            initial_gas: gas_limit,
            context,
            return_data: Vec::new(),
            stopped: false,
            reverted: false,
            logs: Vec::new(),
        }
    }
    
    /// Execute bytecode until completion or error
    pub fn execute(&mut self) -> Result<ExecutionResult> {
        loop {
            // Check if execution should stop
            if self.stopped || self.reverted {
                break;
            }
            
            // Check PC bounds
            if self.pc >= self.context.code.len() {
                break;
            }
            
            // Fetch and execute next instruction
            self.execute_next_instruction()?;
        }
        
        Ok(ExecutionResult {
            success: !self.reverted,
            gas_used: self.initial_gas - self.gas,
            output: self.return_data.clone(),
            logs: self.logs.clone(),
            contract_address: None,
        })
    }
    
    /// Execute the next instruction at the current PC
    fn execute_next_instruction(&mut self) -> Result<()> {
        // TODO: This will be implemented in Phase 2 when we add opcodes
        // For now, just increment PC to avoid infinite loops
        self.pc += 1;
        Ok(())
    }
    
    /// Check if we have enough gas for an operation
    pub fn check_gas(&self, required: Gas) -> Result<()> {
        if self.gas < required {
            Err(Error::OutOfGas(self.gas))
        } else {
            Ok(())
        }
    }
    
    /// Consume gas for an operation
    pub fn consume_gas(&mut self, amount: Gas) -> Result<()> {
        self.check_gas(amount)?;
        self.gas -= amount;
        Ok(())
    }
    
    /// Stop execution
    pub fn stop(&mut self) {
        self.stopped = true;
    }
    
    /// Revert execution
    pub fn revert(&mut self, reason: String) {
        self.reverted = true;
        self.return_data = reason.into_bytes();
    }
    
    /// Return data and stop execution
    pub fn return_data(&mut self, data: Bytes) {
        self.return_data = data;
        self.stopped = true;
    }
}

/// Gas costs for different operations
pub mod gas {
    use super::*;
    
    /// Base gas cost for most operations
    pub const BASE: Gas = 2;
    
    /// Gas cost for stack operations
    pub const STACK_PUSH: Gas = 3;
    pub const STACK_POP: Gas = 2;
    pub const STACK_DUP: Gas = 3;
    pub const STACK_SWAP: Gas = 3;
    
    /// Gas cost for memory operations
    pub const MEMORY_LOAD: Gas = 3;
    pub const MEMORY_STORE: Gas = 3;
    pub const MEMORY_STORE8: Gas = 3;
    
    /// Gas cost for storage operations
    pub const STORAGE_LOAD: Gas = 200;
    pub const STORAGE_STORE: Gas = 20000;
    pub const STORAGE_STORE_CLEAR: Gas = 5000;
    
    /// Gas cost for arithmetic operations
    pub const ADD: Gas = 3;
    pub const MUL: Gas = 5;
    pub const SUB: Gas = 3;
    pub const DIV: Gas = 5;
    pub const MOD: Gas = 5;
    pub const EXP: Gas = 10; // Base cost, additional cost for exponent size
    
    /// Gas cost for comparison operations
    pub const LT: Gas = 3;
    pub const GT: Gas = 3;
    pub const EQ: Gas = 3;
    pub const ISZERO: Gas = 3;
    
    /// Gas cost for bitwise operations
    pub const AND: Gas = 3;
    pub const OR: Gas = 3;
    pub const XOR: Gas = 3;
    pub const NOT: Gas = 3;
    
    /// Gas cost for control flow
    pub const JUMP: Gas = 8;
    pub const JUMPI: Gas = 10;
    pub const JUMPDEST: Gas = 1;
    pub const STOP: Gas = 0;
    
    /// Gas cost for context operations
    pub const ADDRESS: Gas = 2;
    pub const CALLER: Gas = 2;
    pub const CALLVALUE: Gas = 2;
    pub const ORIGIN: Gas = 2;
    
    /// Calculate gas cost for memory expansion
    pub fn memory_expansion_cost(current_size: usize, new_size: usize) -> Gas {
        if new_size <= current_size {
            return 0;
        }
        
        let new_words = (new_size + 31) / 32; // Round up to word boundary
        let current_words = (current_size + 31) / 32;
        
        let new_cost = new_words * new_words / 512 + 3 * new_words;
        let current_cost = current_words * current_words / 512 + 3 * current_words;
        
        (new_cost - current_cost) as Gas
    }
}

// Re-export submodules
pub mod stack;
pub mod memory;
pub mod storage;
pub mod context;
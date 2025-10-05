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
        // Fetch opcode
        let opcode_byte = self.context.code[self.pc];
        let opcode = match opcodes::Opcode::from_byte(opcode_byte) {
            Some(op) => op,
            None => return Err(Error::InvalidOpcode(opcode_byte)),
        };
        
        // Check gas cost
        let gas_cost = opcode.gas_cost();
        self.consume_gas(gas_cost)?;
        
        // TODO: Add additional opcodes as they are implemented
        match opcode {
            opcode if opcode.is_stack_opcode() => {
                opcodes::stack::execute_stack_opcode(opcode, self)?;
            }
            opcode if opcode.is_arithmetic_opcode() => {
                opcodes::arithmetic::execute_arithmetic_opcode(opcode, self)?;
            }
            _ => {
                return Err(Error::NotImplementedOpcode(opcode_byte));
            }
        }
        
        // Increment PC (unless opcode modified it)
        if !opcode.modifies_pc() {
            self.pc += 1;
        }
        
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

// Re-export submodules
pub mod stack;
pub mod memory;
pub mod storage;
pub mod context;
pub mod opcodes;
//! Execution Context for EVM
//! 
//! The execution context contains all the information needed to execute
//! a transaction or contract call, including caller information, block
//! context, and input data.

use crate::types::*;

/// Execution context for EVM operations
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    /// Contract address being executed
    pub address: Address,
    
    /// Caller address (who initiated this call)
    pub caller: Address,
    
    /// Transaction origin (who signed the original transaction)
    pub origin: Address,
    
    /// ETH value sent with this call
    pub value: Wei,
    
    /// Input data for this call
    pub data: Bytes,
    
    /// Bytecode being executed
    pub code: Bytes,
    
    /// Block context
    pub block: BlockContext,
    
    /// Gas price for this transaction
    pub gas_price: Wei,
    
    /// Whether this is a static call (no state modifications allowed)
    pub is_static: bool,
}

impl ExecutionContext {
    /// Create a new execution context
    pub fn new(
        address: Address,
        caller: Address,
        origin: Address,
        value: Wei,
        data: Bytes,
        code: Bytes,
        block: BlockContext,
        gas_price: Wei,
    ) -> Self {
        Self {
            address,
            caller,
            origin,
            value,
            data,
            code,
            block,
            gas_price,
            is_static: false,
        }
    }
    
    /// Create a static call context (no state modifications allowed)
    pub fn new_static(
        address: Address,
        caller: Address,
        origin: Address,
        value: Wei,
        data: Bytes,
        code: Bytes,
        block: BlockContext,
        gas_price: Wei,
    ) -> Self {
        Self {
            address,
            caller,
            origin,
            value,
            data,
            code,
            block,
            gas_price,
            is_static: true,
        }
    }
    
    /// Get the size of input data
    pub fn data_size(&self) -> usize {
        self.data.len()
    }
    
    /// Get the size of bytecode
    pub fn code_size(&self) -> usize {
        self.code.len()
    }
    
    /// Load a word from input data at the given offset
    /// 
    /// # Arguments
    /// * `offset` - Byte offset in input data
    /// 
    /// # Returns
    /// Returns a 256-bit word, zero-padded if offset is beyond data size
    pub fn load_data(&self, offset: usize) -> Word {
        if offset >= self.data.len() {
            return Word::zero();
        }
        
        let mut bytes = [0u8; 32];
        let end = (offset + 32).min(self.data.len());
        let actual_size = end - offset;
        
        bytes[..actual_size].copy_from_slice(&self.data[offset..end]);
        Word::from_big_endian(&bytes)
    }
    
    /// Load a range of bytes from input data
    /// 
    /// # Arguments
    /// * `offset` - Starting byte offset
    /// * `size` - Number of bytes to load
    /// 
    /// # Returns
    /// Returns bytes, zero-padded if offset+size exceeds data size
    pub fn load_data_range(&self, offset: usize, size: usize) -> Vec<u8> {
        if offset >= self.data.len() {
            return vec![0u8; size];
        }
        
        let end = (offset + size).min(self.data.len());
        let actual_size = end - offset;
        
        let mut result = vec![0u8; size];
        result[..actual_size].copy_from_slice(&self.data[offset..end]);
        result
    }
    
    /// Load a word from bytecode at the given offset
    /// 
    /// # Arguments
    /// * `offset` - Byte offset in bytecode
    /// 
    /// # Returns
    /// Returns a 256-bit word, zero-padded if offset is beyond code size
    pub fn load_code(&self, offset: usize) -> Word {
        if offset >= self.code.len() {
            return Word::zero();
        }
        
        let mut bytes = [0u8; 32];
        let end = (offset + 32).min(self.code.len());
        let actual_size = end - offset;
        
        bytes[..actual_size].copy_from_slice(&self.code[offset..end]);
        Word::from_big_endian(&bytes)
    }
    
    /// Load a range of bytes from bytecode
    /// 
    /// # Arguments
    /// * `offset` - Starting byte offset
    /// * `size` - Number of bytes to load
    /// 
    /// # Returns
    /// Returns bytes, zero-padded if offset+size exceeds code size
    pub fn load_code_range(&self, offset: usize, size: usize) -> Vec<u8> {
        if offset >= self.code.len() {
            return vec![0u8; size];
        }
        
        let end = (offset + size).min(self.code.len());
        let actual_size = end - offset;
        
        let mut result = vec![0u8; size];
        result[..actual_size].copy_from_slice(&self.code[offset..end]);
        result
    }
    
    /// Check if this is a contract creation (empty address)
    pub fn is_contract_creation(&self) -> bool {
        self.address.is_zero()
    }
    
    /// Check if this is a static call
    pub fn is_static_call(&self) -> bool {
        self.is_static
    }
}

impl Default for ExecutionContext {
    fn default() -> Self {
        Self {
            address: Address::zero(),
            caller: Address::zero(),
            origin: Address::zero(),
            value: Wei::zero(),
            data: Vec::new(),
            code: Vec::new(),
            block: BlockContext::default(),
            gas_price: Wei::zero(),
            is_static: false,
        }
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_execution_context_creation() {
        let address = Address::from([1u8; 20]);
        let caller = Address::from([2u8; 20]);
        let origin = Address::from([3u8; 20]);
        let value = Wei::from(1000);
        let data = vec![0x01, 0x02, 0x03, 0x04];
        let code = vec![0x60, 0x01, 0x60, 0x02, 0x01]; // PUSH1 1 PUSH1 2 ADD
        let block = BlockContext::default();
        let gas_price = Wei::from(20);
        
        let context = ExecutionContext::new(
            address,
            caller,
            origin,
            value,
            data.clone(),
            code.clone(),
            block,
            gas_price,
        );
        
        assert_eq!(context.address, address);
        assert_eq!(context.caller, caller);
        assert_eq!(context.origin, origin);
        assert_eq!(context.value, value);
        assert_eq!(context.data, data);
        assert_eq!(context.code, code);
        assert_eq!(context.gas_price, gas_price);
        assert!(!context.is_static);
    }
    
    #[test]
    fn test_load_data() {
        let data = vec![0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0];
        let context = ExecutionContext {
            data,
            ..Default::default()
        };
        
        // Load first 4 bytes as word
        let word = context.load_data(0);
        assert_eq!(word, Word::from(0x12345678u64) << 224);
        
        // Load beyond data size should return zero
        let word = context.load_data(100);
        assert_eq!(word, Word::zero());
    }
    
    #[test]
    fn test_load_data_range() {
        let data = vec![0x01, 0x02, 0x03, 0x04, 0x05];
        let context = ExecutionContext {
            data,
            ..Default::default()
        };
        
        // Load range within data
        let result = context.load_data_range(1, 3);
        assert_eq!(result, vec![0x02, 0x03, 0x04]);
        
        // Load range beyond data size should be zero-padded
        let result = context.load_data_range(3, 5);
        assert_eq!(result, vec![0x04, 0x05, 0x00, 0x00, 0x00]);
    }
    
    #[test]
    fn test_load_code() {
        let code = vec![0x60, 0x01, 0x60, 0x02, 0x01]; // PUSH1 1 PUSH1 2 ADD
        let context = ExecutionContext {
            code,
            ..Default::default()
        };
        
        // Load first instruction
        let word = context.load_code(0);
        assert_eq!(word, Word::from(0x6001u64) << 240);
        
        // Load beyond code size should return zero
        let word = context.load_code(100);
        assert_eq!(word, Word::zero());
    }
    
    #[test]
    fn test_contract_creation() {
        let context = ExecutionContext::default();
        assert!(context.is_contract_creation());
        
        let context = ExecutionContext {
            address: Address::from([1u8; 20]),
            ..Default::default()
        };
        assert!(!context.is_contract_creation());
    }
    
    #[test]
    fn test_static_call() {
        let context = ExecutionContext::default();
        assert!(!context.is_static_call());
        
        let context = ExecutionContext {
            is_static: true,
            ..Default::default()
        };
        assert!(context.is_static_call());
    }
}
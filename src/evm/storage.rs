//! EVM Storage implementation
//! 
//! The EVM storage is a persistent key-value store where each
//! contract can store 256-bit words indexed by 256-bit keys.
//! Storage persists across transactions and is part of the world state.

use crate::types::*;
use std::collections::HashMap;

/// EVM storage implementation
#[derive(Debug, Clone)]
pub struct Storage {
    /// Storage data (key -> value mapping)
    data: HashMap<Word, Word>,
}

impl Storage {
    /// Create a new empty storage
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
    
    /// Load a value from storage
    /// 
    /// # Arguments
    /// * `key` - Storage key
    /// 
    /// # Returns
    /// Returns the stored value, or zero if key doesn't exist
    pub fn load(&self, key: &Word) -> Word {
        self.data.get(key).copied().unwrap_or(Word::zero())
    }
    
    /// Store a value in storage
    /// 
    /// # Arguments
    /// * `key` - Storage key
    /// * `value` - Value to store
    pub fn store(&mut self, key: Word, value: Word) {
        if value.is_zero() {
            // If storing zero, remove the key to save space
            self.data.remove(&key);
        } else {
            self.data.insert(key, value);
        }
    }
    
    /// Check if a key exists in storage
    pub fn contains_key(&self, key: &Word) -> bool {
        self.data.contains_key(key)
    }
    
    /// Get the number of storage slots used
    pub fn len(&self) -> usize {
        self.data.len()
    }
    
    /// Check if storage is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    
    /// Clear all storage
    pub fn clear(&mut self) {
        self.data.clear();
    }
    
    /// Get all storage entries (for debugging)
    pub fn entries(&self) -> impl Iterator<Item = (&Word, &Word)> {
        self.data.iter()
    }
    
    /// Get a reference to the underlying HashMap (for debugging)
    pub fn data(&self) -> &HashMap<Word, Word> {
        &self.data
    }
}

impl Default for Storage {
    fn default() -> Self {
        Self::new()
    }
}

/// Storage operations for gas calculation
impl Storage {
    /// Calculate gas cost for a storage operation
    /// 
    /// # Arguments
    /// * `key` - Storage key
    /// * `new_value` - New value to store
    /// 
    /// # Returns
    /// Gas cost for the storage operation
    pub fn operation_cost(&self, key: &Word, new_value: &Word) -> Gas {
        let current_value = self.load(key);
        
        if current_value.is_zero() && !new_value.is_zero() {
            // Setting a zero slot to non-zero: SSTORE cost
            20000
        } else if !current_value.is_zero() && new_value.is_zero() {
            // Setting a non-zero slot to zero: SSTORE + refund
            20000 // We'll handle refunds separately
        } else if !current_value.is_zero() && !new_value.is_zero() {
            // Setting a non-zero slot to non-zero: SSTORE cost
            20000
        } else {
            // Setting zero to zero: no cost
            0
        }
    }
    
    /// Calculate gas refund for a storage operation
    /// 
    /// # Arguments
    /// * `key` - Storage key
    /// * `new_value` - New value to store
    /// 
    /// # Returns
    /// Gas refund for the storage operation
    pub fn operation_refund(&self, key: &Word, new_value: &Word) -> Gas {
        let current_value = self.load(key);
        
        if !current_value.is_zero() && new_value.is_zero() {
            // Setting a non-zero slot to zero: refund
            15000
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_storage_load_store() {
        let mut storage = Storage::new();
        
        // Load from empty storage should return zero
        assert_eq!(storage.load(&Word::from(1)), Word::zero());
        
        // Store a value
        let key = Word::from(42);
        let value = Word::from(0x1234567890abcdefu64);
        storage.store(key, value);
        
        // Load it back
        assert_eq!(storage.load(&key), value);
    }
    
    #[test]
    fn test_storage_zero_value() {
        let mut storage = Storage::new();
        
        // Store a non-zero value
        let key = Word::from(42);
        storage.store(key, Word::from(100));
        assert!(storage.contains_key(&key));
        assert_eq!(storage.len(), 1);
        
        // Store zero value should remove the key
        storage.store(key, Word::zero());
        assert!(!storage.contains_key(&key));
        assert_eq!(storage.len(), 0);
        assert_eq!(storage.load(&key), Word::zero());
    }
    
    #[test]
    fn test_storage_operation_cost() {
        let mut storage = Storage::new();
        let key = Word::from(42);
        
        // Setting zero to non-zero: SSTORE cost
        let cost = storage.operation_cost(&key, &Word::from(100));
        assert_eq!(cost, 20000);
        
        // Store the value
        storage.store(key, Word::from(100));
        
        // Setting non-zero to non-zero: SSTORE cost
        let cost = storage.operation_cost(&key, &Word::from(200));
        assert_eq!(cost, 20000);
        
        // Setting non-zero to zero: SSTORE cost + refund
        let cost = storage.operation_cost(&key, &Word::zero());
        assert_eq!(cost, 20000);
        
        // Setting zero to zero: no cost
        let cost = storage.operation_cost(&key, &Word::zero());
        assert_eq!(cost, 0);
    }
    
    #[test]
    fn test_storage_operation_refund() {
        let mut storage = Storage::new();
        let key = Word::from(42);
        
        // Store a non-zero value
        storage.store(key, Word::from(100));
        
        // Setting non-zero to zero: refund
        let refund = storage.operation_refund(&key, &Word::zero());
        assert_eq!(refund, 15000);
        
        // Setting non-zero to non-zero: no refund
        let refund = storage.operation_refund(&key, &Word::from(200));
        assert_eq!(refund, 0);
        
        // Setting zero to zero: no refund
        storage.store(key, Word::zero());
        let refund = storage.operation_refund(&key, &Word::zero());
        assert_eq!(refund, 0);
    }
    
    #[test]
    fn test_storage_clear() {
        let mut storage = Storage::new();
        
        // Add some data
        storage.store(Word::from(1), Word::from(100));
        storage.store(Word::from(2), Word::from(200));
        assert_eq!(storage.len(), 2);
        
        // Clear storage
        storage.clear();
        assert!(storage.is_empty());
        assert_eq!(storage.len(), 0);
    }
}
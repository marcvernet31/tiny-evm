//! EVM Memory implementation
//! 
//! The EVM memory is a linear, byte-addressable array that can grow
//! dynamically during execution. It's used for temporary storage
//! and data passing between operations.

use crate::types::*;

/// EVM memory implementation
#[derive(Debug, Clone)]
pub struct Memory {
    /// Memory data (byte array)
    data: Vec<u8>,
}

impl Memory {
    /// Create a new empty memory
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
        }
    }
    
    /// Load a 32-byte word from memory at the given offset
    /// 
    /// * Explanation for retards
    /// Goes to position offset in memory and reads the 32 bytes from that position.
    /// Converts it into a 256-bit word (EVM word size), padding with zeros if necessary.
    /// saturating_sub is a substraction that returns 0 if the result is negative. 
    /// In this case if the offset is greater than the memory size, it returns 0.
    /// 
    /// # Arguments
    /// * `offset` - Byte offset in memory
    /// 
    /// # Returns
    /// Returns a 256-bit word, zero-padded if offset is beyond memory size
    pub fn load(&mut self, offset: usize) -> Word {
        // So, when we try to access memory outside of the memory size, the EVM specification says that we should return 0 on this new memory.
        // The question I had, was why expand the memory instead of just returning 0 x size? The answer as in many other things we will see is because the 
        // EVM specification says so (explained somewhere here: https://snoozetime.github.io/2018/11/28/ethereum-vm-4.html).
        // The EVM actually charges gas for this memory expansion, its a way to keep memory consistent in case it needs to be accessed later.
        self.expand_to(offset + 32);
        
        // Load 32 bytes and convert to Word
        let mut bytes = [0u8; 32];
        let end = (offset + 32).min(self.data.len());
        let actual_size = end.saturating_sub(offset);
        
        if actual_size > 0 {
            bytes[..actual_size].copy_from_slice(&self.data[offset..end]);
        }
        
        Word::from_big_endian(&bytes)
    }
    
    /// Store a 32-byte word to memory at the given offset
    /// 
    /// # Arguments
    /// * `offset` - Byte offset in memory
    /// * `value` - 256-bit word to store
    pub fn store(&mut self, offset: usize, value: Word) {
        // Ensure memory is large enough
        self.expand_to(offset + 32);
        
        // Convert word to bytes and store
        let mut bytes = [0u8; 32];
        value.to_big_endian(&mut bytes);
        
        // Store bytes
        let end = offset + 32;
        if end <= self.data.len() {
            self.data[offset..end].copy_from_slice(&bytes);
        }
    }
    
    /// Store a single byte to memory at the given offset
    /// 
    /// # Arguments
    /// * `offset` - Byte offset in memory
    /// * `value` - Byte to store (only low 8 bits are used)
    pub fn store_byte(&mut self, offset: usize, value: u8) {
        // Ensure memory is large enough
        self.expand_to(offset + 1);
        
        // Store byte
        if offset < self.data.len() {
            self.data[offset] = value;
        }
    }
    
    /// Load a range of bytes from memory
    /// 
    /// # Arguments
    /// * `offset` - Starting byte offset
    /// * `size` - Number of bytes to load
    /// 
    /// # Returns
    /// Returns a slice of bytes, zero-padded if offset+size exceeds memory size
    pub fn load_range(&mut self, offset: usize, size: usize) -> Vec<u8> {
        // Ensure memory is large enough
        self.expand_to(offset + size);
        
        // Load bytes
        let end = (offset + size).min(self.data.len());
        let actual_size = end.saturating_sub(offset);
        
        if actual_size == 0 {
            return vec![0u8; size];
        }
        
        let mut result = vec![0u8; size];
        result[..actual_size].copy_from_slice(&self.data[offset..end]);
        result
    }
    
    /// Store a range of bytes to memory
    /// 
    /// # Arguments
    /// * `offset` - Starting byte offset
    /// * `data` - Bytes to store
    pub fn store_range(&mut self, offset: usize, data: &[u8]) {
        if data.is_empty() {
            return;
        }
        
        // Ensure memory is large enough
        self.expand_to(offset + data.len());
        
        // Store bytes
        let end = offset + data.len();
        if end <= self.data.len() {
            self.data[offset..end].copy_from_slice(data);
        }
    }
    
    /// Get the current memory size in bytes
    pub fn size(&self) -> usize {
        self.data.len()
    }
    
    /// Get the current memory size in words (32-byte chunks)
    pub fn size_words(&self) -> usize {
        (self.data.len() + 31) / 32
    }
    
    /// Calculate gas cost for memory expansion
    /// 
    /// # Arguments
    /// * `offset` - Starting offset of the operation
    /// * `size` - Size of the operation
    /// 
    /// # Returns
    /// Gas cost for expanding memory to accommodate the operation
    pub fn expansion_cost(&self, offset: usize, size: usize) -> Gas {
        if size == 0 {
            return 0;
        }
        
        let new_size = offset + size;
        let current_words: usize = self.size_words();
        // Equivalent to ⌈n / 32⌉. We round up as memory needs to be allocated in words.
        let new_words: usize = (new_size + 31) / 32; 
        
        if new_words <= current_words {
            return 0;
        }
        
        // Gas cost formula: new_words^2 / 512 + 3 * new_words
        let new_cost = (new_words * new_words) / 512 + 3 * new_words;
        let current_cost = (current_words * current_words) / 512 + 3 * current_words;
        
        (new_cost - current_cost) as Gas
    }
    
    /// Expand memory to at least the given size
    pub fn expand_to(&mut self, size: usize) {
        if size > self.data.len() {
            self.data.resize(size, 0);
        }
    }
    
    /// Clear all memory
    pub fn clear(&mut self) {
        self.data.clear();
    }
    
    /// Get a reference to the memory data (for debugging)
    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

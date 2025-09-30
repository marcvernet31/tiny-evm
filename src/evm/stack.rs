//! EVM Stack implementation
//! 
//! The EVM stack is a LIFO (Last In, First Out) data structure
//! that can hold up to 1024 256-bit words. It's used for temporary
//! storage during execution.

use crate::types::*;

/// Maximum stack depth allowed by the EVM
const MAX_STACK_DEPTH: usize = 1024;

/// EVM execution stack
#[derive(Debug, Clone)]
pub struct Stack {
    /// Stack data (Vec of 256-bit words)
    data: Vec<Word>,
}

impl Stack {
    /// Create a new empty stack
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
        }
    }
    
    /// Push a value onto the stack
    /// 
    /// # Errors
    /// Returns `StackOverflow` if stack is at maximum depth
    pub fn push(&mut self, value: Word) -> Result<()> {
        if self.data.len() >= MAX_STACK_DEPTH {
            return Err(Error::StackOverflow);
        }
        self.data.push(value);
        Ok(())
    }
    
    /// Pop a value from the stack
    /// 
    /// # Errors
    /// Returns `StackUnderflow` if stack is empty
    pub fn pop(&mut self) -> Result<Word> {
        self.data.pop().ok_or(Error::StackUnderflow)
    }
    
    /// Peek at a value at a specific depth from the top
    /// 
    /// # Arguments
    /// * `depth` - Depth from top (0 = top of stack, 1 = second from top, etc.)
    /// 
    /// # Errors
    /// Returns `StackUnderflow` if depth exceeds stack size
    pub fn peek(&self, depth: usize) -> Result<Word> {
        if depth >= self.data.len() {
            return Err(Error::StackUnderflow);
        }
        Ok(self.data[self.data.len() - 1 - depth])
    }
    
    /// Duplicate a value at a specific depth
    /// 
    /// # Arguments
    /// * `depth` - Depth from top (1-16, where 1 = duplicate top item)
    /// 
    /// # Errors
    /// Returns `StackUnderflow` if depth exceeds stack size
    /// Returns `StackOverflow` if stack would exceed maximum depth
    pub fn dup(&mut self, depth: usize) -> Result<()> {
        if depth == 0 || depth > 16 {
            return Err(Error::InvalidMemoryAccess("Invalid DUP depth".to_string()));
        }
        
        let value = self.peek(depth - 1)?;
        self.push(value)
    }
    
    /// Swap values at specific depths
    /// 
    /// # Arguments
    /// * `depth` - Depth from top (1-16, where 1 = swap top two items)
    /// 
    /// # Errors
    /// Returns `StackUnderflow` if depth exceeds stack size
    pub fn swap(&mut self, depth: usize) -> Result<()> {
        if depth == 0 || depth > 16 {
            return Err(Error::InvalidMemoryAccess("Invalid SWAP depth".to_string()));
        }
        
        if self.data.len() <= depth {
            return Err(Error::StackUnderflow);
        }
        
        let top_index = self.data.len() - 1;
        let swap_index = top_index - depth;
        
        self.data.swap(top_index, swap_index);
        Ok(())
    }
    
    /// Get the current stack depth
    pub fn depth(&self) -> usize {
        self.data.len()
    }
    
    /// Check if the stack is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    
    /// Check if the stack is at maximum capacity
    pub fn is_full(&self) -> bool {
        self.data.len() >= MAX_STACK_DEPTH
    }
    
    /// Get the maximum allowed depth
    pub fn max_depth() -> usize {
        MAX_STACK_DEPTH
    }
    
    /// Clear the stack
    pub fn clear(&mut self) {
        self.data.clear();
    }
    
    /// Get a reference to the stack data (for debugging)
    pub fn data(&self) -> &[Word] {
        &self.data
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_stack_push_pop() {
        let mut stack = Stack::new();
        
        // Test basic push/pop
        stack.push(Word::from(42)).unwrap();
        assert_eq!(stack.depth(), 1);
        
        let value = stack.pop().unwrap();
        assert_eq!(value, Word::from(42));
        assert!(stack.is_empty());
    }
    
    #[test]
    fn test_stack_overflow() {
        let mut stack = Stack::new();
        
        // Fill stack to maximum capacity
        for i in 0..MAX_STACK_DEPTH {
            stack.push(Word::from(i)).unwrap();
        }
        
        // Next push should fail
        assert!(stack.push(Word::from(999)).is_err());
    }
    
    #[test]
    fn test_stack_underflow() {
        let mut stack = Stack::new();
        
        // Pop from empty stack should fail
        assert!(stack.pop().is_err());
    }
    
    #[test]
    fn test_stack_peek() {
        let mut stack = Stack::new();
        
        stack.push(Word::from(1)).unwrap();
        stack.push(Word::from(2)).unwrap();
        stack.push(Word::from(3)).unwrap();
        
        // Peek at different depths
        assert_eq!(stack.peek(0).unwrap(), Word::from(3)); // Top
        assert_eq!(stack.peek(1).unwrap(), Word::from(2)); // Second
        assert_eq!(stack.peek(2).unwrap(), Word::from(1)); // Third
        
        // Peek beyond stack size should fail
        assert!(stack.peek(3).is_err());
    }
    
    #[test]
    fn test_stack_dup() {
        let mut stack = Stack::new();
        
        stack.push(Word::from(1)).unwrap();
        stack.push(Word::from(2)).unwrap();
        stack.push(Word::from(3)).unwrap();
        
        // DUP1: duplicate top item
        stack.dup(1).unwrap();
        assert_eq!(stack.depth(), 4);
        assert_eq!(stack.peek(0).unwrap(), Word::from(3)); // Top
        assert_eq!(stack.peek(1).unwrap(), Word::from(3)); // Duplicated
        
        // DUP2: duplicate second item
        stack.dup(2).unwrap();
        assert_eq!(stack.depth(), 5);
        assert_eq!(stack.peek(0).unwrap(), Word::from(3)); // Top
        assert_eq!(stack.peek(2).unwrap(), Word::from(2)); // Duplicated
    }
    
    #[test]
    fn test_stack_swap() {
        let mut stack = Stack::new();
        
        stack.push(Word::from(1)).unwrap();
        stack.push(Word::from(2)).unwrap();
        stack.push(Word::from(3)).unwrap();
        
        // SWAP1: swap top two items
        stack.swap(1).unwrap();
        assert_eq!(stack.peek(0).unwrap(), Word::from(2)); // Was second
        assert_eq!(stack.peek(1).unwrap(), Word::from(3)); // Was first
        
        // SWAP2: swap top and third items
        stack.swap(2).unwrap();
        assert_eq!(stack.peek(0).unwrap(), Word::from(1)); // Was third
        assert_eq!(stack.peek(2).unwrap(), Word::from(2)); // Was first
    }
    
    #[test]
    fn test_stack_invalid_operations() {
        let mut stack = Stack::new();
        
        // DUP with invalid depth
        assert!(stack.dup(0).is_err());
        assert!(stack.dup(17).is_err());
        
        // SWAP with invalid depth
        assert!(stack.swap(0).is_err());
        assert!(stack.swap(17).is_err());
        
        // DUP/SWAP on empty stack
        assert!(stack.dup(1).is_err());
        assert!(stack.swap(1).is_err());
    }
}
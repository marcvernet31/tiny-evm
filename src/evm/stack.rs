//! EVM Stack implementation
//! 
//! The EVM stack is a LIFO (Last In, First Out) data structure
//! that can hold up to 1024 256-bit words. It's used for temporary
//! storage during execution.

use crate::types::*;

const MAX_STACK_DEPTH: usize = 1024;


#[derive(Debug, Clone)]
pub struct Stack {
    data: Vec<Word>,
}

impl Stack {
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
    
    /// Duplicate a value at a specific depth to the top of the stack
    /// 
    /// # Arguments
    /// * `depth` - Depth from top (0-15, where 0 = duplicate top item)
    /// 
    /// # Explanation
    /// This function will be used for the DUP opcode, which is in general used to reuse function parameters, access repeated values, etc.
    /// The number 16 goes from the 16 DUP opcodes in the EVM specification (DUP1, DUP2, ..., DUP16), each one accesses the specified depth.
    /// Haters will ask why having 16 opcodes instead of just having a DUP opcode with a parameter, this is to save gas for byte space,
    /// the DUP10 opcode is only 1 byte, while any compunation of op code + numberic value would be at least 2 bytes.
    /// Depth only goes to 16 because Vitalik said so.
    /// 
    /// For dup and swap there is an argument for keepeng the iniial depth indexation to 0 (top element be 0), 
    /// because you have DUP1 and SWAP1, and can generate confusion. 
    /// I felt that is was also confusing to have diferent depths indexations for swap, dup vs peek. Perhaps I can change back in the future.
    /// 
    /// # Errors
    /// Returns `StackUnderflow` if depth exceeds stack size
    /// Returns `StackOverflow` if stack would exceed maximum depth
    pub fn dup(&mut self, depth: usize) -> Result<()> {
        if depth > 15 {
            return Err(Error::InvalidMemoryAccess("Invalid DUP depth".to_string()));
        }
        
        let value = self.peek(depth)?;
        self.push(value)
    }
    
    /// Swap value top with value at a specific depth
    /// 
    /// # Arguments
    /// * `depth` - Depth from top (0-15, where 0 = swap top two items)
    /// 
    /// # Errors
    /// Returns `StackUnderflow` if depth exceeds stack size
    pub fn swap(&mut self, depth: usize) -> Result<()> {
        if depth > 15 {
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

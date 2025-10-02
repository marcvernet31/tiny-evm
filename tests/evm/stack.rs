//! Unit tests for EVM Stack implementation

use tinyevm::evm::stack::Stack;
use tinyevm::types::*;

#[test]
fn test_stack_basic_operations() {
    let mut stack = Stack::new();
    
    // Test push/pop
    stack.push(Word::from(42)).unwrap();
    assert_eq!(stack.depth(), 1);
    
    let value = stack.pop().unwrap();
    assert_eq!(value, Word::from(42));
    assert!(stack.is_empty());
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


#[test]
fn test_stack_overflow() {
    let mut stack = Stack::new();
    
    // Fill stack to maximum capacity
    for i in 0..Stack::max_depth() {
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

    // DUP0: duplicate top item
    stack.dup(0).unwrap();
    assert_eq!(stack.depth(), 4);
    assert_eq!(stack.peek(0).unwrap(), Word::from(3)); // Top (duplicated)
    assert_eq!(stack.peek(1).unwrap(), Word::from(3)); // Original top
    
    // DUP1: duplicate second item
    stack.dup(1).unwrap();
    assert_eq!(stack.depth(), 5);
    assert_eq!(stack.peek(0).unwrap(), Word::from(3)); 
    assert_eq!(stack.peek(1).unwrap(), Word::from(3)); 
    assert_eq!(stack.peek(2).unwrap(), Word::from(3)); 
}

#[test]
fn test_stack_dup_operations() {
    let mut stack = Stack::new();
    
    stack.push(Word::from(1)).unwrap();
    stack.push(Word::from(2)).unwrap();
    stack.push(Word::from(3)).unwrap();
    
    // DUP0: duplicate top item
    stack.dup(0).unwrap(); // -> { 1, 2, 3, 3 }
    assert_eq!(stack.depth(), 4);
    assert_eq!(stack.peek(0).unwrap(), Word::from(3));
    assert_eq!(stack.peek(1).unwrap(), Word::from(3));

    // DUP1: duplicate second item
    stack.dup(2).unwrap(); // -> { 1, 2, 3, 3, 2 }
    assert_eq!(stack.depth(), 5);
    assert_eq!(stack.peek(0).unwrap(), Word::from(2));
    assert_eq!(stack.peek(1).unwrap(), Word::from(3));
    assert_eq!(stack.peek(2).unwrap(), Word::from(3));
}

#[test]
fn test_stack_swap_operations() {
    let mut stack = Stack::new();
    
    stack.push(Word::from(1)).unwrap();
    stack.push(Word::from(2)).unwrap();
    stack.push(Word::from(3)).unwrap();
    
    // SWAP0: swap top two items
    stack.swap(0).unwrap(); // -> { 1, 2, 3 }
    assert_eq!(stack.peek(0).unwrap(), Word::from(3));
    assert_eq!(stack.peek(1).unwrap(), Word::from(2));
    
    // SWAP1: swap top and third items
    stack.swap(1).unwrap(); // -> { 1, 3, 2 }
    assert_eq!(stack.peek(0).unwrap(), Word::from(2));
    assert_eq!(stack.peek(2).unwrap(), Word::from(1));
}

#[test]
fn test_stack_edge_cases() {
    let mut stack = Stack::new();
    
    // DUP with invalid depth
    assert!(stack.dup(16).is_err());
    
    // SWAP with invalid depth
    assert!(stack.swap(16).is_err());
    
    // DUP/SWAP on empty stack
    assert!(stack.dup(0).is_err());
    assert!(stack.swap(0).is_err());
}
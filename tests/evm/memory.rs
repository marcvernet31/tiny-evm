//! Unit tests for EVM Memory implementation

use tinyevm::evm::memory::Memory;
use tinyevm::types::*;

#[test]
fn test_memory_load_store() {
    let mut memory = Memory::new();
    
    // Store a word
    let value = Word::from(0x1234567890abcdefu64);
    memory.store(0, value);
    
    // Load it back
    let loaded = memory.load(0);
    assert_eq!(loaded, value);
}

#[test]
fn test_memory_load_store_with_offset() {
    let mut memory = Memory::new();
    
    // Store a word
    let value = Word::from(0x1234567890abcdefu64);
    memory.store(5, value);
    
    // Load it back
    let loaded = memory.load(5);
    assert_eq!(loaded, value);
}

#[test]
fn test_memory_load_beyond_size() {
    let mut memory = Memory::new();
    
    // Load from beyond memory size should return zero
    let value = memory.load(100);
    assert_eq!(value, Word::zero());
}

#[test]
fn test_memory_store_byte() {
    let mut memory = Memory::new();
    
    // Store individual bytes
    memory.store_byte(0, 0x12);
    memory.store_byte(1, 0x34);
    memory.store_byte(2, 0x56);
    
    // Load as word (should be zero-padded, big-endian format)
    let value = memory.load(0);

    // Use from_str_radix because from has as a parameter a 64 bit int and we want to pass a 256 bit int.
    assert_eq!(value, Word::from_str_radix("1234560000000000000000000000000000000000000000000000000000000000", 16).unwrap());
}

#[test]
fn test_memory_load_range() {
    let mut memory = Memory::new();
    
    // Store some data
    let data = vec![0x01, 0x02, 0x03, 0x04, 0x05];
    memory.store_range(10, &data);
    
    // Load range
    let loaded = memory.load_range(10, 5);
    assert_eq!(loaded, data);
    
    // Load beyond size should be zero-padded
    let loaded = memory.load_range(10, 10);
    assert_eq!(loaded, vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00]);
}

#[test]
fn test_memory_expansion_cost() {
    let mut memory = Memory::new();
    
    // Cost for first word
    let cost = memory.expansion_cost(0, 32);
    assert_eq!(cost, 3);

    memory.expand_to(32);
    
    // Cost for second word
    let cost = memory.expansion_cost(32, 32);
    assert_eq!(cost, 3);
}

#[test]
fn test_memory_size() {
    let mut memory = Memory::new();
    
    assert_eq!(memory.size(), 0);
    assert_eq!(memory.size_words(), 0);
    
    // Store something
    memory.store(0, Word::from(42));
    assert_eq!(memory.size(), 32);
    assert_eq!(memory.size_words(), 1);
    
    // Store beyond current size
    memory.store(100, Word::from(100));
    assert_eq!(memory.size(), 132);
    assert_eq!(memory.size_words(), 5);
}
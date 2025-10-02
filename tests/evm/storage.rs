//! Unit tests for EVM Storage implementation

use tinyevm::evm::storage::Storage;
use tinyevm::types::*;

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
    let key_2 = Word::from(69);

    
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
    let cost = storage.operation_cost(&key_2, &Word::zero());
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

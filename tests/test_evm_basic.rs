//! Integration tests for basic EVM functionality

use tinyevm::evm::{EVM, ExecutionContext};
use tinyevm::state::State;
use tinyevm::types::*;

#[test]
fn test_evm_creation() {
    let context = ExecutionContext::default();
    let evm = EVM::new(context, 1000);
    
    assert_eq!(evm.gas, 1000);
    assert_eq!(evm.initial_gas, 1000);
    assert_eq!(evm.pc, 0);
    assert!(!evm.stopped);
    assert!(!evm.reverted);
}

#[test]
fn test_evm_gas_operations() {
    let context = ExecutionContext::default();
    let mut evm = EVM::new(context, 1000);
    
    // Check gas
    assert!(evm.check_gas(500).is_ok());
    assert!(evm.check_gas(1500).is_err());
    
    // Consume gas
    evm.consume_gas(300).unwrap();
    assert_eq!(evm.gas, 700);
    
    // Try to consume too much gas
    assert!(evm.consume_gas(800).is_err());
}

#[test]
fn test_evm_execution_states() {
    let context = ExecutionContext::default();
    let mut evm = EVM::new(context, 1000);
    
    // Test stop
    evm.stop();
    assert!(evm.stopped);
    
    // Test revert
    let mut evm = EVM::new(context, 1000);
    evm.revert("Test revert".to_string());
    assert!(evm.reverted);
    assert_eq!(evm.return_data, b"Test revert");
    
    // Test return data
    let mut evm = EVM::new(context, 1000);
    evm.return_data(vec![0x01, 0x02, 0x03]);
    assert!(evm.stopped);
    assert_eq!(evm.return_data, vec![0x01, 0x02, 0x03]);
}

#[test]
fn test_evm_stack_operations() {
    let context = ExecutionContext::default();
    let mut evm = EVM::new(context, 1000);
    
    // Test stack operations
    evm.stack.push(Word::from(42)).unwrap();
    evm.stack.push(Word::from(24)).unwrap();
    
    assert_eq!(evm.stack.depth(), 2);
    assert_eq!(evm.stack.peek(0).unwrap(), Word::from(24));
    assert_eq!(evm.stack.peek(1).unwrap(), Word::from(42));
    
    let value = evm.stack.pop().unwrap();
    assert_eq!(value, Word::from(24));
    assert_eq!(evm.stack.depth(), 1);
}

#[test]
fn test_evm_memory_operations() {
    let context = ExecutionContext::default();
    let mut evm = EVM::new(context, 1000);
    
    // Test memory operations
    evm.memory.store(0, Word::from(0x1234567890abcdefu64));
    evm.memory.store(32, Word::from(0xfedcba0987654321u64));
    
    let value1 = evm.memory.load(0);
    let value2 = evm.memory.load(32);
    
    assert_eq!(value1, Word::from(0x1234567890abcdefu64));
    assert_eq!(value2, Word::from(0xfedcba0987654321u64));
    
    // Test memory range operations
    let data = vec![0x01, 0x02, 0x03, 0x04];
    evm.memory.store_range(100, &data);
    let loaded = evm.memory.load_range(100, 4);
    assert_eq!(loaded, data);
}

#[test]
fn test_evm_storage_operations() {
    let context = ExecutionContext::default();
    let mut evm = EVM::new(context, 1000);
    
    // Test storage operations
    evm.storage.store(Word::from(1), Word::from(100));
    evm.storage.store(Word::from(2), Word::from(200));
    
    assert_eq!(evm.storage.load(&Word::from(1)), Word::from(100));
    assert_eq!(evm.storage.load(&Word::from(2)), Word::from(200));
    assert_eq!(evm.storage.load(&Word::from(3)), Word::zero());
}

#[test]
fn test_evm_execution_with_empty_code() {
    let context = ExecutionContext {
        code: vec![],
        ..Default::default()
    };
    let mut evm = EVM::new(context, 1000);
    
    // Execute with empty code should complete immediately
    let result = evm.execute().unwrap();
    assert!(result.success);
    assert_eq!(result.gas_used, 0);
    assert!(result.output.is_empty());
}

#[test]
fn test_evm_execution_context() {
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
        data,
        code,
        block,
        gas_price,
    );
    
    let evm = EVM::new(context, 1000);
    
    assert_eq!(evm.context.address, address);
    assert_eq!(evm.context.caller, caller);
    assert_eq!(evm.context.origin, origin);
    assert_eq!(evm.context.value, value);
    assert_eq!(evm.context.data, data);
    assert_eq!(evm.context.code, code);
    assert_eq!(evm.context.gas_price, gas_price);
}
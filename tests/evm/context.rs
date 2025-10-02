//! Unit tests for EVM Context implementation

use tinyevm::evm::context::ExecutionContext;
use tinyevm::types::*;


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
    
    // Load first 8 bytes as word (all available data)
    // With << 192 we are shifting the bits 192 positions which will make the first 8 bytes the last 8 bytes of the word
    // Before shift: 0x123456789abcdef0
    // After shift: 0x123456789abcdef000000000000000000000000000000000000000000000000000
    let word = context.load_data(0);
    assert_eq!(word, Word::from(0x123456789abcdef0u64) << 192);
    
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
    assert_eq!(word, Word::from(0x6001600201u64) << 216);
    
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
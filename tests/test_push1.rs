//! Tests for PUSH1 opcode implementation

use tinyevm::evm::context::ExecutionContext;
use tinyevm::*;
use tinyevm::evm::*;
use tinyevm::evm::opcodes::*;

#[test]
fn test_push1_basic() {
    // Test basic PUSH1 functionality
    // Bytecode: PUSH1 0x42 (push 66)
    let bytecode = vec![0x60, 0x42]; // PUSH1 0x42
    
    let context = ExecutionContext {
        address: Address::zero(),
        caller: Address::zero(),
        origin: Address::zero(),
        value: Word::zero(),
        data: vec![],
        code: bytecode,
        block: BlockContext {
            number: 1,
            timestamp: 1000,
            difficulty: Word::zero(),
            gas_limit: 1000000,
            coinbase: Address::zero(),
            chain_id: 1,
            base_fee: Some(Word::zero()),
        },
        gas_price: Word::zero(),
        is_static: false,
    };
    
    let mut evm = EVM::new(context, 100000);
    let result = evm.execute().unwrap();
    
    assert!(result.success);
    assert_eq!(evm.stack.depth(), 1);
    assert_eq!(evm.stack.peek(0).unwrap(), Word::from(0x42));
    assert_eq!(evm.pc, 2); // Should be at position 2 (after PUSH1 0x42)
}

#[test]
fn test_push1_zero() {
    // Test PUSH1 with zero value
    let bytecode = vec![0x60, 0x00]; // PUSH1 0x00
    
    let context = ExecutionContext {
        address: Address::zero(),
        caller: Address::zero(),
        origin: Address::zero(),
        value: Word::zero(),
        data: vec![],
        code: bytecode,
        block: BlockContext {
            number: 1,
            timestamp: 1000,
            difficulty: Word::zero(),
            gas_limit: 1000000,
            coinbase: Address::zero(),
            chain_id: 1,
            base_fee: Some(Word::zero()),
        },
        gas_price: Word::zero(),
        is_static: false,
    };
    
    let mut evm = EVM::new(context, 100000);
    let result = evm.execute().unwrap();
    
    assert!(result.success);
    assert_eq!(evm.stack.depth(), 1);
    assert_eq!(evm.stack.peek(0).unwrap(), Word::zero());
}

#[test]
fn test_push1_max_value() {
    // Test PUSH1 with maximum byte value (0xFF = 255)
    let bytecode = vec![0x60, 0xFF]; // PUSH1 0xFF
    
    let context = ExecutionContext {
        address: Address::zero(),
        caller: Address::zero(),
        origin: Address::zero(),
        value: Word::zero(),
        data: vec![],
        code: bytecode,
        block: BlockContext {
            number: 1,
            timestamp: 1000,
            difficulty: Word::zero(),
            gas_limit: 1000000,
            coinbase: Address::zero(),
            chain_id: 1,
            base_fee: Some(Word::zero()),
        },
        gas_price: Word::zero(),
        is_static: false,
    };
    
    let mut evm = EVM::new(context, 100000);
    let result = evm.execute().unwrap();
    
    assert!(result.success);
    assert_eq!(evm.stack.depth(), 1);
    assert_eq!(evm.stack.peek(0).unwrap(), Word::from(0xFF));
}

#[test]
fn test_push1_multiple() {
    // Test multiple PUSH1 operations
    let bytecode = vec![0x60, 0x01, 0x60, 0x02, 0x60, 0x03]; // PUSH1 1, PUSH1 2, PUSH1 3
    
    let context = ExecutionContext {
        address: Address::zero(),
        caller: Address::zero(),
        origin: Address::zero(),
        value: Word::zero(),
        data: vec![],
        code: bytecode,
        block: BlockContext {
            number: 1,
            timestamp: 1000,
            difficulty: Word::zero(),
            gas_limit: 1000000,
            coinbase: Address::zero(),
            chain_id: 1,
            base_fee: Some(Word::zero()),
        },
        gas_price: Word::zero(),
        is_static: false,
    };
    
    let mut evm = EVM::new(context, 100000);
    let result = evm.execute().unwrap();
    
    assert!(result.success);
    assert_eq!(evm.stack.depth(), 3);
    assert_eq!(evm.stack.peek(0).unwrap(), Word::from(3)); // Top of stack
    assert_eq!(evm.stack.peek(1).unwrap(), Word::from(2));
    assert_eq!(evm.stack.peek(2).unwrap(), Word::from(1)); // Bottom of stack
}

#[test]
fn test_push1_insufficient_code() {
    // Test PUSH1 with insufficient code (missing immediate byte)
    let bytecode = vec![0x60]; // PUSH1 without immediate byte
    
    let context = ExecutionContext {
        address: Address::zero(),
        caller: Address::zero(),
        origin: Address::zero(),
        value: Word::zero(),
        data: vec![],
        code: bytecode,
        block: BlockContext {
            number: 1,
            timestamp: 1000,
            difficulty: Word::zero(),
            gas_limit: 1000000,
            coinbase: Address::zero(),
            chain_id: 1,
            base_fee: Some(Word::zero()),
        },
        gas_price: Word::zero(),
        is_static: false,
    };
    
    let mut evm = EVM::new(context, 100000);
    let result = evm.execute();
    
    assert!(result.is_err());
    match result.unwrap_err() {
        Error::InvalidJump(_) => {}, // Expected error
        _ => panic!("Expected InvalidJump error"),
    }
}

#[test]
fn test_push1_gas_consumption() {
    // Test that PUSH1 consumes the correct amount of gas
    let bytecode = vec![0x60, 0x42]; // PUSH1 0x42
    
    let context = ExecutionContext {
        address: Address::zero(),
        caller: Address::zero(),
        origin: Address::zero(),
        value: Word::zero(),
        data: vec![],
        code: bytecode,
        block: BlockContext {
            number: 1,
            timestamp: 1000,
            difficulty: Word::zero(),
            gas_limit: 1000000,
            coinbase: Address::zero(),
            chain_id: 1,
            base_fee: Some(Word::zero()),
        },
        gas_price: Word::zero(),
        is_static: false,
    };
    
    let initial_gas = 100000;
    let mut evm = EVM::new(context, initial_gas);
    let result = evm.execute().unwrap();
    
    assert!(result.success);
    
    // PUSH1 should consume 3 gas (VERY_LOW)
    let expected_gas_used = 3;
    assert_eq!(result.gas_used, expected_gas_used);
    assert_eq!(evm.gas, initial_gas - expected_gas_used);
}

#[test]
fn test_push1_with_other_opcodes() {
    // Test PUSH1 in combination with other opcodes (POP)
    // TODO: Implement this test once we got more opcodes
}

#[test]
fn test_push1_stack_overflow() {
    // Test PUSH1 when stack is at maximum capacity
    let mut bytecode = vec![];
    
    // Fill stack to capacity (1024 items) with PUSH1 0x01
    for _ in 0..1024 {
        bytecode.extend_from_slice(&[0x60, 0x01]);
    }
    
    // Add one more PUSH1 to trigger stack overflow
    bytecode.extend_from_slice(&[0x60, 0x01]);
    
    let context = ExecutionContext {
        address: Address::zero(),
        caller: Address::zero(),
        origin: Address::zero(),
        value: Word::zero(),
        data: vec![],
        code: bytecode,
        block: BlockContext {
            number: 1,
            timestamp: 1000,
            difficulty: Word::zero(),
            gas_limit: 1000000,
            coinbase: Address::zero(),
            chain_id: 1,
            base_fee: Some(Word::zero()),
        },
        gas_price: Word::zero(),
        is_static: false,
    };
    
    let mut evm = EVM::new(context, 100000);
    let result = evm.execute();
    
    assert!(result.is_err());
    match result.unwrap_err() {
        Error::StackOverflow => {}, // Expected error
        _ => panic!("Expected StackOverflow error"),
    }
}

#[test]
fn test_push1_opcode_enum() {
    // Test that PUSH1 opcode is correctly identified
    assert_eq!(Opcode::from_byte(0x60), Some(Opcode::PUSH1));
    assert_eq!(Opcode::PUSH1 as u8, 0x60);
    assert_eq!(Opcode::PUSH1.immediate_bytes(), 1);
    assert!(!Opcode::PUSH1.is_jump());
}

#[test]
fn test_push1_gas_cost() {
    // Test that PUSH1 has the correct gas cost
    use tinyevm::gas::*;
    assert_eq!(Opcode::PUSH1.gas_cost(), costs::PUSH1);
    assert_eq!(Opcode::PUSH1.gas_cost(), costs::VERY_LOW);
}
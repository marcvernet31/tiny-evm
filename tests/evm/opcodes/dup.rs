use tinyevm::evm::EVM;
use tinyevm::evm::context::ExecutionContext;
use tinyevm::evm::opcodes::Opcode;
use tinyevm::types::{Address, Word, BlockContext};

#[test]
fn test_dup1_basic() {
    // Test DUP1 functionality - duplicate top item
    let bytecode = vec![
        0x60, 0x42,           // PUSH1 0x42
        0x80,                  // DUP1
    ];
    
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
    assert_eq!(evm.stack.depth(), 2);
    // After DUP1: stack should have 0x42, 0x42 (top to bottom)
    assert_eq!(evm.stack.peek(0).unwrap(), Word::from(0x42));
    assert_eq!(evm.stack.peek(1).unwrap(), Word::from(0x42));
    assert_eq!(evm.pc, 3); // Should be at position 3 (after PUSH1 + DUP1)
}

#[test]
fn test_dup1_zero_values() {
    // Test DUP1 with zero values
    let bytecode = vec![
        0x60, 0x00,           // PUSH1 0x00
        0x80,                  // DUP1
    ];
    
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
    assert_eq!(evm.stack.depth(), 2);
    assert_eq!(evm.stack.peek(0).unwrap(), Word::zero());
    assert_eq!(evm.stack.peek(1).unwrap(), Word::zero());
}

#[test]
fn test_dup1_max_values() {
    // Test DUP1 with maximum values
    let bytecode = vec![
        0x63, 0xff, 0xff, 0xff, 0xff, // PUSH4 0xffffffff
        0x80,                         // DUP1
    ];
    
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
    assert_eq!(evm.stack.depth(), 2);
    let max_value = Word::from(0xFFFFFFFFu32);
    assert_eq!(evm.stack.peek(0).unwrap(), max_value);
    assert_eq!(evm.stack.peek(1).unwrap(), max_value);
}

#[test]
fn test_dup2_basic() {
    // Test DUP2 functionality - duplicate second item
    let bytecode = vec![
        0x60, 0x11,           // PUSH1 0x11
        0x60, 0x22,           // PUSH1 0x22
        0x81,                  // DUP2
    ];
    
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
    // After DUP2: stack should have 0x11, 0x22, 0x11 (top to bottom)
    assert_eq!(evm.stack.peek(0).unwrap(), Word::from(0x11));
    assert_eq!(evm.stack.peek(1).unwrap(), Word::from(0x22));
    assert_eq!(evm.stack.peek(2).unwrap(), Word::from(0x11));
    assert_eq!(evm.pc, 5); // Should be at position 5 (after PUSH1 + PUSH1 + DUP2)
}

#[test]
fn test_dup3_basic() {
    // Test DUP3 functionality - duplicate third item
    let bytecode = vec![
        0x60, 0x11,           // PUSH1 0x11
        0x60, 0x22,           // PUSH1 0x22
        0x60, 0x33,           // PUSH1 0x33
        0x82,                  // DUP3
    ];
    
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
    assert_eq!(evm.stack.depth(), 4);
    // After DUP3: stack should have 0x11, 0x33, 0x22, 0x11 (top to bottom)
    assert_eq!(evm.stack.peek(0).unwrap(), Word::from(0x11));
    assert_eq!(evm.stack.peek(1).unwrap(), Word::from(0x33));
    assert_eq!(evm.stack.peek(2).unwrap(), Word::from(0x22));
    assert_eq!(evm.stack.peek(3).unwrap(), Word::from(0x11));
    assert_eq!(evm.pc, 7); // Should be at position 7 (after PUSH1 * 3 + DUP3)
}

#[test]
fn test_dup16_basic() {
    // Test DUP16 functionality with 16 stack items
    let bytecode = vec![
        0x60, 0x01, // PUSH1 0x01
        0x60, 0x02, // PUSH1 0x02
        0x60, 0x03, // PUSH1 0x03
        0x60, 0x04, // PUSH1 0x04
        0x60, 0x05, // PUSH1 0x05
        0x60, 0x06, // PUSH1 0x06
        0x60, 0x07, // PUSH1 0x07
        0x60, 0x08, // PUSH1 0x08
        0x60, 0x09, // PUSH1 0x09
        0x60, 0x0a, // PUSH1 0x0a
        0x60, 0x0b, // PUSH1 0x0b
        0x60, 0x0c, // PUSH1 0x0c
        0x60, 0x0d, // PUSH1 0x0d
        0x60, 0x0e, // PUSH1 0x0e
        0x60, 0x0f, // PUSH1 0x0f
        0x60, 0x10, // PUSH1 0x10
        0x8f,       // DUP16
    ];
    
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
    assert_eq!(evm.stack.depth(), 17);
    // After DUP16: stack should have 0x01, 0x10, 0x0f, ..., 0x02, 0x01 (top to bottom)
    assert_eq!(evm.stack.peek(0).unwrap(), Word::from(0x01)); // Original 1st item
    assert_eq!(evm.stack.peek(1).unwrap(), Word::from(0x10)); // Original 16th item
    assert_eq!(evm.stack.peek(2).unwrap(), Word::from(0x0f)); // Original 15th item
    assert_eq!(evm.stack.peek(16).unwrap(), Word::from(0x01)); // Original 1st item (duplicated)
    assert_eq!(evm.pc, 33); // PUSH1 (2 bytes) * 16 + DUP16 (1 byte) = 32 + 1 = 33
}

#[test]
fn test_dup_insufficient_stack() {
    // Test DUP with insufficient stack items
    let bytecode = vec![
        0x60, 0x42,           // PUSH1 0x42
        0x82,                  // DUP3 (requires 3 items, but only 1 available)
    ];
    
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
    // Should fail with stack underflow
}

#[test]
fn test_dup_edge_cases() {
    // Test DUP with edge case values
    let bytecode = vec![
        0x60, 0x00,           // PUSH1 0x00
        0x60, 0xff,           // PUSH1 0xff
        0x60, 0x01,            // PUSH1 0x01
        0x82,                  // DUP3 (duplicate 0x00)
    ];
    
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
    assert_eq!(evm.stack.depth(), 4);
    // After DUP3: stack should have 0x00, 0x01, 0xff, 0x00 (top to bottom)
    assert_eq!(evm.stack.peek(0).unwrap(), Word::from(0x00));
    assert_eq!(evm.stack.peek(1).unwrap(), Word::from(0x01));
    assert_eq!(evm.stack.peek(2).unwrap(), Word::from(0xff));
    assert_eq!(evm.stack.peek(3).unwrap(), Word::from(0x00));
}

#[test]
fn test_multiple_dup_operations() {
    // Test multiple DUP operations
    let bytecode = vec![
        0x60, 0x11,           // PUSH1 0x11
        0x60, 0x22,           // PUSH1 0x22
        0x60, 0x33,           // PUSH1 0x33
        0x80,                  // DUP1 (duplicate 0x33)
        0x81,                  // DUP2 (duplicate 0x22)
        0x82,                  // DUP3 (duplicate 0x11)
    ];
    
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
    assert_eq!(evm.stack.depth(), 6);
    // After operations: 0x33, 0x33, 0x33, 0x33, 0x22, 0x11 (top to bottom)
    assert_eq!(evm.stack.peek(0).unwrap(), Word::from(0x33));
    assert_eq!(evm.stack.peek(1).unwrap(), Word::from(0x33));
    assert_eq!(evm.stack.peek(2).unwrap(), Word::from(0x33));
    assert_eq!(evm.stack.peek(3).unwrap(), Word::from(0x33));
    assert_eq!(evm.stack.peek(4).unwrap(), Word::from(0x22));
    assert_eq!(evm.stack.peek(5).unwrap(), Word::from(0x11));
    assert_eq!(evm.pc, 9); // Should be at position 9 (after all operations)
}

#[test]
fn test_dup_with_other_opcodes() {
    // Test DUP combined with other stack operations
    let bytecode = vec![
        0x60, 0x42,           // PUSH1 0x42
        0x60, 0x24,           // PUSH1 0x24
        0x80,                  // DUP1 (duplicate 0x24)
        0x60, 0x33,            // PUSH1 0x33 (add another item)
    ];
    
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
    assert_eq!(evm.stack.depth(), 4);
    // After DUP1 and PUSH1: stack should have 0x33, 0x24, 0x24, 0x42 (top to bottom)
    assert_eq!(evm.stack.peek(0).unwrap(), Word::from(0x33));
    assert_eq!(evm.stack.peek(1).unwrap(), Word::from(0x24));
    assert_eq!(evm.stack.peek(2).unwrap(), Word::from(0x24));
    assert_eq!(evm.stack.peek(3).unwrap(), Word::from(0x42));
}

#[test]
fn test_dup_gas_consumption() {
    // Test that DUP operations consume gas correctly
    let bytecode = vec![
        0x60, 0x42,           // PUSH1 0x42
        0x80,                  // DUP1
    ];
    
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
    // Gas should be consumed (exact amount depends on implementation)
    assert!(evm.gas < 100000);
}

#[test]
fn test_dup_opcode_enum() {
    // Test that DUP opcodes are correctly identified
    assert_eq!(Opcode::DUP1 as u8, 0x80);
    assert_eq!(Opcode::DUP2 as u8, 0x81);
    assert_eq!(Opcode::DUP3 as u8, 0x82);
    assert_eq!(Opcode::DUP16 as u8, 0x8f);
    
    // Test opcode identification methods
    assert!(Opcode::DUP1.is_dup());
    assert!(Opcode::DUP2.is_dup());
    assert!(Opcode::DUP16.is_dup());
    assert!(!Opcode::PUSH1.is_dup());
    assert!(!Opcode::SWAP1.is_dup());
}

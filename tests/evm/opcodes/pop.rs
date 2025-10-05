use tinyevm::evm::EVM;
use tinyevm::evm::context::ExecutionContext;
use tinyevm::evm::opcodes::Opcode;
use tinyevm::types::{Address, Word, BlockContext};

#[test]
fn test_pop_basic() {
    // Test POP functionality - remove top item
    let bytecode = vec![
        0x60, 0x42,           // PUSH1 0x42
        0x60, 0x24,           // PUSH1 0x24
        0x50,                 // POP (remove 0x24)
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
    assert_eq!(evm.stack.depth(), 1);
    // After POP: only 0x42 should remain
    assert_eq!(evm.stack.peek(0).unwrap(), Word::from(0x42));
    assert_eq!(evm.pc, 5); // Should be at position 5 (after PUSH1 + PUSH1 + POP)
}

#[test]
fn test_pop_single_item() {
    // Test POP with single item on stack
    let bytecode = vec![
        0x60, 0x42,           // PUSH1 0x42
        0x50,                 // POP
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
    assert_eq!(evm.stack.depth(), 0);
}

#[test]
fn test_pop_empty_stack() {
    // Test POP with empty stack - should fail
    let bytecode = vec![
        0x50,                 // POP (no items to pop)
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
fn test_multiple_pop_operations() {
    // Test multiple POP operations
    let bytecode = vec![
        0x60, 0x11,           // PUSH1 0x11
        0x60, 0x22,           // PUSH1 0x22
        0x60, 0x33,           // PUSH1 0x33
        0x60, 0x44,           // PUSH1 0x44
        0x50,                 // POP (remove 0x44)
        0x50,                 // POP (remove 0x33)
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
    // After two POPs: stack should have 0x22, 0x11 (top to bottom)
    assert_eq!(evm.stack.peek(0).unwrap(), Word::from(0x22));
    assert_eq!(evm.stack.peek(1).unwrap(), Word::from(0x11));
    assert_eq!(evm.pc, 10); // Should be at position 10 (after all operations)
}

#[test]
fn test_pop_with_push_operations() {
    // Test POP combined with PUSH operations
    let bytecode = vec![
        0x60, 0x11,           // PUSH1 0x11
        0x60, 0x22,           // PUSH1 0x22
        0x50,                 // POP (remove 0x22)
        0x60, 0x33,           // PUSH1 0x33
        0x50,                 // POP (remove 0x33)
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
    assert_eq!(evm.stack.depth(), 1);
    // After operations: only 0x11 should remain
    assert_eq!(evm.stack.peek(0).unwrap(), Word::from(0x11));
}

#[test]
fn test_pop_with_dup_operations() {
    // Test POP combined with DUP operations
    let bytecode = vec![
        0x60, 0x42,           // PUSH1 0x42
        0x80,                 // DUP1 (duplicate 0x42)
        0x50,                 // POP (remove one 0x42)
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
    assert_eq!(evm.stack.depth(), 1);
    // After DUP1 and POP: only one 0x42 should remain
    assert_eq!(evm.stack.peek(0).unwrap(), Word::from(0x42));
}

#[test]
fn test_pop_with_swap_operations() {
    // Test POP combined with SWAP operations
    let bytecode = vec![
        0x60, 0x11,           // PUSH1 0x11
        0x60, 0x22,           // PUSH1 0x22
        0x60, 0x33,           // PUSH1 0x33
        0x90,                 // SWAP1 (swap 0x33 and 0x22)
        0x50,                 // POP (remove 0x22 which is now on top)
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
    // After SWAP1 and POP: stack should have 0x33, 0x11 (top to bottom)
    assert_eq!(evm.stack.peek(0).unwrap(), Word::from(0x33));
    assert_eq!(evm.stack.peek(1).unwrap(), Word::from(0x11));
}

#[test]
fn test_pop_zero_values() {
    // Test POP with zero values
    let bytecode = vec![
        0x60, 0x00,           // PUSH1 0x00
        0x60, 0x00,           // PUSH1 0x00
        0x50,                 // POP
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
    assert_eq!(evm.stack.depth(), 1);
    assert_eq!(evm.stack.peek(0).unwrap(), Word::zero());
}

#[test]
fn test_pop_max_values() {
    // Test POP with maximum values
    let bytecode = vec![
        0x63, 0xff, 0xff, 0xff, 0xff, // PUSH4 0xffffffff
        0x63, 0xee, 0xee, 0xee, 0xee, // PUSH4 0xeeeeeeee
        0x50,                          // POP (remove 0xeeeeeeee)
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
    assert_eq!(evm.stack.depth(), 1);
    let max_value = Word::from(0xFFFFFFFFu32);
    assert_eq!(evm.stack.peek(0).unwrap(), max_value);
}

#[test]
fn test_pop_gas_consumption() {
    // Test that POP operations consume gas correctly
    let bytecode = vec![
        0x60, 0x42,           // PUSH1 0x42
        0x50,                 // POP
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
fn test_pop_opcode_enum() {
    // Test that POP opcode is correctly identified
    assert_eq!(Opcode::POP as u8, 0x50);
    
    // Test opcode identification methods
    assert!(Opcode::POP.is_stack_opcode());
    assert!(!Opcode::POP.is_push());
    assert!(!Opcode::POP.is_dup());
    assert!(!Opcode::POP.is_swap());
}

#[test]
fn test_pop_all_items() {
    // Test popping all items from stack
    let bytecode = vec![
        0x60, 0x11,           // PUSH1 0x11
        0x60, 0x22,           // PUSH1 0x22
        0x60, 0x33,           // PUSH1 0x33
        0x50,                 // POP
        0x50,                 // POP
        0x50,                 // POP
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
    assert_eq!(evm.stack.depth(), 0);
}

#[test]
fn test_pop_underflow_after_operations() {
    // Test POP underflow after other operations
    let bytecode = vec![
        0x60, 0x42,           // PUSH1 0x42
        0x50,                 // POP
        0x50,                 // POP (should fail - stack empty)
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


use tinyevm::evm::EVM;
use tinyevm::evm::context::ExecutionContext;
use tinyevm::evm::opcodes::Opcode;
use tinyevm::types::{Address, Word, BlockContext};

#[test]
fn test_add_basic() {
    // Test ADD functionality - add two numbers
    let bytecode = vec![
        0x60, 0x05,           // PUSH1 5
        0x60, 0x03,           // PUSH1 3
        0x01,                 // ADD (3 + 5 = 8)
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
    assert_eq!(evm.stack.peek(0).unwrap(), Word::from(8));
    assert_eq!(evm.pc, 5); // After PUSH1 + PUSH1 + ADD
}

#[test]
fn test_add_zero() {
    // Test ADD with zero
    let bytecode = vec![
        0x60, 0x00,           // PUSH1 0
        0x60, 0x05,           // PUSH1 5
        0x01,                 // ADD (5 + 0 = 5)
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
    assert_eq!(evm.stack.peek(0).unwrap(), Word::from(5));
}

#[test]
fn test_add_both_zero() {
    // Test ADD with both operands zero
    let bytecode = vec![
        0x60, 0x00,           // PUSH1 0
        0x60, 0x00,           // PUSH1 0
        0x01,                 // ADD (0 + 0 = 0)
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
fn test_add_large_numbers() {
    // Test ADD with larger numbers
    let bytecode = vec![
        0x61, 0x03, 0xe8,     // PUSH2 1000
        0x61, 0x07, 0xd0,     // PUSH2 2000
        0x01,                 // ADD (2000 + 1000 = 3000)
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
    assert_eq!(evm.stack.peek(0).unwrap(), Word::from(3000));
}

#[test]
fn test_add_max_values() {
    // Test ADD with maximum 4-byte values
    let bytecode = vec![
        0x63, 0xff, 0xff, 0xff, 0xff, // PUSH4 0xffffffff (4294967295)
        0x60, 0x01,                     // PUSH1 1
        0x01,                           // ADD (should wrap in U256 space)
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
    // 0xffffffff + 1 = 0x100000000 (no overflow in U256)
    assert_eq!(evm.stack.peek(0).unwrap(), Word::from(0x100000000u64));
}

#[test]
fn test_add_multiple_operations() {
    // Test multiple ADD operations: (5 + 3) + 2 = 10
    let bytecode = vec![
        0x60, 0x05,           // PUSH1 5
        0x60, 0x03,           // PUSH1 3
        0x01,                 // ADD (5 + 3 = 8)
        0x60, 0x02,           // PUSH1 2
        0x01,                 // ADD (8 + 2 = 10)
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
    assert_eq!(evm.stack.peek(0).unwrap(), Word::from(10));
    assert_eq!(evm.pc, 8); // After all operations
}

#[test]
fn test_add_with_dup() {
    // Test ADD combined with DUP: duplicate 5, then add (5 + 5 = 10)
    let bytecode = vec![
        0x60, 0x05,           // PUSH1 5
        0x80,                 // DUP1 (duplicate 5)
        0x01,                 // ADD (5 + 5 = 10)
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
    assert_eq!(evm.stack.peek(0).unwrap(), Word::from(10));
}

#[test]
fn test_add_insufficient_stack() {
    // Test ADD with insufficient stack items (only 1 item)
    let bytecode = vec![
        0x60, 0x05,           // PUSH1 5
        0x01,                 // ADD (should fail - needs 2 items)
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
fn test_add_empty_stack() {
    // Test ADD with empty stack
    let bytecode = vec![
        0x01,                 // ADD (should fail - empty stack)
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
fn test_add_commutative() {
    // Test that ADD is commutative: a + b = b + a
    // First: 5 + 3
    let bytecode1 = vec![
        0x60, 0x05,           // PUSH1 5
        0x60, 0x03,           // PUSH1 3
        0x01,                 // ADD
    ];
    
    let context1 = ExecutionContext {
        address: Address::zero(),
        caller: Address::zero(),
        origin: Address::zero(),
        value: Word::zero(),
        data: vec![],
        code: bytecode1,
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
    
    let mut evm1 = EVM::new(context1, 100000);
    let result1 = evm1.execute().unwrap();
    let value1 = evm1.stack.peek(0).unwrap();
    
    // Second: 3 + 5
    let bytecode2 = vec![
        0x60, 0x03,           // PUSH1 3
        0x60, 0x05,           // PUSH1 5
        0x01,                 // ADD
    ];
    
    let context2 = ExecutionContext {
        address: Address::zero(),
        caller: Address::zero(),
        origin: Address::zero(),
        value: Word::zero(),
        data: vec![],
        code: bytecode2,
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
    
    let mut evm2 = EVM::new(context2, 100000);
    let result2 = evm2.execute().unwrap();
    let value2 = evm2.stack.peek(0).unwrap();
    
    assert!(result1.success && result2.success);
    assert_eq!(value1, value2);
    assert_eq!(value1, Word::from(8));
}

#[test]
fn test_add_gas_consumption() {
    // Test that ADD operations consume gas correctly
    let bytecode = vec![
        0x60, 0x05,           // PUSH1 5
        0x60, 0x03,           // PUSH1 3
        0x01,                 // ADD
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
    // Gas should be consumed
    assert!(evm.gas < 100000);
    // ADD costs 3 gas, PUSH1 costs 3 gas each
    assert!(result.gas_used > 0);
}

#[test]
fn test_add_opcode_enum() {
    // Test that ADD opcode is correctly identified
    assert_eq!(Opcode::ADD as u8, 0x01);
}

#[test]
fn test_add_chain_operations() {
    // Test chaining: 1 + 2 + 3 + 4 = 10
    let bytecode = vec![
        0x60, 0x01,           // PUSH1 1
        0x60, 0x02,           // PUSH1 2
        0x01,                 // ADD (1 + 2 = 3)
        0x60, 0x03,           // PUSH1 3
        0x01,                 // ADD (3 + 3 = 6)
        0x60, 0x04,           // PUSH1 4
        0x01,                 // ADD (6 + 4 = 10)
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
    assert_eq!(evm.stack.peek(0).unwrap(), Word::from(10));
}

// ============================================================================
// MUL TESTS
// ============================================================================

#[test]
fn test_mul_basic() {
    // Test MUL functionality - multiply two numbers
    let bytecode = vec![
        0x60, 0x05,           // PUSH1 5
        0x60, 0x03,           // PUSH1 3
        0x02,                 // MUL (3 * 5 = 15)
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
    assert_eq!(evm.stack.peek(0).unwrap(), Word::from(15));
}

#[test]
fn test_mul_zero() {
    // Test MUL with zero
    let bytecode = vec![
        0x60, 0x00,           // PUSH1 0
        0x60, 0x05,           // PUSH1 5
        0x02,                 // MUL (5 * 0 = 0)
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

// ============================================================================
// SUB TESTS
// ============================================================================

#[test]
fn test_sub_basic() {
    // Test SUB functionality - subtract two numbers
    let bytecode = vec![
        0x60, 0x05,           // PUSH1 5
        0x60, 0x03,           // PUSH1 3
        0x03,                 // SUB (5 - 3 = 2)
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
    assert_eq!(evm.stack.peek(0).unwrap(), Word::from(2));
}

#[test]
fn test_sub_underflow() {
    // Test SUB underflow (wrapping behavior)
    // 3 - 5 should wrap around (not saturate to 0)
    let bytecode = vec![
        0x60, 0x05,           // PUSH1 5
        0x60, 0x03,           // PUSH1 3
        0x03,                 // SUB (3 - 5 = wraps to large number)
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
    // 3 - 5 = -2 in two's complement = 2^256 - 2 (a very large number)
    // Should NOT be 0 (that would be saturating)
    assert_ne!(evm.stack.peek(0).unwrap(), Word::zero());
}

// ============================================================================
// DIV TESTS
// ============================================================================

#[test]
fn test_div_basic() {
    // Test DIV functionality - divide two numbers
    let bytecode = vec![
        0x60, 0x02,           // PUSH1 2
        0x60, 0x0a,           // PUSH1 10
        0x04,                 // DIV (10 / 2 = 5)
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
    assert_eq!(evm.stack.peek(0).unwrap(), Word::from(5));
}

#[test]
fn test_div_by_zero() {
    // Test DIV by zero - should return 0 (not error!)
    let bytecode = vec![
        0x60, 0x00,           // PUSH1 0
        0x60, 0x0a,           // PUSH1 10
        0x04,                 // DIV (10 / 0 = 0)
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
    assert_eq!(evm.stack.peek(0).unwrap(), Word::zero());  // Returns 0!
}

// ============================================================================
// MOD TESTS
// ============================================================================

#[test]
fn test_mod_basic() {
    // Test MOD functionality - modulo operation
    let bytecode = vec![
        0x60, 0x03,           // PUSH1 3
        0x60, 0x0a,           // PUSH1 10
        0x06,                 // MOD (10 % 3 = 1)
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
    assert_eq!(evm.stack.peek(0).unwrap(), Word::from(1));
}

#[test]
fn test_mod_by_zero() {
    // Test MOD by zero - should return 0 (not error!)
    let bytecode = vec![
        0x60, 0x00,           // PUSH1 0
        0x60, 0x0a,           // PUSH1 10
        0x06,                 // MOD (10 % 0 = 0)
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
    assert_eq!(evm.stack.peek(0).unwrap(), Word::zero());  // Returns 0!
}

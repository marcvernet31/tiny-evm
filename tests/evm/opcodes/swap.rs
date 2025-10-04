//! Tests for SWAP opcode implementation

use tinyevm::evm::context::ExecutionContext;
use tinyevm::*;
use tinyevm::evm::*;
use tinyevm::evm::opcodes::*;

#[test]
fn test_swap1_basic() {
    // Test basic SWAP1 functionality
    // Bytecode: PUSH1 0x42, PUSH1 0x24, SWAP1
    let bytecode = vec![0x60, 0x42, 0x60, 0x24, 0x90]; // PUSH1 0x42, PUSH1 0x24, SWAP1
    
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
    // After SWAP1: stack should have 0x42, 0x24 (top to bottom)
    assert_eq!(evm.stack.peek(0).unwrap(), Word::from(0x42));
    assert_eq!(evm.stack.peek(1).unwrap(), Word::from(0x24));
    assert_eq!(evm.pc, 5); // Should be at position 5 (after all operations)
}

#[test]
fn test_swap1_zero_values() {
    // Test SWAP1 with zero values
    let bytecode = vec![0x60, 0x00, 0x60, 0x00, 0x90]; // PUSH1 0x00, PUSH1 0x00, SWAP1
    
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
    assert_eq!(evm.stack.peek(0).unwrap(), Word::from(0x00));
    assert_eq!(evm.stack.peek(1).unwrap(), Word::from(0x00));
}

#[test]
fn test_swap1_max_values() {
    // Test SWAP1 with maximum values
    let bytecode = vec![0x60, 0xFF, 0x60, 0xFE, 0x90]; // PUSH1 0xFF, PUSH1 0xFE, SWAP1
    
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
    assert_eq!(evm.stack.peek(0).unwrap(), Word::from(0xFF));
    assert_eq!(evm.stack.peek(1).unwrap(), Word::from(0xFE));
}

#[test]
fn test_swap2_basic() {
    // Test SWAP2 functionality
    // Bytecode: PUSH1 0x11, PUSH1 0x22, PUSH1 0x33, SWAP2
    let bytecode = vec![0x60, 0x11, 0x60, 0x22, 0x60, 0x33, 0x91]; // PUSH1 0x11, PUSH1 0x22, PUSH1 0x33, SWAP2
    
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
    // After SWAP2: stack should have 0x11, 0x22, 0x33 (top to bottom)
    assert_eq!(evm.stack.peek(0).unwrap(), Word::from(0x11));
    assert_eq!(evm.stack.peek(1).unwrap(), Word::from(0x22));
    assert_eq!(evm.stack.peek(2).unwrap(), Word::from(0x33));
}

#[test]
fn test_swap3_basic() {
    // Test SWAP3 functionality
    // Bytecode: PUSH1 0x11, PUSH1 0x22, PUSH1 0x33, PUSH1 0x44, SWAP3
    let bytecode = vec![0x60, 0x11, 0x60, 0x22, 0x60, 0x33, 0x60, 0x44, 0x92]; // PUSH1 0x11, PUSH1 0x22, PUSH1 0x33, PUSH1 0x44, SWAP3
    
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
    // After SWAP3: stack should have 0x11, 0x33, 0x22, 0x44 (top to bottom)
    assert_eq!(evm.stack.peek(0).unwrap(), Word::from(0x11));
    assert_eq!(evm.stack.peek(1).unwrap(), Word::from(0x33));
    assert_eq!(evm.stack.peek(2).unwrap(), Word::from(0x22));
    assert_eq!(evm.stack.peek(3).unwrap(), Word::from(0x44));
}

#[test]
fn test_swap16_basic() {
    // Test SWAP16 functionality with 17 stack items
    let mut bytecode = Vec::new();
    
    // Push 17 values (0x01 to 0x11)
    for i in 1..=17 {
        bytecode.push(0x60); // PUSH1
        bytecode.push(i);    // value
    }
    
    // Add SWAP16
    bytecode.push(0x9F); // SWAP16
    
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
    // After SWAP16: first and 17th items should be swapped
    assert_eq!(evm.stack.peek(0).unwrap(), Word::from(0x01));  // 17th item moved to top
    assert_eq!(evm.stack.peek(16).unwrap(), Word::from(0x11)); // 1st item moved to 17th position
}

#[test]
fn test_swap_insufficient_stack() {
    // Test SWAP1 with insufficient stack items (only 1 item)
    let bytecode = vec![0x60, 0x42, 0x90]; // PUSH1 0x42, SWAP1
    
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
    
    // Should fail because SWAP1 needs at least 2 stack items
    assert!(result.is_err());
}

#[test]
fn test_swap_gas_consumption() {
    // Test that SWAP operations consume gas
    let bytecode = vec![0x60, 0x42, 0x60, 0x24, 0x90]; // PUSH1 0x42, PUSH1 0x24, SWAP1
    
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
    let initial_gas = evm.gas;
    let result = evm.execute().unwrap();
    
    assert!(result.success);
    assert!(evm.gas < initial_gas); // Gas should be consumed
}

#[test]
fn test_multiple_swap_operations() {
    // Test multiple SWAP operations in sequence
    let bytecode = vec![
        0x60, 0x11,           // PUSH1 0x11
        0x60, 0x22,           // PUSH1 0x22
        0x60, 0x33,           // PUSH1 0x33
        0x90,                 // SWAP1 (swap 0x33 and 0x22)
        0x91,                 // SWAP2 (swap 0x22 and 0x11)
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
    
    // After operations: 0x11, 0x33, 0x22 (top to bottom)
    assert_eq!(evm.stack.peek(0).unwrap(), Word::from(0x11));
    assert_eq!(evm.stack.peek(1).unwrap(), Word::from(0x33));
    assert_eq!(evm.stack.peek(2).unwrap(), Word::from(0x22));
    assert_eq!(evm.pc, 8); // Should be at position 8 (after all operations)
}

#[test]
fn test_swap_with_other_opcodes() {
    // Test SWAP combined with other stack operations
    let bytecode = vec![
        0x60, 0x42,           // PUSH1 0x42
        0x60, 0x24,           // PUSH1 0x24
        0x90,                 // SWAP1
        0x60, 0x33,           // PUSH1 0x33 (add another item)
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
    // After SWAP1 and PUSH1: stack should have 0x33, 0x42, 0x24 (top to bottom)
    assert_eq!(evm.stack.peek(0).unwrap(), Word::from(0x33));
    assert_eq!(evm.stack.peek(1).unwrap(), Word::from(0x42));
    assert_eq!(evm.stack.peek(2).unwrap(), Word::from(0x24));
}

#[test]
fn test_swap_opcode_enum() {
    // Test that SWAP opcodes are correctly identified
    assert_eq!(Opcode::SWAP1 as u8, 0x90);
    assert_eq!(Opcode::SWAP2 as u8, 0x91);
    assert_eq!(Opcode::SWAP3 as u8, 0x92);
    assert_eq!(Opcode::SWAP16 as u8, 0x9F);
}

#[test]
fn test_swap_edge_cases() {
    // Test SWAP with edge case values
    let bytecode = vec![
        0x60, 0x00,           // PUSH1 0x00
        0x60, 0xFF,           // PUSH1 0xFF
        0x90,                 // SWAP1
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
    assert_eq!(evm.stack.peek(0).unwrap(), Word::from(0x00));
    assert_eq!(evm.stack.peek(1).unwrap(), Word::from(0xFF));
}

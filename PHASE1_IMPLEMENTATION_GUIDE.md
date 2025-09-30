# Phase 1: Foundation - Implementation Guide

This guide provides detailed instructions for implementing Phase 1 of the TinyEVM project. Phase 1 focuses on building the foundational infrastructure that will support the EVM execution engine.

## Overview

Phase 1 establishes the core data structures and basic functionality needed for EVM execution:

1. **Core Types** - Fundamental data types (Address, Hash, Word, etc.)
2. **EVM Core** - Main EVM struct and execution framework
3. **Stack** - LIFO data structure for temporary storage
4. **Memory** - Linear byte-addressable memory
5. **Storage** - Persistent key-value storage
6. **State Management** - Account and world state management
7. **Gas Metering** - Gas calculation and consumption tracking

## File Structure

```
src/
├── main.rs                 # Entry point with basic tests
├── types.rs               # Core types and error definitions
├── evm/
│   ├── mod.rs            # Main EVM struct and execution loop
│   ├── stack.rs          # Stack implementation
│   ├── memory.rs         # Memory implementation
│   ├── storage.rs        # Storage implementation
│   └── context.rs        # Execution context
├── state/
│   └── mod.rs            # State management
└── gas/
    └── mod.rs            # Gas metering system

tests/
├── unit/                 # Unit tests for individual components
│   ├── test_stack.rs
│   ├── test_memory.rs
│   ├── test_storage.rs
│   ├── test_gas.rs
│   └── test_state.rs
└── integration/          # Integration tests
    └── test_evm_basic.rs
```

## Implementation Steps

### Step 1: Core Types (`src/types.rs`)

**Purpose**: Define fundamental data types used throughout the EVM.

**Key Components**:
- `Address` - 20-byte Ethereum address
- `Hash` - 32-byte Keccak hash
- `Word` - 256-bit EVM word
- `Bytes` - Dynamic byte array
- `Error` enum - Comprehensive error handling
- `ExecutionResult` - Result of EVM execution
- `BlockContext` - Block information for execution

**Implementation Notes**:
- Use `ethereum-types` crate for Address, Hash, and Word
- Implement utility methods for common operations
- Use `thiserror` for clean error handling
- Add `Serialize`/`Deserialize` for future JSON-RPC support

### Step 2: Stack Implementation (`src/evm/stack.rs`)

**Purpose**: Implement the EVM execution stack (LIFO, max 1024 items).

**Key Methods**:
- `push(value)` - Push value onto stack
- `pop()` - Pop value from stack
- `peek(depth)` - Peek at value at specific depth
- `dup(depth)` - Duplicate value at specific depth
- `swap(depth)` - Swap values at specific depths

**Implementation Notes**:
- Use `Vec<Word>` for internal storage
- Enforce 1024 item limit
- Handle stack overflow/underflow errors
- Support DUP1-DUP16 and SWAP1-SWAP16 operations
- Add comprehensive unit tests

### Step 3: Memory Implementation (`src/evm/memory.rs`)

**Purpose**: Implement linear, byte-addressable memory that can grow dynamically.

**Key Methods**:
- `load(offset)` - Load 32-byte word from memory
- `store(offset, value)` - Store 32-byte word to memory
- `store_byte(offset, value)` - Store single byte
- `load_range(offset, size)` - Load range of bytes
- `store_range(offset, data)` - Store range of bytes
- `expansion_cost(offset, size)` - Calculate gas cost for expansion

**Implementation Notes**:
- Use `Vec<u8>` for internal storage
- Automatically expand memory as needed
- Zero-pad when loading beyond memory size
- Implement gas cost calculation for memory expansion
- Handle word alignment (32-byte boundaries)

### Step 4: Storage Implementation (`src/evm/storage.rs`)

**Purpose**: Implement persistent key-value storage for contracts.

**Key Methods**:
- `load(key)` - Load value from storage
- `store(key, value)` - Store value in storage
- `operation_cost(key, new_value)` - Calculate gas cost
- `operation_refund(key, new_value)` - Calculate gas refund

**Implementation Notes**:
- Use `HashMap<Word, Word>` for internal storage
- Remove keys when storing zero values (gas optimization)
- Implement proper gas cost calculation
- Handle storage refunds for clearing slots
- Add utility methods for debugging

### Step 5: Execution Context (`src/evm/context.rs`)

**Purpose**: Hold execution context information for EVM operations.

**Key Components**:
- `address` - Contract address being executed
- `caller` - Address that initiated the call
- `origin` - Address that signed the original transaction
- `value` - ETH value sent with the call
- `data` - Input data for the call
- `code` - Bytecode being executed
- `block` - Block context information

**Key Methods**:
- `load_data(offset)` - Load word from input data
- `load_data_range(offset, size)` - Load range from input data
- `load_code(offset)` - Load word from bytecode
- `load_code_range(offset, size)` - Load range from bytecode

**Implementation Notes**:
- Support both contract calls and contract creation
- Handle static calls (no state modifications)
- Provide convenient data access methods
- Zero-pad when accessing beyond data/code bounds

### Step 6: State Management (`src/state/mod.rs`)

**Purpose**: Manage world state including accounts, balances, and storage.

**Key Components**:
- `Account` - Account information (balance, nonce, code hash)
- `State` - World state manager
- `StateSnapshot` - For reverting failed operations

**Key Methods**:
- `get_account(address)` - Get account information
- `get_balance(address)` - Get account balance
- `add_balance(address, amount)` - Add balance
- `sub_balance(address, amount)` - Subtract balance
- `transfer(from, to, amount)` - Transfer between accounts
- `get_code(address)` - Get contract code
- `set_code(address, code)` - Set contract code
- `load_storage(address, key)` - Load from contract storage
- `store_storage(address, key, value)` - Store to contract storage
- `snapshot()` - Create state snapshot
- `revert_to_snapshot(snapshot)` - Revert to snapshot

**Implementation Notes**:
- Distinguish between EOAs and contracts
- Implement proper balance checks
- Support state snapshots for reverting
- Handle nonce management
- Link storage to contract addresses

### Step 7: Gas Metering (`src/gas/mod.rs`)

**Purpose**: Track gas consumption and calculate costs for operations.

**Key Components**:
- `GasMeter` - Gas consumption tracker
- `costs` module - Gas cost constants
- Utility functions for complex gas calculations

**Key Methods**:
- `consume(amount)` - Consume gas
- `has_gas(required)` - Check if enough gas available
- `add_refund(amount)` - Add gas refund
- `apply_refunds()` - Apply refunds (up to 1/2 of gas used)

**Gas Cost Functions**:
- `memory_expansion_cost()` - Memory expansion costs
- `exp_cost()` - Exponentiation costs
- `sha3_cost()` - SHA3 operation costs
- `log_cost()` - Log operation costs
- `call_cost()` - Call operation costs

**Implementation Notes**:
- Implement all standard EVM gas costs
- Handle gas refunds properly
- Support complex gas calculations
- Add comprehensive cost constants

### Step 8: EVM Core (`src/evm/mod.rs`)

**Purpose**: Main EVM struct that orchestrates execution.

**Key Components**:
- `EVM` struct - Main execution engine
- `execute()` - Main execution loop
- Gas cost constants
- Execution state management

**Key Methods**:
- `new(context, gas_limit)` - Create new EVM instance
- `execute()` - Execute bytecode until completion
- `check_gas(required)` - Check if enough gas available
- `consume_gas(amount)` - Consume gas for operation
- `stop()` - Stop execution
- `revert(reason)` - Revert execution with reason
- `return_data(data)` - Return data and stop

**Implementation Notes**:
- Integrate all components (stack, memory, storage, gas)
- Implement basic execution loop (opcodes in Phase 2)
- Handle execution states (stopped, reverted)
- Support execution results and logging

## Testing Strategy

### Unit Tests

Each component should have comprehensive unit tests:

1. **Stack Tests** (`tests/unit/test_stack.rs`)
   - Basic push/pop operations
   - Stack overflow/underflow
   - DUP and SWAP operations
   - Edge cases and error conditions

2. **Memory Tests** (`tests/unit/test_memory.rs`)
   - Load/store operations
   - Memory expansion
   - Range operations
   - Gas cost calculations

3. **Storage Tests** (`tests/unit/test_storage.rs`)
   - Load/store operations
   - Zero value handling
   - Gas cost calculations
   - Refund calculations

4. **Gas Tests** (`tests/unit/test_gas.rs`)
   - Gas consumption
   - Refund handling
   - Cost calculations
   - Edge cases

5. **State Tests** (`tests/unit/test_state.rs`)
   - Account operations
   - Balance management
   - Code storage
   - Snapshot/revert functionality

### Integration Tests

Test component interactions:

1. **EVM Basic Tests** (`tests/integration/test_evm_basic.rs`)
   - EVM creation and initialization
   - Gas operations
   - Execution states
   - Component integration

### Running Tests

```bash
# Run all tests
cargo test

# Run unit tests only
cargo test --test unit

# Run integration tests only
cargo test --test integration

# Run specific test file
cargo test --test test_stack

# Run with output
cargo test -- --nocapture
```

## Validation Checklist

Before moving to Phase 2, ensure:

- [ ] All unit tests pass
- [ ] All integration tests pass
- [ ] Stack operations work correctly
- [ ] Memory operations work correctly
- [ ] Storage operations work correctly
- [ ] Gas metering works correctly
- [ ] State management works correctly
- [ ] Error handling is comprehensive
- [ ] Documentation is complete
- [ ] Code follows Rust best practices

## Common Pitfalls

1. **Stack Overflow**: Always check stack depth before pushing
2. **Memory Bounds**: Handle out-of-bounds memory access gracefully
3. **Gas Underflow**: Check gas before consuming
4. **State Consistency**: Ensure state snapshots work correctly
5. **Error Propagation**: Use proper error types and propagation
6. **Word Alignment**: Ensure 32-byte alignment for memory operations
7. **Zero Values**: Handle zero values correctly in storage

## Next Steps

After completing Phase 1:

1. **Phase 2**: Implement basic opcodes (arithmetic, stack, memory)
2. **Phase 3**: Add state management and storage opcodes
3. **Phase 4**: Implement transaction execution
4. **Phase 5**: Add advanced features (calls, creates)
5. **Phase 6**: Polish and optimization

## Resources

- [Ethereum Yellow Paper](https://ethereum.github.io/yellowpaper/paper.pdf)
- [EVM Opcodes Reference](https://ethereum.org/en/developers/docs/evm/opcodes/)
- [Rust Documentation](https://doc.rust-lang.org/book/)
- [Ethereum Types Crate](https://docs.rs/ethereum-types/latest/ethereum_types/)

## Support

If you encounter issues:

1. Check the test cases for expected behavior
2. Review the error messages carefully
3. Ensure all dependencies are properly configured
4. Verify gas calculations match the specification
5. Check memory alignment and bounds

Remember: Phase 1 is the foundation. Take time to get it right - it will make all subsequent phases much easier!
# TinyEVM: High-Level Implementation Guide

> A production-quality Ethereum Virtual Machine implementation in Rust

## Project Vision

Build an EVM that can execute real Solidity smart contracts and serve as a killer portfolio piece. The goal is to show you understand virtual machines, state management, cryptography, and complex system design.

**Timeline:** 6-8 weeks  
**Difficulty:** Medium-High  
**Impact:** Career-defining portfolio project

---

## System Architecture

```
                    ┌─────────────────┐
                    │   CLI/RPC API   │
                    └────────┬────────┘
                             │
                    ┌────────▼────────┐
                    │   Transaction   │
                    │    Executor     │
                    └────────┬────────┘
                             │
              ┌──────────────┼──────────────┐
              │              │              │
     ┌────────▼────────┐    │    ┌────────▼────────┐
     │   World State   │◄───┼───►│   EVM Engine    │
     │  (Accounts/     │    │    │  (Interpreter)  │
     │   Storage)      │    │    └────────┬────────┘
     └─────────────────┘    │             │
                            │    ┌────────▼────────┐
                            │    │  Stack/Memory/  │
                            │    │    Storage      │
                            │    └─────────────────┘
                            │
                   ┌────────▼────────┐
                   │   Gas Metering  │
                   └─────────────────┘
```

---

## Component Breakdown

### 1. **EVM Core** (The Heart)
The virtual machine that executes bytecode.

**What it does:**
- Reads bytecode instruction by instruction
- Maintains execution stack, memory, and storage
- Executes ~140 opcodes (ADD, MUL, JUMP, CALL, etc.)
- Tracks gas consumption
- Returns execution results

**Key files:**
```
src/evm/
  ├── mod.rs           # Main EVM struct and execution loop
  ├── stack.rs         # Stack operations (max 1024 items)
  ├── memory.rs        # Linear byte-addressable memory
  ├── storage.rs       # Persistent key-value storage
  └── opcodes/
      ├── arithmetic.rs  # ADD, MUL, DIV, etc.
      ├── comparison.rs  # LT, GT, EQ, etc.
      ├── bitwise.rs     # AND, OR, XOR, SHL, SHR
      ├── stack.rs       # PUSH, POP, DUP, SWAP
      ├── memory.rs      # MLOAD, MSTORE
      ├── storage.rs     # SLOAD, SSTORE
      ├── control.rs     # JUMP, JUMPI, RETURN
      ├── context.rs     # CALLER, CALLVALUE, etc.
      └── crypto.rs      # SHA3 (Keccak256)
```

**Dependencies:** Stack, Memory, Storage  
**Used by:** Transaction Executor

---

### 2. **World State** (The Database)
Manages all accounts, balances, contract code, and storage.

**What it does:**
- Stores account balances and nonces
- Stores smart contract bytecode
- Manages contract storage (key-value per contract)
- Handles state snapshots for reverting failed calls

**Key files:**
```
src/state/
  ├── mod.rs        # State manager (main interface)
  ├── account.rs    # Account structure (balance, nonce, code)
  └── database.rs   # Storage backend (HashMap for now)
```

**Dependencies:** None (it's the foundation)  
**Used by:** EVM Core, Transaction Executor

---

### 3. **Transaction Executor** (The Orchestrator)
Validates and executes transactions, updating world state.

**What it does:**
- Validates transactions (signature, nonce, balance)
- Deducts gas upfront
- Calls EVM for contract execution or contract creation
- Refunds unused gas
- Updates account balances
- Produces transaction receipts

**Key files:**
```
src/executor/
  ├── mod.rs          # Main executor logic
  └── validator.rs    # Transaction validation
```

**Dependencies:** EVM Core, World State  
**Used by:** CLI/RPC layer

---

### 4. **Transaction & Types** (The Data Layer)
Core data structures used throughout the system.

**What it includes:**
- Transaction structure (to, value, data, signature)
- Address (20 bytes)
- Hash (32 bytes)
- U256 (256-bit unsigned integer)
- Execution context (caller, origin, block info)
- Execution result (success, gas used, output)

**Key files:**
```
src/
  ├── types.rs        # Address, Hash, U256 type aliases
  ├── transaction.rs  # Transaction struct and methods
  └── context.rs      # Execution and block context
```

**Dependencies:** External crates (primitive-types, ethereum-types)  
**Used by:** Everything

---

### 5. **Gas Metering** (The Cost System)
Tracks computational costs to prevent infinite loops.

**What it does:**
- Assigns gas cost to each opcode
- Tracks gas consumption during execution
- Handles memory expansion costs
- Throws OutOfGas errors

**Key files:**
```
src/gas/
  ├── mod.rs       # Gas constants and calculation
  └── schedule.rs  # Gas costs per opcode
```

**Dependencies:** None  
**Used by:** EVM Core

---

### 6. **CLI/RPC Interface** (The User Interface)
How users interact with your EVM.

**What it does:**
- Command-line tool for deploying contracts
- JSON-RPC server for programmatic access
- Read contract state
- Send transactions

**Key files:**
```
src/
  ├── main.rs       # CLI entry point
  └── rpc/
      ├── mod.rs    # JSON-RPC server
      └── methods.rs # eth_call, eth_sendTransaction, etc.
```

**Dependencies:** Transaction Executor, World State  
**Used by:** End users

---

## Implementation Phases

### Phase 1: Foundation (Week 1)
**Goal:** Get basic infrastructure working

**Build:**
1. Project setup with proper dependencies
2. Core types (Address, Hash, U256)
3. Stack implementation (push, pop, dup, swap)
4. Memory implementation (load, store)
5. Storage implementation (simple HashMap)

**Milestone:** Can push/pop values and read/write memory

---

### Phase 2: Basic EVM (Weeks 2-3)
**Goal:** Execute simple bytecode

**Build:**
1. EVM struct with execution loop
2. Implement 30-40 core opcodes:
   - Arithmetic: ADD, SUB, MUL, DIV, MOD
   - Stack: PUSH1-32, POP, DUP1-16, SWAP1-16
   - Comparison: LT, GT, EQ, ISZERO
   - Bitwise: AND, OR, XOR, NOT
   - Control: JUMP, JUMPI, JUMPDEST, STOP
   - Memory: MLOAD, MSTORE
   - Context: CALLER, CALLVALUE, ADDRESS
3. Gas metering for each opcode
4. Simple test suite

**Milestone:** Can execute bytecode like: PUSH1 5 PUSH1 3 ADD (result: 8)

---

### Phase 3: State Management (Week 4)
**Goal:** Persistent state and accounts

**Build:**
1. Account structure (balance, nonce, code hash)
2. World state manager
3. Storage operations (SLOAD, SSTORE)
4. Account balance transfers
5. Nonce management

**Milestone:** Can store and retrieve contract storage values

---

### Phase 4: Transactions (Week 5)
**Goal:** Execute full transactions

**Build:**
1. Transaction structure with signature
2. Transaction validation (nonce, balance, signature recovery)
3. Transaction executor
4. Value transfers
5. Contract deployment (CREATE)
6. Gas refunds and miner payments

**Milestone:** Can deploy a contract and call its functions

---

### Phase 5: Advanced Features (Week 6)
**Goal:** Support contract interactions

**Build:**
1. CALL opcode (contract-to-contract calls)
2. RETURN and REVERT opcodes
3. Event logs (LOG0-LOG4)
4. Remaining opcodes (SHA3, BLOCKHASH, etc.)
5. State snapshots for reverting

**Milestone:** Can deploy and interact with ERC-20 token

---

### Phase 6: Polish & Testing (Weeks 7-8)
**Goal:** Production quality

**Build:**
1. Comprehensive test suite
2. Ethereum official test vectors
3. CLI tool for easy interaction
4. Performance benchmarks
5. Documentation and examples
6. Optional: JSON-RPC server

**Milestone:** Passes Ethereum test suite, has benchmarks vs other EVMs

---

## Where to Start: Day 1 Action Plan

### Step 1: Project Setup (30 minutes)
```bash
cargo new tinyevm
cd tinyevm
```

Add to `Cargo.toml`:
```toml
[dependencies]
primitive-types = "0.12"
ethereum-types = "0.14"
sha3 = "0.10"
thiserror = "1.0"
anyhow = "1.0"
```

### Step 2: Create Basic Types (1 hour)
Create `src/types.rs`:
```rust
pub use ethereum_types::{H160 as Address, H256 as Hash, U256};
pub type Bytes = Vec<u8>;
```

### Step 3: Build Stack (2 hours)
Create `src/stack.rs`:
- Implement Stack struct with Vec<U256>
- Add push, pop, dup, swap methods
- Add stack depth limit (1024)
- Write basic tests

### Step 4: Build Memory (2 hours)
Create `src/memory.rs`:
- Implement Memory struct with Vec<u8>
- Add load/store methods for U256 values
- Handle automatic expansion
- Write tests

### Step 5: First Opcode (1 hour)
Create `src/evm/mod.rs`:
- Create basic EVM struct
- Implement PUSH1 and ADD opcodes
- Write test that pushes 5, pushes 3, adds them

**End of Day 1:** You should have a mini-EVM that can execute: `PUSH1 5 PUSH1 3 ADD` and return 8

---

## Testing Strategy

### Unit Tests
Test each opcode individually:
```rust
#[test]
fn test_add_opcode() {
    let mut evm = EVM::new();
    evm.stack.push(U256::from(5)).unwrap();
    evm.stack.push(U256::from(3)).unwrap();
    evm.op_add().unwrap();
    assert_eq!(evm.stack.pop().unwrap(), U256::from(8));
}
```

### Integration Tests
Test contract deployment and execution:
```rust
#[test]
fn test_simple_contract() {
    let bytecode = hex::decode("6005600301").unwrap(); // PUSH1 5 PUSH1 3 ADD STOP
    let result = execute_bytecode(bytecode);
    assert!(result.success);
}
```

### Ethereum Test Vectors
Use official Ethereum test suite (later phases):
```rust
#[test]
fn test_ethereum_general_state_tests() {
    // Load JSON test files
    // Execute and compare results
}
```

### Benchmark Tests
Compare performance against other EVMs:
```rust
#[bench]
fn bench_fibonacci_contract(b: &mut Bencher) {
    let contract = deploy_fibonacci();
    b.iter(|| contract.call("fib", &[U256::from(20)]));
}
```

---

## Success Criteria

### Minimum Viable Product (MVP)
✅ Executes basic bytecode (arithmetic, stack ops)  
✅ Implements ~50 core opcodes  
✅ Has working memory and storage  
✅ Can deploy simple contracts  
✅ Gas metering works  

### Portfolio-Ready
✅ All above +  
✅ Implements 100+ opcodes  
✅ Passes Ethereum test vectors  
✅ Can run real Solidity contracts (ERC-20)  
✅ Has CLI for deployment and interaction  
✅ Documented with technical blog post  
✅ Performance benchmarks included  

### Career-Defining
✅ All above +  
✅ JSON-RPC server implementation  
✅ Contract-to-contract calls working  
✅ Performance competitive with other implementations  
✅ Optional: Simple blockchain layer  
✅ Optional: Merkle Patricia Trie for state  

---

## Key Resources

### Documentation
- **Ethereum Yellow Paper**: Complete EVM specification
- **EVM Opcodes**: evm.codes (interactive opcode reference)
- **Solidity Docs**: For understanding contract behavior

### Reference Implementations
- **revm** (Rust): Study for performance patterns
- **evmone** (C++): Study for optimization techniques
- **go-ethereum**: Reference for correctness

### Testing
- **Ethereum Tests Repo**: github.com/ethereum/tests
- **Remix IDE**: For generating test bytecode
- **Foundry**: For Solidity compilation

---

## Next Steps

1. **Read this document fully** - Understand the big picture
2. **Follow Day 1 plan** - Get your hands dirty immediately
3. **Build incrementally** - Don't jump ahead, master each layer
4. **Test obsessively** - Every opcode should have tests
5. **Document as you go** - You'll write about this later

The key is to start simple and build up. Don't try to implement everything at once. Get one opcode working, then two, then ten. Before you know it, you'll have a working EVM.

Ready to start coding?

### 1.1 Project Setup

**Cargo.toml dependencies:**
```toml
[dependencies]
# Big integer arithmetic
primitive-types = "0.12"  # U256, H256, H160
ethereum-types = "0.14"

# Hashing & Crypto
sha3 = "0.10"
secp256k1 = { version = "0.28", features = ["recovery"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
hex = "0.4"
rlp = "0.5"

# Error handling
thiserror = "1.0"
anyhow = "1.0"

# Testing
proptest = "1.0"

[dev-dependencies]
criterion = "0.5"
```

### 1.2 Basic Types

**File: `src/types.rs`**

Implement these fundamental types:

```rust
// Address: 20-byte Ethereum address
pub type Address = H160;

// Hash: 32-byte Keccak hash
pub type Hash = H256;

// Word: 256-bit EVM word
pub type Word = U256;

// Bytes: Dynamic byte array
pub type Bytes = Vec<u8>;
```

### 1.3 EVM Core Structures

**File: `src/evm/mod.rs`**

```rust
pub struct EVM {
    /// Execution stack (max 1024 items)
    stack: Stack,
    
    /// Linear memory (byte-addressable)
    memory: Memory,
    
    /// Persistent storage (word -> word mapping)
    storage: Storage,
    
    /// Program counter
    pc: usize,
    
    /// Gas remaining
    gas: u64,
    
    /// Current execution context
    context: ExecutionContext,
    
    /// Return data from last call
    return_data: Bytes,
    
    /// Execution result
    stopped: bool,
    reverted: bool,
}
```

**File: `src/evm/stack.rs`**

```rust
pub struct Stack {
    data: Vec<Word>,
}

impl Stack {
    const MAX_DEPTH: usize = 1024;
    
    pub fn new() -> Self;
    pub fn push(&mut self, value: Word) -> Result<()>;
    pub fn pop(&mut self) -> Result<Word>;
    pub fn peek(&self, depth: usize) -> Result<Word>;
    pub fn swap(&mut self, depth: usize) -> Result<()>;
    pub fn dup(&mut self, depth: usize) -> Result<()>;
}
```

**File: `src/evm/memory.rs`**

```rust
pub struct Memory {
    data: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self;
    pub fn load(&mut self, offset: usize) -> Word;
    pub fn store(&mut self, offset: usize, value: Word);
    pub fn store_byte(&mut self, offset: usize, value: u8);
    pub fn load_range(&mut self, offset: usize, size: usize) -> &[u8];
    pub fn store_range(&mut self, offset: usize, data: &[u8]);
    pub fn size(&self) -> usize;
    
    // Gas calculation for memory expansion
    pub fn expansion_cost(&self, offset: usize, size: usize) -> u64;
}
```

**File: `src/evm/storage.rs`**

```rust
use std::collections::HashMap;

pub struct Storage {
    data: HashMap<Word, Word>,
}

impl Storage {
    pub fn new() -> Self;
    pub fn load(&self, key: &Word) -> Word;
    pub fn store(&mut self, key: Word, value: Word);
}
```

### 1.4 Execution Context

**File: `src/evm/context.rs`**

```rust
pub struct ExecutionContext {
    /// Contract address being executed
    pub address: Address,
    
    /// Caller address
    pub caller: Address,
    
    /// Transaction origin
    pub origin: Address,
    
    /// ETH value sent
    pub value: Word,
    
    /// Input data
    pub data: Bytes,
    
    /// Bytecode being executed
    pub code: Bytes,
    
    /// Block context
    pub block: BlockContext,
}

pub struct BlockContext {
    pub number: u64,
    pub timestamp: u64,
    pub difficulty: Word,
    pub gas_limit: u64,
    pub coinbase: Address,
    pub chain_id: u64,
}
```

---

## Phase 2: Opcode Implementation (Weeks 2-3)

### 2.1 Opcode Enum

**File: `src/evm/opcode.rs`**

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Opcode {
    // Arithmetic (0x00-0x0f)
    STOP = 0x00,
    ADD = 0x01,
    MUL = 0x02,
    SUB = 0x03,
    DIV = 0x04,
    SDIV = 0x05,
    MOD = 0x06,
    SMOD = 0x07,
    ADDMOD = 0x08,
    MULMOD = 0x09,
    EXP = 0x0a,
    SIGNEXTEND = 0x0b,
    
    // Comparison (0x10-0x1f)
    LT = 0x10,
    GT = 0x11,
    SLT = 0x12,
    SGT = 0x13,
    EQ = 0x14,
    ISZERO = 0x15,
    AND = 0x16,
    OR = 0x17,
    XOR = 0x18,
    NOT = 0x19,
    BYTE = 0x1a,
    SHL = 0x1b,
    SHR = 0x1c,
    SAR = 0x1d,
    
    // Crypto (0x20)
    SHA3 = 0x20,
    
    // Context (0x30-0x3f)
    ADDRESS = 0x30,
    BALANCE = 0x31,
    ORIGIN = 0x32,
    CALLER = 0x33,
    CALLVALUE = 0x34,
    CALLDATALOAD = 0x35,
    CALLDATASIZE = 0x36,
    CALLDATACOPY = 0x37,
    CODESIZE = 0x38,
    CODECOPY = 0x39,
    GASPRICE = 0x3a,
    EXTCODESIZE = 0x3b,
    EXTCODECOPY = 0x3c,
    RETURNDATASIZE = 0x3d,
    RETURNDATACOPY = 0x3e,
    EXTCODEHASH = 0x3f,
    
    // Block (0x40-0x4f)
    BLOCKHASH = 0x40,
    COINBASE = 0x41,
    TIMESTAMP = 0x42,
    NUMBER = 0x43,
    DIFFICULTY = 0x44,
    GASLIMIT = 0x45,
    CHAINID = 0x46,
    SELFBALANCE = 0x47,
    BASEFEE = 0x48,
    
    // Storage & Memory (0x50-0x5f)
    POP = 0x50,
    MLOAD = 0x51,
    MSTORE = 0x52,
    MSTORE8 = 0x53,
    SLOAD = 0x54,
    SSTORE = 0x55,
    JUMP = 0x56,
    JUMPI = 0x57,
    PC = 0x58,
    MSIZE = 0x59,
    GAS = 0x5a,
    JUMPDEST = 0x5b,
    
    // Push (0x60-0x7f)
    PUSH1 = 0x60,
    PUSH2 = 0x61,
    // ... PUSH3-PUSH31
    PUSH32 = 0x7f,
    
    // Dup (0x80-0x8f)
    DUP1 = 0x80,
    DUP2 = 0x81,
    // ... DUP3-DUP15
    DUP16 = 0x8f,
    
    // Swap (0x90-0x9f)
    SWAP1 = 0x90,
    SWAP2 = 0x91,
    // ... SWAP3-SWAP15
    SWAP16 = 0x9f,
    
    // Logging (0xa0-0xa4)
    LOG0 = 0xa0,
    LOG1 = 0xa1,
    LOG2 = 0xa2,
    LOG3 = 0xa3,
    LOG4 = 0xa4,
    
    // System (0xf0-0xff)
    CREATE = 0xf0,
    CALL = 0xf1,
    CALLCODE = 0xf2,
    RETURN = 0xf3,
    DELEGATECALL = 0xf4,
    CREATE2 = 0xf5,
    STATICCALL = 0xfa,
    REVERT = 0xfd,
    INVALID = 0xfe,
    SELFDESTRUCT = 0xff,
}

impl Opcode {
    pub fn from_byte(byte: u8) -> Option<Self>;
    pub fn gas_cost(&self) -> u64;
}
```

### 2.2 Opcode Implementation Strategy

Implement opcodes in this order (easiest to hardest):

**Week 2: Basic Operations (Days 1-4)**
1. Stack operations: PUSH, POP, DUP, SWAP
2. Arithmetic: ADD, SUB, MUL, DIV, MOD
3. Comparison: LT, GT, EQ, ISZERO
4. Bitwise: AND, OR, XOR, NOT, SHL, SHR
5. Control flow: JUMP, JUMPI, JUMPDEST, STOP, PC

**Week 2: Memory & Context (Days 5-7)**
6. Memory: MLOAD, MSTORE, MSTORE8, MSIZE
7. Context: CALLER, CALLVALUE, ADDRESS, ORIGIN
8. Calldata: CALLDATALOAD, CALLDATASIZE, CALLDATACOPY

**Week 3: Advanced Operations (Days 1-3)**
9. Storage: SLOAD, SSTORE
10. Crypto: SHA3 (Keccak256)
11. Block info: NUMBER, TIMESTAMP, COINBASE, etc.
12. Code operations: CODESIZE, CODECOPY

**Week 3: Returns & Calls (Days 4-7)**
13. RETURN, REVERT
14. Basic CALL (without value transfer initially)
15. CREATE (contract deployment)

### 2.3 Execution Loop

**File: `src/evm/interpreter.rs`**

```rust
impl EVM {
    pub fn execute(&mut self) -> Result<ExecutionResult> {
        loop {
            // Check if execution should stop
            if self.stopped || self.reverted {
                break;
            }
            
            // Check PC bounds
            if self.pc >= self.context.code.len() {
                break;
            }
            
            // Fetch opcode
            let opcode = match Opcode::from_byte(self.context.code[self.pc]) {
                Some(op) => op,
                None => return Err(Error::InvalidOpcode),
            };
            
            // Check gas
            let gas_cost = self.gas_cost(&opcode)?;
            if self.gas < gas_cost {
                return Err(Error::OutOfGas);
            }
            self.gas -= gas_cost;
            
            // Execute opcode
            self.execute_opcode(opcode)?;
            
            // Increment PC (unless opcode modified it)
            if !opcode.is_jump() {
                self.pc += 1 + opcode.immediate_bytes();
            }
        }
        
        Ok(ExecutionResult {
            success: !self.reverted,
            gas_used: self.initial_gas - self.gas,
            output: self.return_data.clone(),
            logs: self.logs.clone(),
        })
    }
    
    fn execute_opcode(&mut self, opcode: Opcode) -> Result<()> {
        match opcode {
            Opcode::STOP => self.op_stop(),
            Opcode::ADD => self.op_add(),
            Opcode::MUL => self.op_mul(),
            // ... etc
        }
    }
}
```

### 2.4 Example Opcode Implementations

**File: `src/evm/opcodes/arithmetic.rs`**

```rust
impl EVM {
    pub(crate) fn op_add(&mut self) -> Result<()> {
        let a = self.stack.pop()?;
        let b = self.stack.pop()?;
        let result = a.overflowing_add(b).0; // Wrapping addition
        self.stack.push(result)?;
        Ok(())
    }
    
    pub(crate) fn op_mul(&mut self) -> Result<()> {
        let a = self.stack.pop()?;
        let b = self.stack.pop()?;
        let result = a.overflowing_mul(b).0;
        self.stack.push(result)?;
        Ok(())
    }
    
    pub(crate) fn op_sub(&mut self) -> Result<()> {
        let a = self.stack.pop()?;
        let b = self.stack.pop()?;
        let result = a.overflowing_sub(b).0;
        self.stack.push(result)?;
        Ok(())
    }
    
    pub(crate) fn op_div(&mut self) -> Result<()> {
        let a = self.stack.pop()?;
        let b = self.stack.pop()?;
        let result = if b.is_zero() {
            Word::zero()
        } else {
            a / b
        };
        self.stack.push(result)?;
        Ok(())
    }
    
    pub(crate) fn op_exp(&mut self) -> Result<()> {
        let base = self.stack.pop()?;
        let exponent = self.stack.pop()?;
        let result = base.overflowing_pow(exponent).0;
        self.stack.push(result)?;
        Ok(())
    }
}
```

**File: `src/evm/opcodes/stack.rs`**

```rust
impl EVM {
    pub(crate) fn op_push(&mut self, n: usize) -> Result<()> {
        let start = self.pc + 1;
        let end = start + n;
        
        if end > self.context.code.len() {
            return Err(Error::InvalidJump);
        }
        
        let mut value = Word::zero();
        for (i, &byte) in self.context.code[start..end].iter().enumerate() {
            value = value | (Word::from(byte) << (8 * (n - 1 - i)));
        }
        
        self.stack.push(value)?;
        self.pc = end - 1; // -1 because main loop will increment
        Ok(())
    }
    
    pub(crate) fn op_dup(&mut self, n: usize) -> Result<()> {
        self.stack.dup(n)?;
        Ok(())
    }
    
    pub(crate) fn op_swap(&mut self, n: usize) -> Result<()> {
        self.stack.swap(n)?;
        Ok(())
    }
}
```

**File: `src/evm/opcodes/memory.rs`**

```rust
impl EVM {
    pub(crate) fn op_mload(&mut self) -> Result<()> {
        let offset = self.stack.pop()?.as_usize();
        let value = self.memory.load(offset);
        self.stack.push(value)?;
        Ok(())
    }
    
    pub(crate) fn op_mstore(&mut self) -> Result<()> {
        let offset = self.stack.pop()?.as_usize();
        let value = self.stack.pop()?;
        self.memory.store(offset, value);
        Ok(())
    }
    
    pub(crate) fn op_mstore8(&mut self) -> Result<()> {
        let offset = self.stack.pop()?.as_usize();
        let value = self.stack.pop()?;
        let byte = (value.low_u64() & 0xff) as u8;
        self.memory.store_byte(offset, byte);
        Ok(())
    }
}
```

**File: `src/evm/opcodes/crypto.rs`**

```rust
use sha3::{Digest, Keccak256};

impl EVM {
    pub(crate) fn op_sha3(&mut self) -> Result<()> {
        let offset = self.stack.pop()?.as_usize();
        let size = self.stack.pop()?.as_usize();
        
        let data = self.memory.load_range(offset, size);
        let mut hasher = Keccak256::new();
        hasher.update(data);
        let hash = hasher.finalize();
        
        let result = Word::from_big_endian(&hash);
        self.stack.push(result)?;
        Ok(())
    }
}
```

---

## Phase 3: State Management (Week 4)

### 3.1 Account Model

**File: `src/state/account.rs`**

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Account {
    /// Account balance in Wei
    pub balance: Word,
    
    /// Transaction nonce
    pub nonce: u64,
    
    /// Contract code hash (empty for EOAs)
    pub code_hash: Hash,
    
    /// Storage root hash
    pub storage_root: Hash,
}

impl Account {
    pub fn new_eoa() -> Self; // Externally Owned Account
    pub fn new_contract(code: Bytes) -> Self;
    pub fn is_contract(&self) -> bool;
}
```

### 3.2 World State

**File: `src/state/mod.rs`**

```rust
pub struct State {
    /// Account states
    accounts: HashMap<Address, Account>,
    
    /// Contract storage (address -> storage map)
    storage: HashMap<Address, Storage>,
    
    /// Contract codes
    codes: HashMap<Hash, Bytes>,
}

impl State {
    pub fn new() -> Self;
    
    // Account operations
    pub fn get_account(&self, address: &Address) -> Option<&Account>;
    pub fn get_account_mut(&mut self, address: &Address) -> &mut Account;
    pub fn set_account(&mut self, address: Address, account: Account);
    pub fn account_exists(&self, address: &Address) -> bool;
    
    // Balance operations
    pub fn get_balance(&self, address: &Address) -> Word;
    pub fn add_balance(&mut self, address: &Address, amount: Word);
    pub fn sub_balance(&mut self, address: &Address, amount: Word) -> Result<()>;
    pub fn transfer(&mut self, from: &Address, to: &Address, value: Word) -> Result<()>;
    
    // Nonce operations
    pub fn get_nonce(&self, address: &Address) -> u64;
    pub fn increment_nonce(&mut self, address: &Address);
    
    // Code operations
    pub fn get_code(&self, address: &Address) -> Option<&Bytes>;
    pub fn set_code(&mut self, address: Address, code: Bytes);
    
    // Storage operations
    pub fn get_storage(&self, address: &Address, key: &Word) -> Word;
    pub fn set_storage(&mut self, address: &Address, key: Word, value: Word);
    
    // Snapshot & revert (for call failures)
    pub fn snapshot(&self) -> StateSnapshot;
    pub fn revert_to_snapshot(&mut self, snapshot: StateSnapshot);
}
```

### 3.3 State Database

**File: `src/state/database.rs`**

For persistence (optional but impressive):

```rust
use std::path::Path;

pub trait StateDB {
    fn get_account(&self, address: &Address) -> Result<Option<Account>>;
    fn put_account(&mut self, address: &Address, account: &Account) -> Result<()>;
    fn get_code(&self, hash: &Hash) -> Result<Option<Bytes>>;
    fn put_code(&mut self, hash: &Hash, code: &Bytes) -> Result<()>;
    fn get_storage(&self, address: &Address, key: &Word) -> Result<Word>;
    fn put_storage(&mut self, address: &Address, key: &Word, value: &Word) -> Result<()>;
}

// In-memory implementation
pub struct MemoryDB {
    accounts: HashMap<Address, Account>,
    codes: HashMap<Hash, Bytes>,
    storage: HashMap<(Address, Word), Word>,
}

// Optional: RocksDB implementation for persistence
#[cfg(feature = "rocksdb")]
pub struct RocksDB {
    db: rocksdb::DB,
}
```

---

## Phase 4: Transaction Execution (Week 5)

### 4.1 Transaction Types

**File: `src/transaction.rs`**

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transaction {
    /// Sender nonce
    pub nonce: u64,
    
    /// Gas price
    pub gas_price: Word,
    
    /// Gas limit
    pub gas_limit: u64,
    
    /// Recipient address (None for contract creation)
    pub to: Option<Address>,
    
    /// Value transferred
    pub value: Word,
    
    /// Transaction data
    pub data: Bytes,
    
    /// Signature (v, r, s)
    pub v: u64,
    pub r: Word,
    pub s: Word,
}

impl Transaction {
    pub fn is_contract_creation(&self) -> bool;
    pub fn recover_sender(&self) -> Result<Address>;
    pub fn hash(&self) -> Hash;
    pub fn rlp_encode(&self) -> Bytes;
    pub fn rlp_decode(data: &[u8]) -> Result<Self>;
}
```

### 4.2 Transaction Executor

**File: `src/executor.rs`**

```rust
pub struct TransactionExecutor {
    state: State,
    block_context: BlockContext,
}

impl TransactionExecutor {
    pub fn new(state: State, block_context: BlockContext) -> Self;
    
    pub fn execute_transaction(&mut self, tx: Transaction) -> Result<TransactionReceipt> {
        // 1. Recover sender
        let sender = tx.recover_sender()?;
        
        // 2. Validate transaction
        self.validate_transaction(&tx, &sender)?;
        
        // 3. Pay upfront gas cost
        let gas_cost = Word::from(tx.gas_limit) * tx.gas_price;
        self.state.sub_balance(&sender, gas_cost)?;
        
        // 4. Increment nonce
        self.state.increment_nonce(&sender);
        
        // 5. Execute transaction
        let result = if tx.is_contract_creation() {
            self.execute_create(&tx, &sender)?
        } else {
            self.execute_call(&tx, &sender)?
        };
        
        // 6. Refund unused gas
        let gas_refund = tx.gas_limit - result.gas_used;
        self.state.add_balance(&sender, Word::from(gas_refund) * tx.gas_price);
        
        // 7. Pay miner
        self.state.add_balance(
            &self.block_context.coinbase,
            Word::from(result.gas_used) * tx.gas_price
        );
        
        Ok(TransactionReceipt {
            success: result.success,
            gas_used: result.gas_used,
            logs: result.logs,
            contract_address: result.contract_address,
        })
    }
    
    fn execute_call(&mut self, tx: &Transaction, sender: &Address) -> Result<ExecutionResult> {
        let to = tx.to.unwrap();
        
        // Transfer value
        if !tx.value.is_zero() {
            self.state.transfer(sender, &to, tx.value)?;
        }
        
        // Get contract code
        let code = self.state.get_code(&to).cloned().unwrap_or_default();
        
        // Create execution context
        let context = ExecutionContext {
            address: to,
            caller: *sender,
            origin: *sender,
            value: tx.value,
            data: tx.data.clone(),
            code,
            block: self.block_context.clone(),
        };
        
        // Execute
        let mut evm = EVM::new(context, tx.gas_limit, &mut self.state);
        evm.execute()
    }
    
    fn execute_create(&mut self, tx: &Transaction, sender: &Address) -> Result<ExecutionResult> {
        // Calculate new contract address
        let contract_address = self.calculate_create_address(sender);
        
        // Transfer value to new contract
        if !tx.value.is_zero() {
            self.state.transfer(sender, &contract_address, tx.value)?;
        }
        
        // Create execution context (init code)
        let context = ExecutionContext {
            address: contract_address,
            caller: *sender,
            origin: *sender,
            value: tx.value,
            data: vec![],
            code: tx.data.clone(), // Init code
            block: self.block_context.clone(),
        };
        
        // Execute init code
        let mut evm = EVM::new(context, tx.gas_limit, &mut self.state);
        let result = evm.execute()?;
        
        if result.success {
            // Store deployed code
            self.state.set_code(contract_address, result.output.clone());
        }
        
        Ok(ExecutionResult {
            contract_address: Some(contract_address),
            ..result
        })
    }
    
    fn calculate_create_address(&self, sender: &Address) -> Address {
        let nonce = self.state.get_nonce(sender);
        // RLP encode [sender, nonce] and hash
        let mut stream = rlp::RlpStream::new_list(2);
        stream.append(&sender.as_bytes());
        stream.append(&nonce);
        let hash = keccak256(&stream.out());
        Address::from_slice(&hash[12..])
    }
}
```

### 4.3 Transaction Receipt

**File: `src/transaction.rs`**

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionReceipt {
    pub success: bool,
    pub gas_used: u64,
    pub contract_address: Option<Address>,
    pub logs: Vec<Log>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Log {
    pub address: Address,
    pub topics: Vec<Hash>,
    pub data: Bytes,
}
```

---

## Phase 5: Contract Calls & CREATE (Week 6)

### 5.1 Call Opcodes

**File: `src/evm/opcodes/call.rs`**

```rust
impl EVM {
    pub(crate) fn op_call(&mut self) -> Result<()> {
        let gas = self.stack.pop()?.as_u64();
        let to = Address::from(self.stack.pop()?);
        let value = self.stack.pop()?;
        let args_offset = self.stack.pop()?.as_usize();
        let args_size = self.stack.pop()?.as_usize();
        let ret_offset = self.stack.pop()?.as_usize();
        let ret_size = self.stack.pop()?.as_usize();
        
        // Get call data
        let call_data = self.memory.load_range(args_offset, args_size).to_vec();
        
        // Transfer value
        if !value.is_zero() {
            self.state.transfer(&self.context.address, &to, value)?;
        }
        
        // Get target code
        let code = self.state.get_code(&to).cloned().unwrap_or_default();
        
        // Create sub-context
        let sub_context = ExecutionContext {
            address: to,
            caller: self.context.address,
            origin: self.context.origin,
            value,
            data: call_data,
            code,
            block: self.context.block.clone(),
        };
        
        // Execute subcall
        let mut sub_evm = EVM::new(sub_context, gas, self.state);
        let result = sub_evm.execute()?;
        
        // Store return data
        self.return_data = result.output.clone();
        
        // Copy return data to memory
        let copy_size = ret_size.min(result.output.len());
        self.memory.store_range(ret_offset, &result.output[..copy_size]);
        
        // Push success flag
        self.stack.push(if result.success { Word::one() } else { Word::zero() })?;
        
        Ok(())
    }
    
    pub(crate) fn op_staticcall(&mut self) -> Result<()> {
        // Similar to CALL but:
        // - No value transfer
        // - State modifications forbidden
        // - Check self.is_static flag
        todo!()
    }
    
    pub(crate) fn op_delegatecall(&mut self) -> Result<()> {
        // Similar to CALL but:
        // - Caller/value context preserved
        // - msg.sender is same as parent call
        todo!()
    }
}
```

### 5.2 CREATE Opcodes

**File: `src/evm/opcodes/create.rs`**

```rust
//! Core types used throughout the TinyEVM implementation
//! 
//! This module defines the fundamental data types that represent
//! Ethereum concepts like addresses, hashes, and 256-bit words.

use ethereum_types::{H160, H256, U256};
use serde::{Deserialize, Serialize};

/// Ethereum address (20 bytes)
pub type Address = H160;

/// Keccak-256 hash (32 bytes)
pub type Hash = H256;

/// EVM word (256-bit unsigned integer)
pub type Word = U256;

/// Dynamic byte array
pub type Bytes = Vec<u8>;

/// Gas amount (64-bit unsigned integer)
pub type Gas = u64;

/// Block number (64-bit unsigned integer)
pub type BlockNumber = u64;

/// Transaction nonce (64-bit unsigned integer)
pub type Nonce = u64;

/// Wei amount (same as Word, but semantically different)
pub type Wei = Word;

/// Result type for EVM operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error types for the EVM
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Stack overflow: maximum depth exceeded")]
    StackOverflow,
    
    #[error("Stack underflow: not enough items on stack")]
    StackUnderflow,
    
    #[error("Out of gas: {0} gas remaining")]
    OutOfGas(Gas),
    
    #[error("Invalid opcode: 0x{0:02x}")]
    InvalidOpcode(u8),
    
    #[error("Invalid jump destination: {0}")]
    InvalidJump(usize),
    
    #[error("Memory access out of bounds: offset {0}, size {1}")]
    MemoryOutOfBounds(usize, usize),
    
    #[error("Invalid memory access: {0}")]
    InvalidMemoryAccess(String),
    
    #[error("Execution reverted: {0}")]
    ExecutionReverted(String),
    
    #[error("Invalid transaction: {0}")]
    InvalidTransaction(String),
    
    #[error("Account not found: {0:?}")]
    AccountNotFound(Address),
    
    #[error("Insufficient balance: required {0}, available {1}")]
    InsufficientBalance(Wei, Wei),
    
    #[error("Invalid signature: {0}")]
    InvalidSignature(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Hex decoding error: {0}")]
    HexDecode(#[from] hex::FromHexError),
    
    #[error("RLP decoding error: {0}")]
    RlpDecode(#[from] rlp::DecoderError),
}

/// Execution result from EVM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    /// Whether execution was successful
    pub success: bool,
    
    /// Gas consumed during execution
    pub gas_used: Gas,
    
    /// Return data from execution
    pub output: Bytes,
    
    /// Event logs emitted during execution
    pub logs: Vec<Log>,
    
    /// Address of created contract (if any)
    pub contract_address: Option<Address>,
}

/// Event log emitted during execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Log {
    /// Address that emitted the log
    pub address: Address,
    
    /// Log topics (indexed parameters)
    pub topics: Vec<Hash>,
    
    /// Log data (non-indexed parameters)
    pub data: Bytes,
}

/// Block context for execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockContext {
    /// Block number
    pub number: BlockNumber,
    
    /// Block timestamp
    pub timestamp: u64,
    
    /// Block difficulty
    pub difficulty: Word,
    
    /// Gas limit for the block
    pub gas_limit: Gas,
    
    /// Block coinbase (miner address)
    pub coinbase: Address,
    
    /// Chain ID
    pub chain_id: u64,
    
    /// Base fee (EIP-1559)
    pub base_fee: Option<Wei>,
}

impl Default for BlockContext {
    fn default() -> Self {
        Self {
            number: 0,
            timestamp: 0,
            difficulty: Word::zero(),
            gas_limit: 30_000_000, // 30M gas
            coinbase: Address::zero(),
            chain_id: 1, // Mainnet
            base_fee: None,
        }
    }
}

/// Utility functions for common operations
pub fn word_to_usize(word: &Word) -> usize {
    word.low_u64() as usize
}

pub fn word_to_u64(word: &Word) -> u64 {
    word.low_u64()
}

pub fn word_is_zero(word: &Word) -> bool {
    word.is_zero()
}

pub fn word_to_hash(word: &Word) -> Hash {
    let mut bytes = [0u8; 32];
    word.to_big_endian(&mut bytes);
    Hash::from(bytes)
}

/// Utility functions for addresses
pub fn address_is_zero(address: &Address) -> bool {
    address == &Address::zero()
}

pub fn address_as_bytes(address: &Address) -> [u8; 20] {
    let mut bytes = [0u8; 20];
    bytes.copy_from_slice(address.as_bytes());
    bytes
}

/// Utility functions for hashes
pub fn hash_is_zero(hash: &Hash) -> bool {
    hash == &Hash::zero()
}

pub fn hash_as_bytes(hash: &Hash) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    bytes.copy_from_slice(hash.as_bytes());
    bytes
}
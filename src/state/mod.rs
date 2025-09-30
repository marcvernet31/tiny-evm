//! State Management for TinyEVM
//! 
//! This module manages the world state, including accounts, balances,
//! contract code, and storage. It provides the foundation for all
//! stateful operations in the EVM.

use crate::types::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Account information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    /// Account balance in Wei
    pub balance: Wei,
    
    /// Transaction nonce
    pub nonce: Nonce,
    
    /// Contract code hash (empty for EOAs)
    pub code_hash: Hash,
    
    /// Storage root hash (for future Merkle Patricia Trie implementation)
    pub storage_root: Hash,
}

impl Account {
    /// Create a new externally owned account (EOA)
    pub fn new_eoa() -> Self {
        Self {
            balance: Wei::zero(),
            nonce: 0,
            code_hash: Hash::zero(),
            storage_root: Hash::zero(),
        }
    }
    
    /// Create a new contract account
    pub fn new_contract(code: &[u8]) -> Self {
        let code_hash = if code.is_empty() {
            Hash::zero()
        } else {
            // In a real implementation, this would be the Keccak256 hash
            // For now, we'll use a simple hash
            Hash::from_slice(&code[..32.min(code.len())])
        };
        
        Self {
            balance: Wei::zero(),
            nonce: 0,
            code_hash,
            storage_root: Hash::zero(),
        }
    }
    
    /// Check if this is a contract account
    pub fn is_contract(&self) -> bool {
        !self.code_hash.is_zero()
    }
    
    /// Check if this is an externally owned account
    pub fn is_eoa(&self) -> bool {
        self.code_hash.is_zero()
    }
}

/// World state manager
#[derive(Debug, Clone)]
pub struct State {
    /// Account states
    accounts: HashMap<Address, Account>,
    
    /// Contract storage (address -> storage map)
    storage: HashMap<Address, crate::evm::storage::Storage>,
    
    /// Contract codes (code_hash -> code)
    codes: HashMap<Hash, Bytes>,
}

impl State {
    /// Create a new empty state
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
            storage: HashMap::new(),
            codes: HashMap::new(),
        }
    }
    
    /// Get an account by address
    pub fn get_account(&self, address: &Address) -> Option<&Account> {
        self.accounts.get(address)
    }
    
    /// Get a mutable reference to an account
    pub fn get_account_mut(&mut self, address: &Address) -> &mut Account {
        self.accounts.entry(*address).or_insert_with(Account::new_eoa)
    }
    
    /// Set an account
    pub fn set_account(&mut self, address: Address, account: Account) {
        self.accounts.insert(address, account);
    }
    
    /// Check if an account exists
    pub fn account_exists(&self, address: &Address) -> bool {
        self.accounts.contains_key(address)
    }
    
    /// Get account balance
    pub fn get_balance(&self, address: &Address) -> Wei {
        self.accounts
            .get(address)
            .map(|account| account.balance)
            .unwrap_or(Wei::zero())
    }
    
    /// Add balance to an account
    pub fn add_balance(&mut self, address: &Address, amount: Wei) {
        let account = self.get_account_mut(address);
        account.balance = account.balance.overflowing_add(amount).0;
    }
    
    /// Subtract balance from an account
    pub fn sub_balance(&mut self, address: &Address, amount: Wei) -> Result<()> {
        let account = self.get_account_mut(address);
        if account.balance < amount {
            return Err(Error::InsufficientBalance(amount, account.balance));
        }
        account.balance = account.balance.overflowing_sub(amount).0;
        Ok(())
    }
    
    /// Transfer balance between accounts
    pub fn transfer(&mut self, from: &Address, to: &Address, amount: Wei) -> Result<()> {
        self.sub_balance(from, amount)?;
        self.add_balance(to, amount);
        Ok(())
    }
    
    /// Get account nonce
    pub fn get_nonce(&self, address: &Address) -> Nonce {
        self.accounts
            .get(address)
            .map(|account| account.nonce)
            .unwrap_or(0)
    }
    
    /// Increment account nonce
    pub fn increment_nonce(&mut self, address: &Address) {
        let account = self.get_account_mut(address);
        account.nonce += 1;
    }
    
    /// Get contract code
    pub fn get_code(&self, address: &Address) -> Option<&Bytes> {
        let account = self.accounts.get(address)?;
        if account.code_hash.is_zero() {
            return None;
        }
        self.codes.get(&account.code_hash)
    }
    
    /// Set contract code
    pub fn set_code(&mut self, address: Address, code: Bytes) {
        let code_hash = if code.is_empty() {
            Hash::zero()
        } else {
            // In a real implementation, this would be the Keccak256 hash
            Hash::from_slice(&code[..32.min(code.len())])
        };
        
        // Update account
        let account = self.get_account_mut(&address);
        account.code_hash = code_hash;
        
        // Store code
        if !code_hash.is_zero() {
            self.codes.insert(code_hash, code);
        }
    }
    
    /// Get storage for an account
    pub fn get_storage(&mut self, address: &Address) -> &mut crate::evm::storage::Storage {
        self.storage.entry(*address).or_insert_with(crate::evm::storage::Storage::new)
    }
    
    /// Load from storage
    pub fn load_storage(&self, address: &Address, key: &Word) -> Word {
        self.storage
            .get(address)
            .map(|storage| storage.load(key))
            .unwrap_or(Word::zero())
    }
    
    /// Store to storage
    pub fn store_storage(&mut self, address: &Address, key: Word, value: Word) {
        let storage = self.get_storage(address);
        storage.store(key, value);
    }
    
    /// Create a snapshot of the current state
    pub fn snapshot(&self) -> StateSnapshot {
        StateSnapshot {
            accounts: self.accounts.clone(),
            storage: self.storage.clone(),
        }
    }
    
    /// Revert to a previous snapshot
    pub fn revert_to_snapshot(&mut self, snapshot: StateSnapshot) {
        self.accounts = snapshot.accounts;
        self.storage = snapshot.storage;
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

/// State snapshot for reverting failed operations
#[derive(Debug, Clone)]
pub struct StateSnapshot {
    accounts: HashMap<Address, Account>,
    storage: HashMap<Address, crate::evm::storage::Storage>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_account_creation() {
        let eoa = Account::new_eoa();
        assert!(eoa.is_eoa());
        assert!(!eoa.is_contract());
        assert_eq!(eoa.balance, Wei::zero());
        assert_eq!(eoa.nonce, 0);
        
        let contract = Account::new_contract(&[0x60, 0x01, 0x60, 0x02, 0x01]);
        assert!(!contract.is_eoa());
        assert!(contract.is_contract());
    }
    
    #[test]
    fn test_state_operations() {
        let mut state = State::new();
        let address = Address::from([1u8; 20]);
        
        // Test account creation
        assert!(!state.account_exists(&address));
        assert_eq!(state.get_balance(&address), Wei::zero());
        
        // Test balance operations
        state.add_balance(&address, Wei::from(1000));
        assert_eq!(state.get_balance(&address), Wei::from(1000));
        
        state.sub_balance(&address, Wei::from(300)).unwrap();
        assert_eq!(state.get_balance(&address), Wei::from(700));
        
        // Test insufficient balance
        assert!(state.sub_balance(&address, Wei::from(1000)).is_err());
    }
    
    #[test]
    fn test_transfer() {
        let mut state = State::new();
        let from = Address::from([1u8; 20]);
        let to = Address::from([2u8; 20]);
        
        // Add balance to sender
        state.add_balance(&from, Wei::from(1000));
        
        // Transfer
        state.transfer(&from, &to, Wei::from(300)).unwrap();
        
        assert_eq!(state.get_balance(&from), Wei::from(700));
        assert_eq!(state.get_balance(&to), Wei::from(300));
    }
    
    #[test]
    fn test_nonce_operations() {
        let mut state = State::new();
        let address = Address::from([1u8; 20]);
        
        assert_eq!(state.get_nonce(&address), 0);
        
        state.increment_nonce(&address);
        assert_eq!(state.get_nonce(&address), 1);
        
        state.increment_nonce(&address);
        assert_eq!(state.get_nonce(&address), 2);
    }
    
    #[test]
    fn test_code_operations() {
        let mut state = State::new();
        let address = Address::from([1u8; 20]);
        let code = vec![0x60, 0x01, 0x60, 0x02, 0x01];
        
        // Initially no code
        assert!(state.get_code(&address).is_none());
        
        // Set code
        state.set_code(address, code.clone());
        assert_eq!(state.get_code(&address), Some(&code));
        
        // Check account is now a contract
        let account = state.get_account(&address).unwrap();
        assert!(account.is_contract());
    }
    
    #[test]
    fn test_storage_operations() {
        let mut state = State::new();
        let address = Address::from([1u8; 20]);
        let key = Word::from(42);
        let value = Word::from(100);
        
        // Initially zero
        assert_eq!(state.load_storage(&address, &key), Word::zero());
        
        // Store value
        state.store_storage(&address, key, value);
        assert_eq!(state.load_storage(&address, &key), value);
    }
    
    #[test]
    fn test_snapshot_revert() {
        let mut state = State::new();
        let address = Address::from([1u8; 20]);
        
        // Add some state
        state.add_balance(&address, Wei::from(1000));
        state.store_storage(&address, Word::from(1), Word::from(100));
        
        // Create snapshot
        let snapshot = state.snapshot();
        
        // Modify state
        state.add_balance(&address, Wei::from(500));
        state.store_storage(&address, Word::from(1), Word::from(200));
        
        // Verify changes
        assert_eq!(state.get_balance(&address), Wei::from(1500));
        assert_eq!(state.load_storage(&address, &Word::from(1)), Word::from(200));
        
        // Revert
        state.revert_to_snapshot(snapshot);
        
        // Verify reverted state
        assert_eq!(state.get_balance(&address), Wei::from(1000));
        assert_eq!(state.load_storage(&address, &Word::from(1)), Word::from(100));
    }
}
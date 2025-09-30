//! Unit tests for State Management implementation

use tinyevm::state::{State, Account};
use tinyevm::types::*;

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
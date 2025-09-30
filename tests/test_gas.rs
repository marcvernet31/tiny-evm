//! Unit tests for Gas Metering implementation

use tinyevm::gas::{GasMeter, costs, memory_expansion_cost, exp_cost, sha3_cost, log_cost, call_cost};
use tinyevm::types::*;

#[test]
fn test_gas_meter_creation() {
    let meter = GasMeter::new(1000);
    assert_eq!(meter.gas_remaining(), 1000);
    assert_eq!(meter.initial_gas(), 1000);
    assert_eq!(meter.gas_used(), 0);
}

#[test]
fn test_gas_consumption() {
    let mut meter = GasMeter::new(1000);
    
    // Consume gas
    meter.consume(100).unwrap();
    assert_eq!(meter.gas_remaining(), 900);
    assert_eq!(meter.gas_used(), 100);
    
    // Consume more gas
    meter.consume(200).unwrap();
    assert_eq!(meter.gas_remaining(), 700);
    assert_eq!(meter.gas_used(), 300);
}

#[test]
fn test_gas_insufficient() {
    let mut meter = GasMeter::new(100);
    
    // Try to consume more gas than available
    assert!(meter.consume(200).is_err());
    assert_eq!(meter.gas_remaining(), 100); // Should not change
}

#[test]
fn test_gas_refunds() {
    let mut meter = GasMeter::new(1000);
    
    // Consume some gas
    meter.consume(500).unwrap();
    
    // Add refunds
    meter.add_refund(100);
    assert_eq!(meter.refunds(), 100);
    
    // Apply refunds
    meter.apply_refunds();
    assert_eq!(meter.gas_remaining(), 600); // 500 + 100 refund
    assert_eq!(meter.refunds(), 0);
}

#[test]
fn test_gas_refund_limit() {
    let mut meter = GasMeter::new(1000);
    
    // Consume gas
    meter.consume(200).unwrap();
    
    // Add refunds (more than 1/2 of gas used)
    meter.add_refund(150);
    
    // Apply refunds (should be limited to 1/2 of gas used = 100)
    meter.apply_refunds();
    assert_eq!(meter.gas_remaining(), 300); // 200 + 100 refund (limited)
    assert_eq!(meter.refunds(), 0);
}

#[test]
fn test_gas_costs() {
    // Test various gas costs
    assert_eq!(costs::ADD, 3);
    assert_eq!(costs::MUL, 5);
    assert_eq!(costs::STORAGE_STORE, 20000);
    assert_eq!(costs::CALL, 100);
    assert_eq!(costs::CREATE, 32000);
}

#[test]
fn test_memory_expansion_cost() {
    // No expansion
    assert_eq!(memory_expansion_cost(100, 50), 0);
    
    // Small expansion
    assert_eq!(memory_expansion_cost(0, 32), 3); // 1 word
    
    // Larger expansion
    assert_eq!(memory_expansion_cost(0, 64), 5); // 2 words
}

#[test]
fn test_exp_cost() {
    // Zero exponent
    assert_eq!(exp_cost(&Word::zero()), costs::EXP);
    
    // Small exponent
    assert_eq!(exp_cost(&Word::from(1)), costs::EXP + 50);
    
    // Larger exponent
    assert_eq!(exp_cost(&Word::from(256)), costs::EXP + 8 * 50);
}

#[test]
fn test_sha3_cost() {
    assert_eq!(sha3_cost(0), costs::LOW);
    assert_eq!(sha3_cost(32), costs::LOW + costs::LOW);
    assert_eq!(sha3_cost(64), costs::LOW + 2 * costs::LOW);
}

#[test]
fn test_log_cost() {
    assert_eq!(log_cost(0, 0), costs::LOG0);
    assert_eq!(log_cost(1, 0), costs::LOG1);
    assert_eq!(log_cost(2, 32), costs::LOG2 + costs::LOW);
    assert_eq!(log_cost(5, 0), 0); // Invalid
}

#[test]
fn test_call_cost() {
    // Call without value
    assert_eq!(call_cost(&Wei::zero(), true), costs::CALL);
    
    // Call with value
    assert_eq!(call_cost(&Wei::from(1000), true), costs::CALL + 9000);
    
    // Callcode without value
    assert_eq!(call_cost(&Wei::zero(), false), costs::CALLCODE);
}
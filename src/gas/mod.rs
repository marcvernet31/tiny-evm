//! Gas Metering System for TinyEVM
//! 
//! This module handles gas calculation and consumption for all EVM operations.
//! Gas is used to prevent infinite loops and ensure computational costs are paid.

use crate::types::*;

/// Gas meter for tracking gas consumption
#[derive(Debug, Clone)]
pub struct GasMeter {
    /// Gas remaining
    gas: Gas,
    
    /// Initial gas limit
    initial_gas: Gas,
    
    /// Gas refunds (to be applied at the end)
    refunds: Gas,
}

impl GasMeter {
    /// Create a new gas meter with the given gas limit
    pub fn new(gas_limit: Gas) -> Self {
        Self {
            gas: gas_limit,
            initial_gas: gas_limit,
            refunds: 0,
        }
    }
    
    /// Get remaining gas
    pub fn gas_remaining(&self) -> Gas {
        self.gas
    }
    
    /// Get initial gas limit
    pub fn initial_gas(&self) -> Gas {
        self.initial_gas
    }
    
    /// Get gas used
    pub fn gas_used(&self) -> Gas {
        self.initial_gas - self.gas
    }
    
    /// Check if we have enough gas for an operation
    pub fn has_gas(&self, required: Gas) -> bool {
        self.gas >= required
    }
    
    /// Consume gas for an operation
    /// 
    /// # Errors
    /// Returns `OutOfGas` if not enough gas is available
    pub fn consume(&mut self, amount: Gas) -> Result<()> {
        if self.gas < amount {
            return Err(Error::OutOfGas(self.gas));
        }
        self.gas -= amount;
        Ok(())
    }
    
    /// Add gas refund (to be applied at the end)
    pub fn add_refund(&mut self, amount: Gas) {
        self.refunds = self.refunds.saturating_add(amount);
    }
    
    /// Get total refunds
    pub fn refunds(&self) -> Gas {
        self.refunds
    }
    
    /// Apply refunds (up to 1/2 of gas used)
    pub fn apply_refunds(&mut self) {
        let max_refund = self.gas_used() / 2;
        let refund = self.refunds.min(max_refund);
        self.gas += refund;
        self.refunds = 0;
    }
    
    /// Reset gas meter
    pub fn reset(&mut self, gas_limit: Gas) {
        self.gas = gas_limit;
        self.initial_gas = gas_limit;
        self.refunds = 0;
    }
}

/// Gas costs for different operations
pub mod costs {
    use super::*;
    
    // Base costs
    pub const BASE: Gas = 2;
    pub const VERY_LOW: Gas = 3;
    pub const LOW: Gas = 5;
    pub const MID: Gas = 8;
    pub const HIGH: Gas = 10;
    pub const EXT: Gas = 20;
    pub const SPECIAL: Gas = 0;
    
    // Stack operations
    pub const STACK_PUSH: Gas = VERY_LOW;
    pub const STACK_POP: Gas = BASE;
    pub const STACK_DUP: Gas = VERY_LOW;
    pub const STACK_SWAP: Gas = VERY_LOW;
    
    // Memory operations
    pub const MEMORY_LOAD: Gas = VERY_LOW;
    pub const MEMORY_STORE: Gas = VERY_LOW;
    pub const MEMORY_STORE8: Gas = VERY_LOW;
    
    // Storage operations
    pub const STORAGE_LOAD: Gas = 200;
    pub const STORAGE_STORE: Gas = 20000;
    pub const STORAGE_STORE_CLEAR: Gas = 5000;
    
    // Arithmetic operations
    pub const ADD: Gas = VERY_LOW;
    pub const MUL: Gas = LOW;
    pub const SUB: Gas = VERY_LOW;
    pub const DIV: Gas = LOW;
    pub const MOD: Gas = LOW;
    pub const SDIV: Gas = LOW;
    pub const SMOD: Gas = LOW;
    pub const ADDMOD: Gas = MID;
    pub const MULMOD: Gas = MID;
    pub const EXP: Gas = 10; // Base cost, additional for exponent size
    
    // Comparison operations
    pub const LT: Gas = VERY_LOW;
    pub const GT: Gas = VERY_LOW;
    pub const SLT: Gas = VERY_LOW;
    pub const SGT: Gas = VERY_LOW;
    pub const EQ: Gas = VERY_LOW;
    pub const ISZERO: Gas = VERY_LOW;
    
    // Bitwise operations
    pub const AND: Gas = VERY_LOW;
    pub const OR: Gas = VERY_LOW;
    pub const XOR: Gas = VERY_LOW;
    pub const NOT: Gas = VERY_LOW;
    pub const BYTE: Gas = VERY_LOW;
    pub const SHL: Gas = VERY_LOW;
    pub const SHR: Gas = VERY_LOW;
    pub const SAR: Gas = VERY_LOW;
    
    // Control flow
    pub const JUMP: Gas = MID;
    pub const JUMPI: Gas = HIGH;
    pub const JUMPDEST: Gas = 1;
    pub const STOP: Gas = 0;
    pub const RETURN: Gas = 0;
    pub const REVERT: Gas = 0;
    
    // Context operations
    pub const ADDRESS: Gas = BASE;
    pub const CALLER: Gas = BASE;
    pub const CALLVALUE: Gas = BASE;
    pub const ORIGIN: Gas = BASE;
    pub const CALLDATALOAD: Gas = VERY_LOW;
    pub const CALLDATASIZE: Gas = BASE;
    pub const CALLDATACOPY: Gas = VERY_LOW;
    pub const CODESIZE: Gas = BASE;
    pub const CODECOPY: Gas = VERY_LOW;
    pub const GASPRICE: Gas = BASE;
    pub const EXTCODESIZE: Gas = EXT;
    pub const EXTCODECOPY: Gas = EXT;
    pub const RETURNDATASIZE: Gas = BASE;
    pub const RETURNDATACOPY: Gas = VERY_LOW;
    pub const EXTCODEHASH: Gas = EXT;
    
    // Block operations
    pub const BLOCKHASH: Gas = EXT;
    pub const COINBASE: Gas = BASE;
    pub const TIMESTAMP: Gas = BASE;
    pub const NUMBER: Gas = BASE;
    pub const DIFFICULTY: Gas = BASE;
    pub const GASLIMIT: Gas = BASE;
    pub const CHAINID: Gas = BASE;
    pub const SELFBALANCE: Gas = LOW;
    pub const BASEFEE: Gas = BASE;
    
    // Logging operations
    pub const LOG0: Gas = 375;
    pub const LOG1: Gas = 750;
    pub const LOG2: Gas = 1125;
    pub const LOG3: Gas = 1500;
    pub const LOG4: Gas = 1875;
    
    // System operations
    pub const CREATE: Gas = 32000;
    pub const CALL: Gas = 100;
    pub const CALLCODE: Gas = 100;
    pub const DELEGATECALL: Gas = 100;
    pub const STATICCALL: Gas = 100;
    pub const CREATE2: Gas = 32000;
    pub const SELFDESTRUCT: Gas = 5000;
    
    // Push operations (0x60-0x7f)
    pub const PUSH1: Gas = VERY_LOW;
    pub const PUSH2: Gas = VERY_LOW;
    pub const PUSH3: Gas = VERY_LOW;
    pub const PUSH4: Gas = VERY_LOW;
    pub const PUSH5: Gas = VERY_LOW;
    pub const PUSH6: Gas = VERY_LOW;
    pub const PUSH7: Gas = VERY_LOW;
    pub const PUSH8: Gas = VERY_LOW;
    pub const PUSH9: Gas = VERY_LOW;
    pub const PUSH10: Gas = VERY_LOW;
    pub const PUSH11: Gas = VERY_LOW;
    pub const PUSH12: Gas = VERY_LOW;
    pub const PUSH13: Gas = VERY_LOW;
    pub const PUSH14: Gas = VERY_LOW;
    pub const PUSH15: Gas = VERY_LOW;
    pub const PUSH16: Gas = VERY_LOW;
    pub const PUSH17: Gas = VERY_LOW;
    pub const PUSH18: Gas = VERY_LOW;
    pub const PUSH19: Gas = VERY_LOW;
    pub const PUSH20: Gas = VERY_LOW;
    pub const PUSH21: Gas = VERY_LOW;
    pub const PUSH22: Gas = VERY_LOW;
    pub const PUSH23: Gas = VERY_LOW;
    pub const PUSH24: Gas = VERY_LOW;
    pub const PUSH25: Gas = VERY_LOW;
    pub const PUSH26: Gas = VERY_LOW;
    pub const PUSH27: Gas = VERY_LOW;
    pub const PUSH28: Gas = VERY_LOW;
    pub const PUSH29: Gas = VERY_LOW;
    pub const PUSH30: Gas = VERY_LOW;
    pub const PUSH31: Gas = VERY_LOW;
    pub const PUSH32: Gas = VERY_LOW;
    
    // Dup operations (0x80-0x8f)
    pub const DUP1: Gas = VERY_LOW;
    pub const DUP2: Gas = VERY_LOW;
    pub const DUP3: Gas = VERY_LOW;
    pub const DUP4: Gas = VERY_LOW;
    pub const DUP5: Gas = VERY_LOW;
    pub const DUP6: Gas = VERY_LOW;
    pub const DUP7: Gas = VERY_LOW;
    pub const DUP8: Gas = VERY_LOW;
    pub const DUP9: Gas = VERY_LOW;
    pub const DUP10: Gas = VERY_LOW;
    pub const DUP11: Gas = VERY_LOW;
    pub const DUP12: Gas = VERY_LOW;
    pub const DUP13: Gas = VERY_LOW;
    pub const DUP14: Gas = VERY_LOW;
    pub const DUP15: Gas = VERY_LOW;
    pub const DUP16: Gas = VERY_LOW;
    
    // Swap operations (0x90-0x9f)
    pub const SWAP1: Gas = VERY_LOW;
    pub const SWAP2: Gas = VERY_LOW;
    pub const SWAP3: Gas = VERY_LOW;
    pub const SWAP4: Gas = VERY_LOW;
    pub const SWAP5: Gas = VERY_LOW;
    pub const SWAP6: Gas = VERY_LOW;
    pub const SWAP7: Gas = VERY_LOW;
    pub const SWAP8: Gas = VERY_LOW;
    pub const SWAP9: Gas = VERY_LOW;
    pub const SWAP10: Gas = VERY_LOW;
    pub const SWAP11: Gas = VERY_LOW;
    pub const SWAP12: Gas = VERY_LOW;
    pub const SWAP13: Gas = VERY_LOW;
    pub const SWAP14: Gas = VERY_LOW;
    pub const SWAP15: Gas = VERY_LOW;
    pub const SWAP16: Gas = VERY_LOW;
}

/// Calculate gas cost for memory expansion
pub fn memory_expansion_cost(current_size: usize, new_size: usize) -> Gas {
    if new_size <= current_size {
        return 0;
    }
    
    let new_words = (new_size + 31) / 32; // Round up to word boundary
    let current_words = (current_size + 31) / 32;
    
    let new_cost = (new_words * new_words) / 512 + 3 * new_words;
    let current_cost = (current_words * current_words) / 512 + 3 * current_words;
    
    (new_cost - current_cost) as Gas
}

/// Calculate gas cost for exponentiation
pub fn exp_cost(exponent: &Word) -> Gas {
    if exponent.is_zero() {
        return costs::EXP;
    }
    
    let bit_length = 256 - exponent.leading_zeros();
    let cost = costs::EXP + (bit_length * 50) as Gas;
    
    cost
}

/// Calculate gas cost for SHA3 operation
pub fn sha3_cost(data_size: usize) -> Gas {
    costs::LOW + ((data_size + 31) / 32) as Gas * costs::LOW
}

/// Calculate gas cost for log operation
pub fn log_cost(topics: usize, data_size: usize) -> Gas {
    let base_cost = match topics {
        0 => costs::LOG0,
        1 => costs::LOG1,
        2 => costs::LOG2,
        3 => costs::LOG3,
        4 => costs::LOG4,
        _ => return 0, // Invalid
    };
    
    base_cost + data_size as Gas * costs::LOW
}

/// Calculate gas cost for call operation
pub fn call_cost(value: &Wei, is_call: bool) -> Gas {
    let base_cost = if is_call {
        costs::CALL
    } else {
        costs::CALLCODE
    };
    
    if value.is_zero() {
        base_cost
    } else {
        base_cost + 9000 // Additional cost for value transfer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
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
}
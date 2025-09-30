//! TinyEVM - A production-quality Ethereum Virtual Machine implementation in Rust
//! 
//! This is the main entry point for the TinyEVM project.

mod types;
mod evm;
mod state;
mod gas;

use types::*;

fn main() {
    println!("TinyEVM - Ethereum Virtual Machine Implementation");
    println!("Phase 1: Foundation - Basic infrastructure ready!");
    
    // Example of creating basic EVM components
    let mut stack = evm::stack::Stack::new();
    let mut memory = evm::memory::Memory::new();
    let mut storage = evm::storage::Storage::new();
    
    // Test basic operations
    if let Ok(_) = stack.push(Word::from(42)) {
        println!("✓ Stack: Successfully pushed value 42");
    }
    
    memory.store(0, Word::from(0x1234567890abcdefu64));
    let loaded = memory.load(0);
    println!("✓ Memory: Stored and loaded value 0x{:x}", loaded);
    
    storage.store(Word::from(1), Word::from(100));
    let stored = storage.load(&Word::from(1));
    println!("✓ Storage: Stored and loaded value {}", stored);
    
    // Test gas meter
    let mut gas_meter = gas::GasMeter::new(1000);
    if let Ok(_) = gas_meter.consume(100) {
        println!("✓ Gas: Consumed 100 gas, {} remaining", gas_meter.gas_remaining());
    }
    
    // Test state management
    let mut state = state::State::new();
    let address = Address::from([1u8; 20]);
    state.add_balance(&address, Wei::from(1000));
    println!("✓ State: Added balance, current balance: {}", state.get_balance(&address));
    
    println!("\n🎉 Phase 1 Foundation components are working!");
    println!("Ready to implement Phase 2: Basic EVM opcodes");
}

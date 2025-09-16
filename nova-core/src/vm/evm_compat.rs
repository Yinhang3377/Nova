//! Minimal EVM-compatible execution shim used for tests.

use crate::data_model::Transaction;
use crate::vm::Receipt;

pub fn execute(tx: &Transaction) -> Receipt {
    // For now, the shim simply returns the payload as the output and success=true.
    Receipt {
        success: true,
        output: tx.payload.clone(),
    }
}

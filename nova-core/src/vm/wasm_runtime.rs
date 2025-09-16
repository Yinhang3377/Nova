//! Minimal WASM runtime shim used for tests.

use crate::data_model::Transaction;
use crate::vm::Receipt;

pub fn execute(tx: &Transaction) -> Receipt {
    // Echo payload for now.
    Receipt {
        success: true,
        output: tx.payload.clone(),
    }
}

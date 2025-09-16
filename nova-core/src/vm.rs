//! VM abstractions and tiny implementations for tests

use crate::data_model::Transaction;
use serde::{ Deserialize, Serialize };

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Receipt {
    pub success: bool,
    pub output: Vec<u8>,
}

pub trait VM {
    fn execute(&self, tx: &Transaction) -> Receipt;
}

pub mod evm_compat;
pub mod wasm_runtime;

/// convenience helpers
pub fn execute_evm_tx(tx: &Transaction) -> Receipt {
    evm_compat::execute(tx)
}

pub fn execute_wasm_tx(tx: &Transaction) -> Receipt {
    wasm_runtime::execute(tx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_model::{ Transaction, TxType };

    #[test]
    fn evm_exec_echoes_payload() {
        let tx = Transaction { tx_type: TxType::Transfer, payload: b"hello".to_vec() };
        let r = execute_evm_tx(&tx);
        assert!(r.success);
        assert_eq!(r.output, tx.payload);
    }

    #[test]
    fn wasm_exec_echoes_payload() {
        let tx = Transaction { tx_type: TxType::CabinUpload, payload: b"wasm".to_vec() };
        let r = execute_wasm_tx(&tx);
        assert!(r.success);
        assert_eq!(r.output, tx.payload);
    }
}

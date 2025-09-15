use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlockHeader {
    pub parent_hash: Vec<u8>,
    pub merkle_root: Vec<u8>,
    pub poh_digest: Vec<u8>,
    pub number: u64,
    pub timestamp: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub tx_type: TxType,
    pub payload: Vec<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TxType {
    CabinUpload,
    CrossBridge,
    Transfer,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::poh;

    #[test]
    fn poh_and_serde_roundtrip() {
        // create a seed
        let seed = poh::random_seed();
        // generate a couple of digests
        let d1 = poh::generate_poh(&seed, 1);
        let d2 = poh::generate_poh(&d1, 2);

        let header = BlockHeader {
            parent_hash: d1.clone(),
            merkle_root: vec![0u8; 32],
            poh_digest: d2.clone(),
            number: 42,
            timestamp: 1_700_000_000,
        };

        let block = Block {
            header: header.clone(),
            transactions: vec![],
        };

        // serde roundtrip
        let ser = serde_json::to_vec(&block).expect("serialize");
        let de: Block = serde_json::from_slice(&ser).expect("deserialize");

        assert_eq!(de.header.number, header.number);
        assert_eq!(de.header.poh_digest, header.poh_digest);
        assert_eq!(de.header.parent_hash, header.parent_hash);
    }
}

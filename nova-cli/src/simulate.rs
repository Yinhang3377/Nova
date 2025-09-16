use anyhow::Result;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn run(count: usize, storm: bool, json: bool, backend: &str) -> Result<()> {
    let mut previous = nova_core::poh::random_seed();
    let mut number: u64 = 1;

    // select storage backend via factory (None when backend == "none")
    let storage_box: Option<Box<dyn nova_core::storage::Storage>> = match backend {
        "none" => None,
        name => Some(nova_core::storage::factory::open_backend(name)),
    };

    // convert to an Option<&dyn Storage> for convenience
    let storage_ref: Option<&dyn nova_core::storage::Storage> = storage_box.as_deref();

    for i in 0..count {
        let poh = nova_core::poh::generate_poh(&previous, (i as u64) + 1);
        let ts = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        // build a Block value and persist it to storage (serialize as JSON bytes)
        let header = nova_core::data_model::BlockHeader {
            parent_hash: previous.clone(),
            merkle_root: vec![0u8; 32],
            poh_digest: poh.clone(),
            number,
            timestamp: ts,
        };

        let block = nova_core::data_model::Block {
            header: header.clone(),
            transactions: vec![],
        };

        if let Some(s) = storage_ref {
            let key = format!("block:{}", number).into_bytes();
            let value = serde_json::to_vec(&block)?;
            s.put(key.clone(), value);
            let stored = s.get(&key).is_some();

            if json {
                let obj = serde_json::json!({
                    "number": number,
                    "timestamp": ts,
                    "poh": hex::encode(&poh),
                    "stored": stored
                });
                println!("{}", obj);
            } else {
                println!(
                    "block {} ts={} poh={} stored={}",
                    number,
                    ts,
                    hex::encode(&poh),
                    stored
                );
            }
        } else {
            if json {
                let obj = serde_json::json!({
                    "number": number,
                    "timestamp": ts,
                    "poh": hex::encode(&poh)
                });
                println!("{}", obj);
            } else {
                println!("block {} ts={} poh={}", number, ts, hex::encode(&poh));
            }
        }

        previous = poh;
        number += 1;

        if storm {
            // in storm mode, generate extra internal pulses
            let _ = nova_core::poh::generate_poh(&previous, (i as u64) + 100);
        }
    }

    Ok(())
}

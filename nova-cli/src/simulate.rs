use anyhow::Result;
use std::time::{ SystemTime, UNIX_EPOCH };

pub fn run(count: usize, storm: bool, json: bool) -> Result<()> {
    let mut previous = nova_core::poh::random_seed();
    let mut number: u64 = 1;

    for i in 0..count {
        let poh = nova_core::poh::generate_poh(&previous, (i as u64) + 1);
        let ts = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        if json {
            let obj =
                serde_json::json!({
                "number": number,
                "timestamp": ts,
                "poh": hex::encode(&poh)
            });
            println!("{}", obj);
        } else {
            println!("block {} ts={} poh={}", number, ts, hex::encode(&poh));
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

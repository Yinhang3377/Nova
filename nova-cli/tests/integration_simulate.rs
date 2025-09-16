use assert_cmd::Command;
use serde_json::Value;

fn validate_poh_hex(s: &str) -> bool {
    // expect 64 hex chars (32 bytes)
    s.len() == 64 && s.chars().all(|c| c.is_ascii_hexdigit())
}

#[test]
fn simulate_runs_and_prints_blocks() {
    let mut cmd = Command::cargo_bin("nova-cli").unwrap();
    let output = cmd.args(["simulate", "--count", "3", "--json"]).output().unwrap();
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    // each line is a JSON object
    let mut line_count = 0usize;
    for line in stdout.lines() {
        if line.trim().is_empty() {
            continue;
        }
        line_count += 1;
        let v: Value = serde_json::from_str(line).expect("line should be valid json");
        // number
        let num = v.get("number").expect("number field");
        assert!(num.is_number());
        assert!(num.as_i64().unwrap() >= 1);
        // timestamp
        let ts = v.get("timestamp").expect("timestamp field");
        assert!(ts.is_number());
        // poh
        let poh = v.get("poh").expect("poh field");
        assert!(poh.is_string());
        let poh_s = poh.as_str().unwrap();
        assert!(validate_poh_hex(poh_s), "poh must be 64 hex chars");
        // stored (should be present and true when using the default mem backend)
        let stored = v.get("stored").expect("stored field");
        assert!(stored.is_boolean());
        assert!(stored.as_bool().unwrap());
    }
    assert_eq!(line_count, 3, "should have three JSON lines");

    // Second: explicit `--backend none` - stored should NOT be present
    let mut cmd2 = Command::cargo_bin("nova-cli").unwrap();
    let output2 = cmd2
        .args(["simulate", "--count", "2", "--json", "--backend", "none"])
        .output()
        .unwrap();
    assert!(output2.status.success());

    let stdout2 = String::from_utf8_lossy(&output2.stdout);
    let mut line_count2 = 0usize;
    for line in stdout2.lines() {
        if line.trim().is_empty() {
            continue;
        }
        line_count2 += 1;
        let v: Value = serde_json::from_str(line).expect("line should be valid json");
        // stored must be absent when backend=none
        assert!(v.get("stored").is_none(), "stored must be absent for backend=none");
    }
    assert_eq!(line_count2, 2, "should have two JSON lines for backend=none");
}

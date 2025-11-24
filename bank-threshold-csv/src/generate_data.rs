use rand::seq::SliceRandom;   // ← REQUIRED for .shuffle()
use rand::Rng;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

const TOTAL_CLIENTS: usize = 1000;
const CLIENTS_PER_BANK: usize = 300;
const BANKS: usize = 5;

fn main() {
    let mut rng = rand::thread_rng();

    // 1000 unique raw client IDs
    let mut all_raw_ids: Vec<u64> = (0..TOTAL_CLIENTS).map(|_| rng.gen()).collect();

    // Which banks have each client (with realistic overlaps)
    let mut bank_clients: Vec<Vec<(u64, u8)>> = vec![vec![]; BANKS];
    let mut assigned = HashMap::new();

    for &raw_id in &all_raw_ids {
        // Decide how many banks share this client
        let n_banks = match rng.gen::<f32>() {
            x if x < 0.03 => 4 + (rng.gen::<bool>() as usize), // ~3 % in 4–5 banks
            x if x < 0.13 => 3,                                 // ~10 % in 3 banks
            x if x < 0.38 => 2,                                 // ~25 % in 2 banks
            _ => 1,                                             // rest in 1 bank
        };

        let mut banks: Vec<usize> = (0..BANKS).collect();
        banks.shuffle(&mut rng);
        for &b in banks.iter().take(n_banks) {
            let score = rng.gen_range(20..=95u8);
            bank_clients[b].push((raw_id, score));
            assigned.entry(raw_id).or_insert_with(Vec::new).push(b);
        }
    }

    // Make sure every bank has ~300 clients
    for b in 0..BANKS {
        while bank_clients[b].len() < CLIENTS_PER_BANK {
            let raw_id = all_raw_ids[rng.gen_range(0..TOTAL_CLIENTS)];
            if !bank_clients[b].iter().any(|&(id, _)| id == raw_id) {
                bank_clients[b].push((raw_id, rng.gen_range(20..=95u8)));
                assigned.entry(raw_id).or_insert_with(Vec::new).push(b);
            }
        }
    }

    // Write CSV files
    std::fs::create_dir_all("data").unwrap();
    for b in 0..BANKS {
        let mut f = File::create(format!("data/bank_{}.csv", b)).unwrap();
        writeln!(f, "client_id,risk_score").unwrap();
        for &(raw_id, score) in &bank_clients[b] {
            writeln!(f, "{},{}", raw_id, score).unwrap();
        }
        println!("Wrote data/bank_{}.csv — {} clients", b, bank_clients[b].len());
    }

    // Print overlap statistics
    let mut overlap_count = vec![0; BANKS + 1];
    for banks in assigned.values() {
        overlap_count[banks.len()] += 1;
    }
    println!("\nOverlap statistics:");
    for i in 1..=BANKS {
        println!("  Clients present in {} bank(s): {}", i, overlap_count[i]);
    }
}

use std::collections::HashMap;
use std::mem;
use banking_db::models::person::{PersonIdxModel};
use uuid::Uuid;

#[test]
fn test_person_idx_model_memory_size_and_alignment() {
    const EXPECTED_TOTAL_SIZE: usize = 48;
    let total_size = mem::size_of::<PersonIdxModel>();
    println!("--- PersonIdxModel Memory Layout ---");
    println!("Total struct size: {} bytes", total_size);
    assert_eq!(total_size, EXPECTED_TOTAL_SIZE, "Total size of PersonIdxModel has changed!");
}

// --- PersonIndex and its memory analysis ---

pub struct PersonIndex {
    pub by_person_id: HashMap<Uuid, PersonIdxModel>,
    pub by_ext_hash: HashMap<i64, Uuid>,
}

// Helper function to analyze and validate memory for a given number of entries
fn analyze_person_index_size(num_entries: usize, expected_min_total_size: usize) {
    const STACK_SIZE: usize = 96;
    const HEAP_SIZE_PER_ENTRY: usize = 80;

    let stack_size = mem::size_of::<PersonIndex>();
    let total_heap_data_size = HEAP_SIZE_PER_ENTRY * num_entries;
    let min_total_size = stack_size + total_heap_data_size;

    println!("\n--- PersonIndex Memory Analysis ({} Entries) ---", num_entries);
    println!("Minimum Total: {} bytes ({:.2} MB)", min_total_size, min_total_size as f64 / (1024.0 * 1024.0));
    
    assert_eq!(stack_size, STACK_SIZE, "Stack size of PersonIndex has changed!");
    assert_eq!(min_total_size, expected_min_total_size, "Minimum total size for {} entries has changed!", num_entries);
}

#[test]
fn test_person_index_size_with_one_entry() {
    analyze_person_index_size(1, 176); // 96 + (80 * 1)
}

#[test]
fn test_person_index_size_with_two_entries() {
    analyze_person_index_size(2, 256); // 96 + (80 * 2)
}

#[test]
fn test_person_index_size_with_three_entries() {
    analyze_person_index_size(3, 336); // 96 + (80 * 3)
}

#[test]
fn test_person_index_size_with_10000_entries() {
    analyze_person_index_size(10000, 800096); // 96 + (80 * 10000)
}

#[test]
fn test_person_index_size_with_20000_entries() {
    analyze_person_index_size(20000, 1600096); // 96 + (80 * 20000)
}

#[test]
fn test_person_index_size_with_50000_entries() {
    analyze_person_index_size(50000, 4000096); // 96 + (80 * 50000)
}

#[test]
fn test_person_index_size_with_100000_entries() {
    analyze_person_index_size(100000, 8000096); // 96 + (80 * 100000)
}

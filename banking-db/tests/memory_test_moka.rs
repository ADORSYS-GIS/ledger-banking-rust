use banking_db::models::person::PersonIdxModel;
use moka::sync::Cache;
use std::mem;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct PersonCache {
    #[allow(dead_code)]
    by_id: Cache<Uuid, Arc<PersonIdxModel>>,
    #[allow(dead_code)]
    by_hash: Cache<i64, Uuid>,
}

fn analyze_moka_cache_size(num_entries: usize) {
    const HEAP_SIZE_PER_ENTRY: usize = mem::size_of::<Arc<PersonIdxModel>>() + mem::size_of::<Uuid>() + mem::size_of::<i64>();

    println!("\n--- Moka PersonCache Memory Analysis ({num_entries} Entries) ---");

    // 1. Stack Size
    let stack_size = mem::size_of::<PersonCache>();
    println!("Stack Size of PersonCache struct: {stack_size} bytes");

    // 2. Heap Allocation for the data itself
    let total_heap_data_size = HEAP_SIZE_PER_ENTRY * num_entries;
    println!("Heap Allocation (Data Only):");
    println!("  - Size per entry (on heap): {HEAP_SIZE_PER_ENTRY} bytes");
    println!("  - Total Heap Data for {} entries: {} bytes ({:.2} MB)",
        num_entries,
        total_heap_data_size,
        total_heap_data_size as f64 / (1024.0 * 1024.0)
    );
    
    // 3. Total Footprint
    let min_total_size = stack_size + total_heap_data_size;
    println!("\nMinimum Total Memory Footprint ({num_entries} Entries):");
    println!("  - {stack_size} bytes (Stack) + {total_heap_data_size} bytes (Heap Data) + Moka's Internal Overhead");
    println!("  - Minimum Total: {} bytes ({:.2} MB) + overhead",
        min_total_size,
        min_total_size as f64 / (1024.0 * 1024.0)
    );
    println!("\nNOTE: Moka's actual memory usage will be higher due to its sophisticated, concurrent data structures.");
}

#[test]
fn test_moka_cache_with_100000_entries() {
    analyze_moka_cache_size(100000);
}
use banking_db::models::person::CountrySubdivisionIdxModel;
use moka::sync::Cache;
use std::mem;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct CountrySubdivisionCache {
    #[allow(dead_code)]
    by_id: Cache<Uuid, Arc<CountrySubdivisionIdxModel>>,
    #[allow(dead_code)]
    by_country_id: Cache<Uuid, Vec<Arc<CountrySubdivisionIdxModel>>>,
}

#[test]
fn test_country_subdivision_idx_model_size() {
    const EXPECTED_SIZE: usize = 64;
    let actual_size = mem::size_of::<CountrySubdivisionIdxModel>();
    println!("--- CountrySubdivisionIdxModel Memory Layout ---");
    println!("{:<25} | {:<12} | {:<14}", "Field", "Size (bytes)", "Align (bytes)");
    println!("--------------------------|--------------|-----------------");
    println!("{:<25} | {:<12} | {:<14}", "country_subdivision_id (Uuid)", mem::size_of::<Uuid>(), mem::align_of::<Uuid>());
    println!("{:<25} | {:<12} | {:<14}", "country_id (Uuid)", mem::size_of::<Uuid>(), mem::align_of::<Uuid>());
    println!("{:<25} | {:<12} | {:<14}", "code", mem::size_of_val(""), mem::align_of_val(""));
    println!("{:<25} | {:<12} | {:<14}", "is_active (bool)", mem::size_of::<bool>(), mem::align_of::<bool>());
    println!("-----------------------------------------------------");
    println!("Actual size: {} bytes", actual_size);
    println!("Expected size: {} bytes", EXPECTED_SIZE);
    assert_eq!(actual_size, EXPECTED_SIZE, "Size of CountrySubdivisionIdxModel has changed!");
}

#[test]
fn test_country_subdivision_cache_size() {
    const NUM_ENTRIES: usize = 3750; // 250 countries * 15 provinces avg
    const MODEL_SIZE: usize = 64; // Corrected size

    println!("\n--- Moka CountrySubdivisionCache Memory Analysis ({} Entries) ---", NUM_ENTRIES);

    // 1. Stack Size
    let stack_size = mem::size_of::<CountrySubdivisionCache>();
    println!("Stack Size of CountrySubdivisionCache struct: {} bytes", stack_size);

    // 2. Heap Allocation
    let by_id_entry_size = mem::size_of::<Uuid>() + mem::size_of::<Arc<CountrySubdivisionIdxModel>>();
    let by_country_id_entry_size = mem::size_of::<Uuid>() + mem::size_of::<Vec<Arc<CountrySubdivisionIdxModel>>>();
    
    let total_model_heap_size = MODEL_SIZE * NUM_ENTRIES;
    let total_by_id_heap_size = by_id_entry_size * NUM_ENTRIES;
    let total_by_country_id_heap_size = by_country_id_entry_size * 250; // One entry per country
    
    let total_heap_data_size = total_model_heap_size + total_by_id_heap_size + total_by_country_id_heap_size;

    println!("Heap Allocation (Data Only):");
    println!("  - Total for {} models: {} bytes", NUM_ENTRIES, total_model_heap_size);
    println!("  - Total for by_id index: {} bytes", total_by_id_heap_size);
    println!("  - Total for by_country_id index: {} bytes", total_by_country_id_heap_size);
    println!("  - Total Heap Data: {} bytes ({:.2} KB)",
        total_heap_data_size,
        total_heap_data_size as f64 / 1024.0
    );
    
    // 3. Total Footprint
    let min_total_size = stack_size + total_heap_data_size;
    println!("\nMinimum Total Memory Footprint ({} Entries):", NUM_ENTRIES);
    println!("  - Minimum Total: {} bytes ({:.2} KB) + Moka's Internal Overhead",
        min_total_size,
        min_total_size as f64 / 1024.0
    );
}
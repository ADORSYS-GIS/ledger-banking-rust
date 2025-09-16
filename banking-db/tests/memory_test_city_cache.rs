use moka::sync::Cache;
use std::mem;
use std::sync::Arc;
use uuid::Uuid;

// Redefined LocalityIdxModel with hashing for optimization
#[derive(Clone)]
pub struct LocalityIdxModel {
    pub locality_id: Uuid,
    pub country_id: Uuid,
    pub country_subdivision_id: Option<Uuid>,
    pub code_hash: Option<i64>,
    pub is_active: bool,
}

#[derive(Clone)]
pub struct LocalityCache {
    #[allow(dead_code)]
    by_id: Cache<Uuid, Arc<LocalityIdxModel>>,
    #[allow(dead_code)]
    by_country_subdivision_id: Cache<Uuid, Vec<Arc<LocalityIdxModel>>>,
}

#[test]
fn test_locality_idx_model_size() {
    const EXPECTED_SIZE: usize = 72;
    let actual_size = mem::size_of::<LocalityIdxModel>();
    println!("--- Optimized LocalityIdxModel Memory Layout ---");
    println!("Actual size: {actual_size} bytes");
    println!("Expected size: {EXPECTED_SIZE} bytes");
    assert_eq!(actual_size, EXPECTED_SIZE, "Size of LocalityIdxModel has changed!");
}

#[test]
fn test_locality_cache_size_for_cameroon_with_villages() {
    const NUM_LOCALITIES: usize = 12000; // Comprehensive estimate including villages
    const NUM_STATES: usize = 10; // Cameroon has 10 regions
    const MODEL_SIZE: usize = 72;

    println!("\n--- Moka LocalityCache Memory Analysis ({NUM_LOCALITIES} Entries) ---");

    // 1. Stack Size
    let stack_size = mem::size_of::<LocalityCache>();
    println!("Stack Size of LocalityCache struct: {stack_size} bytes");

    // 2. Heap Allocation
    let by_id_entry_size = mem::size_of::<Uuid>() + mem::size_of::<Arc<LocalityIdxModel>>();
    let by_country_subdivision_id_entry_size = mem::size_of::<Uuid>() + mem::size_of::<Vec<Arc<LocalityIdxModel>>>();
    
    let total_model_heap_size = MODEL_SIZE * NUM_LOCALITIES;
    let total_by_id_heap_size = by_id_entry_size * NUM_LOCALITIES;
    let total_by_country_subdivision_id_heap_size = by_country_subdivision_id_entry_size * NUM_STATES;
    
    let total_heap_data_size = total_model_heap_size + total_by_id_heap_size + total_by_country_subdivision_id_heap_size;

    println!("Heap Allocation (Data Only):");
    println!("  - Total for {NUM_LOCALITIES} models: {total_model_heap_size} bytes");
    println!("  - Total for by_id index: {total_by_id_heap_size} bytes");
    println!("  - Total for by_country_subdivision_id index: {total_by_country_subdivision_id_heap_size} bytes");
    println!("  - Total Heap Data: {} bytes ({:.2} MB)",
        total_heap_data_size,
        total_heap_data_size as f64 / (1024.0 * 1024.0)
    );
    
    // 3. Total Footprint
    let min_total_size = stack_size + total_heap_data_size;
    println!("\nMinimum Total Memory Footprint ({NUM_LOCALITIES} Entries):");
    println!("  - Minimum Total: {} bytes ({:.2} MB) + Moka's Internal Overhead",
        min_total_size,
        min_total_size as f64 / (1024.0 * 1024.0)
    );
}
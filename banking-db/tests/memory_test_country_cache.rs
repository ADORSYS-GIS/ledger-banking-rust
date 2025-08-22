use banking_db::models::person::CountryIdxModel;
use heapless::String as HeaplessString;
use moka::sync::Cache;
use std::mem;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct CountryCache {
    #[allow(dead_code)]
    by_id: Cache<Uuid, Arc<CountryIdxModel>>,
    #[allow(dead_code)]
    by_iso2: Cache<HeaplessString<2>, Uuid>,
}

#[test]
fn test_country_idx_model_size() {
    const EXPECTED_SIZE: usize = 40;
    let actual_size = mem::size_of::<CountryIdxModel>();

    println!("--- CountryIdxModel Memory Layout ---");
    println!("{:<25} | {:<12} | {:<14}", "Field", "Size (bytes)", "Align (bytes)");
    println!("--------------------------|--------------|-----------------");
    println!("{:<25} | {:<12} | {:<14}", "country_id (Uuid)", mem::size_of::<Uuid>(), mem::align_of::<Uuid>());
    println!("{:<25} | {:<12} | {:<14}", "iso2 (HeaplessString<2>)", mem::size_of::<HeaplessString<2>>(), mem::align_of::<HeaplessString<2>>());
    println!("{:<25} | {:<12} | {:<14}", "is_active (bool)", mem::size_of::<bool>(), mem::align_of::<bool>());
    println!("-----------------------------------------------------");
    println!("Actual size: {} bytes", actual_size);
    println!("Expected size: {} bytes", EXPECTED_SIZE);
    
    assert_eq!(actual_size, EXPECTED_SIZE, "Size of CountryIdxModel has changed!");
}

#[test]
fn test_country_cache_size_for_all_countries() {
    const NUM_COUNTRIES: usize = 250;
    const MODEL_SIZE: usize = 40; // Corrected size

    println!("\n--- Moka CountryCache Memory Analysis ({} Entries) ---", NUM_COUNTRIES);

    // 1. Stack Size
    let stack_size = mem::size_of::<CountryCache>();
    println!("Stack Size of CountryCache struct: {} bytes", stack_size);

    // 2. Heap Allocation per entry
    let by_id_entry_size = mem::size_of::<Uuid>() + mem::size_of::<Arc<CountryIdxModel>>();
    let by_iso2_entry_size = mem::size_of::<HeaplessString<2>>() + mem::size_of::<Uuid>();
    let total_heap_per_entry = MODEL_SIZE + by_id_entry_size + by_iso2_entry_size;
    
    let total_heap_data_size = total_heap_per_entry * NUM_COUNTRIES;

    println!("Heap Allocation (Data Only):");
    println!("  - Size per entry (on heap): {} bytes ({} model + {} by_id + {} by_iso2)", 
        total_heap_per_entry, MODEL_SIZE, by_id_entry_size, by_iso2_entry_size);
    println!("  - Total Heap Data for {} countries: {} bytes ({:.2} KB)",
        NUM_COUNTRIES,
        total_heap_data_size,
        total_heap_data_size as f64 / 1024.0
    );
    
    // 3. Total Footprint
    let min_total_size = stack_size + total_heap_data_size;
    println!("\nMinimum Total Memory Footprint ({} Countries):", NUM_COUNTRIES);
    println!("  - Minimum Total: {} bytes ({:.2} KB) + Moka's Internal Overhead",
        min_total_size,
        min_total_size as f64 / 1024.0
    );
}
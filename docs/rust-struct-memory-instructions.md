# Rust Struct Design & Memory Allocation Instructions

## Core Principles

### 1. Choose Appropriate String Types
- **`[u8; N]`**: For fixed-length ASCII data (e.g., currency codes, product IDs)
- **`HeaplessString<N>`**: For variable-length UTF-8 data with known max size
- **`String`**: Only when truly variable/unbounded length is needed
- **Enum**: For known sets of values (e.g., currencies, statuses)

### 2. Memory Allocation Strategy

#### Stack Allocation Guidelines
- **< 256 bytes**: Always use stack
- **256 bytes - 2KB**: Use stack for temporary objects, heap for long-lived
- **> 2KB**: Prefer heap allocation
- **Recursive functions**: Always use heap to prevent stack overflow

#### When to Box (Heap Allocate)
```rust
// DON'T: Large struct on stack in recursive function
fn recursive_process(account: Account) { ... }  // ❌ 1KB copy each call

// DO: Use Box for large structs
fn recursive_process(account: Box<Account>) { ... }  // ✅ 8 bytes moved
```

### 3. Struct Design Patterns

#### For Fixed-Size Fields
```rust
// GOOD: No heap allocations, fully stack-allocated
pub struct Account {
    pub id: Uuid,                    // 16 bytes, fixed
    pub code: [u8; 12],              // 12 bytes, fixed
    pub currency: [u8; 3],           // 3 bytes, fixed
    pub balance: Decimal,            // ~16 bytes, fixed
    pub status: AccountStatus,       // enum, fixed size
}

// BETTER: With validation
pub struct CurrencyCode([u8; 3]);
impl CurrencyCode {
    pub fn new(code: &str) -> Result<Self, Error> {
        // Validate input
    }
}
```

#### For Variable-Length Fields with Bounds
```rust
// Use heapless for bounded variable data
use heapless::String as HeaplessString;

pub struct Account {
    pub description: HeaplessString<200>,  // Max 200 chars, no heap
    pub notes: HeaplessString<500>,        // Max 500 chars, no heap
}
```

### 4. Performance Optimization Rules

#### Function Parameters
```rust
// AVOID: Copying large structs
fn process(account: Account) { ... }  // ❌ Copies entire struct

// PREFER: Borrowing
fn process(account: &Account) { ... }  // ✅ Just passes reference

// FOR OWNERSHIP TRANSFER: Use Box
fn store(account: Box<Account>) { ... }  // ✅ Moves 8 bytes
```

#### Collections
```rust
// For collections, data goes on heap anyway
let accounts: Vec<Box<Account>> = vec![];  // Good for large structs
let accounts: Vec<Account> = vec![];       // OK if struct is small
```

### 5. Serialization Helpers
```rust
// For [u8; N] fields that represent strings
fn serialize_as_str<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer {
    let s = std::str::from_utf8(bytes)
        .map_err(serde::ser::Error::custom)?;
    serializer.serialize_str(s)
}

fn deserialize_from_str<'de, D, const N: usize>(
    deserializer: D
) -> Result<[u8; N], D::Error>
where D: Deserializer<'de> {
    let s = String::deserialize(deserializer)?;
    let bytes = s.as_bytes();
    bytes.try_into()
        .map_err(|_| serde::de::Error::custom("Invalid length"))
}
```

## Decision Flowchart

```
Is the data size known at compile time?
├─ YES → Is it always exactly N bytes?
│   ├─ YES → Use [u8; N] or custom type
│   └─ NO → Use HeaplessString<N> or HeaplessVec<T, N>
└─ NO → Is there a reasonable maximum size?
    ├─ YES → Use HeaplessString<MAX> or HeaplessVec<T, MAX>
    └─ NO → Use String/Vec (heap allocated)

Is the struct > 1KB?
├─ YES → Is it used in recursive functions?
│   ├─ YES → Always use Box<T>
│   └─ NO → Is it moved frequently?
│       ├─ YES → Use Box<T>
│       └─ NO → Stack is OK for temporary use
└─ NO → Prefer stack allocation
```

## Code Generation Rules

1. **Default to stack allocation** for structs under 1KB
2. **Use fixed-size arrays** for data with known length (IDs, codes)
3. **Prefer HeaplessString** over String when max size is known
4. **Box large structs** (>1KB) when storing or moving them
5. **Use references** (`&T`) for function parameters to avoid copies
6. **Implement Display/Debug** for [u8; N] fields for better ergonomics
7. **Add validation** in constructors for constrained types
8. **Use enums** for closed sets of values instead of strings

## Example Transformation

```rust
// BEFORE: Inefficient design
pub struct Account {
    pub id: String,              // Heap allocated
    pub currency: String,        // Heap allocated
    pub description: String,     // Heap allocated
    pub balance: f64,           // Imprecise
}

// AFTER: Optimized design
pub struct Account {
    pub id: Uuid,                           // Fixed 16 bytes
    pub currency: Currency,                 // Enum, 1 byte
    pub description: HeaplessString<200>,   // Fixed size, no heap
    pub balance: Decimal,                   // Precise, fixed size
}

#[derive(Debug, Copy, Clone)]
pub enum Currency { USD, EUR, GBP, JPY }
```

## Performance Impact Summary

- Stack allocation: ~0-2 ns
- Heap allocation: ~50-100 ns
- Copying 1KB struct: ~10-50 ns
- Moving Box<T>: ~1 ns
- Following pointer indirection: ~1-3 ns

Choose based on usage patterns, not just allocation cost.
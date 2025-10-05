---
argument-hint: <file to process>
description: Generates a thread-safe, immutable cache implementation for Rust structs based on caching instructions in comment blocks.
---

# Prompt for the Generation of Immutable Caches

This command generates a thread-safe, immutable cache implementation for each Rust struct in a given file, based on caching instructions provided in a `# Cache` comment block.

**Rule Precedence:** The generation of an `...IdxModelCache` is driven entirely by the `/// # Cache` comment block on the corresponding `...Model` struct.

For detailed instructions on the generation process, refer to the [Repository and Indexing Strategy](../../docs/guidelines/repository-and-indexing.md).
# Prompt: Generate Application-Managed Indexes for a Module

Apply the rules defined in 'docs/guidelines/repository-and-indexing.md' to generate the necessary code and database schema for application-managed indexes for a given module.

**Rule Precedence:** All generation logic is driven by structured comments within the source `...Model` struct. These in-code instructions always take precedence over general guidelines.

**Inputs:**
-   `<module_name>`: The name of the module (e.g., `person`).
-   `<entity_name>`: The name of the entity (e.g., `country`).

---

### Steps

1.  **Generate Index Model from Comment Instructions** in `banking-db/src/models/{module_name}/{entity_name}.rs`.
2.  **Generate Repository Methods** in `banking-db/src/repository/{module_name}/{entity_name}_repository.rs`.
3.  **Generate Database Migration Script** in `banking-db-postgres/migrations/<init_order>_initial_schema_{module_name}.sql`.
4.  **Apply Hashing Strategy** for string-based indexes.

For detailed instructions on each step, refer to the [Repository and Indexing Strategy](../../docs/guidelines/repository-and-indexing.md).

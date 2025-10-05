# Command: Module Layout

**Objective**: Refactors a module to the new entity-based layout.

**Parameters**:
-   `<module_name>`: The name of the module to refactor (e.g., audit).

## Instructions

Refactor the `{{module_name}}` module to follow the entity-based file layout.

You must adhere to the following process:
1.  Identify all entities within the `{{module_name}}` module by inspecting the contents of `banking-db/src/models/{{module_name}}.rs`.
2.  For each entity, apply the file and directory restructuring pattern across all four relevant crates (`banking-db`, `banking-db-postgres`, `banking-logic`, and `banking-api`).
3.  Move the contents of the existing monolithic module files into the new, entity-specific files.
4.  Update all `mod.rs` files to correctly reflect the new module structure.

For a detailed breakdown of the required file structure and the steps to follow, you must use the guide located at: `docs/guidelines/module_2_entity_layout.md`. Use the `audit` module in that guide as a concrete example of the expected outcome.
# Module Dependency Graph

This document outlines the dependency graph for the database models, which determines the correct initialization order for database scripts. The analysis is based on foreign key relationships implied by `Uuid` fields in the model structs.

## Dependency Tiers

### Tier 0: Foundational Modules
These are the most basic entities that other modules build upon.
- `person`

### Tier 1: Core Concepts
These modules depend on Tier 0.
- `product` (depends on `person`)
- `customer` (depends on `person`)
- `reason_and_purpose` (depends on `person`)
- `calendar` (depends on `person`)

### Tier 2: Infrastructure and Seeding
These modules build upon the core concepts.
- `agent_network` (depends on `person`, `reason_and_purpose`)
- `reason_and_purpose_seeds` (depends on `reason_and_purpose`)

### Tier 3: The Central `Account` Module
The `account` module is central and has many dependencies.
- `account` (depends on `product`, `agent_network`, `reason_and_purpose`, `person`, `customer`)

### Tier 4: Account-Dependent Modules
These modules heavily rely on the existence of `account`.
- `account_hold` (depends on `account`, `reason_and_purpose`, `person`, `customer`)
- `collateral` (depends on `person`, `account`)
- `transaction` (depends on `account`, `agent_network`, `person`, `reason_and_purpose`)
- `casa` (depends on `account`, `person`, `reason_and_purpose`, `agent_network`)

### Tier 5: High-Level Business Logic Modules
These modules represent more complex business functionalities.
- `fee` (depends on `account`, `transaction`, `product`, `person`, `reason_and_purpose`)
- `loan` (depends on `account`, `person`, `reason_and_purpose`, `casa`)
- `daily_collection` (depends on `person`, `reason_and_purpose`, `customer`, `account`, `collateral`)
- `compliance` (depends on `customer`, `agent_network`, `account`, `transaction`, `person`, `reason_and_purpose`)

### Tier 6: Composite and Workflow Modules
These modules orchestrate or depend on the outcomes of the high-level business modules.
- `channel` (depends on `fee`, `transaction`)
- `workflow` (depends on `account`, `person`, `customer`, `product`, `transaction`, `reason_and_purpose`)

### Tier 7: Views
Views should be created last as they read from all the underlying tables.
- `reason_view`

## Recommended Initialization Order

1.  **Tier 0:** `person`
2.  **Tier 1:** `product`, `customer`, `reason_and_purpose`, `calendar`
3.  **Tier 2:** `agent_network`, `reason_and_purpose_seeds`
4.  **Tier 3:** `account`
5.  **Tier 4:** `account_hold`, `collateral`, `transaction`, `casa`
6.  **Tier 5:** `fee`, `loan`, `daily_collection`, `compliance`
7.  **Tier 6:** `channel`, `workflow`
8.  **Tier 7:** `reason_view`
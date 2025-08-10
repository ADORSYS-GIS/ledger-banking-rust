# Ledger Banking Rust - Core Banking System

## Executive Summary

Enterprise-grade core banking system built with Rust supporting multi-product banking (savings, current accounts, loans), agent networks, compliance, and workflow management.

**Current Status**: PostgreSQL repository implementations **100% complete** (13 of 13 repositories) with 298+ tests passing. Service implementations 88% complete (15/17). Comprehensive identifier normalization across entire codebase ensuring consistent `id` field naming.

## Architecture & Stack

### Active Workspace (4 crates)
```
banking-api/           # Domain models & service traits (✅ Complete)
banking-db/            # Database abstraction layer (✅ Complete)  
banking-logic/         # Business logic implementations (🚧 88% complete)
banking-db-postgres/   # PostgreSQL implementation (✅ 100% complete)
```

### Technology Stack
- **Runtime**: Rust 2021 + Tokio async
- **Database**: SQLx + PostgreSQL with native UUID support
- **Memory**: heapless::String for stack allocation, Blake3 hashing
- **Cache**: Moka high-performance async cache
- **Financial**: rust_decimal for precision arithmetic

### Design Patterns
- **Onion Architecture**: Clean separation of concerns
- **Repository Pattern**: Database abstraction with traits
- **Domain-Driven Design**: Rich domain models with business rules
- **Builder Pattern**: Clippy-compliant object construction

## Domain Models (16 models - 100% Complete)

### Core Banking
- **Customer**: CIF with risk ratings, KYC status, builder pattern
- **Account**: Unified model supporting all product types (savings/current/loan)
- **Transaction**: Multi-stage validation with audit trails
- **Agent Network**: Hierarchical structure (Network → Branch → Terminal)
- **Person**: Comprehensive person/entity management with addresses
- **Collateral**: Enterprise collateral management with valuations
- **Workflow**: Multi-step processes with approvals
- **Compliance**: KYC/AML with sanctions screening
- **Calendar**: Business day calculations
- **Daily Collection**: Agent-mediated collection programs with route optimization

### Memory Optimization
- **HeaplessString<N>**: Stack-allocated bounded strings (60-70% heap reduction)
- **Enum Status Fields**: Type-safe status management vs String
- **Currency Codes**: ISO 4217 compliant HeaplessString<3>

## Development Guidelines

process file docs/guidelines/development.md whenever you want to 
- define new struct
- fix or modify existing structs 
- define new fields 
- fix or modify existing fields
- define new enums
- fix or modify existing enums

## Implementation Status

process file docs/progress/progress-tracking.md only when you want to 
- read or update the completion status of the project
- check for next steps and gaps in the implementation.

## Development Workflow

### Testing (Database, Test Isolation)

process file docs/guidelines/testing.md only when you want to 
- define new tests
- fix or modify existing tests.
- run tests



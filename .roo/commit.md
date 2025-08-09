refactor(compliance): Normalize ID reference patterns across modules

Refactor compliance ID handling to use consistent reference patterns across domain models, services, and database schemas

Components affected:
- banking-api/src/domain/compliance.rs: Refactored ID validation logic
- banking-db/src/models/compliance.rs: Updated schema to use normalized ID references
- banking-db-postgres/migrations/001_initial_schema.sql: Modified schema to match new ID patterns
- banking-logic/src/mappers/compliance_mapper.rs: Updated mapping strategies
- banking-logic/src/services/compliance_service_impl.rs: Modified service layer to handle normalized IDs
- banking-logic/src/services/lifecycle_service_impl.rs: Updated lifecycle management for new ID patterns

Benefits:
- Improved consistency across compliance workflows
- Easier maintenance of ID validation logic
- Better data integrity through standardized references

🤖 Generated with [Claude Code](https://claude.ai/code)

Co-Authored-By: Claude <noreply@anthropic.com>

🤖 Generated with [Qwen Code](https://tongyi.aliyun.com/qianwen/)

Co-Authored-By: Qwen <noreply@alibaba.com>
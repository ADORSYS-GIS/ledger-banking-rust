-- Main table for model AuditLogModel
CREATE TABLE audit_log (
    id UUID PRIMARY KEY,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_by_person_id UUID NOT NULL
);

-- Index table for AuditLog has been removed as per user request.
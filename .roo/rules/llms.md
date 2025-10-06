# Ledger Banking Rust - LLM Rules

This document outlines the key rules and guidelines for LLM-based development on the Ledger Banking Rust project.

## 1. Architecture & Design

For a comprehensive overview of the project's architecture, design patterns, and domain models, refer to the [Architecture Overview](../../.docs/architecture/overview.md).

## 2. Development Guidelines

All development work should adhere to the established patterns and best practices. For detailed information, refer to the main [Development Guidelines](../../.docs/guidelines/development.md).

## 3. Context Provisioning

To ensure the model has sufficient context for any given task, the file listings for the following directories must always be provided:
- `.docs/architecture`
- `.docs/guidelines`

This allows the model to proactively request relevant documentation and adhere to the project's established patterns and best practices.

## 4. Implementation Status

To understand the current progress of the project, identify next steps, or update the completion status, refer to the [Progress Tracking](../../.docs/progress/progress-tracking.md) document.

## 5. Instruction Precedence

In case of conflicting instructions, instructions provided in code comments (e.g., `/// # Cache`) take precedence over external instructions found in command files or other documentation.

## 6. General LLM Instructions

- Keep responses concise.
- When coding, return code only, or if necessary, only minimal instructions for the code assistant. Explanations are not needed.
- When fixing a specific code block, return only the corrected code and minimal instructions.
- Keep the summary of the task concise.

# Git Commit Analysis and Signing Prompt

This prompt provides a structured approach for analyzing changes and creating properly signed commits in the ledger-banking-rust project.

## Usage

Use this prompt when you need to:
- Analyze all pending changes in the git working directory
- Create a comprehensive commit message
- Commit with proper signing (-s -S flags)
- use "git add --all" if you want to add all changes

## Prompt Template

```
Analyze all changes and commit with -s -S.
```

## Expected Workflow

When this prompt is used, the assistant should:

1. **Analyze Changes**:
   - Run `git status` to see all modified/added/deleted files
   - Run `git diff --stat` to understand the scope of changes
   - Identify the primary purpose and scope of the changes

2. **Create Commit Template** (if requested):
   - Generate a comprehensive commit template
   - Store it in `prompts/commit.md`

3. **Stage Relevant Changes**:
   - Use `git add` to stage files related to the main task
   - Handle file deletions with `git rm` if needed
   - Focus on logically related changes for a single commit

4. **Create Comprehensive Commit Message**:
   - Follow conventional commit format: `<type>(<scope>): <subject>`
   - Include detailed body explaining the changes
   - Specify affected components
   - Explain benefits and impact
   - Add Claude Code attribution
   - Add Qwen Code attribution

5. **Commit with Proper Signing**:
   - Use both `-s` (DCO sign-off) and `-S` (GPG signature) flags
   - Verify the commit was properly signed

## Commit Message Structure

```
<type>(<scope>): <subject>

<detailed description of changes and why they were made>

Components affected:
- Component 1: Description of changes
- Component 2: Description of changes
- Component 3: Description of changes

Benefits:
- Benefit 1: Specific improvement achieved
- Benefit 2: Specific improvement achieved
- Benefit 3: Specific improvement achieved

🤖 Generated with [Claude Code](https://claude.ai/code)

Co-Authored-By: Claude <noreply@anthropic.com>

🤖 Generated with [Qwen Code](https://tongyi.aliyun.com/qianwen/)

Co-Authored-By: Qwen <noreply@alibaba.com>

```

## Commit Types

- **feat**: New feature or capability
- **fix**: Bug fix or error correction
- **refactor**: Code restructuring without functionality changes
- **perf**: Performance improvements
- **style**: Code formatting, whitespace, style changes
- **docs**: Documentation updates
- **test**: Adding or updating tests
- **chore**: Maintenance tasks, dependency updates
- **build**: Build system or CI/CD changes
- **revert**: Reverting previous changes

## Scope Guidelines

- **workspace**: Multi-crate changes spanning several components
- **api**: Changes to banking-api domain models
- **db**: Changes to banking-db models or repositories
- **logic**: Changes to banking-logic services or mappers
- **schema**: Database schema changes
- **migration**: Database migration files

## Signing Requirements

Always commit with both flags:
```bash
git commit -s -S -m "commit message"
```

- `-s`: Adds Signed-off-by line for DCO compliance
- `-S`: Adds GPG signature for authenticity verification

## Pre-Commit Verification

Before committing, ensure:
- [ ] All tests pass (`cargo test --workspace`)
- [ ] Code compiles without warnings (`cargo check --workspace`)
- [ ] Changes are logically grouped
- [ ] Commit message is comprehensive and follows format
- [ ] Proper signing flags are used

## Example Usage

**Input**: "Analyze all changes and commit with -s -S"

**Expected Output**:
1. Analysis of current git status and changes
2. Creation of this template file (if requested)
3. Staging of relevant changes
4. Creation of comprehensive commit with proper signing
5. Verification of commit signature

This ensures consistent, well-documented, and properly signed commits across the banking system development lifecycle.
# Smart Organizer Project State

Updated: 2026-10-04

## Product and Users

Smart Organizer is a Windows-first, local-only desktop tool for people who want to clean a folder without losing control of their files. Its central promise is plan first: scan metadata, review proposed moves, apply only after confirmation, and undo completed operations when possible.

## Current User Flow

The implemented path is Home → choose a folder → scan direct child files → review a category-based or rule-directed plan → select operations → apply moves → inspect local history → undo a transaction. Rules can be edited, validated, parsed from two deterministic example phrases, saved locally, and applied when enabled move rules match.

## Implementation Status

- React 19 + TypeScript + Vite frontend with Zustand state.
- Tauri 2/Rust backend with typed commands for health, app info, scanning, planning, applying, undoing, history, and rules.
- Scanner ignores directories, hidden/system entries, symlinks, and special files; it reads only the selected folder's first layer.
- Planner classifies known extensions, skips `Other`, rejects forged paths, and creates conflict-safe destination names.
- Filesystem engine validates root containment, never overwrites an existing destination, records partial failures, and supports undo.
- History and rules are stored as JSON in the Tauri application data directory.
- Rule parsing remains a two-example seam, rename rules are rejected until their semantics are implemented, and Settings remains a placeholder.

## Architecture and Constraints

`src/services/runtime.ts` is the frontend/backend boundary. React owns presentation and transient UI state; Rust owns path validation, filesystem changes, transactions, and persistence. Keep file operations plan-based, local, non-overwriting, and reversible. Do not let natural-language parsing directly touch the filesystem.

## Verification Baseline

`pnpm test` passes 3 frontend tests; `pnpm typecheck` passes; `pnpm build` passes; `cargo test --manifest-path src-tauri/Cargo.toml` passes 12 Rust tests. There is no repository-level CI currently running these checks, no lint script, no end-to-end test, and no release automation.

## Known Gaps

The parser is intentionally a two-example mock; rename rules and Settings are not implemented; the scanner is non-recursive; UI timestamps are hard-coded as “Just now”; and frontend coverage does not exercise the scan → plan → apply → undo flow.

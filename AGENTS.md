# Repository Guidelines

## Project Structure & Module Organization

This repository is a Tauri 2 desktop app. The React/TypeScript frontend lives in `src/`: pages are in `src/pages`, reusable UI in `src/components`, state in `src/store`, runtime adapters in `src/services`, and shared types in `src/types`. Rust backend code is under `src-tauri/src`, with filesystem, scanning, rules, organization, history, and command modules separated by responsibility. Static catalog data belongs in `src-tauri/resources`; platform icons are in `src-tauri/icons`. Tests are in `tests/` and use the `*.test.ts` naming pattern.

## Build, Test, and Development Commands

Install dependencies with `pnpm install`. Use `pnpm dev` for the Vite browser preview and `pnpm tauri dev` for the native desktop shell (Windows requires the Rust MSVC toolchain and C++ build tools). Run `pnpm typecheck` for TypeScript validation, `pnpm test` for the Vitest suite, and `pnpm build` for the production frontend build. To verify a native release without packaging, use `pnpm tauri build --no-bundle`.

## Coding Style & Naming Conventions

Use four-space indentation in TypeScript/TSX and Rust, double-quoted strings in TypeScript, and semicolons where the existing code uses them. Prefer focused, typed functions and keep React components in PascalCase (`RulesPage.tsx`); use camelCase for variables, hooks, and services. Rust modules and files use snake_case. Keep frontend/backend boundaries explicit through shared TypeScript types and typed Tauri commands. Run `pnpm typecheck` and `pnpm build` before submitting frontend changes.

## Testing Guidelines

Vitest runs tests in the Node environment and discovers `tests/**/*.test.ts`. Add regression coverage beside related tests, naming cases after observable behavior (for example, `appStore.test.ts`). Keep tests deterministic and avoid touching real user files; filesystem behavior should be tested through safe fixtures or isolated abstractions.

## Commit & Pull Request Guidelines

Use short, imperative commit subjects, matching the repository history (for example, `Implement resume core domain` and `Define resume V2 data model`). Keep commits focused. Pull requests should describe the user-visible change, identify affected frontend/backend areas, link the relevant issue or task, and include screenshots or a short recording for UI changes. Report commands run and call out any platform-specific or unverified behavior.

## Safety & Configuration

Preserve the product invariants: plan before execution, never overwrite silently, record operations, and undo where possible. Rust owns scanning, path validation, file operations, transactions, and persistence; React should only present state and request typed commands. Never commit secrets, local paths, generated `target/` output, or user files.

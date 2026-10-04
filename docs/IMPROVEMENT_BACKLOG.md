# Improvement Backlog

Scoring: Impact, Urgency, Confidence, Effort, and Risk are each 1–5. Effort and Risk are costs, so lower is better.

## Completed this iteration

- Enabled persisted move rules now participate in plan generation with first-match precedence and safe relative destinations.
- Unsupported rename rules are rejected explicitly instead of being saved as no-ops.
- Added Rust coverage for rule precedence and updated the README and project state.

## P1 — Apply saved rules when generating a plan

- **Status:** Completed in this iteration; retain this entry as historical context until end-to-end UI coverage confirms the command contract.
- **Problem:** `rule_store` persisted rules, but `organizer::generate_plan` previously only used scanner categories and never evaluated rules.
- **Why:** The Rules page promises that saved rules affect the next plan; today that promise is false, so a core surface creates no product value.
- **User impact:** Users can spend time defining rules and believe their files will be organized differently than they are.
- **Plan:** Introduce a pure rule-evaluation layer over `ScannedFile`, pass validated enabled rules into planning, preserve category fallback behavior when no rule matches, and add Rust unit tests for matching, precedence, and unsafe destinations.
- **Risk:** Medium; rule precedence and backward compatibility need explicit semantics.
- **Effort:** 3/5. **Priority score:** Impact 5, Urgency 4, Confidence 5, Effort 3, Risk 3.

## P1 — Make local JSON persistence crash-safe

- **Problem:** History and rules write a temporary file, delete the existing file, then rename; a crash between deletion and rename can lose local records.
- **Why:** These records are the user's audit trail and undo index.
- **User impact:** History or saved rules may disappear after an interrupted write.
- **Plan:** Use a durable replace strategy with flush/sync where supported, retain the old file until the replacement succeeds, and test interrupted/invalid-file recovery.
- **Risk:** Medium across Windows filesystem semantics.
- **Effort:** 3/5. **Priority score:** Impact 4, Urgency 3, Confidence 4, Effort 3, Risk 3.

## P2 — Cover the end-to-end organization workflow

- **Problem:** Frontend tests cover only three store mutations; no test crosses scan, plan, apply, history, and undo boundaries.
- **Why:** File-moving behavior is the highest-risk product path.
- **User impact:** Regressions can reach users without a workflow-level signal.
- **Plan:** Add Rust integration coverage for command/domain contracts and a browser-level smoke test with a safe fixture or mocked Tauri bridge.
- **Risk:** Test harness complexity.
- **Effort:** 4/5. **Priority score:** Impact 5, Urgency 3, Confidence 5, Effort 4, Risk 3.

## P2 — Replace MVP placeholders and hard-coded feedback

- **Problem:** Settings is a placeholder, rule parsing supports two phrases, and activity timestamps are hard-coded to “Just now”.
- **Why:** These reduce trust and make the product appear more complete than it is.
- **User impact:** Confusing expectations and stale activity history.
- **Plan:** Label unsupported capabilities explicitly, derive timestamps from stored values, and implement only the smallest settings slice needed for safety preferences.
- **Risk:** Low.
- **Effort:** 3/5. **Priority score:** Impact 3, Urgency 2, Confidence 5, Effort 3, Risk 1.

## P3 — Establish engineering automation

- **Problem:** No lint command, CI workflow, or release/versioning check exists.
- **Why:** Quality gates depend on manual discipline.
- **User impact:** Lower release confidence rather than an immediate user-facing failure.
- **Plan:** Add focused linting, run frontend and Rust tests in CI, and document Windows build prerequisites.
- **Risk:** Low to medium due to tool configuration.
- **Effort:** 3/5. **Priority score:** Impact 3, Urgency 2, Confidence 4, Effort 3, Risk 2.

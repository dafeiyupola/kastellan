//! Observation-phase support for CASSANDRA rule iteration.
//!
//! The agent's `ConstitutionalGuard` and `DeterministicPolicy` reviewer
//! stages are wired for real (see `core::cassandra::review`): Stage −1
//! screens the instruction for principle violations and Stage 0 enforces
//! the data-classification invariants. Only `NoopReviewStage` always
//! `Approve`s, and that one is a test seam. The CASSANDRA design plan §9 and HANDOVER
//! both prescribe the same approach to designing real rule sets:
//! **observe** — run varied prompts through the live agent, capture what
//! the planner produces, and iterate candidate rules against that frozen
//! dataset rather than against assumptions.
//!
//! This module owns the **dataset infrastructure**:
//!
//! - [`capture::CaptureJson`] — on-disk envelope per (fixture, date, model)
//!   baseline; one file per capture, never overwritten.
//! - Pure helpers ([`capture::parse_fixture_prompt`],
//!   [`capture::slug_model`], [`capture::capture_filename`],
//!   [`capture::extract_plans_from_audit_rows`]) — unit-tested; no I/O.
//! - [`capture::write_capture_to_dir`] — IO helper; refuses to overwrite.
//! - [`capture::fetch_audit_rows_for_task`] — async DB helper for the
//!   orchestrator.
//!
//! The orchestrator itself lives in `core/tests/observation_capture.rs`
//! and is `#[ignore]`-flagged so `cargo test --workspace` does not invoke
//! it (the live-LLM dep is not CI-friendly).
//!
//! The rule-iteration follow-up slice (not in this slice) will consume
//! the captured `plans[].plan_json` values by re-running
//! `ChainReviewStage::new(vec![candidate_rule])` against them and
//! reporting which fixtures' verdicts changed.

pub mod capture;
pub mod replay;

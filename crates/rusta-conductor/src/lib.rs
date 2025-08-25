//! Episode FSM (S0–S3) → Autonomy Contract (S4+)
#![deny(warnings)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum State {
    Sense,
    Plan,
    Probe,
    Propose,
    Validate,
    Reflect,
    Remember,
    End,
}

pub struct Conductor;

impl Conductor {
    pub fn step(_s: State) -> State {
        // Hardcoded while immature → reproducible & safe.
        // TODO: implement guardrails, budgets, and transitions.
        State::End
    }
}

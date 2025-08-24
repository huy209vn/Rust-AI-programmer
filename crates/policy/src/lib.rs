#[derive(Clone, Debug)]
pub enum Mode { RELAXED, STRICT }

#[derive(Clone, Debug)]
pub enum TraceLevel { OFF, HEADS, SUMMARY, FULL }

pub struct Policy {
    pub mode: Mode,
    pub trace_level: TraceLevel,
    pub tau_answer: f32,
    pub tau_patch: f32,
    pub k_minimality: u32,
    pub explore_budget: u32,
}

impl Default for Policy {
    fn default() -> Self {
        Self {
            mode: Mode::RELAXED,          // free by default
            trace_level: TraceLevel::OFF, // no traces by default
            tau_answer: 0.70,
            tau_patch: 0.85,
            k_minimality: 20,
            explore_budget: 3,
        }
    }
}

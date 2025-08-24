use policy::Policy;

pub trait ToolBus {
    fn cargo_check(&mut self, _ws: &str) -> anyhow::Result<()> { Ok(()) }
}

pub struct Planner { pub policy: Policy }
impl Default for Planner { fn default() -> Self { Self { policy: Policy::default() } } }

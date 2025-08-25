#![deny(warnings)]
/// Dataset traits, MixtureSampler, provenance ledger
pub trait Dataset { fn len(&self) -> usize; }
pub struct MixtureSampler;
impl MixtureSampler { pub fn new() -> Self { Self } }

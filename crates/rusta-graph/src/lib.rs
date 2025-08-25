#![deny(warnings)]
/// ProjectGraph (RA snapshot cache)
pub struct Graph;
impl Graph {
    pub fn snapshot_key(_commit: &str, _edition: &str, _ra_hash: &str) -> String {
        // TODO: stable cache key
        "snap-KEY".into()
    }
}

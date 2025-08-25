#![deny(warnings)]
/// Tool adapters: cargo_check, ra_type_of, test, indexer, patcher, doc_index, net
pub mod cargo_check {
    pub fn run(_path: &str) -> Result<(), String> {
        // TODO: spawn cargo check with normalized diagnostics & timeouts
        Err("not implemented".into())
    }
}
pub mod ra_type_of {
    pub fn query(_file: &str, _line: usize, _col: usize) -> Result<String, String> {
        // TODO: call RA type-of
        Err("not implemented".into())
    }
}

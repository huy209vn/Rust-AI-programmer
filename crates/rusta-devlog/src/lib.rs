#![deny(warnings)]
/// WAL writer + crash recovery for DevLogs
pub struct Wal;
impl Wal {
    pub fn append(_entry_json: &str) -> std::io::Result<()> {
        // TODO: fsync & recovery markers
        Ok(())
    }
}

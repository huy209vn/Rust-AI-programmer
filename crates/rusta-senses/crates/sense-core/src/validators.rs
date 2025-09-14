use crate::error::{SenseError, SenseResult};
use crate::traits::adapter::AdapterOut;

/// Sanity-check invariants for any `AdapterOut`.
///
/// - `emb.len() == shape.n * shape.d_model`
/// - `sidecar.lengths` (if present) matches `shape`
/// - `sidecar.evidence_hash8` mirrors `evidence.hash8`
#[must_use]
pub fn validate_adapter_out(out: &AdapterOut) -> SenseResult<()> {
    let expected = out.shape.len();
    if out.emb.len() != expected {
        return Err(SenseError::Internal(format!(
            "embeddings length {} != expected {} (n*d_model)", out.emb.len(), expected
        )));
    }
    if let Some(lengths) = out.sidecar.lengths {
        if lengths != out.shape {
            return Err(SenseError::Internal("sidecar.lengths != shape".into()));
        }
    }
    if out.sidecar.evidence_hash8 != out.evidence.hash8 {
        return Err(SenseError::Internal("sidecar.evidence_hash8 mismatch".into()));
    }
    Ok(())
}

What’s in sense-core right now (contracts only)

Directory (from your latest snapshot):

sense-core/
├─ Cargo.toml
├─ README.md
├─ LICENSE-{MIT,APACHE}
└─ src/
   ├─ lib.rs                 # module map, forbid unsafe, docs
   ├─ bytes.rs               # Bytes<'a> = Borrowed/Owned(Arc<[u8]>)
   ├─ frame.rs               # ByteFrame<'a>, Timestamp, Hints, SourceId
   ├─ status.rs              # Status {Ok, Warn, Degraded, Fail} (+ helpers)
   ├─ error.rs               # SenseError + ErrorCode + status_hint()
   ├─ determinism.rs         # Determinism(D1/D2/D3), BudgetClass, RandSeed, EmbeddingsShape
   ├─ config.rs              # Caps, Features bitflags, SenseCfg, EngineCfg, InspectPolicy
   ├─ evidence.rs            # EvidenceHandle, short blake3 hash helpers
   ├─ sidecar.rs             # Sidecar + BackendInfo + modality sidecars
   ├─ registry.rs            # AdapterId, DetectorId, CapabilityManifest (serde)
   ├─ constants.rs           # default caps, d_model per modality, schema versions
   ├─ validators.rs          # validate_adapter_out(..) invariants
   ├─ prelude.rs             # one-stop re-exports for downstreams
   ├─ ffi.rs (feature = "ffi") # FFI type mirrors (skeleton)
   └─ traits/
      ├─ adapter.rs          # Modality, AdapterCtx, AdapterOut, Adapter trait
      └─ detector.rs         # Route, Detector trait
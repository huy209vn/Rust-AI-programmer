use schemars::JsonSchema; use serde::{Deserialize,Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PatchMeta{ pub patch_id:String, pub summary:String, pub minimality_k:u32 }

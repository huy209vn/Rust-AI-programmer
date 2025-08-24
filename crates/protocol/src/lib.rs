pub mod rpc { use schemars::JsonSchema; use serde::{Deserialize, Serialize};
  #[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
  #[serde(tag = "method", content = "params")]
  pub enum Method {
    #[serde(rename = "cargo.check")] CargoCheck{ workspace_root: String },
    #[serde(rename = "cargo.test")]  CargoTest{ workspace_root: String },
    #[serde(rename = "ra.type_of")]  RaTypeOf{ file: String, byte_offset: u32 },
    #[serde(rename = "index.update")] IndexUpdate{ changed_files: Vec<String> },
    #[serde(rename = "doc.lookup")] DocLookup{ symbol: String },
    #[serde(rename = "patch.apply")] PatchApply{ patch_id: String, diff_unified: String },
    #[serde(rename = "patch.revert")] PatchRevert{ rollback_token: String },
  }
  #[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
  pub struct ToolRequest { pub jsonrpc: &'static str, pub id: String, pub method: Method }
}

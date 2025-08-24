pub mod trace { use chrono::{DateTime,Utc}; use schemars::JsonSchema; use serde::{Serialize,Deserialize};
  #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
  pub struct Anchor{ pub source:String,pub path:String,pub start:u32,pub end:u32,pub hash:String }
  #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
  pub struct Plan{ pub plan_text:String }
  #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
  pub struct Probe{ pub tool:String, pub target:String, pub result_summary:String }
  #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
  pub struct Decide{ pub decision:String }
  #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
  pub struct PatchStep{ pub patch_id:String, pub result_code:String }
  #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
  pub struct Sketch{ pub notes:String }
  #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
  pub struct Tool{ pub name:String, #[serde(default)] pub args: serde_json::Value, #[serde(default)] pub obs: serde_json::Value }
  #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
  #[serde(tag="kind", content="data")]
  pub enum Step{ PLAN(Plan), PROBE(Probe), DECIDE(Decide), PATCH(PatchStep), SKETCH(Sketch), TOOL(Tool) }
  #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
  pub struct TraceEnvelope{ pub id:String, pub session_id:String, pub ts:DateTime<Utc>, pub step:Step, #[serde(default)] pub anchors:Vec<Anchor>, #[serde(default)] pub evidence:Vec<String>, pub conf:f32 }
}

pub mod patch { use schemars::JsonSchema; use serde::{Serialize,Deserialize};
  #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
  pub struct PatchMeta{ pub patch_id:String, pub summary:String, pub minimality_k:u32 }
  #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
  pub struct Hunk{ pub file:String, pub added:u32, pub removed:u32 }
  #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
  pub struct PatchDoc{ pub meta:PatchMeta, pub hunks:Vec<Hunk>, pub diff_unified:String }
}

pub mod memory { use chrono::{DateTime,Utc}; use schemars::JsonSchema; use serde::{Serialize,Deserialize};
  #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
  pub struct Manifest{ pub version:u32, pub repo:String, pub created_utc:DateTime<Utc>, pub entries:Vec<Entry> }
  #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
  pub struct Entry{ pub kind:String, pub path:String, pub sha256:String }
}

pub mod validator {
  use anyhow::{anyhow, Result};
  use super::trace::*;

  /// Strict validator (Phase-0 contracts): anchors required for DECIDE/PATCH.
  pub fn validate(env: &TraceEnvelope) -> Result<()> {
    validate_with_mode(env, true)
  }

  /// Mode-aware validation: when `strict=false`, anchors are not required.
  pub fn validate_with_mode(env: &TraceEnvelope, strict: bool) -> Result<()> {
    if !(0.0..=1.0).contains(&env.conf) { return Err(anyhow!("conf out of range")); }
    if strict {
        match env.step {
            Step::DECIDE(_) | Step::PATCH(_) => {
                if env.anchors.is_empty() { return Err(anyhow!("anchors required in STRICT mode")); }
            },
            _ => {}
        }
    }
    if env.id.len() < 16 { return Err(anyhow!("id too short")); }
    Ok(())
  }
}

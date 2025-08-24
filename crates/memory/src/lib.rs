use chrono::{DateTime,Utc}; use serde::{Serialize,Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest{ pub version:u32, pub repo:String, pub created_utc:DateTime<Utc> }

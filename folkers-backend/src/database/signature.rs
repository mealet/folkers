use surrealdb::sql::Thing;
use serde::{Deserialize, Serialize};

/// Database record with signature content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordSignatureRecord {
    pub id: Option<Thing>,
    pub record_id: String,
    pub base64: String,
    pub pubkey: String,
    pub signed_by: String,
}

/// Payload for signing record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignRecordPayload {
    pub private_key: String
}

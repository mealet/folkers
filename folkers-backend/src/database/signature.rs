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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignRecordPayload {
    pub private_key: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyRecordPayload {
    pub public_key: String
}

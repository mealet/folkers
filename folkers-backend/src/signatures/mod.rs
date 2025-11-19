use anyhow::anyhow;

use rand_core::OsRng;
use ed25519_dalek::{
    Signature, Signer, SigningKey, Verifier, VerifyingKey,
    SIGNATURE_LENGTH, PUBLIC_KEY_LENGTH
};

use sha2::{Sha256, Digest};
use base64::{engine::general_purpose, Engine as _};

use crate::database::person::PersonRecord;

pub struct RecordSignature {
    pub record_id: String,
    pub base64: String,
    pub pubkey: String
}

pub fn generate_signing_keypair() -> (String, String) {
    let mut csprng = OsRng;
    let signing_key: SigningKey = SigningKey::generate(&mut csprng);

    return (
        general_purpose::STANDARD.encode(signing_key.to_bytes()),
        general_purpose::STANDARD.encode(signing_key.verifying_key().to_bytes())
    );
}

pub fn sign_record(
    record: PersonRecord,
    key: String
) -> anyhow::Result<RecordSignature> {
    let key_bytes= general_purpose::STANDARD.decode(key)?;
    let key = SigningKey::from_bytes(
        &vector_to_sized::<u8, PUBLIC_KEY_LENGTH>(key_bytes)?
    );


    let mut hasher = Sha256::new();

    let record_data = serde_json::to_vec(&record)?;
    hasher.update(&record_data);

    let record_hash = hasher.finalize();
    let signature = key.sign(&record_hash);

    Ok(
        RecordSignature {
            record_id: record.id.map(|id| id.id.to_string()).unwrap_or_default(),
            base64: general_purpose::STANDARD.encode(signature.to_bytes()),
            pubkey: general_purpose::STANDARD.encode(key.verifying_key())
        }
    )
}

pub fn verify_record(
    record: PersonRecord,
    record_signature: RecordSignature,
) -> anyhow::Result<bool> {
     let mut hasher = Sha256::new();

    let record_data = serde_json::to_vec(&record)?;
    hasher.update(&record_data);

    let record_hash = hasher.finalize();   

    let signature_bytes = general_purpose::STANDARD.decode(record_signature.base64)?;
    let pubkey_bytes = general_purpose::STANDARD.decode(record_signature.pubkey)?;

    let signature = Signature::from_bytes(
        &vector_to_sized::<u8, SIGNATURE_LENGTH>(signature_bytes)?
    );

    let pubkey = VerifyingKey::from_bytes(
        &vector_to_sized::<u8, PUBLIC_KEY_LENGTH>(pubkey_bytes)?
    )?;
    
    Ok(pubkey.verify(&record_hash, &signature).is_ok())
}


fn vector_to_sized<T, const N: usize>(vector: Vec<T>) -> anyhow::Result<[T; N]> {
    Ok(
        vector.try_into().or_else(|_| {
            Err(anyhow!("Unable to get sized slice of vector"))
        })?
    )
}

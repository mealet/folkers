import type { SurrealThing } from "./surreal";

export interface RecordSignatureRecord {
  id: SurrealThing;
  record_id: string;
  base64: string;
  pubkey: string;
  signed_by: string;
}

export interface SignRecordPayload {
  private_key: string;
}

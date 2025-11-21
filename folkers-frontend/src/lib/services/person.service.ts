import { api } from "$lib/api/client";
import type { CreatePersonRecord, PersonRecord } from "$lib/types/person";
import type { RecordSignatureRecord, SignRecordPayload } from "$lib/types/signature";

export class PersonService {
	static async list_persons(): Promise<PersonRecord[]> {
		return await api.get<PersonRecord[]>("/persons");
	}

	static async get_person(id: string): Promise<PersonRecord> {
		return await api.get<PersonRecord>(`/persons/${id}`);
	}

	static async create_person(payload: CreatePersonRecord): Promise<PersonRecord> {
		return await api.post<PersonRecord>("/persons/create", payload);
	}

	static async update_person(id: string, payload: CreatePersonRecord): Promise<PersonRecord> {
		return await api.patch<PersonRecord>(`/persons/${id}`, payload);
	}

	static async delete_person(id: string): Promise<PersonRecord> {
		return await api.delete<PersonRecord>(`/persons/${id}`);
	}

	static async verify_person(id: string): Promise<RecordSignatureRecord> {
		return await api.get<RecordSignatureRecord>(`/persons/${id}/verify`);
	}

	static async sign_person(id: string, payload: SignRecordPayload): Promise<RecordSignatureRecord> {
		return await api.post<RecordSignatureRecord>(`/persons/${id}/sign`, payload);
	}
}

import { api } from '$lib/api/client';
import type { PersonRecord } from '$lib/types/person';

export class PersonService {
  static async list_persons(): Promise<PersonRecord[]> {
    return await api.get<PersonRecord[]>('/persons');
  }

  static async get_person(id: string): Promise<PersonRecord> {
    return await api.get<PersonRecord>(`/persons/${id}`);
  }
}

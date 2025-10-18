import { api } from '$lib/api/client';
import type { User } from '$lib/types/auth';

export class UserService {
  static async list_users(): Promise<User[]> {
    return await api.get('/users');
  }
}

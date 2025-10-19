import { api } from '$lib/api/client';
import type { CreateUser, User } from '$lib/types/auth';

export class UserService {
  static async list_users(): Promise<User[]> {
    return await api.get('/users');
  }

  static async create_user(payload: CreateUser): Promise<User> {
    return await api.post('/users/create', payload);
  }
}

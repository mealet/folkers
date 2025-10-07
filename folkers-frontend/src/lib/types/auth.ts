export enum UserRole {
  Watcher = 'watcher',
  Editor = 'Editor',
  Admin = 'Admin'
}

export interface User {
  id: string;
  username: string;
  role: UserRole;
}

export interface LoginCredentials {
  username: string;
  password: string;
}

export interface AuthResponse {
  user: User;
  access_token: string;
}

export interface ApiResponse<T> {
  data: T;
  message?: string;
  success: boolean;
}

export interface ApiErrorResponse {
  message: string;
  code?: string;
  status?: string;
}

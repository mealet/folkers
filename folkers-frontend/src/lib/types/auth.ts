export interface User {
  username: string;
  role: string;
}

export interface LoginCredentials {
  username: string;
  password: string;
}

export interface AuthResponse {
  token: string;
  token_type: string;
}

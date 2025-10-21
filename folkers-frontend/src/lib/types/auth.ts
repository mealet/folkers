import type { SurrealThing } from "./surreal";

export interface User {
	id: SurrealThing;
	username: string;
	password: string;
	role: string;
	created_by: string;
	creation_datetime: Date;
}

export interface LoginCredentials {
	username: string;
	password: string;
}

export interface AuthResponse {
	token: string;
	token_type: string;
}

export interface CreateUser {
	username: string;
	password: string;
	role: string;
	created_by: string;
}

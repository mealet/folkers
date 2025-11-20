import { api } from "$lib/api/client";
import { get } from "svelte/store";
import { setToken, loggedUser, initializeAuth } from "$lib/stores/auth";
import { type AuthResponse, type LoginCredentials, type User } from "$lib/types/auth";

export class AuthService {
	static async login(credentials: LoginCredentials): Promise<User> {
		const response = await api.post<AuthResponse>("/login", credentials);

		setToken(response.token);
		await initializeAuth();

		const logged_user = get(loggedUser);

		if (logged_user) {
			return logged_user;
		}

		throw new Error("Error loading logged user data");
	}

	static async signatureKeygen(): Promise<string> {
		return (
			await api.fetch("/signature-keygen", {
				method: "POST"
			})
		).json();
	}

	static async signatureReset(): Promise<void> {
		await api.fetch("/signature-reset", {
			method: "DELETE"
		});
	}
}

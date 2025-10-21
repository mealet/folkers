import { writable } from "svelte/store";
import { browser } from "$app/environment";
import type { User } from "$lib/types/auth";
import { api } from "$lib/api/client";

export interface TokenPayload {
	exp: number;
	sub: string;
	username: string;
	role: string;
}

export const accessTokenStorage = "access_token";

export const isAuthenticated = writable<boolean>(false);
export const loggedUser = writable<User | null>(null);
export const isLoading = writable<boolean>(true);

const cookieSecure = () => {
	if (!browser) return "";
	return location.protocol === "https:" ? "Secure;" : "";
};

export function isTokenExpired(token: string | null): boolean {
	if (!token) return true;

	try {
		const payload = JSON.parse(atob(token.split(".")[1]));
		const exp = payload.exp * 1000;
		return Date.now() >= exp;
	} catch {
		return true;
	}
}

export function getTokenData(token: string | null): TokenPayload | null {
	if (!token) return null;

	try {
		const payload = JSON.parse(atob(token.split(".")[1])) as TokenPayload;
		return payload;
	} catch {
		return null;
	}
}

export function setToken(token: string): void {
	if (!browser) return;

	const tokenData = getTokenData(token);

	if (!tokenData) return;

	const expires = new Date(tokenData.exp * 1000).toUTCString();
	document.cookie = `${accessTokenStorage}=${token}; Path=/; Expires=${expires}; SameSite=Lax; ${cookieSecure()}`;
}

export function getToken(): string | null {
	if (!browser) return null;

	const cookieString = document.cookie;
	const match = cookieString.match(new RegExp(`${accessTokenStorage}=([^;]+)`));
	return match ? match[1] : null;
}

export function clearAuth(): void {
	if (browser) {
		document.cookie = `${accessTokenStorage}=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT; SameSite=Lax; ${cookieSecure()}`;
	}

	isAuthenticated.set(false);
	loggedUser.set(null);
}

export function logout(): void {
	clearAuth();
	if (browser) {
		window.location.href = "/login";
	}
}

export function handleTokenExpired(): void {
	clearAuth();
	if (browser) {
		window.location.href = "/login";
	}
}

export async function initializeAuth(): Promise<void> {
	if (browser) {
		const token = getToken();

		if (token) {
			try {
				const response = await api.get<User>("/me");
				isAuthenticated.set(true);
				loggedUser.set(response);
			} catch (error) {
				console.error("Auth initialization error: ", error);
			}
		} else {
			clearAuth();
		}

		isLoading.set(false);
	}
}

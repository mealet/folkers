import { writable } from 'svelte/store';
import { browser } from '$app/environment';
import type { User } from '$lib/types/auth';

interface TokenPayload {
  exp: number;
  sub: string;
  username: string;
  role: string;
}

const accessTokenStorage = 'access_token';

export const isAuthenticated = writable<boolean>(false);
export const loggedUser = writable<User | null>(null);
export const isLoading = writable<boolean>(true);

export function isTokenExpired(token: string | null): boolean {
  if (!token) return true;

  try {
    const payload = JSON.parse(atob(token.split('.')[1]));
    const exp = payload.exp * 1000;
    return Date.now() >= exp;
  } catch {
    return true;
  }
}

export function getTokenData(token: string | null): TokenPayload | null {
  if (!token) return null;

  try {
    const payload = JSON.parse(atob(token.split('.')[1])) as TokenPayload;
    return payload;
  } catch {
    return null;
  }
}

export function setToken(token: string): void {
  localStorage.setItem(accessTokenStorage, token);
  isAuthenticated.set(true);

  const tokenData = getTokenData(token);

  if (tokenData) {
    loggedUser.set({
      username: tokenData.username,
      role: tokenData.role
    });
  }
}

export function getToken(): string | null {
  if (browser) {
    return localStorage.getItem(accessTokenStorage);
  }
  return null;
}

export function clearAuth(): void {
  localStorage.removeItem(accessTokenStorage);
  isAuthenticated.set(false);
  loggedUser.set(null);
}

export function logout(): void {
  clearAuth();
  if (browser) {
    window.location.href = '/login';
  }
}

export function handleTokenExpired(): void {
  clearAuth();
  if (browser) {
    window.location.href = '/login';
  }
}

export function initializeAuth(): void {
  if (browser) {
    const token = getToken();

    if (token) {
      isAuthenticated.set(true);

      const token_data = getTokenData(token);

      if (token_data) {
        const logged_user: User = {
          username: token_data.username,
          role: token_data.role
        };

        loggedUser.set(logged_user);
      }
    } else {
      clearAuth();
    }

    isLoading.set(false);
  }
}

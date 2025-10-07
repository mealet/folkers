import { writable } from 'svelte/store';
import { browser } from '$app/environment';
import type { User } from '$lib/types/auth';

interface TokenPayload {
  [sub: string]: unknown;
  exp: number;
  user?: User;
}

const accessTokenStorage = 'access_token';

export const isAuthenticated = writable<boolean>(false);
export const user = writable<User | null>(null);
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
  if (browser) {
    localStorage.setItem(accessTokenStorage, token);
    isAuthenticated.set(true);

    const tokenData = getTokenData(token);
    if (tokenData?.user) {
      user.set(tokenData.user);
    }
  }
}

export function getToken(): string | null {
  if (browser) {
    return localStorage.getItem(accessTokenStorage);
  }
  return null;
}

export function clearAuth(): void {
  if (browser) {
    localStorage.removeItem(accessTokenStorage);
    isAuthenticated.set(false);
    user.set(null);
  }
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
    } else {
      clearAuth();
    }

    isLoading.set(false);
  }
}

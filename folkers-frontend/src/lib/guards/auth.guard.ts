import { getToken } from '$lib/stores/auth';
import { redirect } from '@sveltejs/kit';

export function requireAuth(): void {
  const token = getToken();

  if (!token) {
    throw redirect(302, '/login');
  }
}

export function requireGuest(): void {
  const token = getToken();

  if (token) {
    throw redirect(302, '/');
  }
}

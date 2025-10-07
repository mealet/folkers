import { browser } from '$app/environment';
import { getToken } from '$lib/stores/auth';
import { redirect } from '@sveltejs/kit';

export function load({ url }) {
  if (browser) {
    const token: string | null = getToken();
    const pathname = url.pathname;

    if (token === null && pathname !== '/login') {
      throw redirect(302, '/login');
    }

    if (token !== null && pathname === '/login') {
      throw redirect(302, '/');
    }
  }

  return {};
}

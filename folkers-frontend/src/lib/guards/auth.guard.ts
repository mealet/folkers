import { getToken, type TokenPayload } from '$lib/stores/auth';
import { api } from '$lib/api/client';
import { ADMIN_ROLE } from '$lib';

export async function authGuard(): Promise<boolean> {
	try {
		const token = getToken();

		if (!token) {
			return false;
		}

		// const response = await fetch(`${API_ENDPOINT}/me`, {
		// 	method: 'GET',
		// 	headers: {
		// 		Authorization: `Bearer ${token}`
		// 	}
		// });
		//
		// if (!response.ok) {
		// 	return false;
		// }

		await api.get('/me');
	} catch {
		return false;
	}

	return true;
}

export async function adminGuardServer(token: string): Promise<boolean> {
	try {
		const payload: TokenPayload = JSON.parse(atob(token.split('.')[1]));
		return payload.role == ADMIN_ROLE;
	} catch {
		return false;
	}
}

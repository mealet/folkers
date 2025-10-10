import { getToken } from '$lib/stores/auth';
import { api } from '$lib/api/client';

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

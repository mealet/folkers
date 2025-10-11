import { getToken } from '$lib/stores/auth';

export const API_ENDPOINT =
	process.env.NODE_ENV === 'production' ? '/api' : 'http://localhost:3000';

class ApiClientError extends Error {
	constructor(
		message: string,
		public status?: number,
		public code?: string
	) {
		super(message);
		this.name = 'ApiClientError';
	}
}

class ApiClient {
	private baseUrl: string;

	constructor(baseUrl: string) {
		this.baseUrl = baseUrl;
	}

	async fetch(endpoint: string, options: RequestInit = {}): Promise<Response> {
		const token = getToken();
		const url = `${this.baseUrl}${endpoint}`;

		const config: RequestInit = {
			headers: {
				'Content-Type': 'application/json',
				...options.headers
			},
			...options
		};

		if (token) {
			(config.headers as Record<string, string>).Authorization = `Bearer ${token}`;
		}

		return await fetch(url, config);
	}

	async request<T>(endpoint: string, options: RequestInit = {}): Promise<T> {
		const token = getToken();
		const url = `${this.baseUrl}${endpoint}`;

		const config: RequestInit = {
			headers: {
				'Content-Type': 'application/json',
				...options.headers
			},
			...options
		};

		if (token) {
			(config.headers as Record<string, string>).Authorization = `Bearer ${token}`;
		}

		const response = await fetch(url, config);

		if (response.status === 401) {
			// handleTokenExpired();
			throw new ApiClientError('Authentication required', 401);
		}

		if (!response.ok) {
			const errorData = await response.json().catch(() => ({}));

			throw new ApiClientError(
				errorData.message || `HTTP Error ${response.status}`,
				response.status,
				errorData.code
			);
		}

		return (await response.json()) as T;
	}

	async upload(file: File, fieldName = 'photo'): Promise<string> {
		const token = getToken();
		const url = `${this.baseUrl}/upload`;

		const formData = new FormData();
		formData.append(fieldName, file);

		const headers: Record<string, string> = {};

		if (token) {
			headers.Authorization = `Bearer ${token}`;
		}

		const response = await fetch(url, {
			method: 'POST',
			headers: headers,
			body: formData
		});

		if (response.status === 401) {
			throw new ApiClientError('Authentication required', 401);
		}

		if (!response.ok) {
			const errorData = await response.json().catch(() => ({}));

			throw new ApiClientError(
				errorData.message || `HTTP Error ${response.status}`,
				response.status,
				errorData.code
			);
		}

		return await response.json();
	}

	async get<T>(endpoint: string, options: RequestInit = {}): Promise<T> {
		return this.request(endpoint, { ...options, method: 'GET' });
	}

	async post<T>(endpoint: string, data?: unknown, options: RequestInit = {}): Promise<T> {
		return this.request<T>(endpoint, {
			...options,
			method: 'POST',
			body: data ? JSON.stringify(data) : undefined
		});
	}

	async patch<T>(endpoint: string, data?: unknown, options: RequestInit = {}): Promise<T> {
		return this.request<T>(endpoint, {
			...options,
			method: 'PATCH',
			body: data ? JSON.stringify(data) : undefined
		});
	}

	async delete<T>(endpoint: string, options: RequestInit = {}): Promise<T> {
		return this.request<T>(endpoint, { ...options, method: 'DELETE' });
	}
}

export const api = new ApiClient(API_ENDPOINT);

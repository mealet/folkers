import { api } from "$lib/api/client";

export const SERVER_MEDIA_PREFIX = "@/";

export class MediaService {
	static async get(url: string): Promise<string> {
		if (url.startsWith(SERVER_MEDIA_PREFIX)) {
			const rawHash: string = url.slice(SERVER_MEDIA_PREFIX.length);
			const mediaResponse = await api.fetch(`/media/${rawHash}`);
			const blob = await mediaResponse.blob();

			return URL.createObjectURL(blob);
		}
		return url;
	}

	static async upload(file: File): Promise<string> {
		const hash = await api.upload(file);
		return `@/${hash.trim()}`;
	}
}

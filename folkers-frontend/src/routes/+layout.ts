import type { LayoutLoad } from "./$types";
import { authGuard } from "$lib/guards/auth.guard";
import { initializeAuth } from "$lib/stores/auth";

export const load: LayoutLoad = async ({ url }) => {
	const loginEndpoint = "/login";

	if (url.pathname === loginEndpoint) return;

	if (typeof window !== "undefined") {
		const ok = await authGuard();

		if (!ok) {
			window.location.href = "/login";
		}

		await initializeAuth();
	}
};

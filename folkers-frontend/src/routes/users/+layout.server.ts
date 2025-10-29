import type { LayoutServerLoad } from "./$types";
import { error, redirect } from "@sveltejs/kit";
import { adminGuardServer } from "$lib/guards/auth.guard";
import { accessTokenStorage } from "$lib/stores/auth";
import { ApiClientError } from "$lib/api/error";

export const load: LayoutServerLoad = async ({ cookies }) => {
	const token = cookies.get(accessTokenStorage);

	if (!token) {
		throw redirect(302, "/login");
	}

	const isAdmin = await adminGuardServer(token);

	if (!isAdmin) {
		throw error(403, {
			message: new ApiClientError("", 403).describe(),
			status: 403
		});
	}

	return {};
};

import type { LayoutServerLoad } from "./$types";
import { accessTokenStorage } from "$lib/stores/auth";
import { redirect } from "@sveltejs/kit";

export const load: LayoutServerLoad = async ({ cookies, url }) => {
	const loginEndpoint = "/login";
	const token = cookies.get(accessTokenStorage);

	if (!token && url.pathname !== loginEndpoint) {
		throw redirect(302, "/login");
	}

	return {};
};

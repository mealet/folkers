<script lang="ts">
	import type { Snippet } from "svelte";
	import { loggedUser } from "$lib/stores/auth";

	const {
		requiredUsername = null,
		requiredRoles = [],
		adminRoles = [],
		staticAdminAllowed = false,
		children
	}: {
		requiredUsername?: string | null;
		requiredRoles?: string[];
		adminRoles?: string[];
		staticAdminAllowed?: boolean;
		children: Snippet;
	} = $props();

	let user = $derived($loggedUser);
	let hasAccess = $derived.by(() => {
		if (!user) return false;

		if (staticAdminAllowed && user.created_by === "system") {
			return true;
		}

		if (requiredUsername && user.username !== requiredUsername && !adminRoles.includes(user.role)) {
			return false;
		}

		if (
			requiredRoles.length > 0 &&
			!requiredRoles.includes(user.role) &&
			!adminRoles.includes(user.role)
		) {
			return false;
		}

		return true;
	});
</script>

{#if hasAccess}
	{@render children?.()}
{/if}

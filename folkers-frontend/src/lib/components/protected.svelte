<script lang="ts">
	import { loggedUser } from "$lib/stores/auth";

	export let requiredUsername: string | null = null;
	export let requiredRoles: string[] = [];
	export let adminRoles: string[] = [];

	$: user = $loggedUser;
	$: hasAccess = (() => {
		if (!user) return false;

		// проверка по ID
		if (requiredUsername && user.username !== requiredUsername && !adminRoles.includes(user.role)) {
			return false;
		}

		// проверка по ролям
		if (
			requiredRoles.length > 0 &&
			!requiredRoles.includes(user.role) &&
			!adminRoles.includes(user.role)
		) {
			return false;
		}

		return true;
	})();
</script>

{#if hasAccess}
	<slot />
{/if}

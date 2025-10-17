<script lang="ts">
	import { loggedUser } from '$lib/stores/auth';

	export let requiredId: string | null = null;
	export let requiredRoles: string[] = [];
	export let adminRoles: string[] = [];

	$: user = $loggedUser;
	$: hasAccess = (() => {
		if (!user) return false;

		// проверка по ID
		if (requiredId && user.id.id.String !== requiredId && !adminRoles.includes(user.role)) {
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

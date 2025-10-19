<script lang="ts">
	import '../app.css';
	import favicon from '$lib/assets/favicon.svg';

	import { isAuthenticated, loggedUser, logout } from '$lib/stores/auth';
	import { invalidateAll } from '$app/navigation';

	import Protected from '$lib/components/protected.svelte';
	import { ADMIN_ROLE } from '$lib';

	let { children } = $props();

	let authenticated = $derived(isAuthenticated);
	let user = $derived(loggedUser);

	async function handleLogout(event: Event): Promise<void> {
		event.preventDefault();
		logout();

		await invalidateAll();
	}
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<div>
	{#if $authenticated && $user}
		<div class="flex gap-8">
			<a href="/users/{$user.username}" class="hover:text-blue-500">
				{$user.username} ({$user.id.id.String}) | {$user.role}
			</a>

			<a href="/">Main</a>

			<Protected requiredRoles={[ADMIN_ROLE]}>
				<a href="/users">Users</a>
			</Protected>

			<button onclick={handleLogout}> Logout </button>
		</div>
		<hr />
		<br />
	{/if}

	{@render children?.()}
</div>

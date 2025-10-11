<script lang="ts">
	import '../app.css';
	import favicon from '$lib/assets/favicon.svg';

	import { isAuthenticated, loggedUser, logout } from '$lib/stores/auth';
	import { invalidateAll } from '$app/navigation';

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
			<h2>
				{$user.username} ({$user.id.id.String}) | {$user.role}
			</h2>
			<button onclick={handleLogout}> Logout </button>
		</div>
		<hr />
		<br />
	{/if}

	{@render children?.()}
</div>

<script lang="ts">
	import { onMount } from 'svelte';
	import { requireAuth } from '$lib/guards/auth.guard';
	import { loggedUser, logout } from '$lib/stores/auth';
	import { invalidateAll } from '$app/navigation';

	$: user = $loggedUser;

	onMount(() => {
		requireAuth();
	});

	async function handleLogout(): Promise<void> {
		logout();

		await invalidateAll();
	}
</script>

<h1>Welcome back, {user?.username}</h1>
<br />
<button on:click|preventDefault={handleLogout}>Logout</button>

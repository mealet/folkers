<script lang="ts">
	import "../app.css";
	import favicon from "$lib/assets/favicon.svg";

	import { resolve } from "$app/paths";

	import { isAuthenticated, loggedUser, logout } from "$lib/stores/auth";
	import { invalidateAll } from "$app/navigation";

	import Protected from "$lib/components/protected.svelte";
	import { ADMIN_ROLE } from "$lib";

	import { Toast } from "@skeletonlabs/skeleton-svelte";
	import { toaster } from "$lib/stores/toaster";

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

<!-- Header Bar -->
{#if $authenticated && $user}
	<div>
		<div class="flex gap-8">
			<a href={resolve(`/users/${$user.username}`)} class="hover:text-blue-500">
				{$user.username} ({$user.id.id.String}) | {$user.role}
			</a>

			<a href={resolve("/")}>Main</a>

			<Protected requiredRoles={[ADMIN_ROLE]}>
				<a href={resolve("/users")}>Users</a>
			</Protected>

			<button onclick={handleLogout}> Logout </button>
		</div>
		<hr />
		<br />
	</div>
{/if}

{@render children?.()}

<!-- Skeleton UI Toaster -->
<Toast.Group {toaster}>
	{#snippet children(toast)}
		<Toast {toast}>
			<Toast.Message>
				<Toast.Title>{toast.title}</Toast.Title>
				<Toast.Description>{toast.description}</Toast.Description>
			</Toast.Message>
			<Toast.CloseTrigger />
		</Toast>
	{/snippet}
</Toast.Group>

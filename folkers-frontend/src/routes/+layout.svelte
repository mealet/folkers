<script lang="ts">
	import "../app.css";
	import favicon from "$lib/assets/favicon.svg";

	import { resolve } from "$app/paths";

	import { isAuthenticated, loggedUser, logout } from "$lib/stores/auth";
	import { invalidateAll } from "$app/navigation";

	import Protected from "$lib/components/protected.svelte";
	import { ADMIN_ROLE, EDITOR_ROLE } from "$lib";

	import { Toast } from "@skeletonlabs/skeleton-svelte";
	import { toaster } from "$lib/stores/toaster";
	import { ShieldUser, UserIcon, UserPen } from "@lucide/svelte";

	let { children } = $props();

	const authenticated = $derived(isAuthenticated);
	const user = $derived(loggedUser);

	async function handleLogout(event: Event): Promise<void> {
		event.preventDefault();
		logout();

		await invalidateAll();
	}
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
	<title>Folkers</title>
</svelte:head>

<!-- Header Bar -->
{#if $authenticated && $user}
	<header
		class="sticky top-0 z-50 flex items-center justify-between border-b-1 border-surface-700 bg-surface-950/70 px-4 py-4 backdrop-blur-md"
	>
		<div class="flex h-5 items-center space-x-4 text-lg">
			<a href={resolve("/")} class="font-bold hover:text-primary-300">Folkers</a>

			<Protected requiredRoles={[ADMIN_ROLE]}>
				<a href={resolve("/users")} class="hover:text-primary-200">Users</a>
			</Protected>

			<button onclick={handleLogout} class="hover:text-error-600">Logout</button>
		</div>
		<div class="flex items-center space-x-3">
			<a href={resolve(`/users/${$user.username}`)} class="group flex gap-3">
				<span class="group-hover:text-primary-200">{$user.username}</span>

				{#if $user.role === ADMIN_ROLE}
					<ShieldUser class="group-hover:text-primary-200" />
				{:else if $user.role === EDITOR_ROLE}
					<UserPen class="group-hover:text-primary-200" />
				{:else}
					<UserIcon class="group-hover:text-primary-200" />
				{/if}
			</a>
		</div>
	</header>
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

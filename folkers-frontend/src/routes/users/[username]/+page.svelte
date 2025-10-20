<script lang="ts">
	import { page } from '$app/state';
	import { onMount } from 'svelte';

	import { loggedUser } from '$lib/stores/auth';
	import { UserService } from '$lib/services/user.service';

	import type { User } from '$lib/types/auth';

	const userId = page.params.username;
	let user: User | null = null;

	$: allowedToEdit =
		user &&
		$loggedUser &&
		($loggedUser.username === user.created_by || $loggedUser.created_by === 'system') &&
		user.created_by !== 'system';

	onMount(async () => {
		if (userId) user = await UserService.get_user(userId);
	});

	function formatDate(dateInput: string | Date): string {
		const date = typeof dateInput === 'string' ? new Date(dateInput) : dateInput;
		const pad = (n: number) => n.toString().padStart(2, '0');

		const day = pad(date.getDate());
		const month = pad(date.getMonth() + 1);
		const year = date.getFullYear();
		const hours = pad(date.getHours());
		const minutes = pad(date.getMinutes());

		return `${day}.${month}.${year} ${hours}:${minutes}`;
	}

	async function gotoCreator() {
		if (user) {
			window.location.href = `/users/${user.created_by}`;
		}
	}

	async function deleteUser() {
		if (user && allowedToEdit) {
			try {
				await UserService.delete_user(user.username);

				window.location.href = '/users';
			} catch (error) {
				console.error('Error deleting user: ', error);
			}
		}
	}
</script>

{#if user}
	<div class="p-2">
		<h1 class="text-xl">{user.username}</h1>
		<p>Идентификатор: {user.id.id.String}</p>
		<p>Роль: <span class="underline">{user.role}</span></p>
		<p>
			Создан администратором:
			<button class="cursor-pointer text-blue-600" on:click|preventDefault={gotoCreator}>
				{user.created_by}
			</button>
		</p>
		<p>Дата создания: {user.creation_datetime ? formatDate(user.creation_datetime) : '-'}</p>

		{#if allowedToEdit}
			<br />

			<a href="/users/{userId}/edit"
				><button class="cursor-pointer border-1 border-black p-1">Редактировать</button></a
			>
			<button
				class="cursor-pointer border-1 border-black bg-red-500 p-1 text-white"
				on:click|preventDefault={deleteUser}>Удалить</button
			>
		{/if}
	</div>
{:else}
	<h1>Ничего не найдено</h1>
{/if}

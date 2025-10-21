<script lang="ts">
	import { onMount } from "svelte";
	import { page } from "$app/state";
	import { selectableRoles } from "$lib";

	import type { CreateUser, User } from "$lib/types/auth";
	import { ApiClientError } from "$lib/api/error";
	import { UserService } from "$lib/services/user.service";

	const userId = page.params.username;

	let user: User | null = null;
	let payload: CreateUser | null = null;

	let errorMessage = "";

	onMount(async () => {
		if (!userId) return;

		user = await UserService.get_user(userId);
		payload = {
			username: user.username,
			password: "",
			role: user.role,
			created_by: user.created_by
		};
	});

	async function handleSubmit() {
		if (!user || !payload) return;

		try {
			const response = await UserService.patch_user(user.username, payload);

			window.location.href = `/users/${response.username}`;
		} catch (error) {
			if (error instanceof ApiClientError) {
				errorMessage = `Ошибка: ${error.describe()}`;
			} else {
				errorMessage = `Неизвестная ошибка: ${error}`;
			}
		}
	}
</script>

{#if payload}
	<form class="p-2" on:submit={handleSubmit}>
		{#if errorMessage}
			<p class="text-red-500">{errorMessage}</p>

			<br />
		{/if}

		<p>Имя пользователя:</p>
		<input type="text" bind:value={payload.username} class="border-1 border-black p-1" required />

		<br />
		<br />

		<p>Пароль (оставьте пустым если не изменяете):</p>
		<input type="password" bind:value={payload.password} class="border-1 border-black p-1" />

		<br />
		<br />

		<p>Роль:</p>
		<select bind:value={payload.role} class="border-1 border-black p-1" required>
			{#each selectableRoles as role (role.id)}
				<option value={role.id}>{role.label}</option>
			{/each}
		</select>

		<br />
		<br />

		<button class="border-1 border-black p-1">Подтвердить</button>
	</form>
{:else}
	<p>Ничего не найдено</p>
{/if}

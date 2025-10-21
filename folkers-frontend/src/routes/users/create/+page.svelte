<script lang="ts">
	import { selectableRoles } from "$lib";
	import { ApiClientError } from "$lib/api/error";
	import { UserService } from "$lib/services/user.service";
	import type { CreateUser } from "$lib/types/auth";

	let payload: CreateUser = {
		username: "",
		password: "",
		role: "",
		created_by: "auto"
	};

	let errorMessage = "";

	async function handleSubmit() {
		try {
			const new_user = await UserService.create_user(payload);

			window.location.href = `/users/${new_user.username}`;
		} catch (error) {
			if (error instanceof ApiClientError) {
				errorMessage = `Ошибка: ${error.describe()}`;
			} else {
				errorMessage = `Неизвестная ошибка: ${error}`;
			}
		}
	}
</script>

<form class="p-2" on:submit={handleSubmit}>
	{#if errorMessage}
		<p class="text-red-500">{errorMessage}</p>

		<br />
	{/if}

	<input
		type="text"
		placeholder="Имя пользователя"
		bind:value={payload.username}
		required
		class="border-1 border-black p-1"
	/>

	<br />
	<br />

	<input
		type="password"
		placeholder="Пароль"
		bind:value={payload.password}
		required
		class="border-1 border-black p-1"
	/>

	<br />
	<br />

	<select bind:value={payload.role} class="border-1 border-black p-1" required>
		{#each selectableRoles as role (role.id)}
			<option value={role.id}>{role.label}</option>
		{/each}
	</select>

	<br />
	<br />

	<button type="submit" class="border-1 border-black p-1">Создать</button>
</form>

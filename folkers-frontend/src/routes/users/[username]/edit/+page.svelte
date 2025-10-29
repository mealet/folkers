<script lang="ts">
	import { onMount } from "svelte";
	import { page } from "$app/state";

	import { selectableRoles } from "$lib";
	import { toaster } from "$lib/stores/toaster";

	import type { CreateUser, User } from "$lib/types/auth";
	import { ApiClientError } from "$lib/api/error";
	import { UserService } from "$lib/services/user.service";

	const userId = page.params.username;

	let user = $state<User | null>(null);
	let payload = $state<CreateUser | null>(null);

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
				toaster.error({
					title: "Ошибка",
					description: error.describe()
				});
			} else {
				toaster.error({
					title: "Ошибка",
					description: error
				});
			}
		}
	}
</script>

<!-- Centering Div -->
<div class="flex items-center justify-center p-4">
	{#if payload}
		<!-- Content Div -->
		<div class="space-y-4">
			<p class="text-xl font-bold">
				Редактирование пользователя <span class="font-mono">{payload.username}</span>:
			</p>

			<!-- Form Card -->
			<div
				class="block divide-y divide-surface-200-800 overflow-hidden card border-[1px] border-surface-200-800 preset-filled-surface-100-900 shadow-xl"
			>
				<article>
					<form class="space-y-4 p-4" onsubmit={handleSubmit}>
						<!-- Username -->
						<label class="label">
							<span class="label-text">Имя пользователя</span>
							<input
								class="input"
								type="text"
								placeholder="Введите имя пользователя..."
								bind:value={payload.username}
							/>
						</label>

						<!-- Password -->
						<label class="label">
							<span class="label-text">Пароль (оставьте пустым если не меняете)</span>
							<input
								class="input"
								type="password"
								placeholder="Введите пароль..."
								bind:value={payload.password}
							/>
						</label>

						<!-- Role -->
						<label class="label">
							<span class="label-text">Роль</span>

							<select class="select" bind:value={payload.role}>
								{#each selectableRoles as role (role.id)}
									<option value={role.id}>{role.label}</option>
								{/each}
							</select>
						</label>

						<!-- Centering Div -->
						<div class="flex items-center justify-center p-2">
							<!-- Submit Button -->
							<button class="btn preset-filled-primary-500"> Изменить </button>
						</div>
					</form>
				</article>
			</div>
		</div>
	{/if}
</div>

<!-- {#if payload} -->
<!-- 	<form class="p-2" onsubmit={handleSubmit}> -->
<!-- 		{#if errorMessage} -->
<!-- 			<p class="text-red-500">{errorMessage}</p> -->
<!---->
<!-- 			<br /> -->
<!-- 		{/if} -->
<!---->
<!-- 		<p>Имя пользователя:</p> -->
<!-- 		<input type="text" bind:value={payload.username} class="border-1 border-black p-1" required /> -->
<!---->
<!-- 		<br /> -->
<!-- 		<br /> -->
<!---->
<!-- 		<p>Пароль (оставьте пустым если не изменяете):</p> -->
<!-- 		<input type="password" bind:value={payload.password} class="border-1 border-black p-1" /> -->
<!---->
<!-- 		<br /> -->
<!-- 		<br /> -->
<!---->
<!-- 		<p>Роль:</p> -->
<!-- 		<select bind:value={payload.role} class="border-1 border-black p-1" required> -->
<!-- 			{#each selectableRoles as role (role.id)} -->
<!-- 				<option value={role.id}>{role.label}</option> -->
<!-- 			{/each} -->
<!-- 		</select> -->
<!---->
<!-- 		<br /> -->
<!-- 		<br /> -->
<!---->
<!-- 		<button class="border-1 border-black p-1">Подтвердить</button> -->
<!-- 	</form> -->
<!-- {:else} -->
<!-- 	<p>Ничего не найдено</p> -->
<!-- {/if} -->

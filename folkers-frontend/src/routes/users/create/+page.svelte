<script lang="ts">
	import { toaster } from "$lib/stores/toaster";
	import { selectableRoles } from "$lib";

	import { ApiClientError } from "$lib/api/error";
	import { UserService } from "$lib/services/user.service";
	import type { CreateUser } from "$lib/types/auth";

	let payload: CreateUser = $state({
		username: "",
		password: "",
		role: "watcher",
		created_by: "auto"
	});

	async function handleSubmit() {
		try {
			const new_user = await UserService.create_user(payload);

			window.location.href = `/users/${new_user.username}`;
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
	<!-- Content Div -->
	<div class="space-y-4">
		<p class="text-xl font-bold">Создание нового пользователя:</p>

		<!-- Form Card -->
		<div
			class="block w-lg divide-y divide-surface-200-800 overflow-hidden card border-[1px] border-surface-200-800 preset-filled-surface-100-900 shadow-xl"
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
							required
						/>
					</label>

					<!-- Password -->
					<label class="label">
						<span class="label-text">Пароль</span>
						<input
							class="input"
							type="password"
							placeholder="Введите пароль..."
							bind:value={payload.password}
							required
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
					<div class="flex items-center justify-center">
						<!-- Submit Button -->
						<button class="btn preset-filled-primary-500"> Создать </button>
					</div>
				</form>
			</article>
		</div>
	</div>
</div>

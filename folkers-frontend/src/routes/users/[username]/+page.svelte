<script lang="ts">
	import { page } from "$app/state";
	import { onMount } from "svelte";
	import { resolve } from "$app/paths";
	import { toaster } from "$lib/stores/toaster";

	import { loggedUser } from "$lib/stores/auth";
	import { UserService } from "$lib/services/user.service";
	import { ApiClientError } from "$lib/api/error";

	import type { User } from "$lib/types/auth";
	import {
		CalendarIcon,
		CircleX,
		KeyIcon,
		PenIcon,
		ScanFaceIcon,
		ShieldQuestionMarkIcon,
		TrashIcon,
		UserPenIcon,
		CopyIcon,
		CheckIcon
	} from "@lucide/svelte";
	import { ADMIN_ROLE, selectableRoles } from "$lib";
	import { AuthService } from "$lib/services/auth.service";

	const userId = page.params.username;
	let user = $state<User | null>(null);

	let generatedKey = $state<string | null>(null);
	let copied = $state<boolean>(false);

	$effect(() => {
		if (copied) {
			setInterval(() => (copied = false), 1000);
		}
	});

	const allowedToEdit = $derived(
		user &&
			$loggedUser &&
			((user && $loggedUser.username === user.created_by) || $loggedUser.created_by === "system") &&
			user.created_by !== "system"
	);

	const roleLabel = $derived.by((): string | undefined => {
		return selectableRoles.find((value: { id: string; label: string }) => {
			return user && value.id === user.role;
		})?.label;
	});

	onMount(async () => {
		if (userId) user = await UserService.get_user(userId);
	});

	async function deleteUser(event: Event) {
		event.preventDefault();

		try {
			toaster.error({
				title: "Вы уверены?",
				description: "Подтвердите удаление пользователя",
				duration: 8000,
				action: {
					label: "Удалить",
					onClick: async () => {
						if (!user || !allowedToEdit) return;

						await UserService.delete_user(user.username);
						window.location.href = "/users";
					}
				}
			});
		} catch (error) {
			if (error instanceof ApiClientError) {
				toaster.error({
					title: "Ошибка на стороне API",
					description: error.describe()
				});
			} else {
				toaster.error({
					title: "Ошибка на стороне API",
					description: error
				});
			}
		}
	}

	async function generateSignKeypair(event: Event) {
		event.preventDefault();

		try {
			generatedKey = await AuthService.signatureKeygen();
		} catch (error) {
			if (error instanceof ApiClientError) {
				toaster.error({
					title: "Ошибка на стороне API",
					description: error.describe()
				});
			} else {
				toaster.error({
					title: "Ошибка на стороне API",
					description: error
				});
			}
		}
	}

	async function resetSignKeypair(event: Event) {
		event.preventDefault();

		try {
			toaster.error({
				title: "Вы уверены?",
				description: "Это сделает ранее сделанные подписи недействительными",
				duration: 8000,
				action: {
					label: "Удалить",
					onClick: async () => {
						await AuthService.signatureReset();
						location.reload();
					}
				}
			});
		} catch (error) {
			if (error instanceof ApiClientError) {
				toaster.error({
					title: "Ошибка на стороне API",
					description: error.describe()
				});
			} else {
				toaster.error({
					title: "Ошибка на стороне API",
					description: error
				});
			}
		}
	}
</script>

<!-- Centering Div -->
<div class="flex items-center justify-center p-4">
	{#if user}
		<!-- User Card -->
		<div
			class="block w-2xl divide-y divide-surface-200-800 overflow-hidden card border-[1px] border-surface-200-800 preset-filled-surface-100-900 shadow-xl"
		>
			<article class="space-y-3 p-3">
				<!-- Edit/Delete Section -->
				{#if allowedToEdit}
					<div class="float-right">
						<a href={resolve(`/users/${userId}/edit`)} class="btn-icon preset-outlined-surface-500">
							<PenIcon />
						</a>

						<button onclick={deleteUser} class="btn-icon preset-outlined-error-500">
							<TrashIcon />
						</button>
					</div>
				{/if}

				<!-- Username -->
				<h1 class="text-lg font-semibold">{user.username}</h1>

				<!-- Identifier -->
				<div class="flex items-center space-x-2 text-surface-200">
					<ScanFaceIcon size={17} />
					<p>{user.id.id.String}</p>
				</div>

				<!-- Creation Date -->
				<div class="flex items-center space-x-2 text-surface-200">
					<CalendarIcon size={17} />
					<p>{new Date(user.creation_datetime).toLocaleString("ru-RU")}</p>
				</div>

				<!-- Role -->
				<div class="flex items-center space-x-2 text-surface-200">
					<ShieldQuestionMarkIcon size={17} />
					<p>{roleLabel || user.role}</p>
				</div>

				<!-- Public Key (if exists) -->
				{#if user.public_key}
					<div class="flex items-center space-x-2 text-surface-200">
						<KeyIcon size={17} />
						<p>{user.public_key}</p>
					</div>
				{/if}

				<!-- Author -->
				<div class="flex items-center space-x-2 text-surface-200">
					<UserPenIcon size={17} />
					<button
						onclick={() => {
							if (user) {
								window.location.href = resolve(`/users/${user.created_by}`);
							}
						}}
						class="font-mono hover:text-primary-400">{user.created_by}</button
					>
				</div>

				<!-- Keygen Section -->
				{#if user.role === ADMIN_ROLE}
					<hr class="hr" />

					<div class="space-y-3">
						<h4 class="h4">Генерация защищенных ключей для подписи</h4>
						<p>
							Защищенные ключи требуются для подписи записей из базы данных со стороны
							администратора. Такая подпись будет означать что приведённой информации можно
							доверять.
							<br />
							При нажатии на кнопку "Сгенерировать" будет создана пара ключей: публичный и приватный.
							Публичный будет автоматически добавлен вам в профиль и будет виден всем, а приватный вы
							должны сохранить себе для будущего использования.
							<br />
							<br />

							<b>⚠️ ВАЖНО:</b> Ни в коем случае не делитесь приватным ключём подписи! Если это случилось
							- срочно нажимайте "Удалить" на данной странице.
						</p>

						{#if generatedKey}
							<div class="flex items-center space-x-2 text-surface-200">
								<p>
									<b>Ваш секретный ключ:</b> <span>{generatedKey}</span>
								</p>
								<button
									class="m-0 pt-1"
									onclick={() => {
										navigator.clipboard.writeText(generatedKey || "");
										copied = true;
									}}
								>
									{#if !copied}
										<CopyIcon size={14} />
									{:else}
										<CheckIcon size={14} />
									{/if}
								</button>
							</div>
						{/if}

						{#if !user.public_key}
							<button
								class="btn preset-filled-surface-500"
								onclick={generateSignKeypair}
								disabled={generatedKey !== null}>Сгенерировать ключи для подписи</button
							>
						{:else}
							<button class="btn preset-filled-error-500" onclick={resetSignKeypair}
								>Удалить ключи для подписи</button
							>
						{/if}
					</div>
				{/if}
			</article>
		</div>
	{:else}
		<span class="badge preset-filled-error-500 text-[17px]">
			<CircleX size={17} />
			No content here
		</span>
	{/if}
</div>

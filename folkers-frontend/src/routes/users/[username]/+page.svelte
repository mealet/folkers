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
		PenIcon,
		ScanFaceIcon,
		ShieldQuestionMarkIcon,
		TrashIcon,
		UserPenIcon
	} from "@lucide/svelte";
	import { selectableRoles } from "$lib";

	const userId = page.params.username;
	let user = $state<User | null>(null);

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
					<a href={resolve(`/users/${user.created_by}`)} class="hover:text-primary-400"
						>{new Date(user.creation_datetime).toLocaleString("ru-RU")}</a
					>
				</div>

				<!-- Role -->
				<div class="flex items-center space-x-2 text-surface-200">
					<ShieldQuestionMarkIcon size={17} />
					<p>{roleLabel || user.role}</p>
				</div>

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
			</article>
		</div>
	{:else}
		<span class="badge preset-filled-error-500 text-[17px]">
			<CircleX size={17} />
			No content here
		</span>
	{/if}
</div>

<!-- {#if user} -->
<!-- 	<div class="p-2"> -->
<!-- 		<h1 class="text-xl">{user.username}</h1> -->
<!-- 		<p>Идентификатор: {user.id.id.String}</p> -->
<!-- 		<p>Роль: <span class="underline">{user.role}</span></p> -->
<!-- 		<p> -->
<!-- 			Создан администратором: -->
<!-- 			<button class="cursor-pointer text-blue-600" onclick={gotoCreator}> -->
<!-- 				{user.created_by} -->
<!-- 			</button> -->
<!-- 		</p> -->
<!-- 		<p>Дата создания: {user.creation_datetime ? formatDate(user.creation_datetime) : "-"}</p> -->
<!---->
<!-- 		{#if allowedToEdit} -->
<!-- 			<br /> -->
<!---->
<!-- 			<a href={resolve(`/users/${userId}/edit`)} -->
<!-- 				><button class="cursor-pointer border-1 border-black p-1">Редактировать</button></a -->
<!-- 			> -->
<!-- 			<button -->
<!-- 				class="cursor-pointer border-1 border-black bg-red-500 p-1 text-white" -->
<!-- 				onclick={deleteUser}>Удалить</button -->
<!-- 			> -->
<!-- 		{/if} -->
<!-- 	</div> -->
<!-- {:else} -->
<!-- 	<h1>Ничего не найдено</h1> -->
<!-- {/if} -->

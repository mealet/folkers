<script lang="ts">
	import { page } from "$app/state";
	import { onMount } from "svelte";

	import { ACCEPTABLE_MEDIA_TYPES } from "$lib";

	import { PersonService } from "$lib/services/person.service";
	import { MediaService } from "$lib/services/media.service";
	import { ApiClientError } from "$lib/api/error";
	import type { PersonRecord, CreatePersonRecord } from "$lib/types/person";

	const personId = page.params.id;

	const AVATAR_WIDTH = "256px";
	const AVATAR_HEIGHT = "256px";

	let errorMessage = "";

	let person: PersonRecord | null = null;
	let payload: CreatePersonRecord | null = null;

	let avatarFile: FileList | null = null;
	let avatarImageDisplay = "";

	onMount(async () => {
		person = await PersonService.get_person(personId || "");
		payload = {
			name: person.name,
			surname: person.surname,
			patronymic: person.patronymic,

			birthday: person.birthday,
			city: person.city,
			intented_address: person.intented_address,

			summary: person.summary,
			past: person.past,
			traits_good: person.traits_good,
			traits_bad: person.traits_bad,

			avatar: person.avatar,
			media: person.media
		};

		avatarImageDisplay = await MediaService.get(person.avatar || "");
	});

	async function updateAvatarPreview() {
		if (avatarImageDisplay) URL.revokeObjectURL(avatarImageDisplay);
		if (avatarFile) avatarImageDisplay = URL.createObjectURL(avatarFile[0]);
	}

	async function handleForm() {
		if (!payload) return;

		// Uploading avatar
		if (avatarFile) {
			try {
				payload.avatar = await MediaService.upload(avatarFile[0]);
			} catch (error) {
				console.error(error);
			}
		}

		// Convert birthday to ISO string format
		payload.birthday = new Date(payload.birthday).toISOString();

		try {
			const new_person = await PersonService.update_person(person.id.id.String, payload);
			window.location.href = `/persons/${new_person.id.id.String}`;
		} catch (error) {
			if (error instanceof ApiClientError) {
				errorMessage = `Ошибка: ${error.describe()}`;
			} else {
				errorMessage = `Неизвестная ошибка: ${error}`;
			}
		}
	}
</script>

<div>
	{#if payload}
		<form on:submit|preventDefault={handleForm} class="block p-5">
			<p class="text-red-500">{errorMessage}</p>

			<br />

			<div class="flex gap-3">
				<input
					type="text"
					placeholder="Фамилия"
					bind:value={payload.surname}
					required
					class="border-1 border-black p-1"
				/>
				<input
					type="text"
					placeholder="Имя"
					bind:value={payload.name}
					required
					class="border-1 border-black p-1"
				/>
				<input
					type="text"
					placeholder="Отчество"
					bind:value={payload.patronymic}
					required
					class="border-1 border-black p-1"
				/>
			</div>

			<br />

			<div class="">
				<p>Дата рождения:</p>
				<input
					type="date"
					placeholder="Дата рождения"
					bind:value={payload.birthday}
					class="border-1 border-black p-1"
				/>

				<br />
				<br />

				<p>Город проживания</p>
				<input
					type="text"
					placeholder="Москва"
					bind:value={payload.city}
					class="border-1 border-black p-1"
				/>

				<br />
				<br />

				<p>Предполагаемый адресс проживания:</p>
				<input
					type="text"
					placeholder="ул. Пушкина, дом 1"
					bind:value={payload.intented_address}
					class="border-1 border-black p-1"
				/>

				<br />
				<br />

				<div class="flex gap-3">
					<div>
						<p>Хорошие черты:</p>
						<input
							type="text"
							placeholder="Умный, красивый и тд."
							bind:value={payload.traits_good}
							class="border-1 border-black p-1"
						/>
					</div>

					<div>
						<p>Плохие черты:</p>
						<input
							type="text"
							placeholder="Тупой, уродливый и тд."
							bind:value={payload.traits_bad}
							class="border-1 border-black p-1"
						/>
					</div>
				</div>

				<br />

				<div>
					<p>Описание:</p>
					<textarea class="border-1 border-black p-1" bind:value={payload.summary}></textarea>
				</div>

				<br />

				<div>
					<p>Прошлое:</p>
					<textarea class="border-1 border-black p-1" bind:value={payload.past}></textarea>
				</div>

				<br />

				<p>Аватар:</p>

				<img src={avatarImageDisplay} width={AVATAR_WIDTH} height={AVATAR_HEIGHT} alt="No avatar" />
				<br />

				<div class="flex gap-3">
					<div>
						<input
							type="file"
							class="border-1 border-black p-1"
							bind:files={avatarFile}
							on:change={updateAvatarPreview}
							accept={ACCEPTABLE_MEDIA_TYPES}
						/>
					</div>
				</div>

				<br />

				<div>
					<button type="submit" class="border-1 border-black p-1">Обновить</button>
				</div>
			</div>
		</form>
	{/if}
</div>

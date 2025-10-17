<script lang="ts">
	import { api } from '$lib/api/client';
	import { MediaService } from '$lib/services/media.service';
	import { PersonService } from '$lib/services/person.service';
	import type { CreatePersonRecord } from '$lib/types/person';

	const ACCEPTABLE_MEDIA_TYPES = 'image/jpeg, image/png, image/gif, image/webp';

	let avatarFile: FileList | null = null;
	let avatarURL: string = '';

	let mediaFiles: FileList | null = null;
	let mediaURLS: string[] = [''];

	let payload: CreatePersonRecord = {
		name: '',
		surname: '',
		patronymic: '',

		birthday: '',
		city: '',
		intented_address: '',

		summary: '',
		past: '',
		traits_good: '',
		traits_bad: '',

		avatar: null,
		media: []
	};

	async function handleMediaInput(index: number, event: Event) {
		const target = event.target as HTMLInputElement;
		const value = target.value;

		mediaURLS[index] = value;

		if (index === mediaURLS.length - 1 && value.trim() !== '') {
			mediaURLS = [...mediaURLS, ''];
		}
	}

	async function handleForm() {
		// Uploading avatar
		if (avatarFile) {
			try {
				payload.avatar = await MediaService.upload(avatarFile[0]);
			} catch (error) {
				console.error(error);
				payload.avatar = 'NONE';
			}
		} else {
			payload.avatar = avatarURL;
		}

		// Uploading medias
		let payloadMedia: string[] = [];

		if (mediaFiles) {
			for (const mediaFile of mediaFiles) {
				try {
					const hash = await MediaService.upload(mediaFile);
					payloadMedia.push(hash);
				} catch (error) {
					console.error('Error Uploading Media: ', error);
				}
			}
		}

		payloadMedia = [...payloadMedia, ...mediaURLS];

		payload.media = payloadMedia.filter((url) => url.trim().length > 0);

		// Convert birthday to ISO string format
		payload.birthday = new Date(payload.birthday).toISOString();

		// Sending request

		try {
			const new_person = await PersonService.create_person(payload);

			window.location.href = `/persons/${new_person.id.id.String}`;
		} catch (error) {
			console.error('API Error:', error);
		}
	}
</script>

<form on:submit|preventDefault={handleForm} class="block p-5">
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
			required
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

		<p>Аватар:</p>
		<div class="flex gap-3">
			<div>
				<input
					type="file"
					class="border-1 border-black p-1"
					bind:files={avatarFile}
					accept={ACCEPTABLE_MEDIA_TYPES}
				/>
			</div>

			<div>
				<input
					type="url"
					placeholder="Ссылка на изображение"
					class="border-1 border-black p-1"
					disabled={avatarFile !== null}
					bind:value={avatarURL}
				/>
			</div>
		</div>

		<br />

		<p>Медиа:</p>
		<div>
			<input
				type="file"
				class="border-1 border-black p-1"
				bind:files={mediaFiles}
				accept={ACCEPTABLE_MEDIA_TYPES}
				multiple
			/>

			<br />

			{#each mediaURLS as _, i (i)}
				<br />
				<input
					type="url"
					placeholder="Введите URL..."
					bind:value={mediaURLS[i]}
					on:input={(event) => handleMediaInput(i, event)}
					class="border-1 border-black p-1"
				/>
				<br />
			{/each}
		</div>
	</div>

	<br />

	<div>
		<button type="submit" class="border-1 border-black p-1">Создать</button>
	</div>
</form>

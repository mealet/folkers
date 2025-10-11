<script lang="ts">
	import { page } from '$app/state';
	import { onMount } from 'svelte';
	import { PersonService } from '$lib/services/person.service';
	import type { PersonRecord } from '$lib/types/person';
	import { api } from '$lib/api/client';

	import { compile } from 'mdsvex';

	const personId = page.params.id;

	let person: PersonRecord | null = null;
	let summaryRendered: string = '';
	let pastRendered: string = '';

	const MEDIA_PREFIX = '@/';

	let avatarURL: string = '';
	const AVATAR_WIDTH = '256px';
	const AVATAR_HEIGHT = '256px';

	let mediaURLs: string[] = [];
	const MEDIA_WIDTH = '256px';
	const MEDIA_HEIGHT = '256px';

	onMount(async () => {
		person = await PersonService.get_person(personId || '');

		const summaryRenderResult = await compile(person.summary);
		summaryRendered = summaryRenderResult?.code || person.summary;

		const pastRenderResult = await compile(person.summary);
		pastRendered = pastRenderResult?.code || person.summary;

		if (person.avatar && person.avatar.startsWith(MEDIA_PREFIX)) {
			try {
				const rawHash: string = person.avatar.slice(MEDIA_PREFIX.length);
				const avatarResponse = await api.fetch(`/media/${rawHash}`);
				const blob = await avatarResponse.blob();

				avatarURL = URL.createObjectURL(blob);
			} catch (error) {
				console.error('Error with loading avatar: ', error);
			}
		} else {
			avatarURL = person.avatar || '';
		}

		person.media.forEach(async (mediaHash: string, index: number) => {
			if (mediaHash.startsWith('@/')) {
				try {
					const rawHash: string = mediaHash.slice(MEDIA_PREFIX.length);
					const mediaResponse = await api.fetch(`/media/${rawHash}`);
					const blob = await mediaResponse.blob();

					mediaURLs[index] = URL.createObjectURL(blob);
				} catch (error) {
					console.error('Error with loading avatar: ', error);
				}
			} else {
				mediaURLs[index] = mediaHash;
			}
		});
	});
</script>

{#if person}
	<div>
		<div>
			<img src={avatarURL} alt="Avatar Error" width="{AVATAR_WIDTH}," height={AVATAR_HEIGHT} />
		</div>

		<br />
		<hr />

		<h1>{person.surname} {person.name} {person.patronymic}</h1>
		<h3>- Дата рождения: {new Date(person.birthday).toLocaleDateString('ru-RU')}</h3>
		<h3>- Город: {person.city}</h3>
		<h3>- Предполагаемый адрес: {person.intented_address}</h3>

		<h3>- Описание:</h3>
		{@html summaryRendered}

		<h3>- Прошлое:</h3>
		{@html pastRendered}

		<h3>- Хорошие черты: {person.traits_good}</h3>
		<h3>- Плохие черты: {person.traits_bad}</h3>

		<h3>- Автор записи: {person.author.id.String}</h3>

		<hr />
		<br />

		<div class="inline-flex gap-8 p-5">
			{#each mediaURLs as mediaURL, index (index)}
				<img src={mediaURL} width="{MEDIA_WIDTH}," height={MEDIA_HEIGHT} alt="Media Error" />
			{/each}
		</div>
	</div>
{:else}
	<p>No Content</p>
{/if}

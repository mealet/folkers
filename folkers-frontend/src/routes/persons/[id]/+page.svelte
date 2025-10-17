<script lang="ts">
	import { page } from '$app/state';
	import { onMount } from 'svelte';
	import { PersonService } from '$lib/services/person.service';
	import type { PersonRecord } from '$lib/types/person';
	import { api } from '$lib/api/client';

	import { compile } from 'mdsvex';
	import { MediaService } from '$lib/services/media.service';

	const personId = page.params.id;

	let person: PersonRecord | null = null;
	let summaryRendered: string = '';
	let pastRendered: string = '';

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

		if (person.avatar) {
			try {
				avatarURL = await MediaService.get(person.avatar);
			} catch (error) {
				console.error('Avatar fetch error: ', error);
			}
		}

		person.media.forEach(async (mediaHash: string, index: number) => {
			try {
				mediaURLs[index] = await MediaService.get(mediaHash);
			} catch (error) {
				console.error('Media fetch error: ', error);
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

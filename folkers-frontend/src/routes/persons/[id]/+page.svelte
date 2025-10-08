<script lang="ts">
	import { page } from '$app/state';
	import { onMount } from 'svelte';
	import { requireAuth } from '$lib/guards/auth.guard';
	import { PersonService } from '$lib/services/person.service';
	import type { PersonRecord } from '$lib/types/person';

	import { compile } from 'mdsvex';

	const personId = page.params.id;

	let person: PersonRecord | null = null;
	let summaryRendered: string = '';
	let pastRendered: string = '';

	onMount(async () => {
		requireAuth();

		person = await PersonService.get_person(personId || '');

		const summaryRenderResult = await compile(person.summary);
		summaryRendered = summaryRenderResult?.code || person.summary;

		const pastRenderResult = await compile(person.summary);
		pastRendered = pastRenderResult?.code || person.summary;
	});
</script>

{#if person}
	<div>
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
	</div>
{:else}
	<p>No Content</p>
{/if}

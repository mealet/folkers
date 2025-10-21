<script lang="ts">
	import { onMount } from "svelte";
	import { resolve } from "$app/paths";

	import { PersonService } from "$lib/services/person.service";
	import type { PersonRecord } from "$lib/types/person";

	import Protected from "$lib/components/protected.svelte";
	import { ADMIN_ROLE, EDITOR_ROLE } from "$lib";

	import Fuse from "fuse.js";

	let persons: PersonRecord[] = [];

	let query = "";
	let results: PersonRecord[] = [];
	let fuse = Fuse<PersonRecord>;

	onMount(async () => {
		persons = await PersonService.list_persons();

		fuse = new Fuse(persons, {
			keys: ["surname", "name", "patronymic"],
			threshold: 0.3
		});
	});

	$: if (fuse && query.trim()) {
		results = fuse
			.search(query)
			.map((result: { item: PersonRecord; refIndex: number }) => result.item);
	} else {
		results = persons;
	}
</script>

<div class="p-2">
	<Protected requiredRoles={[EDITOR_ROLE]} adminRoles={[ADMIN_ROLE]}>
		<a href={resolve("/persons/create")}
			><button class="border-1 border-black p-1">Создать</button></a
		>

		<br />
		<br />
	</Protected>

	<input type="text" bind:value={query} placeholder="Поиск..." class="rounded border p-2" />

	<br />
	<br />

	<h3>Список людей:</h3>
	<ul>
		{#each results as person (person.id)}
			<li>
				- <a href={resolve(`/persons/${person.id.id.String}`)}
					>{person.surname} {person.name} {person.patronymic}</a
				>
			</li>
		{/each}
	</ul>
</div>

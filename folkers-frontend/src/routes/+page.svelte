<script lang="ts">
	import { onMount } from 'svelte';
	import { authGuard } from '$lib/guards/auth.guard';
	import { PersonService } from '$lib/services/person.service';
	import type { PersonRecord } from '$lib/types/person';
	// import { loggedUser } from '$lib/stores/auth';

	let persons: PersonRecord[] = [];

	onMount(async () => {
		persons = await PersonService.list_persons();
	});

	// let user = $loggedUser;
</script>

<div>
	<h3>List of persons:</h3>
	<ul>
		{#each persons as person (person.id)}
			<li>
				- <a href="/persons/{person.id.id.String}"
					>{person.surname} {person.name} {person.patronymic}</a
				>
			</li>
		{/each}
	</ul>
</div>

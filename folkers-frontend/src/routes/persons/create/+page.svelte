<script lang="ts">
	import type { CreatePersonRecord } from '$lib/types/person';

	let payload: CreatePersonRecord = {
		name: '',
		surname: '',
		patronymic: '',

		birthday: new Date(),
		city: '',
		intented_address: '',

		summary: '',
		past: '',
		traits_good: '',
		traits_bad: '',

		avatar: null,
		media: []
	};

	async function handleForm() {
		console.log(payload);
	}
</script>

<form on:submit|preventDefault={handleForm} class="block p-5">
	<div class="flex">
		<input type="text" placeholder="Фамилия" bind:value={payload.surname} />
		<input type="text" placeholder="Имя" bind:value={payload.name} />
		<input type="text" placeholder="Отчество" bind:value={payload.patronymic} />
	</div>

	<br />

	<div>
		<button type="submit">Создать</button>
	</div>
</form>

<script lang="ts">
	import { page } from '$app/state';
	import { onMount } from 'svelte';
	import { PersonService } from '$lib/services/person.service';
	import type { PersonRecord, CreatePersonRecord } from '$lib/types/person';

	const personId = page.params.id;

	let person: PersonRecord | null = null;
	let payload: CreatePersonRecord | null = null;

	let errorMessage = '';

	onMount(async () => {
		person = await PersonService.get_person(personId || '');
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
	});

	async function handleForm() {}
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

				<div>
					<button type="submit" class="border-1 border-black p-1">Создать</button>
				</div>
			</div>
		</form>
	{/if}
</div>

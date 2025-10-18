<script lang="ts">
	import { page } from '$app/state';
	import { onMount } from 'svelte';

	import { api } from '$lib/api/client';
	import type { User } from '$lib/types/auth';

	const userId = page.params.username;
	let user: User | null = null;

	onMount(async () => {
		user = await api.get(`/users/${userId}`);
	});

	async function gotoCreator() {
		if (user) {
			window.location.href = `/users/${user.created_by}`;
		}
	}

	function formatDate(dateInput: string | Date): string {
		const date = typeof dateInput === 'string' ? new Date(dateInput) : dateInput;
		const pad = (n: number) => n.toString().padStart(2, '0');

		const day = pad(date.getDate());
		const month = pad(date.getMonth() + 1);
		const year = date.getFullYear();
		const hours = pad(date.getHours());
		const minutes = pad(date.getMinutes());

		return `${day}.${month}.${year} ${hours}:${minutes}`;
	}
</script>

{#if user}
	<div class="p-2">
		<h1 class="text-xl">{user.username}</h1>
		<p>Идентификатор: {user.id.id.String}</p>
		<p>Роль: <span class="underline">{user.role}</span></p>
		<p>
			Создан администратором:
			<button class="cursor-pointer text-blue-600" on:click|preventDefault={gotoCreator}>
				{user.created_by}
			</button>
		</p>
		<p>Дата создания: {user.creation_datetime ? formatDate(user.creation_datetime) : '-'}</p>
	</div>
{:else}
	<h1>Ничего не найдено</h1>
{/if}

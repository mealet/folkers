<script lang="ts">
	import { ADMIN_ROLE, rolesOrder } from '$lib';
	import { UserService } from '$lib/services/user.service';
	import type { User } from '$lib/types/auth';

	import { onMount } from 'svelte';
	import Fuse from 'fuse.js';
	import Protected from '$lib/components/protected.svelte';

	let users: User[] = [];

	let query = '';
	let results: User[] = [];
	let fuse = Fuse<User>;

	onMount(async () => {
		users = await UserService.list_users();

		users = users.sort((a: User, b: User) => {
			// if (rolesOrder[a.role] > rolesOrder[b.role]) {
			// 	return -1;
			// }
			//
			// if (rolesOrder[b.role] > rolesOrder[a.role]) {
			// 	return 1;
			// }
			//
			// return 0;

			return rolesOrder[b.role] - rolesOrder[a.role];
		});

		fuse = new Fuse(users, {
			keys: ['username'],
			threshold: 0.3
		});
	});

	$: if (fuse && query.trim()) {
		results = fuse
			.search(query)
			.map((result: { item: PersonRecord; refIndex: number }) => result.item);
	} else {
		results = users;
	}
</script>

<div class="p-2">
	<Protected requiredRoles={[ADMIN_ROLE]}>
		<a href="/users/create"><button class="border-1 border-black p-1">Создать</button></a>
	</Protected>

	<br />
	<br />

	<input type="text" bind:value={query} placeholder="Поиск..." class="rounded border p-2" />

	<br />
	<br />

	<ul>
		{#each results as user (user.id.id.String)}
			<li>
				- <a href="/users/{user.username}">{user.username} ({user.role})</a>
			</li>
		{/each}
	</ul>
</div>

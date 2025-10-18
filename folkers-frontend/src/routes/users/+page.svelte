<script lang="ts">
	import { rolesOrder } from '$lib';
	import { UserService } from '$lib/services/user.service';
	import type { User } from '$lib/types/auth';

	import { onMount } from 'svelte';

	let users: User[] = [];

	onMount(async () => {
		users = await UserService.list_users();

		users = users.sort((a: User, b: User) => {
			return rolesOrder[a.role] - rolesOrder[b.role];
		});
	});
</script>

<div class="p-2">
	<ul>
		{#each users as user (user.id.id.String)}
			<li>
				- <a href="/users/{user.id.id.String}">{user.username} ({user.role})</a>
			</li>
		{/each}
	</ul>
</div>

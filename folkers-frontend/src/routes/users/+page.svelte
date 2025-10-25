<script lang="ts">
	import { onMount } from "svelte";
	import { resolve } from "$app/paths";
	import { goto } from "$app/navigation";

	import { ADMIN_ROLE, EDITOR_ROLE, rolesOrder } from "$lib";
	import { UserService } from "$lib/services/user.service";
	import type { User } from "$lib/types/auth";

	import Fuse from "fuse.js";
	import Protected from "$lib/components/protected.svelte";

	import { Pagination } from "@skeletonlabs/skeleton-svelte";
	import { ArrowLeftIcon, ArrowRightIcon, Plus, SearchIcon } from "@lucide/svelte";

	const PAGE_SIZE = 15;

	let users: User[] = $state([]);
	let page = $state(1);

	let query = $state("");
	let fuse = $state<Fuse<User> | null>(null);

	const filteredResults = $derived.by((): User[] => {
		if (fuse && query.trim()) {
			return fuse.search(query).map((result: { item: User; refIndex: number }) => result.item);
		} else {
			return users;
		}
	});

	const start = $derived((page - 1) * PAGE_SIZE);
	const end = $derived(start + PAGE_SIZE);
	const data = $derived(filteredResults.slice(start, end));

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
			keys: ["username"],
			threshold: 0.3
		});
	});
</script>

<!-- Centering Div -->
<div class="flex items-center justify-center p-4">
	<!-- Content Div -->
	<div class="w-2xl space-y-4">
		<div>
			<p class="text-xl font-bold">Список пользователей:</p>
			<p class="text-lg">Всего: {users.length}</p>
		</div>

		<!-- Interaction Line -->
		<div class="grid w-full grid-cols-[1fr_auto] gap-2">
			<!-- Search Input -->
			<div class="input-group grid-cols-[auto_1fr_auto]">
				<div class="ig-cell preset-tonal">
					<SearchIcon size={16} />
				</div>
				<input
					class="ig-input"
					type="search"
					placeholder="Введите имя пользователя..."
					bind:value={query}
				/>
			</div>

			<!-- Create Button -->
			<Protected requiredRoles={[EDITOR_ROLE]} adminRoles={[ADMIN_ROLE]}>
				<a href={resolve("/users/create")} class="btn-icon preset-filled"><Plus size={18} /></a>
			</Protected>
		</div>

		<!-- Card -->
		{#if users.length > 0}
			<div
				class="block w-2xl divide-y divide-surface-200-800 overflow-hidden card border-[1px] border-surface-200-800 preset-filled-surface-100-900 shadow-xl"
			>
				<article class="space-y-3 p-3">
					<!-- Users Table -->
					<div class="table-wrap rounded-md">
						<table class="table caption-bottom">
							<thead>
								<tr>
									<th>Имя пользователя</th>
									<th>Роль</th>
									<th>Создатель</th>
									<th>Дата создания</th>
								</tr>
							</thead>
							<tbody class="[&>tr]:hover:preset-tonal-secondary">
								{#each data as row (row.username)}
									<tr
										onclick={() => goto(resolve(`/users/${row.username}`))}
										class="cursor-pointer"
									>
										<td>{row.username}</td>
										<td>{row.role}</td>
										<td>{row.created_by}</td>
										<td>{new Date(row.creation_datetime).toLocaleString("ru-RU")}</td>
									</tr>
								{/each}
							</tbody>
						</table>
					</div>
				</article>
			</div>

			<!-- Centering Div -->
			<div class="flex items-center justify-center">
				<!-- Pagination -->
				<Pagination
					count={users.length}
					pageSize={PAGE_SIZE}
					{page}
					onPageChange={(event) => (page = event.page)}
				>
					<Pagination.PrevTrigger>
						<ArrowLeftIcon class="size-4" />
					</Pagination.PrevTrigger>

					<Pagination.Context>
						{#snippet children(pagination)}
							{#each pagination().pages as page, index (page)}
								{#if page.type === "page"}
									<Pagination.Item {...page}>
										{page.value}
									</Pagination.Item>
								{:else}
									<Pagination.Ellipsis {index}>&#8230</Pagination.Ellipsis>
								{/if}
							{/each}
						{/snippet}
					</Pagination.Context>

					<Pagination.NextTrigger>
						<ArrowRightIcon class="size-4" />
					</Pagination.NextTrigger>
				</Pagination>
			</div>
		{/if}
	</div>
</div>

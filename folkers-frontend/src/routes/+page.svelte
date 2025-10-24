<script lang="ts">
	import { onMount } from "svelte";
	import { resolve } from "$app/paths";
	import { compile } from "mdsvex";

	import { PersonService } from "$lib/services/person.service";
	import { MediaService } from "$lib/services/media.service";
	import type { PersonRecord } from "$lib/types/person";

	import Protected from "$lib/components/protected.svelte";
	import { ADMIN_ROLE, EDITOR_ROLE } from "$lib";

	import Fuse from "fuse.js";
	import { Pagination } from "@skeletonlabs/skeleton-svelte";
	import { Building2, Calendar, SearchIcon, ArrowLeftIcon, ArrowRightIcon } from "@lucide/svelte";

	const SUMMARY_PREVIEW_LENGTH = 250;
	const PAGE_SIZE = 5;

	let persons = $state<PersonRecord[]>([]);
	let page = $state(1);

	let query = $state("");
	let fuse = $state<Fuse<PersonRecord> | null>(null);

	const filteredResults = $derived.by((): PersonRecord[] => {
		if (fuse && query.trim()) {
			return fuse
				.search(query)
				.map((result: { item: PersonRecord; refIndex: number }) => result.item);
		} else {
			return persons;
		}
	});

	const start = $derived((page - 1) * PAGE_SIZE);
	const end = $derived(start + PAGE_SIZE);
	const data = $derived(filteredResults.slice(start, end));

	function truncatePreview(source: string, maxLength: number): string {
		if (source.length > maxLength) {
			return source.slice(0, maxLength - 3).trim() + "...";
		}
		return source;
	}

	onMount(async () => {
		const fetchedPersons = await PersonService.list_persons();

		const updatedPersons = await Promise.all(
			fetchedPersons.map(async (person) => {
				const summaryPreview = truncatePreview(person.summary, SUMMARY_PREVIEW_LENGTH);
				const renderResult = await compile(summaryPreview);

				return {
					...person,
					summary: renderResult?.code || person.summary,
					avatar: await MediaService.get(person.avatar || "")
				} as PersonRecord;
			})
		);

		persons = updatedPersons;

		fuse = new Fuse(persons, {
			keys: ["surname", "name", "patronymic"],
			threshold: 0.3
		});
	});
</script>

<!-- Centering Div -->
<div class="flex items-center justify-center p-4">
	<!-- Content Div -->
	<div class="w-2xl space-y-4">
		<p class="text-xl font-bold">Список людей:</p>

		<!-- Search Input -->
		<div class="input-group grid-cols-[auto_1fr_auto]">
			<div class="ig-cell preset-tonal">
				<SearchIcon size={16} />
			</div>
			<input class="ig-input" type="search" placeholder="Фамилия Имя Отчество" bind:value={query} />
		</div>

		<!-- Persons Cards -->
		{#each data as person (person.id)}
			<a
				class="dividy-y block divide-surface-200-800 overflow-hidden card border-[1px] border-surface-200-800 preset-filled-surface-100-900 drop-shadow-lg"
				href={resolve(`/persons/${person.id.id.String}`)}
			>
				<header>
					<img src={person.avatar} alt="" />
				</header>

				<article class="space-y-2 p-3">
					<!-- Header -->
					<h1 class="text-lg font-semibold">{person.surname} {person.name} {person.patronymic}</h1>

					<!-- Birthday -->
					<div class="flex items-center space-x-1 text-surface-200">
						<Calendar size={17} />
						<span class="text-[18px]">{new Date(person.birthday).toLocaleDateString("ru-RU")}</span>
					</div>

					<!-- City -->
					<div class="flex items-center space-x-1 text-surface-200">
						<Building2 size={17} />
						<span class="text-[18px]">{person.city}</span>
					</div>

					<!-- Summary Preview -->
					<div>
						<p>
							<!-- eslint-disable-next-line svelte/no-at-html-tags -->
							{@html person.summary}
						</p>
					</div>
				</article>
			</a>
		{/each}

		<!-- Pagination -->
		<Pagination
			count={persons.length}
			pageSize={PAGE_SIZE}
			{page}
			onPageChange={(event) => (page = event.page)}
			class="flex w-full justify-between"
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
</div>

<!-- <div class="p-2"> -->
<!-- 	<Protected requiredRoles={[EDITOR_ROLE]} adminRoles={[ADMIN_ROLE]}> -->
<!-- 		<a href={resolve("/persons/create")} -->
<!-- 			><button class="border-1 border-black p-1">Создать</button></a -->
<!-- 		> -->
<!---->
<!-- 		<br /> -->
<!-- 		<br /> -->
<!-- 	</Protected> -->
<!---->
<!-- 	<input type="text" bind:value={query} placeholder="Поиск..." class="rounded border p-2" /> -->
<!---->
<!-- 	<br /> -->
<!-- 	<br /> -->
<!---->
<!-- 	<h3>Список людей:</h3> -->
<!-- 	<ul> -->
<!-- 		{#each results as person (person.id)} -->
<!-- 			<li> -->
<!-- 				- <a href={resolve(`/persons/${person.id.id.String}`)} -->
<!-- 					>{person.surname} {person.name} {person.patronymic}</a -->
<!-- 				> -->
<!-- 			</li> -->
<!-- 		{/each} -->
<!-- 	</ul> -->
<!-- </div> -->

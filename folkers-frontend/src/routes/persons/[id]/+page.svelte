<script lang="ts">
	import { page } from "$app/state";
	import { resolve } from "$app/paths";
	import { onMount } from "svelte";

	import { PersonService } from "$lib/services/person.service";
	import type { PersonRecord } from "$lib/types/person";

	import { compile } from "mdsvex";
	import { MediaService } from "$lib/services/media.service";

	import Protected from "$lib/components/protected.svelte";
	import Maybenot from "$lib/components/maybenot.svelte";

	import { ADMIN_ROLE, EDITOR_ROLE } from "$lib";
	import {
		Building2,
		Calendar,
		CircleX,
		FrownIcon,
		LaughIcon,
		MapPinHouse,
		UserPenIcon
	} from "@lucide/svelte";

	import { Dialog, Portal } from "@skeletonlabs/skeleton-svelte";

	const MEDIA_WIDTH = "256px";
	const MEDIA_HEIGHT = "256px";

	const personId = page.params.id;

	let person: PersonRecord | null = null;
	let summaryRendered: string = "";
	let pastRendered: string = "";

	let avatarURL: string = "";
	let mediaURLs: string[] = [];

	onMount(async () => {
		person = await PersonService.get_person(personId || "");

		const summaryRenderResult = await compile(person.summary);
		summaryRendered = summaryRenderResult?.code || person.summary;

		const pastRenderResult = await compile(person.past);
		pastRendered = pastRenderResult?.code || person.past;

		if (person.avatar) {
			try {
				avatarURL = await MediaService.get(person.avatar);
			} catch (error) {
				console.error("Avatar fetch error: ", error);
			}
		}

		person.media.forEach(async (mediaHash: string, index: number) => {
			try {
				mediaURLs[index] = await MediaService.get(mediaHash);
			} catch (error) {
				console.error("Media fetch error: ", error);
			}
		});
	});

	async function handleDelete() {
		if (person) {
			try {
				await PersonService.delete_person(person.id.id.String);
				window.location.href = "/";
			} catch (error) {
				console.error(error);
			}
		}
	}
</script>

<!-- Centering Div -->
<div class="flex items-center justify-center p-4">
	{#if person}
		<!-- Person Card -->
		<div
			class="block w-2xl divide-y divide-surface-200-800 overflow-hidden card border-[1px] border-surface-200-800 preset-filled-surface-100-900 shadow-xl"
		>
			<header>
				<img
					src={avatarURL || "/no_avatar.png"}
					alt="Аватар {person.name}"
					class="aspect-video h-auto w-full cursor-pointer object-cover"
				/>
			</header>

			<article class="space-y-3 p-3">
				<!-- Title -->
				<h1 class="text-lg font-semibold">{person.surname} {person.name} {person.patronymic}</h1>

				<!-- Author -->
				<div class="flex items-center space-x-2 text-surface-200">
					<UserPenIcon size={17} />
					<a href={resolve(`/users/${person.author}`)} class="hover:text-primary-400"
						>{person.author}</a
					>
				</div>

				<!-- Birthday -->
				<div class="flex items-center space-x-2 text-surface-200">
					<Calendar size={17} />
					<span class="text-[18px]">
						<Maybenot prop={person.birthday}>
							{new Date(person.birthday).toLocaleDateString("ru-RU")}
						</Maybenot>
					</span>
				</div>

				<!-- City -->
				<div class="flex items-center space-x-2 text-surface-200">
					<Building2 size={17} />
					<span class="text-[18px]">
						<Maybenot prop={person.city}>
							{person.city}
						</Maybenot>
					</span>
				</div>

				<!-- Intented Address -->
				<div class="flex items-center space-x-2 text-surface-200">
					<MapPinHouse size={17} />
					<span class="text-[18px]">
						<Maybenot prop={person.intented_address}>
							{person.intented_address}
						</Maybenot>
					</span>
				</div>

				<hr class="hr" />

				<!-- Summary -->
				<div>
					<h6 class="h6">Описание:</h6>
					<Maybenot prop={summaryRendered}>
						<!-- eslint-disable-next-line svelte/no-at-html-tags -->
						{@html summaryRendered}
					</Maybenot>
				</div>

				<!-- Past -->
				<div>
					<h6 class="h6">Прошлое:</h6>
					<Maybenot prop={pastRendered}>
						<!-- eslint-disable-next-line svelte/no-at-html-tags -->
						{@html pastRendered}
					</Maybenot>
				</div>

				<!-- Traits -->
				<h6 class="h6">Черты:</h6>
				<Maybenot prop={person.traits_good || person.traits_bad}>
					<div class="flex gap-3">
						<!-- Good Traits -->
						{#if person.traits_good}
							<div class="w-full items-center justify-center">
								<div class="badge w-full preset-outlined-success-500">
									<LaughIcon size={18} class="text-success-500" />
								</div>
								<p class="text-center text-success-400">
									{person.traits_good}
								</p>
							</div>
						{/if}

						<!-- Bad Traits -->
						{#if person.traits_bad}
							<div class="w-full items-center justify-center">
								<div class="badge w-full preset-outlined-error-500">
									<FrownIcon size={18} class="text-error-500" />
								</div>
								<p class="text-center text-error-400">
									{person.traits_bad}
								</p>
							</div>
						{/if}
					</div>
				</Maybenot>

				<!-- Media -->
				{#if mediaURLs.length > 0}
					<section class="space-y-1">
						<h6 class="h6">Медиа:</h6>
						<div
							class="flex w-full gap-2 overflow-x-scroll overflow-y-hidden rounded-md border-1 border-surface-200-800 p-2"
						>
							{#each mediaURLs as mediaURL, index (index)}
								<!-- Skeleton UI Dialog -->
								<Dialog>
									<Dialog.Trigger>
										<img
											src={mediaURL}
											width={MEDIA_WIDTH}
											height={MEDIA_HEIGHT}
											alt=""
											class="cursor-pointer rounded-md hover:opacity-70"
										/>
									</Dialog.Trigger>

									<Portal>
										<Dialog.Backdrop class="fixed inset-0 z-50 bg-surface-50-950/50" />
										<Dialog.Positioner class="fixed inset-0 z-50 flex items-center justify-center">
											<Dialog.Content class="w-5xl space-y-2 card bg-surface-100-900 p-4 shadow-xl">
												<!-- <Dialog.CloseTrigger class="group float-right"> -->
												<!-- 	<XIcon size={20} class="group-hover:text-surface-200" /> -->
												<!-- </Dialog.CloseTrigger> -->

												<img
													src={mediaURL}
													alt=""
													class="aspect-video h-auto w-full cursor-pointer rounded-md object-cover"
												/>
											</Dialog.Content>
										</Dialog.Positioner>
									</Portal>
								</Dialog>
							{/each}
						</div>
					</section>
				{/if}
			</article>
		</div>
	{:else}
		<span class="badge preset-filled-error-500 text-[17px]">
			<CircleX size={17} />
			No content here
		</span>
	{/if}
</div>

<!-- {#if person} -->
<!-- 	<div> -->
<!-- 		<Protected -->
<!-- 			requiredRoles={[EDITOR_ROLE]} -->
<!-- 			requiredUsername={person.author} -->
<!-- 			adminRoles={[ADMIN_ROLE]} -->
<!-- 		> -->
<!-- 			<a href={resolve(`/persons/${personId}/edit`)} -->
<!-- 				><button type="submit" class="border-1 border-black p-1">Редактировать</button></a -->
<!-- 			> -->
<!-- 			<button -->
<!-- 				class="border-1 border-black bg-red-500 p-1 text-white" -->
<!-- 				on:click|preventDefault={handleDelete}>Удалить</button -->
<!-- 			> -->
<!---->
<!-- 			<br /> -->
<!-- 			<br /> -->
<!-- 		</Protected> -->
<!---->
<!-- 		<div> -->
<!-- 			<img src={avatarURL} alt="Avatar Error" width="{AVATAR_WIDTH}," height={AVATAR_HEIGHT} /> -->
<!-- 		</div> -->
<!---->
<!-- 		<br /> -->
<!-- 		<hr /> -->
<!---->
<!-- 		<h1>{person.surname} {person.name} {person.patronymic}</h1> -->
<!-- 		<h3>- Дата рождения: {new Date(person.birthday).toLocaleDateString("ru-RU")}</h3> -->
<!-- 		<h3>- Город: {person.city}</h3> -->
<!-- 		<h3>- Предполагаемый адрес: {person.intented_address}</h3> -->
<!---->
<!-- 		<h3>- Описание:</h3> -->
<!-- 		<!-- eslint-disable-next-line svelte/no-at-html-tags -->
<!-- 		{@html summaryRendered} -->
<!---->
<!-- 		<h3>- Прошлое:</h3> -->
<!-- 		<!-- eslint-disable-next-line svelte/no-at-html-tags -->
<!-- 		{@html pastRendered} -->
<!---->
<!-- 		<h3>- Хорошие черты: {person.traits_good}</h3> -->
<!-- 		<h3>- Плохие черты: {person.traits_bad}</h3> -->
<!---->
<!-- 		<h3> -->
<!-- 			- Автор записи: <a href={resolve(`/users/${person.author}`)} class="text-blue-500" -->
<!-- 				>{person.author}</a -->
<!-- 			> -->
<!-- 		</h3> -->
<!---->
<!-- 		<hr /> -->
<!-- 		<br /> -->
<!---->
<!-- 		<div class="flex gap-4"> -->
<!-- 			{#each mediaURLs as mediaURL, index (index)} -->
<!-- 				<img src={mediaURL} width="{MEDIA_WIDTH}," height={MEDIA_HEIGHT} alt="Media Error" /> -->
<!-- 			{/each} -->
<!-- 		</div> -->
<!-- 	</div> -->
<!-- {:else} -->
<!-- 	<p>No Content</p> -->
<!-- {/if} -->

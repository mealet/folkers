<script lang="ts">
	import { page } from "$app/state";
	import { resolve } from "$app/paths";
	import { onMount } from "svelte";
	import { toaster } from "$lib/stores/toaster";

	import { PersonService } from "$lib/services/person.service";
	import type { PersonRecord } from "$lib/types/person";
	import type { RecordSignatureRecord, SignRecordPayload } from "$lib/types/signature";

	import { MediaService } from "$lib/services/media.service";

	import Protected from "$lib/components/protected.svelte";
	import Maybenot from "$lib/components/maybenot.svelte";

	import { ADMIN_ROLE, EDITOR_ROLE, renderMarkdown } from "$lib";
	import {
		Building2,
		Calendar,
		CircleAlertIcon,
		CircleX,
		FrownIcon,
		LaughIcon,
		MapPinHouse,
		PenIcon,
		SignatureIcon,
		TrashIcon,
		UserPenIcon
	} from "@lucide/svelte";

	import { Dialog, Portal } from "@skeletonlabs/skeleton-svelte";
	import { ApiClientError } from "$lib/api/error";
	import { loggedUser } from "$lib/stores/auth";

	const MEDIA_WIDTH = "256px";
	const MEDIA_HEIGHT = "256px";

	const personId = page.params.id;

	let person: PersonRecord | null = $state(null);

	let signature: RecordSignatureRecord | null = $state(null);
	let signatureAvailable: boolean = $state(false);
	let signPayload: SignRecordPayload = $state({
		private_key: ""
	});

	let summaryRendered: string = $state("");
	let pastRendered: string = $state("");

	let avatarURL: string = $state("");
	let mediaURLs: string[] = $state([]);

	onMount(async () => {
		person = await PersonService.get_person(personId || "");

		try {
			signature = await PersonService.verify_person(personId || "");
			signatureAvailable = true;
		} catch (error) {
			if (error instanceof ApiClientError) {
				signatureAvailable = error.status !== 404;
			}
		}

		const summaryCode = person.summary;
		summaryRendered = await renderMarkdown(summaryCode);

		const pastCode = person.past;
		pastRendered = await renderMarkdown(pastCode);

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
		try {
			toaster.error({
				title: "Вы уверены?",
				description: "Подтвердите удаление записи",
				duration: 8000,
				action: {
					label: "Удалить",
					onClick: async () => {
						if (!person) return;

						await PersonService.delete_person(person.id.id.String);
						window.location.href = "/";
					}
				}
			});
		} catch (error) {
			if (error instanceof ApiClientError) {
				toaster.error({
					title: "Ошибка на стороне API",
					description: error.describe()
				});
			} else {
				toaster.error({
					title: "Ошибка на стороне API",
					description: error
				});
			}
		}
	}

	async function handleSignSubmit(event: Event) {
		event.preventDefault();

		try {
			await PersonService.sign_person(personId || "", signPayload);
			location.reload();
		} catch (error) {
			if (error instanceof ApiClientError) {
				toaster.error({
					title: "Ошибка на стороне API",
					description: error.status === 400 ? "Неверный ключ для подписи" : error.describe()
				});
			} else {
				toaster.error({
					title: "Ошибка на стороне API",
					description: error
				});
			}
		}
	}

	async function handleUnsign(event: Event) {
		event.preventDefault();

		try {
			toaster.error({
				title: "Вы уверены?",
				description: "Вы собираетесь удалить подпись",
				duration: 8000,
				action: {
					label: "Удалить",
					onClick: async () => {
						await PersonService.unsign_person(personId || "");
						location.reload();
					}
				}
			});
		} catch (error) {
			if (error instanceof ApiClientError) {
				toaster.error({
					title: "Ошибка на стороне API",
					description: error.describe()
				});
			} else {
				toaster.error({
					title: "Неизвестная ошибка на стороне API",
					description: error
				});
			}
		}
	}

	async function handleReSign(event: Event) {
		event.preventDefault();

		try {
			await PersonService.unsign_person(personId || "");
			await PersonService.sign_person(personId || "", signPayload);
			location.reload();
		} catch (error) {
			if (error instanceof ApiClientError) {
				toaster.error({
					title: "Ошибка на стороне API",
					description: error.describe()
				});
			} else {
				toaster.error({
					title: "Неизвестная ошибка на стороне API",
					description: error
				});
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
					class="aspect-video h-auto w-full object-cover"
				/>
			</header>

			<article class="space-y-3 p-3">
				<!-- Edit/Delete Section -->
				<Protected
					requiredRoles={[EDITOR_ROLE]}
					adminRoles={[ADMIN_ROLE]}
					requiredUsername={person.author}
				>
					<div class="float-right">
						<a
							href={resolve(`/persons/${personId}/edit`)}
							class="btn-icon preset-outlined-surface-500"
						>
							<PenIcon />
						</a>

						<!-- Sign Section -->
						<Protected requiredRoles={[ADMIN_ROLE]}>
							{#if !signatureAvailable}
								<!-- Apply Signature Dialog -->

								{#if $loggedUser && $loggedUser.public_key}
									<Dialog>
										<Dialog.Trigger>
											<!-- Sign Button -->
											<button class="btn-icon preset-outlined-surface-500">
												<SignatureIcon />
											</button>
										</Dialog.Trigger>

										<Portal>
											<Dialog.Backdrop class="fixed inset-0 z-50 bg-surface-50-950/50" />
											<Dialog.Positioner
												class="fixed inset-0 z-50 flex items-center justify-center"
											>
												<Dialog.Content
													class="w-xl space-y-2 card bg-surface-100-900 p-4 shadow-xl"
												>
													<!-- Sign Dialog Content -->
													<form class="space-y-4" onsubmit={handleSignSubmit}>
														<div>
															<h4 class="h4">Подпись записи из базы данных</h4>
															<p>
																Данная подпись будет означать, что вы проверили указанную на
																странице информацию и подтверждаете её достоверность. <br />
																Сразу после нажатия кнопки "Подписать" на странице появится плашка о
																подписи с вашей стороны.
															</p>
														</div>

														<hr class="hr" />

														<!-- Private Key Field -->
														<label class="label">
															<span class="label-text">Ключ для подписи:</span>
															<input
																class="input"
																type="text"
																placeholder="Введите ваш приватный ключ для подписи..."
																bind:value={signPayload.private_key}
																required
															/>
														</label>

														<!-- Centering Div -->
														<div class="flex items-center justify-center">
															<!-- Submit Button -->
															<button type="submit" class="btn preset-filled-primary-500"
																>Подписать</button
															>
														</div>
													</form>
												</Dialog.Content>
											</Dialog.Positioner>
										</Portal>
									</Dialog>
								{/if}
							{:else}
								<!-- Unsign Button -->
								{#if $loggedUser}
									<Protected
										requiredRoles={[ADMIN_ROLE]}
										requiredUsername={$loggedUser.username}
										staticAdminAllowed={true}
									>
										<button class="btn-icon preset-outlined-error-500" onclick={handleUnsign}>
											<SignatureIcon />
										</button>
									</Protected>
								{/if}
							{/if}
						</Protected>

						<button onclick={handleDelete} class="btn-icon preset-outlined-error-500">
							<TrashIcon />
						</button>
					</div>
				</Protected>

				<!-- Title -->
				<h1 class="text-lg font-semibold">{person.surname} {person.name} {person.patronymic}</h1>

				<!-- Author -->
				<div class="flex items-center space-x-2 text-surface-200">
					<UserPenIcon size={17} />
					<a href={resolve(`/users/${person.author}`)} class="font-mono hover:text-primary-400"
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

				<!-- Signature Section -->
				{#if signatureAvailable}
					{#if signature}
						<div class="cursor-default rounded-md preset-outlined-success-500 p-2">
							<h6 class="h6">
								Запись проверена и подписана <span class="font-mono text-success-500"
									>{signature.signed_by}</span
								>
							</h6>
							<p>
								Base64:
								<button
									class="text-mono text-md w-md cursor-pointer truncate opacity-55 hover:opacity-100"
									onclick={() => {
										if (!signature) return;
										navigator.clipboard.writeText(signature.base64);

										toaster.info({
											title: "Текст скопирован",
											duration: 700
										});
									}}>{signature.base64}</button
								>
							</p>
							<p>
								Публичный ключ:
								<button
									class="text-mono text-md cursor-pointer opacity-55 hover:opacity-100"
									onclick={() => {
										if (!signature) return;
										navigator.clipboard.writeText(signature.pubkey);

										toaster.info({
											title: "Текст скопирован",
											duration: 700
										});
									}}>{signature.pubkey}</button
								>
							</p>
						</div>
					{:else}
						<div class="flex justify-between space-y-2 rounded-md preset-outlined-warning-500 p-2">
							<div class="flex items-center space-x-1">
								<CircleAlertIcon size={17} />
								<p>Запись имеет истёкшую подпись</p>
							</div>
						</div>

						{#if $loggedUser && $loggedUser.role === ADMIN_ROLE && $loggedUser.public_key}
							<Dialog>
								<p>
									Вы как администратор можете

									<Dialog.Trigger>
										<button class="text-blue-400 hover:underline">переподписать</button>
									</Dialog.Trigger>
									данную запись.
								</p>

								<Portal>
									<Dialog.Backdrop class="fixed inset-0 z-50 bg-surface-50-950/50" />
									<Dialog.Positioner class="fixed inset-0 z-50 flex items-center justify-center">
										<Dialog.Content class="w-xl space-y-2 card bg-surface-100-900 p-4 shadow-xl">
											<!-- Sign Dialog Content -->
											<form class="space-y-4" onsubmit={handleReSign}>
												<div>
													<h4 class="h4">Подпись записи из базы данных</h4>
													<p>
														Данная подпись будет означать, что вы проверили указанную на странице
														информацию и подтверждаете её достоверность. <br />
														Сразу после нажатия кнопки "Подписать" на странице появится плашка о подписи
														с вашей стороны.
													</p>
												</div>

												<hr class="hr" />

												<!-- Private Key Field -->
												<label class="label">
													<span class="label-text">Ключ для подписи:</span>
													<input
														class="input"
														type="text"
														placeholder="Введите ваш приватный ключ для подписи..."
														bind:value={signPayload.private_key}
														required
													/>
												</label>

												<!-- Centering Div -->
												<div class="flex items-center justify-center">
													<!-- Submit Button -->
													<button type="submit" class="btn preset-filled-primary-500"
														>Подписать</button
													>
												</div>
											</form>
										</Dialog.Content>
									</Dialog.Positioner>
								</Portal>
							</Dialog>
						{/if}
					{/if}

					<hr class="hr" />
				{/if}

				<!-- Summary -->
				<div>
					<h6 class="h6">Описание:</h6>
					<Maybenot prop={summaryRendered}>
						<div
							class="prose dark:prose-invert rounded-md border-1 border-surface-800 p-2 [&_h1]:h1 [&_h2]:h2 [&_h3]:h3 [&_h4]:h4 [&_h5]:h5 [&_h6]:h6 [&>p+h1]:mt-5 [&>p+h2]:mt-5 [&>p+h3]:mt-5 [&>p+h4]:mt-5 [&>p+h5]:mt-5 [&>p+h6]:mt-5 [&>p+p]:mt-5"
						>
							<!-- eslint-disable-next-line svelte/no-at-html-tags -->
							{@html summaryRendered}
						</div>
					</Maybenot>
				</div>

				<!-- Past -->
				<div>
					<h6 class="h6">Прошлое:</h6>
					<Maybenot prop={pastRendered}>
						<div
							class="prose dark:prose-invert rounded-md border-1 border-surface-800 p-2 [&_h1]:h1 [&_h2]:h2 [&_h3]:h3 [&_h4]:h4 [&_h5]:h5 [&_h6]:h6 [&>p+h1]:mt-5 [&>p+h2]:mt-5 [&>p+h3]:mt-5 [&>p+h4]:mt-5 [&>p+h5]:mt-5 [&>p+h6]:mt-5 [&>p+p]:mt-5"
						>
							<!-- eslint-disable-next-line svelte/no-at-html-tags -->
							{@html pastRendered}
						</div>
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
											class="aspect-video h-auto cursor-pointer rounded-md object-cover hover:opacity-70"
										/>
									</Dialog.Trigger>

									<Portal>
										<Dialog.Backdrop class="fixed inset-0 z-50 bg-surface-50-950/50" />
										<Dialog.Positioner class="fixed inset-0 z-50 flex items-center justify-center">
											<Dialog.Content class="w-5xl space-y-2 card bg-surface-100-900 p-4 shadow-xl">
												<Dialog.CloseTrigger class="group float-right">
													<img
														src={mediaURL}
														alt=""
														class="aspect-video h-auto w-full cursor-pointer rounded-md object-cover"
													/>
												</Dialog.CloseTrigger>
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

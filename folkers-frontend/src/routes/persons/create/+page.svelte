<script lang="ts">
	import { ACCEPTABLE_MEDIA_TYPES, renderMarkdown } from "$lib";
	import { ApiClientError } from "$lib/api/error";
	import type { CreatePersonRecord } from "$lib/types/person";

	import { MediaService } from "$lib/services/media.service";
	import { PersonService } from "$lib/services/person.service";

	import { DatePicker, Portal } from "@skeletonlabs/skeleton-svelte";
	import { SegmentedControl } from "@skeletonlabs/skeleton-svelte";
	import { EyeIcon, PencilIcon } from "@lucide/svelte";

	let payload: CreatePersonRecord = $state({
		name: "",
		surname: "",
		patronymic: "",

		birthday: "",
		city: "",
		intented_address: "",

		summary: "",
		past: "",
		traits_good: "",
		traits_bad: "",

		avatar: null,
		media: []
	});

	let summaryCurrentState: string | null = $state("edit");
	let summaryMarkdownPreview = $derived.by(async () => {
		if (!payload) return "";

		const code = payload.summary;
		const result = await renderMarkdown(code);

		return result;
	});

	let pastCurrentState: string | null = $state("edit");
	let pastMarkdownPreview = $derived.by(async () => {
		if (!payload) return "";

		const code = payload.summary;
		const result = await renderMarkdown(code);

		return result;
	});

	let avatarFile: FileList | null = $state(null);
	let avatarURL: string = $state("");

	let mediaFiles: FileList | null = $state(null);
	let mediaURLS: string[] = $state([""]);

	async function handleMediaInput(index: number, event: Event) {
		event.preventDefault();

		const target = event.target as HTMLInputElement;
		const value = target.value;

		mediaURLS[index] = value;

		if (index === mediaURLS.length - 1 && value.trim() !== "") {
			mediaURLS = [...mediaURLS, ""];
		}
	}

	async function handleSubmit(event: Event) {
		event.preventDefault();

		// Uploading avatar
		if (avatarFile) {
			try {
				payload.avatar = await MediaService.upload(avatarFile[0]);
			} catch (error) {
				console.error(error);
				payload.avatar = "NONE";
			}
		} else {
			payload.avatar = avatarURL;
		}

		// Uploading medias
		let payloadMedia: string[] = [];

		if (mediaFiles) {
			for (const mediaFile of mediaFiles) {
				try {
					const hash = await MediaService.upload(mediaFile);
					payloadMedia.push(hash);
				} catch (error) {
					console.error("Error Uploading Media: ", error);
				}
			}
		}

		payloadMedia = [...payloadMedia, ...mediaURLS];

		payload.media = payloadMedia.filter((url) => url.trim().length > 0);

		// Convert birthday to ISO string format
		payload.birthday = new Date(payload.birthday).toISOString();

		// Sending request

		try {
			const new_person = await PersonService.create_person(payload);

			window.location.href = `/persons/${new_person.id.id.String}`;
		} catch (error) {
			if (error instanceof ApiClientError) {
				console.error(error);
			} else {
				console.error(error);
			}
		}
	}
</script>

<!-- Centering Div -->
<div class="flex items-center justify-center p-4">
	<!-- Content Div -->
	<div class="space-y-4">
		<p class="text-xl font-bold">Создание новой записи о человеке:</p>

		<!-- Form Card -->
		<div
			class="block w-2xl divide-y divide-surface-200-800 overflow-hidden card border-[1px] border-surface-200-800 preset-filled-surface-100-900 shadow-xl"
		>
			<article>
				<form class="space-y-5 p-4" onsubmit={handleSubmit}>
					<!-- Surname Name Patronymic -->
					<label class="label">
						<span class="label-text">ФИО:</span>

						<div class="flex gap-3">
							<input class="input" type="text" placeholder="Фамилия" bind:value={payload.surname} />
							<input class="input" type="text" placeholder="Имя" bind:value={payload.name} />
							<input
								class="input"
								type="text"
								placeholder="Отчество"
								bind:value={payload.patronymic}
							/>
						</div>
					</label>

					<!-- City -->
					<label class="label">
						<span class="label-text">Город проживания:</span>
						<input class="input" type="text" placeholder="г. Волгоград" bind:value={payload.city} />
					</label>

					<!-- Intented Address -->
					<label class="label">
						<span class="label-text">Предполагаемый адрес:</span>
						<input
							class="input"
							type="text"
							placeholder="ул. Пушкина, д. 123"
							bind:value={payload.intented_address}
						/>
					</label>

					<!-- Birthday -->
					<DatePicker
						onValueChange={(e) => (payload.birthday = e.value.toString())}
						locale="ru-RU"
						startOfWeek={0}
					>
						<DatePicker.Label>Дата рождения</DatePicker.Label>
						<DatePicker.Control>
							<DatePicker.Input placeholder="dd.mm.yyyy" />
							<DatePicker.Trigger />
						</DatePicker.Control>
						<Portal>
							<DatePicker.Positioner>
								<DatePicker.Content>
									<DatePicker.YearSelect />
									<DatePicker.MonthSelect />
									<DatePicker.View view="day">
										<DatePicker.Context>
											{#snippet children(datePicker)}
												<DatePicker.ViewControl>
													<DatePicker.PrevTrigger />
													<DatePicker.ViewTrigger disabled>
														<DatePicker.RangeText />
													</DatePicker.ViewTrigger>
													<DatePicker.NextTrigger />
												</DatePicker.ViewControl>
												<DatePicker.Table>
													<DatePicker.TableHead>
														<DatePicker.TableRow>
															{#each datePicker().weekDays as weekDay, id (id)}
																<DatePicker.TableHeader>{weekDay.short}</DatePicker.TableHeader>
															{/each}
														</DatePicker.TableRow>
													</DatePicker.TableHead>
													<DatePicker.TableBody>
														{#each datePicker().weeks as week, id (id)}
															<DatePicker.TableRow>
																{#each week as day, id (id)}
																	<DatePicker.TableCell value={day}>
																		<DatePicker.TableCellTrigger
																			>{day.day}</DatePicker.TableCellTrigger
																		>
																	</DatePicker.TableCell>
																{/each}
															</DatePicker.TableRow>
														{/each}
													</DatePicker.TableBody>
												</DatePicker.Table>
											{/snippet}
										</DatePicker.Context>
									</DatePicker.View>
									<DatePicker.View view="month">
										<DatePicker.Context>
											{#snippet children(datePicker)}
												<DatePicker.ViewControl>
													<DatePicker.PrevTrigger />
													<DatePicker.ViewTrigger>
														<DatePicker.RangeText />
													</DatePicker.ViewTrigger>
													<DatePicker.NextTrigger />
												</DatePicker.ViewControl>
												<DatePicker.Table>
													<DatePicker.TableBody>
														{#each datePicker().getMonthsGrid( { columns: 4, format: "short" } ) as months, id (id)}
															<DatePicker.TableRow>
																{#each months as month, id (id)}
																	<DatePicker.TableCell value={month.value}>
																		<DatePicker.TableCellTrigger
																			>{month.label}</DatePicker.TableCellTrigger
																		>
																	</DatePicker.TableCell>
																{/each}
															</DatePicker.TableRow>
														{/each}
													</DatePicker.TableBody>
												</DatePicker.Table>
											{/snippet}
										</DatePicker.Context>
									</DatePicker.View>
									<DatePicker.View view="year">
										<DatePicker.Context>
											{#snippet children(datePicker)}
												<DatePicker.ViewControl>
													<DatePicker.PrevTrigger />
													<DatePicker.ViewTrigger>
														<DatePicker.RangeText />
													</DatePicker.ViewTrigger>
													<DatePicker.NextTrigger />
												</DatePicker.ViewControl>
												<DatePicker.Table>
													<DatePicker.TableBody>
														{#each datePicker().getYearsGrid({ columns: 4 }) as years, id (id)}
															<DatePicker.TableRow>
																{#each years as year, id (id)}
																	<DatePicker.TableCell value={year.value}>
																		<DatePicker.TableCellTrigger
																			>{year.label}</DatePicker.TableCellTrigger
																		>
																	</DatePicker.TableCell>
																{/each}
															</DatePicker.TableRow>
														{/each}
													</DatePicker.TableBody>
												</DatePicker.Table>
											{/snippet}
										</DatePicker.Context>
									</DatePicker.View>
								</DatePicker.Content>
							</DatePicker.Positioner>
						</Portal>
					</DatePicker>

					<!-- Traits -->
					<label class="label">
						<span class="label-text">Черты:</span>

						<div class="flex gap-3">
							<!-- Good Traits -->
							<input
								class="input preset-outlined-success-500"
								type="text"
								placeholder="Хорошие"
								bind:value={payload.traits_good}
							/>
							<!-- Bad Traits -->
							<input
								class="input preset-outlined-error-500"
								type="text"
								placeholder="Плохие"
								bind:value={payload.traits_bad}
							/>
						</div>
					</label>

					<!-- Summary -->
					<label class="label">
						<span class="label-text">Описание (Markdown):</span>

						<div class="space-y-0">
							<!-- Segmented State Control -->
							<div class="flex items-center gap-4">
								<SegmentedControl
									value={summaryCurrentState}
									onValueChange={(details) => (summaryCurrentState = details.value)}
								>
									<SegmentedControl.Control
										class="rounded-b-0 rounded-t-lg rounded-b-[0px] border-b-0"
									>
										<SegmentedControl.Indicator />
										<!-- Edit Mode -->
										<SegmentedControl.Item value="edit" class="rounded-lg px-2">
											<SegmentedControl.ItemText class="text-xs">
												<PencilIcon class="size-4" />
											</SegmentedControl.ItemText>
											<SegmentedControl.ItemHiddenInput />
										</SegmentedControl.Item>
										<!-- Preview Mode -->
										<SegmentedControl.Item value="preview" class="rounded-lg px-2">
											<SegmentedControl.ItemText class="text-xs">
												<EyeIcon class="size-4" />
											</SegmentedControl.ItemText>
											<SegmentedControl.ItemHiddenInput />
										</SegmentedControl.Item>
										<!-- -->
									</SegmentedControl.Control>
								</SegmentedControl>
							</div>

							<!-- Editor -->
							{#if !summaryCurrentState || summaryCurrentState === "edit"}
								<!-- Text Area -->
								<textarea
									class="textarea h-[250px] resize-none rounded-tl-[0px]"
									placeholder="Пишите здесь..."
									maxlength="4000"
									bind:value={payload.summary}
								></textarea>
							{:else}
								<!-- Preview -->
								<div
									class="prose dark:prose-invert h-[250px] overflow-scroll rounded-lg rounded-tl-[0px] border border-surface-800 p-2 [&_h1]:h1 [&_h2]:h2 [&_h3]:h3 [&_h4]:h4 [&_h5]:h5 [&_h6]:h6 [&>p+h1]:mt-5 [&>p+h2]:mt-5 [&>p+h3]:mt-5 [&>p+h4]:mt-5 [&>p+h5]:mt-5 [&>p+h6]:mt-5 [&>p+p]:mt-5"
								>
									{#await summaryMarkdownPreview then previewValue}
										<!-- eslint-disable-next-line svelte/no-at-html-tags -->
										{@html previewValue}
									{/await}
								</div>
							{/if}
						</div>
					</label>

					<!-- Past -->
					<label class="label">
						<span class="label-text">Прошлое (Markdown):</span>

						<div class="space-y-0">
							<!-- Segmented State Control -->
							<div class="flex items-center gap-4">
								<SegmentedControl
									value={pastCurrentState}
									onValueChange={(details) => (pastCurrentState = details.value)}
								>
									<SegmentedControl.Control
										class="rounded-b-0 rounded-t-lg rounded-b-[0px] border-b-0"
									>
										<SegmentedControl.Indicator />
										<!-- Edit Mode -->
										<SegmentedControl.Item value="edit" class="rounded-lg px-2">
											<SegmentedControl.ItemText class="text-xs">
												<PencilIcon class="size-4" />
											</SegmentedControl.ItemText>
											<SegmentedControl.ItemHiddenInput />
										</SegmentedControl.Item>
										<!-- Preview Mode -->
										<SegmentedControl.Item value="preview" class="rounded-lg px-2">
											<SegmentedControl.ItemText class="text-xs">
												<EyeIcon class="size-4" />
											</SegmentedControl.ItemText>
											<SegmentedControl.ItemHiddenInput />
										</SegmentedControl.Item>
										<!-- -->
									</SegmentedControl.Control>
								</SegmentedControl>
							</div>

							<!-- Editor -->
							{#if !pastCurrentState || pastCurrentState === "edit"}
								<!-- Text Area -->
								<textarea
									class="textarea h-[250px] resize-none rounded-tl-[0px]"
									placeholder="Пишите здесь..."
									maxlength="4000"
									bind:value={payload.past}
								></textarea>
							{:else}
								<!-- Preview -->
								<div
									class="prose dark:prose-invert h-[250px] overflow-scroll rounded-lg rounded-tl-[0px] border border-surface-800 p-2 [&_h1]:h1 [&_h2]:h2 [&_h3]:h3 [&_h4]:h4 [&_h5]:h5 [&_h6]:h6 [&>p+h1]:mt-5 [&>p+h2]:mt-5 [&>p+h3]:mt-5 [&>p+h4]:mt-5 [&>p+h5]:mt-5 [&>p+h6]:mt-5 [&>p+p]:mt-5"
								>
									{#await pastMarkdownPreview then previewValue}
										<!-- eslint-disable-next-line svelte/no-at-html-tags -->
										{@html previewValue}
									{/await}
								</div>
							{/if}
						</div>
					</label>

					<!-- Centering Div -->
					<div class="flex items-center justify-center">
						<!-- Submit Button -->
						<button class="btn preset-filled-primary-500"> Создать </button>
					</div>
				</form>
			</article>
		</div>
	</div>
</div>

<!-- <form on:submit|preventDefault={handleForm} class="block p-5"> -->
<!-- 	<p class="text-red-500">{errorMessage}</p> -->
<!---->
<!-- 	<br /> -->
<!---->
<!-- 	<div class="flex gap-3"> -->
<!-- 		<input -->
<!-- 			type="text" -->
<!-- 			placeholder="Фамилия" -->
<!-- 			bind:value={payload.surname} -->
<!-- 			required -->
<!-- 			class="border-1 border-black p-1" -->
<!-- 		/> -->
<!-- 		<input -->
<!-- 			type="text" -->
<!-- 			placeholder="Имя" -->
<!-- 			bind:value={payload.name} -->
<!-- 			required -->
<!-- 			class="border-1 border-black p-1" -->
<!-- 		/> -->
<!-- 		<input -->
<!-- 			type="text" -->
<!-- 			placeholder="Отчество" -->
<!-- 			bind:value={payload.patronymic} -->
<!-- 			required -->
<!-- 			class="border-1 border-black p-1" -->
<!-- 		/> -->
<!-- 	</div> -->
<!---->
<!-- 	<br /> -->
<!---->
<!-- 	<div class=""> -->
<!-- 		<p>Дата рождения:</p> -->
<!-- 		<input -->
<!-- 			type="date" -->
<!-- 			placeholder="Дата рождения" -->
<!-- 			bind:value={payload.birthday} -->
<!-- 			required -->
<!-- 			class="border-1 border-black p-1" -->
<!-- 		/> -->
<!---->
<!-- 		<br /> -->
<!-- 		<br /> -->
<!---->
<!-- 		<p>Город проживания</p> -->
<!-- 		<input -->
<!-- 			type="text" -->
<!-- 			placeholder="Москва" -->
<!-- 			bind:value={payload.city} -->
<!-- 			class="border-1 border-black p-1" -->
<!-- 		/> -->
<!---->
<!-- 		<br /> -->
<!-- 		<br /> -->
<!---->
<!-- 		<p>Предполагаемый адресс проживания:</p> -->
<!-- 		<input -->
<!-- 			type="text" -->
<!-- 			placeholder="ул. Пушкина, дом 1" -->
<!-- 			bind:value={payload.intented_address} -->
<!-- 			class="border-1 border-black p-1" -->
<!-- 		/> -->
<!---->
<!-- 		<br /> -->
<!-- 		<br /> -->
<!---->
<!-- 		<div class="flex gap-3"> -->
<!-- 			<div> -->
<!-- 				<p>Хорошие черты:</p> -->
<!-- 				<input -->
<!-- 					type="text" -->
<!-- 					placeholder="Умный, красивый и тд." -->
<!-- 					bind:value={payload.traits_good} -->
<!-- 					class="border-1 border-black p-1" -->
<!-- 				/> -->
<!-- 			</div> -->
<!---->
<!-- 			<div> -->
<!-- 				<p>Плохие черты:</p> -->
<!-- 				<input -->
<!-- 					type="text" -->
<!-- 					placeholder="Тупой, уродливый и тд." -->
<!-- 					bind:value={payload.traits_bad} -->
<!-- 					class="border-1 border-black p-1" -->
<!-- 				/> -->
<!-- 			</div> -->
<!-- 		</div> -->
<!---->
<!-- 		<br /> -->
<!---->
<!-- 		<div> -->
<!-- 			<p>Описание:</p> -->
<!-- 			<textarea class="border-1 border-black p-1" bind:value={payload.summary}></textarea> -->
<!-- 		</div> -->
<!---->
<!-- 		<br /> -->
<!---->
<!-- 		<div> -->
<!-- 			<p>Прошлое:</p> -->
<!-- 			<textarea class="border-1 border-black p-1" bind:value={payload.past}></textarea> -->
<!-- 		</div> -->
<!---->
<!-- 		<br /> -->
<!---->
<!-- 		<p>Аватар:</p> -->
<!-- 		<div class="flex gap-3"> -->
<!-- 			<div> -->
<!-- 				<input -->
<!-- 					type="file" -->
<!-- 					class="border-1 border-black p-1" -->
<!-- 					bind:files={avatarFile} -->
<!-- 					accept={ACCEPTABLE_MEDIA_TYPES} -->
<!-- 				/> -->
<!-- 			</div> -->
<!---->
<!-- 			<div> -->
<!-- 				<input -->
<!-- 					type="url" -->
<!-- 					placeholder="Ссылка на изображение" -->
<!-- 					class="border-1 border-black p-1" -->
<!-- 					disabled={avatarFile !== null} -->
<!-- 					bind:value={avatarURL} -->
<!-- 				/> -->
<!-- 			</div> -->
<!-- 		</div> -->
<!---->
<!-- 		<br /> -->
<!---->
<!-- 		<p>Медиа:</p> -->
<!-- 		<div> -->
<!-- 			<input -->
<!-- 				type="file" -->
<!-- 				class="border-1 border-black p-1" -->
<!-- 				bind:files={mediaFiles} -->
<!-- 				accept={ACCEPTABLE_MEDIA_TYPES} -->
<!-- 				multiple -->
<!-- 			/> -->
<!---->
<!-- 			<br /> -->
<!---->
<!-- 			{#each mediaURLS as content, i (i)} -->
<!-- 				<br /> -->
<!-- 				<input -->
<!-- 					type="url" -->
<!-- 					placeholder="Введите URL..." -->
<!-- 					bind:value={mediaURLS[i]} -->
<!-- 					on:input={(event) => handleMediaInput(i, event)} -->
<!-- 					class="border-1 border-black p-1" -->
<!-- 				/> -->
<!-- 				<p class="hidden">{content}</p> -->
<!-- 				<br /> -->
<!-- 			{/each} -->
<!-- 		</div> -->
<!-- 	</div> -->
<!---->
<!-- 	<br /> -->
<!---->
<!-- 	<div> -->
<!-- 		<button type="submit" class="border-1 border-black p-1">Создать</button> -->
<!-- 	</div> -->
<!-- </form> -->

<script lang="ts">
	import { goto } from "$app/navigation";
	import { resolve } from "$app/paths";
	import { toaster } from "$lib/stores/toaster";

	import { ApiClientError } from "$lib/api/error";
	import type { CreatePersonRecord } from "$lib/types/person";

	import { MediaService } from "$lib/services/media.service";
	import { PersonService } from "$lib/services/person.service";

	import { DatePicker, Portal } from "@skeletonlabs/skeleton-svelte";
	import { SegmentedControl } from "@skeletonlabs/skeleton-svelte";
	import { FileUpload, useFileUpload } from "@skeletonlabs/skeleton-svelte";

	import { EyeIcon, ImageIcon, ImagesIcon, PencilIcon } from "@lucide/svelte";
	import { ACCEPTABLE_MEDIA_TYPES, renderMarkdown } from "$lib";

	const id = $props.id();

	let payload: CreatePersonRecord = $state({
		name: "",
		surname: "",
		patronymic: "",

		birthday: "01.01.2000",
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

	let avatarFile: File | null = $state(null);
	let avatarPreviewURL: string = $state("");

	const avatarUpload = useFileUpload({
		id: `${id}-AVATAR`,
		locale: "ru-RU",
		accept: ACCEPTABLE_MEDIA_TYPES,
		maxFiles: 1,

		onFileChange: (props) => {
			const selectedFile = props.acceptedFiles?.[0];
			if (!selectedFile) {
				avatarFile = null;
				return;
			}

			avatarFile = selectedFile;

			const reader = new FileReader();

			reader.onload = (e) => {
				avatarPreviewURL = e.target?.result as string;
			};

			reader.readAsDataURL(selectedFile);
		}
	});

	let mediaFiles: File[] | null = $state(null);

	const mediaUpload = useFileUpload({
		id: `${id}-MEDIA`,
		locale: "ru-RU",
		accept: ACCEPTABLE_MEDIA_TYPES,
		maxFiles: 20,

		onFileChange: (props) => {
			mediaFiles = props.acceptedFiles.length > 0 ? props.acceptedFiles : null;
		}
	});

	async function handleSubmit(event: Event) {
		event.preventDefault();

		let successful = true;

		// Uploading avatar
		if (avatarFile) {
			try {
				const response = await MediaService.upload(avatarFile);
				payload.avatar = response;
			} catch (error) {
				console.error(error);

				payload.avatar = "";
				successful = false;

				const errorDescription = error instanceof ApiClientError ? error.describe() : error;

				toaster.error({
					title: "Ошибка загрузки аватара",
					description: errorDescription
				});
			}
		} else {
			payload.avatar = "";
		}

		// Uploading medias
		let payloadMedia: string[] = [];

		if (mediaFiles) {
			for (const mediaFile of mediaFiles) {
				try {
					const hash = await MediaService.upload(mediaFile);
					payloadMedia.push(hash);
				} catch (error) {
					console.error(error);

					const errorDescription = error instanceof ApiClientError ? error.describe() : error;
					successful = false;

					toaster.error({
						title: "Ошибка загрузки медиа",
						description: errorDescription
					});
				}
			}
		}

		payload.media = payloadMedia;

		// Convert birthday to ISO string format
		payload.birthday = new Date(payload.birthday).toISOString();

		// Sending request

		if (!successful) return;

		try {
			const new_person = await PersonService.create_person(payload);

			goto(resolve(`/persons/${new_person.id.id.String}`));
		} catch (error) {
			console.error(error);

			const errorDescription = error instanceof ApiClientError ? error.describe() : error;

			toaster.error({
				title: "Ошибка создания записи",
				description: errorDescription
			});
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

					<!-- Avatar Upload -->

					<label class="label">
						<span class="label-text text-lg">Аватар:</span>
						<div class="grid w-full gap-4">
							<FileUpload.Provider value={avatarUpload}>
								<FileUpload.Dropzone>
									{#if avatarUpload().acceptedFiles.length < 1}
										<ImageIcon class="size-10" />
										<span>Выберите или перенесите фото сюда</span>
										<FileUpload.Trigger>Обзор</FileUpload.Trigger>
										<FileUpload.HiddenInput />
									{:else}
										<img
											src={avatarPreviewURL}
											alt="Selected Avatar"
											class="aspect-video h-auto w-full rounded-md object-cover"
										/>
									{/if}
								</FileUpload.Dropzone>
								<FileUpload.ItemGroup>
									<FileUpload.Context>
										{#snippet children(fileUpload)}
											{#each fileUpload().acceptedFiles as file (file.name)}
												<FileUpload.Item {file}>
													<FileUpload.ItemName>{file.name}</FileUpload.ItemName>
													<FileUpload.ItemSizeText
														>{(file.size / 1024 / 1024).toFixed(3)} megabytes</FileUpload.ItemSizeText
													>
													<FileUpload.ItemDeleteTrigger />
												</FileUpload.Item>
											{/each}
										{/snippet}
									</FileUpload.Context>
								</FileUpload.ItemGroup>
							</FileUpload.Provider>
						</div>
					</label>

					<!-- Media Upload -->

					<label class="label">
						<span class="label-text text-lg">Медиа:</span>
						<div class="grid w-full gap-4">
							<FileUpload.Provider value={mediaUpload}>
								<FileUpload.Dropzone>
									<ImagesIcon class="size-10" />
									<span>Выберите или перенесите фото сюда</span>
									<FileUpload.Trigger>Обзор</FileUpload.Trigger>
									<FileUpload.HiddenInput />
								</FileUpload.Dropzone>
								<FileUpload.ItemGroup>
									<FileUpload.Context>
										{#snippet children(mediaUpload)}
											{#each mediaUpload().acceptedFiles as file (file.name)}
												<FileUpload.Item {file}>
													<FileUpload.ItemName>{file.name}</FileUpload.ItemName>
													<FileUpload.ItemSizeText
														>{(file.size / 1024 / 1024).toFixed(3)} megabytes</FileUpload.ItemSizeText
													>
													<FileUpload.ItemDeleteTrigger />
												</FileUpload.Item>
											{/each}
										{/snippet}
									</FileUpload.Context>
								</FileUpload.ItemGroup>
							</FileUpload.Provider>
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

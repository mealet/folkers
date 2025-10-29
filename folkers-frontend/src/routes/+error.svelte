<script lang="ts">
	import { page } from "$app/state";
</script>

<!-- Centering Div -->
<div class="mt-4 flex items-center justify-center p-4">
	{#if page.error}
		<!-- Error Card -->
		<div
			class="block max-w-xl min-w-xs overflow-hidden card preset-outlined-error-500 drop-shadow-xl"
		>
			<article class="space-y-4 p-3">
				<div class="flex space-x-3">
					<h1 class="text-xl font-medium">Ошибка</h1>
					<span class="badge preset-filled-error-500 pt-1 pb-1 align-middle font-medium"
						>{page.error.status || "500"}</span
					>
				</div>

				<!-- eslint-disable-next-line svelte/no-at-html-tags -->
				<p>{@html page.error.message.replace("\n", "<br/>")}</p>
			</article>
		</div>
	{/if}
</div>

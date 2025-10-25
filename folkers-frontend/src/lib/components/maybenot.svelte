<script lang="ts">
	let { children, prop } = $props();

	const NOT_SPECIFIED_MESSAGE = "Не указано";
</script>

{#if prop}
	{@render children?.()}
{:else}
	<span class="badge preset-outlined-surface-500">{NOT_SPECIFIED_MESSAGE}</span>
{/if}

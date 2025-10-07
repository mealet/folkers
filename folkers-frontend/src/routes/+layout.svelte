<script lang="ts">
	import '../app.css';
	import favicon from '$lib/assets/favicon.svg';

	import { onMount } from 'svelte';
	import { initializeAuth } from '$lib/stores/auth';

	onMount(() => {
		initializeAuth();
	});

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

{@render children?.()}

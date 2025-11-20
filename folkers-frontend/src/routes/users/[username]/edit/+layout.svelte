<script lang="ts">
	import { page } from "$app/state";

	const username = page.params.username;
</script>

<svelte:head>
	<title>Редактировать {username} - Folkers</title>
</svelte:head>

<slot />

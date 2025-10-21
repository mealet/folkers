<script lang="ts">
	import { AuthService } from "$lib/services/auth.service";
	import type { LoginCredentials } from "$lib/types/auth";

	let credentials: LoginCredentials = {
		username: "",
		password: ""
	};
	let error = "";
	let loading = false;

	async function handleLogin(): Promise<void> {
		if (loading) return;

		loading = true;
		error = "";

		try {
			await AuthService.login(credentials);
			window.location.href = "/";
		} catch (err) {
			error = err instanceof Error ? err.message : "Login failed";
			return;
		} finally {
			loading = false;
		}
	}
</script>

<form>
	<input
		type="text"
		bind:value={credentials.username}
		placeholder="Username"
		required
		disabled={loading}
	/>
	<br />
	<br />
	<input
		type="password"
		bind:value={credentials.password}
		placeholder="Password"
		required
		disabled={loading}
	/>
	<br />
	<br />
	<button disabled={loading} on:click|preventDefault={handleLogin}>
		{loading ? "Logging in..." : "Login"}
	</button>

	{#if error}
		<br />
		<div class="error">{error}</div>
	{/if}
</form>

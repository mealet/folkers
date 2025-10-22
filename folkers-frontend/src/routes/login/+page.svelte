<script lang="ts">
	import { KeyRound } from "@lucide/svelte";
	import { createToaster } from "@skeletonlabs/skeleton-svelte";

	import { AuthService } from "$lib/services/auth.service";
	import type { LoginCredentials } from "$lib/types/auth";

	const bannerSrc = "/banner.png";
	const toaster = createToaster({});

	let credentials: LoginCredentials = {
		username: "",
		password: ""
	};
	let loading = false;

	async function handleLogin(): Promise<void> {
		if (loading) return;

		loading = true;

		try {
			await AuthService.login(credentials);
			window.location.href = "/";
		} catch (err) {
			const error = err instanceof Error ? err.message : "Login failed";

			toaster.error({
				title: "Ошибка",
				description: error
			});
			return;
		} finally {
			loading = false;
		}
	}
</script>

<svelte:head>
	<title>Войти - Folkers</title>
</svelte:head>

<!-- Centering Div -->
<div class="flex min-h-screen items-center justify-center">
	<!-- Skeleton UI Card -->
	<div
		class="dividy-y block max-w-md divide-surface-200-800 overflow-hidden card border-[1px] border-surface-200-800 preset-filled-surface-100-900"
	>
		<!-- Header -->
		<header>
			<img src={bannerSrc} alt="Banner" class="aspect-[21/9]" />
		</header>

		<!-- Article -->
		<article class="space-y-4 p-5">
			<div>
				<h2 class="h4">Вход в аккаунт</h2>
			</div>

			<!-- Login Form -->
			<form on:submit|preventDefault={handleLogin} class="space-y-8">
				<!-- Username Input -->
				<label class="label">
					<span class="label-text">Имя пользователя:</span>
					<input
						type="text"
						bind:value={credentials.username}
						placeholder="Введите логин..."
						required
						disabled={loading}
						class="input border-1"
					/>
				</label>

				<!-- Password Input -->
				<label class="label">
					<span class="label-text">Пароль:</span>
					<input
						type="text"
						bind:value={credentials.password}
						placeholder="Введите пароль..."
						required
						disabled={loading}
						class="input border-1"
					/>
				</label>

				<!-- Confirm Button -->
				<div class="flex justify-center">
					<button disabled={loading} class="btn preset-outlined-surface-500">
						<span>{loading ? "Входим..." : "Войти"}</span>
						<KeyRound size={18} />
					</button>
				</div>
			</form>
		</article>
	</div>
</div>

<!-- Skeleton UI Toaster -->

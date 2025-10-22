<script lang="ts">
	import { KeyRound } from "@lucide/svelte";
	import { toaster } from "$lib/stores/toaster";

	import { AuthService } from "$lib/services/auth.service";
	import { ApiClientError } from "$lib/api/error";
	import type { LoginCredentials } from "$lib/types/auth";

	const bannerSrc = "/banner.png";

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
			const error = err instanceof ApiClientError ? err.status : -1;

			console.error(error);

			toaster.error({
				title: "Ошибка",
				description: describeError(error)
			});
			return;
		} finally {
			loading = false;
		}
	}

	function describeError(code: number | undefined): string {
		const defaultErrorMessage = "Возникла неизвестная ошибка";

		const messages: Record<number, string> = {
			404: "Указанный пользователь не найден",
			401: "Введён неверный пароль",
			500: "Возникла ошибка на стороне сервера"
		};

		return messages[code ?? -1] ?? defaultErrorMessage;
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
			<form on:submit={handleLogin} class="space-y-8">
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
						type="password"
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
						<span>Войти</span>
						<KeyRound size={18} />
					</button>
				</div>
			</form>
		</article>
	</div>
</div>

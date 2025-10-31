export class ApiClientError extends Error {
	constructor(
		message: string,
		public status?: number,
		public code?: string
	) {
		super(message);
		this.name = "ApiClientError";
	}

	describe(): string {
		const messages: Record<number, string> = {
			401: "Ошибка авторизации",
			403: "Ошибка прав доступа.\nВозможно у вас недостаточно прав для выполнения данной операции",
			404: "Ничего не найдено",
			409: "Произошёл конфликт, возможно данный контент уже существует",
			500: "Неизвестная ошибка на стороне сервера"
		};

		return messages[this.status ?? -1] ?? `Неизвестная ошибка c кодом ${this.status ?? -1}`;
	}
}

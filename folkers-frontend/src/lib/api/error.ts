export class ApiClientError extends Error {
  constructor(
    message: string,
    public status?: number,
    public code?: string
  ) {
    super(message);
    this.name = 'ApiClientError';
  }

  describe(): string {
    const messages: Record<number, string> = {
      401: 'Ошибка авторизации',
      403: 'Ошибка прав доступа',
      404: 'Не найдено',
      409: 'Запись уже существует',
      500: 'Ошибка на стороне сервера'
    };

    return messages[this.status ?? -1] ?? 'Неизвестная ошибка';
  }
}

FROM oven/bun:1
WORKDIR /app

COPY bun.lock package.json ./
RUN bun install --frozen-lockfile --production

COPY . .
RUN bun run build

EXPOSE 4173

ENTRYPOINT ["bun", "run", "preview", "--host", "0.0.0.0", "--port", "4173"]

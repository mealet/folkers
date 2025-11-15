# Folkers
English | [Русский](README.ru-RU.md)

Folkers - a private web app for easy people information management. <br/>
It provides fast and intuitive backend API, beautiful UI and safe access.

Core interaction is based on 3 main roles: _watcher_, _editor_, _admin_. Each one have it's own permissions and specifications:
- **Watcher.** Basic user with no editing permissions, only records view mode on main page.
- **Editor.** Can create new records and edit them (only owned).
- **Admin.** Can view, create, edit records (even not owned), create, edit and delete users (only owned).

Besides that Folkers system have extra hidden role: _static admin_. Static admin has _admin_ main role and can do literally everything and it cannot be deleted/edited.
Static admin user can be configured in environment variables (see below).

**⚠️ Project language is Russian (NOT related to politics or something like that) due application targeted users.**

## Features
- **🚀 Fast.** Everything works fast out of the box.
- **🔥 Convenient.** Simple user interface with clear instructions.
- **🧩 Modern.** It is built using modern stack of Rust backend and SvelteKit frontend.
- **✔️ Safe.** Your data is protected by JWT authorization.

## Development Stack
> [!NOTE]
> **Backend:**
> - Language: `Rust`
> - Web Framework: `Axum`
> - Authorization: `Json Web Token`
> - Database: `SurrealDB`
> - Hashing Algorithm: `argon2`
>
> **Frontend:**
> - Production Server: `Nginx`
> - Framework: `SvelteKit`
> - Language: `TypeScript`
> - Runtime: `Bun`
> - Components: `Skeleton UI`
> - Markdown Compiler: `Mdsvex`

## Setup
1. Clone the Github Repository:
```command
git clone https://github.com/mealet/folkers
cd folkers
```
2. Install [Docker](https://www.docker.com/) from official site.
3. Setup environment variables:
```env
FOLKERS_JWT_SECRET = # secret string for jwt tokens
FOLKERS_BASE64_SALT = # base64 encoded salt for hash
FOLKERS_UPLOADS_DIR = # path to directory with uploaded media (optional)

FOLKERS_DB_USERNAME = # database username (default: root) (HIGHLY RECOMMENDED TO CHANGE)
FOLKERS_DB_PASSWORD = # database password (default: root) (HIGHLY RECOMMENDED TO CHANGE)
FOLKERS_DB_NAMESPACE = # database namespace (surrealdb) (default: folkers)
FOLKERS_DB_DATABASE = # database base name (surrealdb) (default: folkers)
FOLKERS_DB_ENDPOINT = # database endpoint (default: localhost:8080, DO NOT CHANGE IF YOU DONT KNOW)

FOLKERS_STATIC_ADMIN_USERNAME = # admin that will be created every start (default: mealet)
FOLKERS_STATIC_ADMIN_PASSWORD = # static admin password (default: mealet)
```
4. Build images:
```command
docker compose build
```
5. Run the application:
```command
docker compose up -d # remove this flag if you want to use interactive mode
```
6. Open http://localhost in browser

## License
The project is licensed under the Apache 2.0 license. <br />
For more information see [LICENSE](LICENSE)

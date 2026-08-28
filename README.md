# Leptos Todo

A minimal full-stack Rust todo application using:

- Leptos 0.8.x (SSR + hydration)
- Axum 0.8.x
- SQLx 0.9.x
- SQLite
- Tailwind CSS 4.3.3
- cargo-leptos 0.3.7+

The backend exposes CRUD operations as Leptos server functions. A shared `SqlitePool` is injected into Leptos server context, migrations are embedded with `sqlx::migrate!()`, and `ActionForm` keeps mutations progressively enhanced.

## Run

Prerequisites: a current stable Rust toolchain and `cargo-leptos`.

```bash
cargo install cargo-leptos --version 0.3.7 --locked
rustup target add wasm32-unknown-unknown
cp .env.example .env
cargo leptos watch
```

Open <http://127.0.0.1:3000>.

The app creates `todo.db` automatically and runs migrations at startup.

## Production build

```bash
cargo leptos build --release
LEPTOS_OUTPUT_NAME=leptos-todo ./target/server/release/leptos-todo
```

For deployment, set `DATABASE_URL` explicitly to a writable SQLite path, for example:

```bash
DATABASE_URL=sqlite:/var/lib/leptos-todo/todo.db
```

## Project structure

```text
leptos-todo/
├── .env.example
├── .gitignore
├── Cargo.toml
├── README.md
├── build.rs
├── rust-toolchain.toml
├── migrations/
│   └── 0001_create_todos.sql
├── public/
│   └── robots.txt
├── src/
│   ├── app.rs
│   ├── lib.rs
│   └── main.rs
└── style/
    └── tailwind.css
```

## Why this shape

- **Stable Leptos:** uses the current stable 0.8 line rather than a 0.9 pre-release.
- **Single package:** cargo-leptos builds the server binary with `ssr` and the WASM library with `hydrate`.
- **Shared pool:** the Axum/Leptos integration injects one cloneable SQLx pool into server-function context instead of reconnecting on every request.
- **Embedded migrations:** `sqlx::migrate!()` runs migrations on startup; `build.rs` ensures migration changes invalidate Cargo builds.
- **Progressive enhancement:** `ActionForm` works through Leptos server actions and supports normal form semantics.
- **Tailwind v4:** CSS-first `@import "tailwindcss"` plus `@source`; no JavaScript config or package manager is needed for this project.
- **No compile-time DB requirement:** this minimal sample uses `query_as` with `FromRow`, so building does not require a live SQLite database or checked-in SQLx offline metadata.

## References used

Official/current references consulted for this implementation:

- Leptos docs: https://docs.rs/leptos/latest/leptos/
- Leptos Axum integration: https://docs.rs/leptos_axum/latest/leptos_axum/
- Leptos Book, Axum extractors/context: https://book.leptos.dev/server/25_server_functions.html
- Official Leptos Axum starter: https://github.com/leptos-rs/start-axum
- Official Leptos examples, including SQLite/Axum todo: https://github.com/leptos-rs/leptos/tree/main/examples
- SQLx docs: https://docs.rs/sqlx/latest/sqlx/
- SQLx migrations: https://docs.rs/sqlx/latest/sqlx/macro.migrate.html
- cargo-leptos: https://github.com/leptos-rs/cargo-leptos
- Tailwind CSS v4 docs: https://tailwindcss.com/docs/installation

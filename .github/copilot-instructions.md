# Unknown Server - AI Coding Assistant Instructions

## Architecture Overview

This is a multi-workspace Rust project with an actor-based backend (`unknown-server`), a distributed actor system (`unknown-actor`), and a static frontend (`unknown-web`). The system follows a modular architecture where the web server integrates with an actor pool for distributed processing.

**Key Components:**

- `unknown-server`: Axum-based web server with PostgreSQL, Redis, session management, and authentication
- `unknown-actor`: Kameo-based actor system with actor pooling for distributed computation
- `unknown-web`: Parcel-bundled static frontend using TailwindCSS, DaisyUI, and HTMX

## Development Environment Setup

**Essential Services (Docker Compose):**

```bash
docker compose up -d  # Starts PostgreSQL, Redis/Valkey, RedisInsight, Adminer
```

**Development Tools (mise-managed):**

- Rust toolchain with `sqlx-cli` for database migrations
- Node.js + pnpm for frontend builds
- AWS CLI for S3/Garage integration

**Configuration:**

- Copy `example.env` to `.env` for local development
- Uses Figment for layered config: files (TOML/YAML) → environment variables
- Database migrations in `unknown-server/migrations/` auto-run on startup

## Key Patterns & Conventions

**Backend Architecture:**

- **State Management**: `AppState` contains shared resources (PgPool, Redis Pool, Actor Pool, Jinja Environment)
- **Authentication**: Uses `axum-login` with Redis-backed sessions, password hashing via `password-auth`
- **Database**: SQLx with compile-time query checking, PostgreSQL with UUID primary keys
- **Actor Integration**: Actor pool abstraction via `unknown_actor_lib::pool` for distributed processing
- **Template Rendering**: MiniJinja with embedded templates via `minijinja_embed::load_templates!`

**Frontend Architecture:**

- **Build System**: Parcel v2 with automatic dependency detection
- **Styling**: TailwindCSS v4 + DaisyUI components, PostCSS processing
- **Interactivity**: HTMX for server-side rendering with minimal JavaScript
- **Components**: PostHTML includes for component reuse (`<include src="...">`)

**Actor System:**

- **Pool Management**: `ActorPool<ServerActor>` with configurable pool size and mailbox capacity
- **Message Patterns**: `Dispatch` for single-actor messages, `Broadcast` for pool-wide messages
- **Simplified Architecture**: No longer uses feature flags - all functionality available by default

## Development Workflows

**Task Management:**

- **Prefer mise tasks**: Use `mise task ls --all` to see available tasks, prefer `mise run <task>` over direct commands
- Available tasks: `garage`, `unknown-web:clean`, `unknown-web:dev`, `unknown-web:build`, `unknown-server:dev`, `unknown-server:build`
- Tasks are defined in `mise.toml` files across workspaces for standardized project commands

**Backend Development:**

```bash
# Database operations
sqlx migrate add <name>           # Create new migration
sqlx migrate run                  # Apply migrations (also auto-runs on server start)

# Server startup
mise run unknown-server:dev      # Preferred: Run backend in debug mode
cargo run --bin unknown-server-web  # Alternative: Direct cargo command
```

**Frontend Development:**

```bash
# Development and builds
mise run unknown-web:dev         # Preferred: Development server with hot reload
mise run unknown-web:build       # Preferred: Production build to dist/
mise run unknown-web:clean       # Clean .parcel-cache and dist/ directories
```

**Development Proxy:**

- `.proxyrc` in `unknown-web/` proxies `/api` requests to backend at `http://localhost:3000`
- Enables full-stack development: frontend dev server handles static assets, backend serves API
- Run both `mise run unknown-web:dev` (frontend) and `mise run unknown-server:dev` (backend) simultaneously

**Asset Integration:**

- Frontend assets embedded in backend binary via `memory-serve` and `load_assets!` macro
- Build process: `unknown-web/dist/` → `unknown-server/assets/` → embedded in binary

## Integration Points

**Database Layer:**

- PostgreSQL with SQLx for type-safe queries
- Redis for session storage and caching
- UUID-based primary keys with timestamptz audit fields

**Actor Communication:**

- HTTP endpoints trigger actor pool operations via `AppState.actor_pool()`
- Async message passing between web handlers and actor workers
- Actor lifecycle managed by Kameo framework

**Authentication Flow:**

- Session-based auth with Redis persistence
- Signup requires token (configurable via `SIGNUP_TOKEN` env var)
- Password hashing with Argon2 via `password-auth`

**External Services:**

- S3-compatible storage via Garage (self-hosted)
- Metrics exposure via Prometheus integration (`/metrics` endpoint)
- Graceful shutdown handling for both HTTP and actor systems

## Common Gotchas

- **Database URLs**: Use double underscore syntax for nested config: `DATABASE__URL`, `REDIS__URL`
- **Asset Embedding**: Frontend changes require rebuilding backend binary for embedded assets
- **Actor Pool**: Pool initialization is async and requires `wait_for_startup_result()`
- **Sessions**: Redis must be running before server startup (no graceful degradation)
- **Migration Safety**: SQLx compile-time checks require database connection during build

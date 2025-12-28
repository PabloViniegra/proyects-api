# Projects API

A modern, feature-rich REST API for managing code projects with technologies, users, and advanced filtering capabilities. Built with Rust, Axum, and SQLite.

## Features

- **Complete CRUD operations** for projects, technologies, and users
- **Advanced filtering** with search, technology, user, rating, and language filters
- **Pagination support** with configurable page size and metadata
- **Many-to-many relationships** between projects and technologies
- **Many-to-many relationships** with roles between projects and users (owner, contributor, viewer)
- **Rating system** for projects (0.0 - 5.0)
- **Input validation** using the `validator` crate
- **Structured error handling** with custom error types
- **Async/await** with Tokio runtime
- **Type-safe routing** with Axum
- **SQLite database** with SQLx and compile-time query verification
- **OpenAPI/Swagger documentation** with interactive UI
- **Comprehensive test suite** with 23+ unit and integration tests

## Quick Start

### Prerequisites

- Rust 1.82+ (edition 2024 support)
- SQLite 3

### Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd proyects-api
```

2. Copy the environment file:
```bash
cp .env.example .env
```

3. Build and run:
```bash
cargo run
```

The server will start on `http://0.0.0.0:3000` by default.

### Test Data

The database will be automatically populated with test data on first run, including:
- **20 technologies** (Rust, Python, JavaScript, TypeScript, React, Docker, etc.)
- **8 users** with realistic names and emails
- **12 projects** with descriptions, ratings, and relationships
- **Technology-project** relationships
- **User-project** relationships with roles (owner, contributor, viewer)

This seed data is useful for development and testing. To start with an empty database:

1. Delete the seed migration before first run:
```bash
rm migrations/20250107000000_seed_test_data.sql
```

2. Or delete the database file to reset:
```bash
rm proyects.db
cargo run
```

### Accessing the API

- **Health Check**: http://localhost:3000/health
- **Swagger UI**: http://localhost:3000/swagger-ui/
- **OpenAPI Spec**: http://localhost:3000/api-docs/openapi.json

## API Endpoints

### Health Check
- `GET /health` - Health check endpoint

### Projects
- `GET /projects` - List all projects with advanced filtering and pagination
- `GET /projects/{id}` - Get a specific project with related technologies and users
- `POST /projects` - Create a new project with optional technologies and users
- `PUT /projects/{id}` - Update a project and its relations
- `DELETE /projects/{id}` - Delete a project (cascades to relations)

### Technologies
- `GET /technologies` - List all technologies
- `POST /technologies` - Create a new technology (unique name constraint)

### Users
- `GET /users` - List all users
- `POST /users` - Create a new user (unique email constraint)

## Advanced Filtering & Pagination

The `GET /projects` endpoint supports extensive query parameters:

```bash
GET /projects?search=rust&tech=rust&user_id=xxx&min_rating=4.0&sort=rating&order=desc&page=1&page_size=10
```

### Available Query Parameters

| Parameter | Type | Description | Default |
|-----------|------|-------------|---------|
| `search` | String | Search in name and description (SQL LIKE) | - |
| `tech` / `technology` | String | Filter by technology name | - |
| `user_id` | UUID | Filter by user ID | - |
| `min_rating` | Float | Minimum rating (0.0-5.0) | - |
| `max_rating` | Float | Maximum rating (0.0-5.0) | - |
| `language` | String | Filter by programming language | - |
| `sort` | String | Sort field: `name`, `created_at`, `updated_at`, `rating` | `created_at` |
| `order` | String | Sort order: `asc`, `desc` | `desc` |
| `page` | u32 | Page number (min: 1) | 1 |
| `page_size` | u32 | Items per page (min: 1, max: 100) | 10 |

### Paginated Response Format

```json
{
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "name": "My Project",
      "description": "A sample project",
      "repository_url": "https://github.com/user/repo",
      "language": "Rust",
      "rating": 4.5,
      "created_at": "2024-01-15T10:30:00Z",
      "updated_at": "2024-01-15T10:30:00Z"
    }
  ],
  "pagination": {
    "page": 1,
    "page_size": 10,
    "total_items": 45,
    "total_pages": 5
  }
}
```

## API Usage Examples

### 1. Create a Technology

```bash
curl -X POST http://localhost:3000/technologies \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Rust",
    "description": "A systems programming language"
  }'
```

### 2. Create a User

```bash
curl -X POST http://localhost:3000/users \
  -H "Content-Type: application/json" \
  -d '{
    "name": "John Doe",
    "email": "john@example.com"
  }'
```

### 3. Create a Project with Relations

```bash
curl -X POST http://localhost:3000/projects \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Awesome Rust Project",
    "description": "A project using Rust and Axum",
    "repository_url": "https://github.com/user/awesome-project",
    "language": "Rust",
    "rating": 4.5,
    "technology_ids": ["<tech-uuid>"],
    "user_ids": ["<user-uuid>"]
  }'
```

**Note**: The first user in `user_ids` becomes the owner, others become contributors.

### 4. Get a Project with Relations

```bash
curl http://localhost:3000/projects/<project-id>
```

**Response:**
```json
{
  "id": "...",
  "name": "Awesome Rust Project",
  "description": "...",
  "repository_url": "...",
  "language": "Rust",
  "rating": 4.5,
  "created_at": "...",
  "updated_at": "...",
  "technologies": [
    {
      "id": "...",
      "name": "Rust",
      "description": "A systems programming language",
      "created_at": "..."
    }
  ],
  "users": [
    {
      "id": "...",
      "name": "John Doe",
      "email": "john@example.com",
      "created_at": "...",
      "role": "owner"
    }
  ]
}
```

### 5. Advanced Filtering Examples

```bash
# Search for Rust projects with rating >= 4.0
curl "http://localhost:3000/projects?search=rust&min_rating=4.0&sort=rating&order=desc"

# Filter by technology
curl "http://localhost:3000/projects?tech=axum"

# Filter by user
curl "http://localhost:3000/projects?user_id=<uuid>"

# Combine filters with pagination
curl "http://localhost:3000/projects?language=rust&min_rating=4.0&page=2&page_size=5"
```

### 6. Update a Project

```bash
curl -X PUT http://localhost:3000/projects/<project-id> \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Updated Project Name",
    "rating": 5.0,
    "technology_ids": ["<new-tech-uuid>"],
    "user_ids": ["<new-user-uuid>"]
  }'
```

**Note**: Providing `technology_ids` or `user_ids` replaces existing associations.

## Database Schema

### Tables

**projects**
- `id` (TEXT, PK) - UUID as string
- `name` (TEXT, NOT NULL)
- `description` (TEXT, NOT NULL)
- `repository_url` (TEXT, NOT NULL)
- `language` (TEXT, NOT NULL)
- `rating` (REAL, nullable) - Check constraint: 0.0 <= rating <= 5.0
- `created_at` (TEXT, NOT NULL) - ISO 8601 timestamp
- `updated_at` (TEXT, NOT NULL) - ISO 8601 timestamp

**technologies**
- `id` (TEXT, PK) - UUID as string
- `name` (TEXT, NOT NULL, UNIQUE)
- `description` (TEXT, nullable)
- `created_at` (TEXT, NOT NULL)

**users**
- `id` (TEXT, PK) - UUID as string
- `name` (TEXT, NOT NULL)
- `email` (TEXT, NOT NULL, UNIQUE)
- `created_at` (TEXT, NOT NULL)

**project_technologies** (pivot table)
- `project_id` (TEXT, FK → projects.id, ON DELETE CASCADE)
- `technology_id` (TEXT, FK → technologies.id, ON DELETE CASCADE)
- `created_at` (TEXT, NOT NULL)
- Primary Key: (project_id, technology_id)

**project_users** (pivot table with role)
- `project_id` (TEXT, FK → projects.id, ON DELETE CASCADE)
- `user_id` (TEXT, FK → users.id, ON DELETE CASCADE)
- `role` (TEXT, NOT NULL) - Check constraint: 'owner', 'contributor', 'viewer'
- `created_at` (TEXT, NOT NULL)
- Primary Key: (project_id, user_id)

### Indexes

- `idx_projects_name` - Fast project name lookups
- `idx_projects_language` - Language filtering
- `idx_projects_rating` - Rating filtering and sorting
- `idx_technologies_name` - Technology name lookups
- `idx_users_email` - User email lookups
- `idx_project_technologies_project` - Project → technologies queries
- `idx_project_technologies_technology` - Technology → projects queries
- `idx_project_users_project` - Project → users queries
- `idx_project_users_user` - User → projects queries
- `idx_project_users_role` - Role filtering

## Development

### Run Tests

```bash
# Run all tests (23+ tests)
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_create_project

# Run doc tests
cargo test --doc

# Run sequentially
cargo test -- --test-threads=1
```

### Code Quality

```bash
# Run linter (no warnings!)
cargo clippy

# Format code
cargo fmt

# Check formatting
cargo fmt -- --check

# Check without building
cargo check

# Build release version
cargo build --release
```

### Database Migrations

Migrations are automatically applied on startup. Files in `/migrations/`:

1. `20250101000000_create_projects_table.sql` - Initial projects table
2. `20250102000000_add_rating_to_projects.sql` - Add rating column
3. `20250103000000_create_technologies_table.sql` - Technologies table
4. `20250104000000_create_users_table.sql` - Users table
5. `20250105000000_create_project_technologies_table.sql` - Project-Technology relations
6. `20250106000000_create_project_users_table.sql` - Project-User relations with roles

## Project Structure

```
proyects-api/
├── migrations/                # Database migrations (6 files)
├── src/
│   ├── error/                 # Error handling
│   │   └── mod.rs            # Custom error types
│   ├── handlers/              # Request handlers
│   │   ├── mod.rs
│   │   ├── projects.rs       # Project endpoints with filtering
│   │   ├── technologies.rs   # Technology endpoints
│   │   └── users.rs          # User endpoints
│   ├── models/                # Data models
│   │   ├── mod.rs
│   │   ├── pagination.rs     # Pagination DTOs
│   │   ├── project.rs        # Project models and relations
│   │   ├── technology.rs     # Technology model
│   │   └── user.rs           # User model and roles
│   ├── routes/                # Route configuration
│   │   └── mod.rs            # Router with OpenAPI
│   ├── state/                 # Application state
│   │   └── mod.rs            # Database connection pool
│   ├── lib.rs                 # Library root
│   └── main.rs                # Binary entry point
├── .env.example               # Environment variables template
├── Cargo.toml                 # Dependencies
└── README.md                  # This file
```

## Technologies & Dependencies

### Core
- **Axum 0.8** - Modern web framework
- **Tokio 1.x** - Async runtime (full features)
- **SQLx 0.8** - Async SQL toolkit with SQLite
- **Serde 1.x** - Serialization/deserialization

### Utilities
- **UUID 1.x** - UUID generation (v4)
- **Chrono 0.4** - Date/time handling
- **Validator 0.19** - Input validation with derive macros
- **Thiserror 2.x** - Error handling

### Web
- **Tower 0.5** - Middleware
- **Tower-HTTP 0.6** - HTTP middleware (CORS, tracing)

### Documentation
- **Utoipa 5.x** - OpenAPI code generation
- **Utoipa-Swagger-UI 9.x** - Interactive API docs

### Development
- **Tracing 0.1** - Structured logging
- **Tracing-subscriber 0.3** - Log formatting
- **Dotenvy 0.15** - Environment variables

## Configuration

All configuration via `.env` file (see `.env.example`):

```bash
# Database
# Format: sqlite:file.db?mode=rwc (rwc = read/write/create)
DATABASE_URL=sqlite:proyects.db?mode=rwc

# Server
HOST=0.0.0.0
PORT=3000

# Logging
RUST_LOG=proyects_api=debug,tower_http=debug,axum=trace,sqlx=info
```

## Testing

The project includes comprehensive tests:
- **Model tests** - Validation, constructors
- **Handler tests** - CRUD operations, relations
- **Integration tests** - End-to-end scenarios
- **Doc tests** - Example code in documentation

All tests use in-memory SQLite databases for isolation.

## Best Practices Implemented

- Type-safe routing with Axum
- Compile-time query verification with SQLx
- Zero-cost abstractions
- Explicit error handling (no panics)
- Iterator patterns over manual loops
- Proper async/await usage
- Comprehensive input validation
- Database connection pooling
- Proper cascading deletes
- Unique constraints
- Indexed foreign keys

## License

MIT

## Contributing

Contributions welcome! Please ensure:
- All tests pass: `cargo test`
- Code formatted: `cargo fmt`
- No clippy warnings: `cargo clippy`
- Add tests for new features
- Update documentation

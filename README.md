# Climate Scenarios Database

A comprehensive web application for exploring and filtering climate scenarios from various publishers. The application follows the 12-factor app methodology.

## Architecture

- **Frontend**: Svelte/SvelteKit with TypeScript
- **Backend**: Rust API with Actix-web
- **Database**: PostgreSQL

## Features

- Browse climate scenarios from various publishers
- Filter scenarios by publisher, region, stakeholder, sector, type, temperature target, and year
- View detailed information about each scenario
- Responsive design that works on all devices

## 12-Factor Compliance

This application follows the [12-factor app](https://12factor.net/) methodology:

1. **Codebase**: One codebase tracked in version control
2. **Dependencies**: Explicitly declared and isolated
3. **Config**: Stored in environment variables
4. **Backing services**: Treated as attached resources
5. **Build, release, run**: Strict separation of build and run stages
6. **Processes**: Application runs as stateless processes
7. **Port binding**: Services exposed via port binding
8. **Concurrency**: Scale via the process model
9. **Disposability**: Fast startup and graceful shutdown
10. **Dev/prod parity**: Development, staging, and production as similar as possible
11. **Logs**: Treated as event streams
12. **Admin processes**: Run admin tasks as one-off processes

## Development Setup

### Prerequisites

- Docker and Docker Compose
- Rust (for local development of the API)
- Node.js (for local development of the frontend)

### Running with Docker

The easiest way to run the entire application stack is with Docker Compose:

```bash
docker-compose up
```

This will start the database, API, and frontend services. The application will be accessible at http://localhost:3000.

### Local Development

#### API (Rust)

```bash
cd api
# Install dependencies and run
cargo run
```

#### Frontend (Svelte)

```bash
cd frontend
# Install dependencies
npm install
# Start development server
npm run dev
```

## API Documentation

The API provides the following endpoints:

- `GET /api/health`: Health check endpoint
- `GET /api/scenarios`: List all scenarios with optional filter parameters
- `GET /api/scenarios/:id`: Get detailed information about a specific scenario
- `GET /api/scenarios/filters/options`: Get available filter options

## Database Schema

The database includes the following main tables:
- `scenarios`: Core climate scenario information
- `publishers`: Organizations that publish scenarios
- `regions`: Geographic regions relevant to scenarios
- `stakeholders`: Groups interested in or affected by scenarios
- `sectors`: Economic sectors addressed in scenarios

## License

This project is open source and available under the [MIT License](LICENSE).
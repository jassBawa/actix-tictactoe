# Tic-Tac-Toe Backend

A tic-tac-toe game backend built with Rust and Actix-web. Supports real-time multiplayer gameplay via WebSockets.

## Features

- User authentication with JWT
- Game creation and joining
- Real-time gameplay via WebSockets
- PostgreSQL database for persistence
- Redis for pub/sub messaging

## Project Structure

- `api/` - HTTP API server with REST endpoints and WebSocket handlers
- `db/` - Database models and queries
- `ws/` - WebSocket connection management and game logic

## Prerequisites

- Rust (latest stable)
- PostgreSQL
- Redis

## Setup

1. Clone the repository
2. Make sure PostgreSQL and Redis are running
3. Copy `.env.example` to `.env` and update with your values:
   ```bash
   cp .env.example .env
   ```
4. Run database migrations
5. Start the server:
   ```bash
   cargo run --bin tic-tac
   ```

The server runs on `http://127.0.0.1:8080` by default.

## API Endpoints

- `POST /auth/register` - Register a new user
- `POST /auth/login` - Login and get JWT token
- `POST /game/create` - Create a new game (requires auth)
- `POST /game/join` - Join an existing game (requires auth)
- `POST /game/move` - Make a move (requires auth)
- `WS /ws` - WebSocket connection for real-time gameplay
- `GET /ping` - Health check

## Development

This is a Cargo workspace with three crates:
- `tic-tac` (api) - Main application
- `db` - Database layer
- `ws` - WebSocket layer


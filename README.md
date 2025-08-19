# One Piece gRPC Challenge (Rust + tonic)

Welcome to the **One Piece gRPC Technical Challenge**!
Your task is to design and implement a gRPC service (or services) that manages **Characters**, **Devil Fruits**, and **Crews** from the One Piece universe. The challenge is designed to test your ability to build scalable, reliable, and well-structured backend systems in Rust.

---

## 📌 Objectives

- Implement **CRUD** operations and relationships for:
  - Characters (Luffy, Zoro, etc.)
  - Devil Fruits (Gomu Gomu no Mi, etc.)
  - Crews (Straw Hat Pirates, etc.)
- Support **filtering, sorting, and cursor-based pagination**.
- Add **streaming APIs**:
  - Server-streaming for searching characters.
  - Bidirectional-streaming for bulk imports.
- Ensure **scalability and observability**:
  - Stateless service
  - Structured logging
  - Graceful shutdown
  - (Bonus) metrics & tracing

---

## 🛠 Tech Requirements

- **Language**: Rust (stable)
- **gRPC**: [`tonic`](https://github.com/hyperium/tonic) + [`prost`](https://github.com/tokio-rs/prost)
- **Async runtime**: `tokio`
- **Persistence**:
  - In-memory HashMap (minimum)
  - Postgres/SQLite with `sqlx` (bonus)
- **Optional extras**:
  - Redis cache
  - Prometheus metrics
  - API key authentication

---

## 📂 Minimum Data Model

### Character
- `id` (uuid/string)
- `name` (string, unique)
- `bounty` (int64)
- `crew_id` (string, optional)
- `devil_fruit_id` (string, optional)
- `roles` (array of strings)

### DevilFruit
- `id` (uuid/string)
- `name` (string, unique)
- `type` (enum: PARAMECIA, LOGIA, ZOAN)
- `description` (string)

### Crew
- `id` (uuid/string)
- `name` (string, unique)
- `ship_name` (string)
- `affiliation` (string)

**Relations**:
- A character may belong to a crew.
- A character may have a devil fruit.
- A devil fruit may only be linked to one character.

---

## 🚀 Getting Started

### 1. Install Dependencies
```bash
cargo add tonic prost tokio uuid tracing tracing-subscriber
cargo add sqlx --features runtime-tokio-rustls,postgres,macros,uuid,time
cargo add anyhow thiserror

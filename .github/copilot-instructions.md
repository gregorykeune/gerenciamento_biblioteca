## Purpose
Help AI coding agents become productive in this Rust "biblioteca" project by describing the architecture, developer workflows, and repository-specific conventions.

## Quick start (build & run)
- Use the workspace root (where `Cargo.toml` is located).
- Typical build/test commands (PowerShell):

```
cargo check
cargo build
cargo run
```

Note: the code references `uuid`, `serde`, `serde_json`, and `chrono` in source comments; verify `Cargo.toml` contains those dependencies before running — running `cargo check` will show missing deps.

## Big-picture architecture
- This repo implements a small library/app around a `Biblioteca` type that manages `Livro`, `Usuario`, and `Emprestimo` entities.
- The core logic lives in `src/main.rs` (contains the `Biblioteca` implementation and definitions), while a `src/biblioteca/` directory exists with module files (`mod.rs`, `livros.rs`, `usuarios.rs`, `emprestimos.rs`). Currently the folder modules are mostly placeholders; `src/main.rs` contains the authoritative implementations.
- Persistence: `Biblioteca::carregar` / `Biblioteca::salvar` serialize a `DadosPersistencia` struct to a JSON file on disk (path supplied to `Biblioteca::nova` / `carregar`). This is the primary cross-cutting integration point (filesystem <-> in-memory HashMaps).

## Key files to inspect or edit
- `Cargo.toml` — project manifest; check/add dependencies (uuid, serde, serde_json, chrono).
- `src/main.rs` — contains the `Biblioteca` type, entities (Livro, Usuario, Emprestimo), error enum `ErroBiblioteca`, and persistence logic. Many behaviours and examples come from here.
- `src/biblioteca/mod.rs` — module re-export / place for refactor if splitting the large `main.rs` into modules.
- `src/biblioteca/*.rs` — intended per-entity modules (present but empty). If you split code out of `main.rs`, put `Livro` in `livros.rs`, `Usuario` in `usuarios.rs`, etc., and keep names and serde derives intact.

## Project-specific conventions and patterns
- Naming is Portuguese: `Livro`, `Usuario`, `Emprestimo`, `ErroBiblioteca`. Keep Portuguese identifiers consistent when adding code.
- Domain logic is synchronous and in-memory. Persistence is explicit via `salvar()`; methods that mutate state do not auto-save. Tests or higher-level code should call `salvar()` when persistence is required.
- IDs use `uuid::Uuid`. Entities expose `.id()` getters that return `Uuid` (by value) and other simple accessors.
- Status enums: `StatusLivro` and `StatusEmprestimo` are used to track lifecycle; prefer these enums over ad-hoc booleans.

## Common tasks (examples)
- Create/load library and add a book (from Rust code):

```rust
use biblioteca::Biblioteca;
let mut b = Biblioteca::nova("data.json".into());
let id = b.adicionar_livro("Título".into(), "Autor".into(), 2020);
b.salvar()?; // explicit persistence
```

- Realizar empréstimo:

```rust
let emprestimo_id = b.realizar_emprestimo(id_livro, id_usuario)?;
```

## Integration points & external deps
- Filesystem: JSON persistence (read/write via `serde_json`).
- Crate deps used in code: `uuid`, `serde` (+ derive), `serde_json`, `chrono`.
- There are no network integrations.

## Troubleshooting notes for agents
- If `cargo check` fails with missing crates, add them to `Cargo.toml` and run `cargo update` / `cargo check` again.
- If you plan to split `src/main.rs` into modules, keep public API signatures unchanged (methods like `adicionar_livro`, `realizar_emprestimo`, `salvar`) to minimize test churn.

## Style & tests
- No tests are present — when adding tests, follow the small-team/simple style: unit tests close to the implementation (e.g., `#[cfg(test)]` modules in the same file) and keep test names in Portuguese to match domain vocabulary.

## Last notes
- Primary source of truth today: `src/main.rs`. `src/biblioteca/` exists as a scaffold for future refactors. If you update behaviour, update both the refactored modules and the core `main.rs` usage or move the authoritative code into the modules and keep `main.rs` minimal.

If anything above is unclear or you'd like the instructions to include more examples (tests, refactor checklist), tell me which area to expand and I'll iterate.

# RusticJourney

A personal learning archive documenting a hands-on journey through Rust — from ownership basics to async runtimes, design patterns, and interview-level system design.

---

## Quick start

```bash
# Build everything in the workspace
cargo build --workspace

# Run all tests across every crate
cargo test --workspace

# Lint the entire workspace (warnings as errors)
cargo clippy --workspace -- -D warnings

# Format all source files
cargo fmt --all

# The DSA sub-folder has its own nested workspace — run it separately
cd a2z-dsa-rust && cargo test
```

---

## Learning path

The crates follow a natural progression. Work through them roughly in this order:

| Stage | Crate / Folder | Focus |
|---|---|---|
| 1 | `RustBook/` | Syntax, ownership, structs, enums, error handling |
| 2 | `rust101-ztm-exercises/` | Solidifying fundamentals with 33 exercises |
| 3 | `lifetimes/` | Lifetime annotations and borrow checker rules |
| 4 | `oops-in-rust/` | Traits, encapsulation, composition over inheritance |
| 5 | `math_app/` | Module system and visibility |
| 6 | `macros/` | Writing a procedural attribute macro |
| 7 | `multithreading/` | `std::thread`, `Mutex`, `Condvar`, channels |
| 8 | `async-rust/` | `async`/`await`, Tokio tasks, `reqwest` |
| 9 | `rust-design-patterns/` | Builder, Observer, Type-state (with tests) |
| 10 | `tiny-rust-projects/` | End-to-end CLI and network tools |
| 11 | `rust-crypto/` | Ed25519 signatures, base64, hex encoding |
| 12 | `tree-of-space/` | Raw pointers, `unsafe`, locking trees |
| 13 | `a2z-dsa-rust/` | Arrays, binary search |
| 14 | `rust-leetcode-exercises/` | LeetCode-style algorithm problems |
| 15 | `interview-questions/` | 79 Rust interview problems |
| 16 | `machine-coding-questions/` | 16 system-design problems |

---

## Repository structure

```
RusticJourney/
├── Cargo.toml                        # root workspace — one build for all crates
├── rustfmt.toml                      # shared formatting rules
├── .clippy.toml                      # shared lint configuration
│
├── RustBook/                         # "The Rust Programming Language" exercises
│   ├── 2_guessing_game/
│   ├── 3_common_programming_concepts/
│   ├── 4_ownership/
│   ├── 5_structures/
│   ├── 6_enums_and_pattern_matching/
│   ├── 7_module_packages/
│   ├── 8_collections/
│   └── 9_error_handling/
│
├── rust101-ztm-exercises/            # Zero-to-Mastery Rust 101 (33 src/bin/ files)
├── lifetimes/                        # lifetime annotation examples
├── oops-in-rust/                     # OOP concepts modelled with traits
├── macros/                           # procedural attribute macro (syn + quote)
├── math_app/                         # module organisation demo
│
├── rust-design-patterns/             # design patterns with tests
│   └── src/bin/
│       ├── builder_pattern.rs        # typed errors, doc comments, #[cfg(test)]
│       ├── observer_pattern.rs       # Tokio broadcast channel
│       └── type_state_pattern.rs     # PhantomData state machine
│
├── async-rust/                       # async/await, Tokio tasks, reqwest client
├── multithreading/                   # threads, Mutex, Condvar, channels
├── tree-of-space/                    # Arc<Mutex<T>>, raw pointers, unsafe
│
├── interview-questions/              # 79 interview problems (src/bin/)
├── machine-coding-questions/         # 16 system-design problems (src/bin/)
│   └── src/bin/
│       ├── stock_exchange.rs         # order-matching engine
│       ├── lru_with_dll.rs           # LRU cache with doubly-linked list
│       ├── two_tier_storage_system.rs
│       ├── kv_store_with_transactions.rs
│       └── ...
├── rust-leetcode-exercises/          # LeetCode solutions
├── rust-cp-templates/                # competitive programming templates
│
├── a2z-dsa-rust/                     # DSA problems (own nested workspace)
│   └── arrays/                       # 19 array problems
│
├── tiny-rust-projects/               # standalone CLI and network tools
│   ├── calculator/                   # expression calculator
│   ├── cat/                          # unix cat clone (clap)
│   ├── grep/                         # unix grep clone (clap)
│   ├── http_server/                  # raw TCP HTTP/1.1 server
│   ├── read-json/                    # serde_json deserialisation
│   ├── write-json/                   # serde_json serialisation
│   └── redis/                        # async Redis server (Tokio)
│
├── rust-crypto/                      # ed25519 signatures, base64, hex
├── RustShortNotes/                   # PDF crash-course notes
└── rust-interview-questions-short-notes/  # quick-reference interview notes
```

---

## Crate highlights

| Crate | Key concepts |
|---|---|
| `rust-design-patterns` | Builder with `thiserror` errors + tests, Observer via Tokio broadcast, Type-state with `PhantomData` |
| `machine-coding-questions` | Stock exchange matcher, LRU cache (raw pointers), two-tier TTL storage, transactional KV store |
| `interview-questions` | 79 problems covering `Arc`, `Mutex`, channels, lifetimes, iterators, unsafe |
| `a2z-dsa-rust/arrays` | 19 array problems (two-pointer, sliding window, prefix sums) |
| `rust-leetcode-exercises` | LeetCode problems — binary search, trees, dynamic programming |
| `async-rust` | Tokio multi-task concurrency, `reqwest` async HTTP |
| `multithreading` | `std::thread`, `Mutex`, `Condvar`, `mpsc` channels |
| `tree-of-space` | Locking spatial tree using `Arc<Mutex<T>>` and raw `*mut` pointers |
| `macros` | Procedural attribute macro built with `syn` and `quote` |
| `rust-crypto` | Ed25519 key generation, signing, verification; base64 and hex encoding |
| `tiny-rust-projects/redis` | Multi-module async TCP server (command parser, in-memory DB, connection handler) |
| `tiny-rust-projects/cat` & `grep` | `clap`-based CLI tools following Unix conventions |

---

## Conventions

- **Error types** — `thiserror` for structured errors; `Result<_, E>` propagated with `?`
- **Derives** — `Debug`, `Clone`, `Default`, `Display` on all domain types
- **Workspace** — shared dependency versions in `[workspace.dependencies]` keep lockfiles coherent
- **Formatting** — `rustfmt.toml`: 100-char lines, `std → external → local` import grouping
- **Tests** — `#[cfg(test)]` modules co-located with the code under test
- **Unsafe** — only used where the standard library offers no alternative, always with a comment explaining why

---

## Resources

- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Learn Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/)
- [Zero To Mastery — Rust 101](https://zerotomastery.io/)
- [LeetCode](https://leetcode.com/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Tokio documentation](https://tokio.rs/)
